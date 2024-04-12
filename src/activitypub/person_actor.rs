use anyhow::Result;
use chrono::format::strftime::StrftimeItems;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::sqlite::Value as SV;

use crate::table::user::User;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonActor {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub following: String,
    pub followers: String,
    pub inbox: String,
    pub outbox: String,
    pub featured: String,
    pub featured_tags: String,
    pub preferred_username: String,
    pub name: String,
    pub summary: Option<String>,
    pub url: String,
    pub manually_approves_followers: bool,
    pub discoverable: bool,
    pub indexable: bool,
    pub published: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memorial: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<String>,
    pub public_key: PublicKey,
    pub tags: Vec<Value>,
    pub attachment: Vec<Value>,
    pub icon: Image,
    pub image: Image,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    pub id: String,
    pub owner: String,
    pub public_key_pem: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    #[serde(rename = "type")]
    kind: String,
    media_type: String,
    url: String,
}

impl PersonActor {
    async fn get_public_key(u: User) -> PublicKey {
        let signing_key_rowset = crate::db::Connection::builder()
            .await
            .execute(
                "SELECT publicKey FROM signing_key WHERE userId = ?",
                &[SV::Integer(u.id as i64)],
            )
            .await;
        let public_key_pem = signing_key_rowset
            .rows()
            .next()
            .unwrap()
            .get::<&str>("publicKey")
            .unwrap();

        PublicKey {
            id: format!("{}#main-key", u.federation_id),
            owner: u.federation_id.to_string(),
            public_key_pem: public_key_pem.to_string(),
        }
    }

    pub async fn create(u: User) -> Result<Self> {
        let fmt = StrftimeItems::new("%Y-%m-%d %H:%M:%S");
        let pub_date = u.published.format_with_items(fmt.clone()).to_string();
        let pk = Self::get_public_key(u.clone()).await;

        let icon = Image {
            kind: "Image".to_string(),
            media_type: "image/jpeg".to_string(),
            url: u.icon_location.to_string(),
        };

        let image = Image {
            kind: "Image".to_string(),
            media_type: "image/jpeg".to_string(),
            url: u.image_location.to_string(),
        };

        Ok(PersonActor {
            context: vec![
                "https://www.w3.org/ns/activitystreams".to_string(),
                "https://w3id.org/security/v1".to_string(),
            ],
            id: u.federation_id.to_string(),
            kind: "Person".to_string(),
            following: u.following.to_string(),
            followers: u.followers.to_string(),
            inbox: u.inbox.to_string(),
            outbox: u.outbox.to_string(),
            featured: u.featured.to_string(),
            featured_tags: u.featured_tags.to_string(),
            preferred_username: u.display_name.to_string(),
            name: u.name.to_string(),
            summary: u.summary.to_owned(),
            url: u.url.to_string(),
            manually_approves_followers: u.manually_approves_followers,
            discoverable: u.discoverable,
            indexable: u.indexable,
            published: pub_date,
            memorial: Some(false),
            devices: None,
            public_key: pk,
            tags: vec![Value::Null],
            attachment: vec![Value::Null],
            icon: icon,
            image: image,
        })
    }
}
