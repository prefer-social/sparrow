use crate::db;
use anyhow::{anyhow, Result};
use passwords::PasswordGenerator;
use serde_json::Value;
use spin_sdk::http::{HeaderValue, Method, Request, Response};
use spin_sdk::sqlite::Value as SV;
use std::str;
use url::Url;

pub async fn create_token() -> String {
    let pg = PasswordGenerator {
        length: 64,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: false,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };

    pg.generate_one().unwrap()
}

pub async fn get_current_time_in_rfc_1123() -> String {
    use chrono::{DateTime, Utc};
    let current_time: DateTime<Utc> = Utc::now();
    current_time.format("%a, %d %b %Y %H:%M:%S GMT").to_string()
}

// https://docs.rs/chrono/latest/chrono/format/strftime/index.html
// 2024-02-27T06:17:54Z
pub async fn get_current_time_in_iso_8601() -> String {
    use chrono::{DateTime, Utc};
    let current_time: DateTime<Utc> = Utc::now();
    current_time.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

pub async fn get_actor_url_from_id(id: String) -> Result<String> {
    let a: Vec<&str> = id.as_str().split("@").collect();
    let host = a.last().unwrap().to_string();

    // @seungjin@mstd.seungjin.net -> https://mstd.seungjin.net/@sungjin

    // 1. Call https://mstd.seungjin.net/.well-known/webfinger?resource=acct:seungjin@mstd.seungjin.net

    let webfinger_url =
        format!("https://{host}/.well-known/webfinger?resource=acct:{id}");
    tracing::debug!(webfinger_url);
    let request = Request::builder()
        .method(Method::Get)
        .header("Accept", "application/activity+json")
        .uri(webfinger_url)
        .build();
    let response: Response = spin_sdk::http::send(request).await?;
    let body = str::from_utf8(response.body()).unwrap();
    let body_obj: Value = serde_json::from_str(body).unwrap();
    let a = body_obj.get("links").unwrap();

    for e in a.as_array().unwrap() {
        let l = e.get::<&str>("rel").unwrap().as_str().unwrap();
        if l.eq("self") {
            let ll = e.get::<&str>("href").unwrap().as_str().unwrap();
            return Ok(ll.to_string());
        }
    }

    Err(anyhow!("no url available"))
}

pub async fn get_local_user(id: i64) -> Result<(String, String)> {
    let qr = crate::db::Connection::builder().await.execute(
        "SELECT user.*, signing_key.privateKey FROM user INNER JOIN signing_key ON user.id = signing_key.userId WHERE user.id = ?",
        &[SV::Integer(id)],
    ).await;

    let private_key =
        qr.rows().next().unwrap().get::<&str>("privateKey").unwrap();

    let federation_id = qr
        .rows()
        .next()
        .unwrap()
        .get::<&str>("federationId")
        .unwrap()
        .to_string();

    Ok((federation_id, private_key.to_string()))
}

pub async fn get_privatekey_with_user_name(name: &str) -> Result<String> {
    let qr = db::Connection::builder().await.execute(
    "SELECT privateKey FROM signing_key JOIN user ON user.id = signing_key.userId WHERE user.name = ?", 
    &[SV::Text(name.to_string())]).await;
    let private_key =
        qr.rows().next().unwrap().get::<&str>("privateKey").unwrap();
    Ok(private_key.to_string())
}

pub async fn get_privatekey_with_actor_url(
    actor_url: String,
) -> Result<String> {
    let qr = db::Connection::builder().await.execute(
    "SELECT privateKey FROM signing_key JOIN user ON user.id = signing_key.userId WHERE user.federationId = ?", 
    &[SV::Text(actor_url)]).await;
    let private_key =
        qr.rows().next().unwrap().get::<&str>("privateKey").unwrap();
    Ok(private_key.to_string())
}

pub async fn get_privatekey_with_db_user_id(id: u16) -> Result<String> {
    let qr = db::Connection::builder()
        .await
        .execute(
            "SELECT privateKey FROM signing_key WHERE usedId= ?",
            &[SV::Text(id.to_string())],
        )
        .await;
    let private_key =
        qr.rows().next().unwrap().get::<&str>("privateKey").unwrap();
    Ok(private_key.to_string())
}

pub async fn get_public_key(actor_url_str: &str) -> Result<String> {
    tracing::debug!(actor_url_str);
    let req = Request::builder()
        .method(Method::Get)
        .uri(actor_url_str)
        .header("Accept", "application/activity+json")
        .build();

    // Send the request and await the response
    let resp: Response = spin_sdk::http::send(req).await?;

    let body = String::from_utf8(resp.body().to_vec()).unwrap();
    tracing::debug!("---------");
    tracing::debug!(body);

    let json: serde_json::Value =
        serde_json::from_str(&body).expect("JSON was not well-formatted");

    let status = resp.status();

    tracing::debug!(status);

    if *status != 200u16 {
        // ignore this request.
        //tracing::debug!(status);
        return Err(anyhow::Error::msg("get_public_key not getting 200"));
    }

    // TODO: Get PublicKey and check id with key_id
    let public_key = json.get("publicKey").expect(
        format!("({status:?}) publicKey not found from: {body:?}").as_str(),
    );
    let public_key_owner: &str =
        public_key.get("owner").unwrap().as_str().unwrap();
    let public_key_pem = public_key
        .get("publicKeyPem")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string(); // Keeping '\n' and new line.

    if actor_url_str.to_string() == public_key_owner.to_string() {
        tracing::debug!(public_key_pem);
        return Ok(public_key_pem);
    }
    // TODO: This shoud not return Err but Ok(None)
    Err(anyhow::Error::msg(format!(
        "key_id not matched {} : {}",
        actor_url_str.to_string(),
        public_key_owner.to_string()
    )))
}

pub async fn get_inbox_from_actor(actor: String) -> Result<String> {
    let req = Request::builder()
        .method(Method::Get)
        .uri(actor)
        .header("Accept", "application/activity+json")
        .build();
    let resp: Response = spin_sdk::http::send(req).await?;
    let body = String::from_utf8(resp.body().to_vec()).unwrap();
    let json: serde_json::Value =
        serde_json::from_str(&body).expect("JSON was not well-formatted");
    let a = json.get::<&str>("inbox").unwrap().as_str().unwrap();
    Ok(a.to_string())
}

pub async fn generate_uuid_v7() -> String {
    "ars".to_string()
}

// debug tool
pub async fn see_headers(headers: impl Iterator<Item = (&str, &HeaderValue)>) {
    for header in headers {
        tracing::debug!("{header:?}");
    }
}

pub async fn clean_last_slash_from_url(c: Url) -> String {
    let a = match c.path() {
        "/" => {
            format!("{:?}", &c.to_string()[..c.to_string().len() - 1])
        }
        _ => c.to_string(),
    };
    println!("{}", a);
    a
}
