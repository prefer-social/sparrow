use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use spin_sdk::sqlite::{QueryResult, Value as SV};

// == Schema Information
//
// Table name: follows
//
//  id                :bigint(8)        not null, primary key
//  created_at        :datetime         not null
//  updated_at        :datetime         not null
//  account_id        :bigint(8)        not null
//  target_account_id :bigint(8)        not null
//  show_reblogs      :boolean          default(TRUE), not null
//  uri               :string
//  notify            :boolean          default(FALSE), not null
//  languages         :string           is an Array
//

pub struct Follow {
    pub id: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub account_id: u64,
    pub target_account_id: u64,
    pub show_rebloges: bool,
    pub uri: String,
    pub notify: bool,
    pub languages: String,
}

impl Follow {
    pub async fn create_table() {
        let create_stmt = r#"CREATE TABLE IF NOT EXISTS follow (
            id                          INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at                  INTEGER NOT NULL,
            updated_at                  INTEGER NOT NULL,
            account_id                  INTEGER NOT NULL, 
            target_account_id           INTEGER NOT NULL,
            show_rebloges               BOOLEAN DEFAULT TRUE NOT NULL,
            uri                         TEXT,
            notify                      BOOLEAN DEFAULT FALSE NOT NULL,
            language                    TEXT,
        "#;
    }

    pub async fn get() {}

    pub async fn get_number_of_followers(account_id: u64) -> Result<u64> {
        let ar = crate::db::Connection::builder()
            .await
            .execute(
                "SELECT COUNT(*) AS COUNT FROM follow WHERE target_account_id = ?",
                &[SV::Text(account_id.to_string())],
            )
            .await;
        let a = ar.rows().next().unwrap().get::<u64>("COUNT");
        Ok(a.unwrap())
    }

    pub async fn get_number_of_followings(account_id: u64) -> Result<u64> {
        let ar = crate::db::Connection::builder()
            .await
            .execute(
                "SELECT COUNT(*) AS COUNT FROM follow WHERE account_id = ?",
                &[SV::Text(account_id.to_string())],
            )
            .await;
        let a = ar.rows().next().unwrap().get::<u64>("COUNT");
        Ok(a.unwrap())
    }

    pub async fn get_followers(account_id: u64) -> Result<Vec<Follow>> {
        let ar = crate::db::Connection::builder()
            .await
            .execute(
                "SELECT * AS COUNT FROM follow WHERE target_account_id = ?",
                &[SV::Text(account_id.to_string())],
            )
            .await;
        let mut rtn: Vec<Follow> = Vec::new();
        for row in ar.rows() {
            let follow = Follow {
                id: row.get::<u64>("id").unwrap(),
                created_at: DateTime::from_timestamp(
                    row.get::<i64>("createdAt").unwrap(),
                    0,
                )
                .unwrap(),
                updated_at: DateTime::from_timestamp(
                    row.get::<i64>("updatedAt").unwrap(),
                    0,
                )
                .unwrap(),
                account_id: row.get::<u64>("accountId").unwrap(),
                target_account_id: row.get::<u64>("targetAccountId").unwrap(),
                show_rebloges: row.get::<bool>("show_rebloges").unwrap(),
                uri: row.get::<&str>("uri").unwrap().to_string(),
                notify: row.get::<bool>("notify").unwrap(),
                languages: row.get::<&str>("languages").unwrap().to_string(),
            };
            rtn.push(follow);
        }
        Ok(rtn)
    }

    pub async fn get_followings(account_id: u64) -> Result<Vec<Follow>> {
        let ar = crate::db::Connection::builder()
            .await
            .execute(
                "SELECT * AS COUNT FROM follow WHERE account_id = ?",
                &[SV::Text(account_id.to_string())],
            )
            .await;
        let mut rtn: Vec<Follow> = Vec::new();
        for row in ar.rows() {
            let follow = Follow {
                id: row.get::<u64>("id").unwrap(),
                created_at: DateTime::from_timestamp(
                    row.get::<i64>("createdAt").unwrap(),
                    0,
                )
                .unwrap(),
                updated_at: DateTime::from_timestamp(
                    row.get::<i64>("updatedAt").unwrap(),
                    0,
                )
                .unwrap(),
                account_id: row.get::<u64>("accountId").unwrap(),
                target_account_id: row.get::<u64>("targetAccountId").unwrap(),
                show_rebloges: row.get::<bool>("show_rebloges").unwrap(),
                uri: row.get::<&str>("uri").unwrap().to_string(),
                notify: row.get::<bool>("notify").unwrap(),
                languages: row.get::<&str>("languages").unwrap().to_string(),
            };
            rtn.push(follow);
        }
        Ok(rtn)
    }

    pub async fn save() {}
}
