use anyhow::Result;
use chrono::offset::Utc;
use chrono::DateTime;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::sqlite::{QueryResult, Value as SV};
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Follower {
    id: String,
    user_id: String,
    federation_id: String,
    object: Value,
    follow_at: DateTime<Utc>,
    unfollow_at: DateTime<Utc>,
    reject_at: DateTime<Utc>,
}

impl Follower {
    pub async fn get(username: &str) -> Result<Vec<Self>> {
        let frs = crate::db::Connection::builder()
        .await.execute(
            "SELECT u.name, f.* FROM follower AS f LEFT JOIN user AS u ON u.id = f.userId WHERE u.name = ?", 
            &[SV::Text(username.to_string())]
        ).await;

        let mut followers: Vec<Follower> = Vec::new();
        for fr in frs.rows() {
            let f = Follower {
                id: fr.get::<&str>("id").unwrap().to_string(),
                user_id: fr.get::<&str>("userId").unwrap().to_string(),
                federation_id: fr
                    .get::<&str>("federationId")
                    .unwrap()
                    .to_string(),
                object: serde_json::from_str(
                    fr.get::<&str>("object").unwrap(),
                )
                .unwrap(),
                follow_at: DateTime::from_timestamp(
                    fr.get::<i64>("follow_at").unwrap(),
                    0,
                )
                .unwrap(),
                unfollow_at: DateTime::from_timestamp(
                    fr.get::<i64>("unfollow_at").unwrap(),
                    0,
                )
                .unwrap(),
                reject_at: DateTime::from_timestamp(
                    fr.get::<i64>("reject_at").unwrap(),
                    0,
                )
                .unwrap(),
            };
            followers.push(f);
        }
        Ok(followers)
    }

    pub async fn count(username: &str) -> usize {
        let u = Self::get(username).await.unwrap();
        u.len()
    }
}
