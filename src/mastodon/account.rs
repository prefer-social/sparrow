// https://docs.joinmastodon.org/entities/Account/

use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

use spin_sdk::sqlite::Value as SV;

use crate::table::user::User;
use crate::Identification;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub username: String,
    pub acct: String,
    pub display_name: String,
    pub locked: bool,
    pub bot: bool,
    pub discoverable: bool,
    pub group: bool,
    pub created_at: DateTime<Utc>,
    pub note: String,
    pub url: Option<String>,
    pub avatar: String,
    pub avatar_static: String,
    pub header: String,
    pub header_static: String,
    pub followers_count: u32,
    pub following_count: u32,
    pub statuses_count: u32,
    pub last_status_at: Option<DateTime<Utc>>,
    pub emojis: Option<Vec<String>>,
    pub fields: Option<Vec<String>>,
}

impl Account {
    // pub async fn get(a: String) -> Self {
    //     let i = Identification::get(a.as_str()).await;
    //     match i {
    //         Identification::ActorUrl(url) => {
    //             // Call Actor and gen Account from querying /api/v1/accounts/:id
    //             todo!();
    //         }
    //         Identification::Account(acct) => {
    //             // convert acct to actorUrl
    //             todo!();
    //         }
    //         Identification::UserId(i) => {
    //             // get username with UserId's i

    //             let account =
    //                 crate::table::account::Account::get_with_userid(i)
    //                     .await
    //                     .unwrap()
    //                     .unwrap()
    //                     .into_iter()
    //                     .nth(0)
    //                     .unwrap();
    //             let user = crate::table::user::User::get(i)
    //                 .await
    //                 .unwrap()
    //                 .into_iter()
    //                 .nth(0)
    //                 .unwrap();

    //             Account {
    //                 id: account.id.to_string(),
    //                 username: account.username.clone(),
    //                 acct: format!("{}@{}", account.username, account.domain,),
    //                 display_name: account.display_name,
    //                 locked: account.locked,
    //                 bot: false,
    //                 discoverable: false,
    //                 group: false,
    //                 created_at: account.created_at,
    //                 note: account.note,
    //                 url: account.url,
    //                 avatar: account.avatar_remote_url.clone().unwrap(),
    //                 avatar_static: account.avatar_remote_url.unwrap(),
    //                 header: account.header_remote_url.clone().unwrap(),
    //                 header_static: account.header_remote_url.unwrap(),
    //                 followers_count: 0,
    //                 following_count: 0,
    //                 statuses_count: 0,
    //                 last_status_at: None,
    //                 emojis: None,
    //                 fields: None,
    //             }
    //         }
    //         Identification::Username(username) => {
    //             let account =
    //                 crate::table::account::Account::get_with_username(
    //                     username,
    //                 )
    //                 .await
    //                 .unwrap()
    //                 .unwrap()
    //                 .into_iter()
    //                 .nth(0)
    //                 .unwrap();

    //             Account {
    //                 id: account.id.to_string(),
    //                 username: account.username.clone(),
    //                 acct: format!(
    //                     "{}@{}",
    //                     account.username,
    //                     account.domain.unwrap()
    //                 ),
    //                 display_name: account.display_name,
    //                 locked: account.locked,
    //                 bot: false,
    //                 discoverable: false,
    //                 group: false,
    //                 created_at: account.created_at,
    //                 note: account.note,
    //                 url: account.url,
    //                 avatar: account.avatar_remote_url.clone().unwrap(),
    //                 avatar_static: account.avatar_remote_url.clone().unwrap(),
    //                 header: account.header_remote_url.clone().unwrap(),
    //                 header_static: account.header_remote_url.unwrap(),
    //                 followers_count: 0,
    //                 following_count: 0,
    //                 statuses_count: 0,
    //                 last_status_at: None,
    //                 emojis: None,
    //                 fields: None,
    //             }
    //         }
    //     }
    // }

    pub async fn get_acct(self) -> String {
        let id = self.display_name;
        //let host = Url::parse(self.fe)
        todo!()
    }
}
