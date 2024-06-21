pub mod activitypub;
pub mod auth;
pub mod db;
pub mod follow_request;
pub mod keys;
pub mod mastodon;
pub mod send;
pub mod storage;
pub mod table;
pub mod utils;
pub mod postbox;
pub mod http_response;

use anyhow::Result;
use regex::Regex;
use spin_sdk::http::{Request, IntoResponse, Response};
use spin_sdk::http_component;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::FmtSubscriber;



/// Identification
/// Username: Local user name. ie: seungjin, seungjin.kim
/// Account: seungjin@prefer.social, @seungjin@prefer.social
/// ActorUrl: https://seungjin.prefer.social, https:://seungjin.prefer.social/users/seungjin

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Ord)]
pub enum Identification {
    Username(String),
    UserId(u64),
    Account(String),
    ActorUrl(String),
    //PublicKy(String), 
    //Bech32(String),
}

impl Identification {
    pub async fn get(a: &str) -> Identification {

        let userid_regex = Regex::new(r"^\d+$").unwrap();

        match userid_regex.is_match(a) {
            true => return Self::UserId(a.parse::<u64>().unwrap()),
            false => (),
        }
    
        let account_regex = 
        Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6}+$)").unwrap();

        match account_regex.is_match(a) {
            true => return Self::Account(a.to_string()),
            false => (),
        }

        let url_regex = 
        Regex::new(r"https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)").unwrap();

        match url_regex.is_match(a) {
            true => return Self::ActorUrl(a.to_string()),
            false => (),
        }

        Identification::Username(a.to_string())
    }

    pub async fn to_actor_url(a: Identification) -> Identification {
        match a {
            Identification::Account(a) => todo!(),
            Identification::Username(a) => todo!(),
            Identification::UserId(a) => todo!(),
            Identification::ActorUrl(a) => Identification::ActorUrl(a),
        }
    }

    pub async fn is_valid_actor(a: Identification)  {
        todo!()
    }

    pub async fn is_valid_username(a: Identification) {
        todo!()
    }
}


/// For testing purpose
#[http_component]
async fn test(req: Request) -> Result<impl IntoResponse> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_env("APP_LOG_LEVEL"))
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    tracing::debug!("test");
    
    //blackbox_test::table_account_create().await;
    //blackbox_test::table_account().await;
    //blackbox_test::create_account().await;
    //blackbox_test::select_account().await;


    let account = table::account::Account::all().await.unwrap();
    tracing::debug!("{:?}",account);
    


    
    Ok(Response::new(200, "TEST"))
}


mod blackbox_test {

    use crate::table::account::Account;
    use chrono::Utc;
    use chrono::DateTime;
    use chrono::NaiveDateTime;
    use tracing::debug;
    


    pub async fn table_account() {
        match crate::table::account::Account::get_with_username("seungjin".to_string()).await.unwrap() {
            Some(va) => {
                tracing::debug!("{:?}", va);
            },
            None => {
                tracing::debug!("account table is None");
            },
        }      
    }

    pub async fn select_account() {
        let account = crate::table::account::Account::select().await.unwrap();
        //.unwrap();
        //tracing::debug!("{:?}", account);

    }

    // pub async fn create_account() {
        
    //     let account = Account {
    //         id: 1,
    //         username: "seungjin".to_string(),
    //         domain: Some("ap.dev.seungjin.net".to_string()),
    //         private_key: Some("".to_string()),
    //         public_key: "".to_string(),
    //         created_at: DateTime::from_utc(NaiveDateTime::from_timestamp(Utc::now().timestamp(),0),Utc),
    //         updated_at: DateTime::from_utc(NaiveDateTime::from_timestamp(Utc::now().timestamp(),0),Utc),
    //         note: "".to_string(),
    //         display_name: "".to_string(),
    //         uri: "".to_string(),
    //         url: None,
    //         avatar_file_name: None,
    //         avatar_content_type: None,
    //         avatar_file_size: None,
    //         avatar_updated_at: None,
    //         header_file_name: None,
    //         header_content_type: None,
    //         header_file_size: None,
    //         header_updated_at: None,
    //         avatar_remote_url: None,
    //         locked: false,
    //         header_remote_url: None,
    //         last_webfingered_at: None,
    //         inbox_url: "https://seungjin.ap.dev.seungjin.net/inbox"
    //             .to_string(),
    //         outbox_url: "https://seungjin.ap.dev.seungjin.net/outbox"
    //             .to_string(),
    //         shared_inbox_url: "".to_string(),
    //         followers_url: "https://seungjin.ap.dev.seungjin.net/followers"
    //             .to_string(),
    //         protocol: 1,
    //         memorial: false,
    //         moved_to_account_id: None,
    //         featured_collection_url: None,
    //         fields: None,
    //         actor_type: None,
    //         discoverable: None,
    //         also_known_as: None,
    //         silenced_at: None,
    //         suspended_at: None,
    //         hide_collections: None,
    //         avatar_storage_schema_version: None,
    //         header_storage_schema_version: None,
    //         devices_url: None,
    //         suspension_origin: None,
    //         sensitized_at: None,
    //         trendable: None,
    //         reviewed_at: None,
    //         requested_review_at: None,
    //         indexable: Some(false),
    //     };

    //     //account.save().await;

    // }

    

}



