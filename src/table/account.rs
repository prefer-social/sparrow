use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::sqlite::Value as SV;

// == Schema Information
//
// Table name: accounts
//
// id                            :bigint(8)        not null, primary key
// username                      :string           default(""), not null
// domain                        :string
// private_key                   :text
// public_key                    :text             default(""), not null
// created_at                    :datetime         not null
// updated_at                    :datetime         not null
// note                          :text             default(""), not null
// display_name                  :string           default(""), not null
// uri                           :string           default(""), not null
// url                           :string
// avatar_file_name              :string
// avatar_content_type           :string
// avatar_file_size              :integer
// avatar_updated_at             :datetime
// header_file_name              :string
// header_content_type           :string
// header_file_size              :integer
// header_updated_at             :datetime
// avatar_remote_url             :string
// locked                        :boolean          default(FALSE), not null
// header_remote_url             :string           default(""), not null
// last_webfingered_at           :datetime
// inbox_url                     :string           default(""), not null
// outbox_url                    :string           default(""), not null
// shared_inbox_url              :string           default(""), not null
// followers_url                 :string           default(""), not null
// protocol                      :integer          default("ostatus"), not null
// memorial                      :boolean          default(FALSE), not null
// moved_to_account_id           :bigint(8)
// featured_collection_url       :string
// fields                        :jsonb
// actor_type                    :string
// discoverable                  :boolean
// also_known_as                 :string           is an Array
// silenced_at                   :datetime
// suspended_at                  :datetime
// hide_collections              :boolean
// avatar_storage_schema_version :integer
// header_storage_schema_version :integer
// devices_url                   :string
// suspension_origin             :integer
// sensitized_at                 :datetime
// trendable                     :boolean
// reviewed_at                   :datetime
// requested_review_at           :datetime
// indexable                     :boolean          default(FALSE), not null

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Account {
    pub id: i64,          // not null, primary key
    pub username: String, // default(""), not null
    pub domain: Option<String>,
    pub private_key: Option<String>,
    pub public_key: String, // default(""), not null
    pub created_at: DateTime<Utc>, // not null
    pub updated_at: DateTime<Utc>, // not null
    pub note: String,       // default(""), not null
    pub display_name: String, // default(""), not null
    pub uri: String,        // default(""), not null
    pub url: Option<String>,
    pub avatar_file_name: Option<String>,
    pub avatar_content_type: Option<String>,
    pub avatar_file_size: Option<u32>,
    pub avatar_updated_at: Option<DateTime<Utc>>,
    pub header_file_name: Option<String>,
    pub header_content_type: Option<String>,
    pub header_file_size: Option<u32>,
    pub header_updated_at: Option<DateTime<Utc>>,
    pub avatar_remote_url: Option<String>,
    pub locked: bool, // default(FALSE), not null
    pub header_remote_url: Option<String>, // default(""), not null
    pub last_webfingered_at: Option<DateTime<Utc>>,
    pub inbox_url: String,        // default(""), not null
    pub outbox_url: String,       // default(""), not null
    pub shared_inbox_url: String, // default(""), not null
    pub followers_url: String,    // default(""), not null
    pub protocol: i64,            // default("ostatus"), not null
    pub memorial: bool,           // default(FALSE), not null
    pub moved_to_account_id: Option<i8>,
    pub featured_collection_url: Option<String>,
    pub fields: Option<Value>,
    pub actor_type: Option<String>,
    pub discoverable: Option<bool>,
    pub also_known_as: Option<String>, // is an Array
    pub silenced_at: Option<DateTime<Utc>>,
    pub suspended_at: Option<DateTime<Utc>>,
    pub hide_collections: Option<bool>,
    pub avatar_storage_schema_version: Option<i64>,
    pub header_storage_schema_version: Option<i64>,
    pub devices_url: Option<String>,
    pub suspension_origin: Option<i64>,
    pub sensitized_at: Option<DateTime<Utc>>,
    pub trendable: Option<bool>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub requested_review_at: Option<DateTime<Utc>>,
    pub indexable: Option<bool>, // default(FALSE), not null
}

impl Account {
    pub async fn create_table() -> Result<()> {
        let create_stmt = r#"CREATE TABLE IF NOT EXISTS account (
            id                         INTEGER PRIMARY KEY AUTOINCREMENT,
            username                      TEXT DEFAULT "" NOT NULL,
            domain                        TEXT,
            private_key                   TEXT,
            public_key                    TEXT DEFAULT "" NOT NULL,
            created_at                    INTEGER NOT NULL,
            updated_at                    INTEGER NOT NULL,
            note                          TEXT DEFAULT "" NOT NULL,
            display_name                  TEXT DEFAULT "" NOT NULL,
            uri                           TEXT DEFAULT "" NOT NULL,
            url                           TEXT,
            avatar_file_name              TEXT,
            avatar_content_type           TEXT,
            avatar_file_size              INTEGER,
            avatar_updated_at             INTEGER,
            header_file_name              TEXT,
            header_content_type           TEXT,
            header_file_size              INTEGER,
            header_updated_at             INTEGER,
            avatar_remote_url             TEXT,
            locked                        BOOLEAN DEFAULT FALSE NOT NULL,
            header_remote_url             TEXT DEFAULT "" NOT NULL,
            last_webfingered_at           INTEGER,
            inbox_url                     TEXT DEFAULT "" NOT NULL,
            outbox_url                    TEXT DEFAULT "" NOT NULL,
            shared_inbox_url              TEXT DEFAULT "" NOT NULL,
            followers_url                 TEXT DEFAULT "" NOT NULL,
            protocol                      INTEGER NOT NULL,
            memorial                      BOOLEAN DEFAULT FALSE NOT NULL,
            moved_to_account_id           INTEGER,
            featured_collection_url       TEXT,
            fields                        VALUE,
            actor_type                    TEXT,
            discoverable                  BOOLEAN,
            also_known_as                 TEXT,
            silenced_at                   INTEGER,
            suspended_at                  INTEGER,
            hide_collections              BOOLEAN,
            avatar_storage_schema_version INTEGER,
            header_storage_schema_version INTEGER,
            devices_url                   TEXT,
            suspension_origin             INTEGER,
            sensitized_at                 INTEGER,
            trendable                     BOOLEAN,
            reviewed_at                   INTEGER,
            requested_review_at           INTEGER,
            indexable                     BOOLEAN
        )"#;

        let _create_table = crate::db::Connection::builder()
            .await
            .execute(create_stmt, &[])
            .await;

        // todo: do a proper thing with create_table QueryResult
        Ok(())
    }

    pub async fn select() -> Result<()> {
        let table_hashmaps =
            crate::table::utils::hashmap_from_table("account".to_string())
                .await
                .unwrap()
                .unwrap();

        for table_hashmap in table_hashmaps {
            let foo = serde_json::to_string(&table_hashmap).unwrap();

            tracing::debug!(foo);

            let account: Value = serde_json::from_str(foo.as_str()).unwrap();

            tracing::debug!("{:?}", account);
        }

        Ok(())
    }

    pub async fn insert(&self) {
        let insert_stmt = r#"INSERT INTO account VALUES (
          ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
          ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
          ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
          ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
          ?, ?, ?, ?, ? )"#
            .to_string();

        //let a = crate::db::Connection::builder().await.executre()

        //tracing::debug!("{}", self.username);
    }

    pub async fn update(&self) {}

    pub async fn delete(&self) {}

    pub async fn get_with_username(
        username: String,
    ) -> Result<Option<Vec<Self>>> {
        let ar = crate::db::Connection::builder()
            .await
            .execute(
                "SELECT * FROM account WHERE username = ?",
                &[SV::Text(username.to_string())],
            )
            .await;

        if ar.rows().count() == 0 {
            return Ok(None);
        }

        let mut rtn: Vec<Account> = Vec::new();

        for row in ar.rows() {
            let account = Account {
                id: row.get::<i64>("id").unwrap(),
                username: row.get::<&str>("id").map(str::to_string).unwrap(),
                domain: row.get::<&str>("domain").map(str::to_string), //Option<String>,
                private_key: row.get::<&str>("privateKey").map(str::to_string),
                public_key: row
                    .get::<&str>("public_key")
                    .unwrap_or("")
                    .to_string(),
                created_at: DateTime::from_timestamp(
                    row.get::<i64>("createdAt").unwrap(),
                    0,
                )
                .unwrap(), // not null
                updated_at: DateTime::from_timestamp(
                    row.get::<i64>("updatedAt").unwrap(),
                    0,
                )
                .unwrap(), // not null
                note: row.get::<&str>("note").map(str::to_string).unwrap(),
                display_name: row
                    .get::<&str>("displayName")
                    .unwrap_or("")
                    .to_string(),
                uri: row.get::<&str>("uri").unwrap_or("").to_string(), // default(""), not null
                url: row.get::<&str>("url").map(str::to_string),
                avatar_file_name: row
                    .get::<&str>("avatarFileName")
                    .map(str::to_string),
                avatar_content_type: row
                    .get::<&str>("avatarContentType")
                    .map(str::to_string),
                avatar_file_size: row.get::<u32>("avatarContentType"),
                avatar_updated_at: DateTime::from_timestamp(
                    row.get::<i64>("avatarUpdatedAt").unwrap(),
                    0,
                ),
                header_file_name: row
                    .get::<&str>("headerFileName")
                    .map(str::to_string), // Option<String>,
                header_content_type: row
                    .get::<&str>("haederContentType")
                    .map(str::to_string), // Option<String>,
                header_file_size: row.get::<u32>("headerFileSize"), // Option<u64>,
                header_updated_at: DateTime::from_timestamp(
                    row.get::<i64>("updatedAt").unwrap(),
                    0,
                ), // Option<DateTime<Utc>>,
                avatar_remote_url: row
                    .get::<&str>("avatarRemoteUrl")
                    .map(str::to_string), // Option<String>,
                locked: row.get::<bool>("locked").unwrap_or(false), // Boolean, // default(FALSE), not null
                header_remote_url: row
                    .get::<&str>("headerRemoteUrl")
                    .map(str::to_string), // Option<String>, // default(""), not null
                last_webfingered_at: DateTime::from_timestamp(
                    row.get::<i64>("lastWebfingeredAt").unwrap(),
                    0,
                ), // Option<DateTime<Utc>>,
                inbox_url: row
                    .get::<&str>("inbox_url")
                    .unwrap_or("")
                    .to_string(), // String,        // default(""), not null
                outbox_url: row
                    .get::<&str>("outbox_url")
                    .unwrap_or("")
                    .to_string(), //String,       // default(""), not null
                shared_inbox_url: row
                    .get::<&str>("sharedInboxUrl")
                    .unwrap_or("")
                    .to_string(), // String, // default(""), not null
                followers_url: row
                    .get::<&str>("followersUrl")
                    .unwrap_or("")
                    .to_string(), // String,    // default(""), not null
                protocol: row.get::<i64>("protocol").unwrap(), // Option<usize>,  // default("ostatus"), not null
                memorial: row.get::<bool>("memorial").unwrap_or(false), // Boolean,        // default(FALSE), not null
                moved_to_account_id: row.get::<i8>("movedToAccountId"), // Option<i8>,
                featured_collection_url: row
                    .get::<&str>("featuredCollectoinUrl")
                    .map(str::to_string), // Option<String>,
                fields: serde_json::from_str(
                    row.get::<&str>("fieled").unwrap(),
                )
                .unwrap(), // Option<Value>,
                actor_type: row.get::<&str>("actorType").map(str::to_string), // Option<String>,
                discoverable: row.get::<bool>("discoverable"), // Option<Boolean>,
                also_known_as: row
                    .get::<&str>("alsoKnownAs")
                    .map(str::to_string), // Option<String>, // is an Array
                silenced_at: DateTime::from_timestamp(
                    row.get::<i64>("silencedAt").unwrap(),
                    0,
                ), // Option<DateTime<Utc>>,
                suspended_at: DateTime::from_timestamp(
                    row.get::<i64>("suspendedAt").unwrap(),
                    0,
                ), // Option<DateTime<Utc>>,
                hide_collections: row.get::<bool>("updatedAt"), // Option<Boolean>,
                avatar_storage_schema_version: row
                    .get::<i64>("avatarStorageSchemaVersion"), // Option<usize>,
                header_storage_schema_version: row
                    .get::<i64>("headerStorageSchemaVersion"), // Option<usize>,
                devices_url: row.get::<&str>("devicesUrl").map(str::to_string), // Option<String>,
                suspension_origin: row.get::<i64>("suspensionOrigin"),
                sensitized_at: DateTime::from_timestamp(
                    row.get::<i64>("sensitizedAt").unwrap(),
                    0,
                ), // Option<DateTime<Utc>>,
                trendable: row.get::<bool>("trendable"), // Option<boolean>,
                reviewed_at: DateTime::from_timestamp(
                    row.get::<i64>("reviewedAt").unwrap(),
                    0,
                ), // Option<DateTime<Utc>>,
                requested_review_at: DateTime::from_timestamp(
                    row.get::<i64>("requestedReviewAt").unwrap(),
                    0,
                ), // Option<DateTime<Utc>>,
                indexable: Some(row.get::<bool>("indexable").unwrap()), // Boolean, // default(FALSE), not null
            };
        }

        Ok(Some(rtn))
    }

    pub async fn get_with_userid(id: u64) -> Result<Option<Vec<Self>>> {
        let ar = crate::db::Connection::builder()
            .await
            .execute(
                "SELECT * FROM account WHERE id = ?",
                &[SV::Text(id.to_string())],
            )
            .await;

        if ar.rows().count() == 0 {
            return Ok(None);
        }

        let mut rtn: Vec<Account> = Vec::new();

        for row in ar.rows() {
            let account = Account {
                id: row.get::<i64>("id").unwrap(),
                username: row.get::<&str>("id").map(str::to_string).unwrap(),
                domain: row.get::<&str>("domain").map(str::to_string), //Option<String>,
                private_key: row.get::<&str>("privateKey").map(str::to_string),
                public_key: row
                    .get::<&str>("public_key")
                    .unwrap_or("")
                    .to_string(),
                created_at: DateTime::from_timestamp(
                    row.get::<i64>("createdAt").unwrap(),
                    0,
                )
                .unwrap(), // not null
                updated_at: DateTime::from_timestamp(
                    row.get::<i64>("updatedAt").unwrap(),
                    0,
                )
                .unwrap(), // not null
                note: row.get::<&str>("note").map(str::to_string).unwrap(),
                display_name: row
                    .get::<&str>("displayName")
                    .unwrap_or("")
                    .to_string(),
                uri: row.get::<&str>("uri").unwrap_or("").to_string(), // default(""), not null
                url: row.get::<&str>("url").map(str::to_string),
                avatar_file_name: row
                    .get::<&str>("avatarFileName")
                    .map(str::to_string),
                avatar_content_type: row
                    .get::<&str>("avatarContentType")
                    .map(str::to_string),
                avatar_file_size: row.get::<u32>("avatarContentType"),
                avatar_updated_at: DateTime::from_timestamp(
                    row.get::<i64>("avatarUpdatedAt").unwrap(),
                    0,
                ),
                header_file_name: row
                    .get::<&str>("headerFileName")
                    .map(str::to_string), // Option<String>,
                header_content_type: row
                    .get::<&str>("haederContentType")
                    .map(str::to_string), // Option<String>,
                header_file_size: row.get::<u32>("headerFileSize"), // Option<u64>,
                header_updated_at: DateTime::from_timestamp(
                    row.get::<i64>("updatedAt").unwrap(),
                    0,
                ), // Option<DateTime<Utc>>,
                avatar_remote_url: row
                    .get::<&str>("avatarRemoteUrl")
                    .map(str::to_string), // Option<String>,
                locked: row.get::<bool>("locked").unwrap_or(false), // Boolean, // default(FALSE), not null
                header_remote_url: row
                    .get::<&str>("headerRemoteUrl")
                    .map(str::to_string), // Option<String>, // default(""), not null
                last_webfingered_at: DateTime::from_timestamp(
                    row.get::<i64>("lastWebfingeredAt").unwrap(),
                    0,
                ), // Option<DateTime<Utc>>,
                inbox_url: row
                    .get::<&str>("inbox_url")
                    .unwrap_or("")
                    .to_string(), // String,        // default(""), not null
                outbox_url: row
                    .get::<&str>("outbox_url")
                    .unwrap_or("")
                    .to_string(), //String,       // default(""), not null
                shared_inbox_url: row
                    .get::<&str>("sharedInboxUrl")
                    .unwrap_or("")
                    .to_string(), // String, // default(""), not null
                followers_url: row
                    .get::<&str>("followersUrl")
                    .unwrap_or("")
                    .to_string(), // String,    // default(""), not null
                protocol: row.get::<i64>("protocol").unwrap(), // Option<usize>,  // default("ostatus"), not null
                memorial: row.get::<bool>("memorial").unwrap_or(false), // Boolean,        // default(FALSE), not null
                moved_to_account_id: row.get::<i8>("movedToAccountId"), // Option<i8>,
                featured_collection_url: row
                    .get::<&str>("featuredCollectoinUrl")
                    .map(str::to_string), // Option<String>,
                fields: serde_json::from_str(
                    row.get::<&str>("fieled").unwrap(),
                )
                .unwrap(), // Option<Value>,
                actor_type: row.get::<&str>("actorType").map(str::to_string), // Option<String>,
                discoverable: row.get::<bool>("discoverable"), // Option<Boolean>,
                also_known_as: row
                    .get::<&str>("alsoKnownAs")
                    .map(str::to_string), // Option<String>, // is an Array
                silenced_at: DateTime::from_timestamp(
                    row.get::<i64>("silencedAt").unwrap(),
                    0,
                ), // Option<DateTime<Utc>>,
                suspended_at: DateTime::from_timestamp(
                    row.get::<i64>("suspendedAt").unwrap(),
                    0,
                ), // Option<DateTime<Utc>>,
                hide_collections: row.get::<bool>("updatedAt"), // Option<Boolean>,
                avatar_storage_schema_version: row
                    .get::<i64>("avatarStorageSchemaVersion"), // Option<usize>,
                header_storage_schema_version: row
                    .get::<i64>("headerStorageSchemaVersion"), // Option<usize>,
                devices_url: row.get::<&str>("devicesUrl").map(str::to_string), // Option<String>,
                suspension_origin: row.get::<i64>("suspensionOrigin"),
                sensitized_at: DateTime::from_timestamp(
                    row.get::<i64>("sensitizedAt").unwrap(),
                    0,
                ), // Option<DateTime<Utc>>,
                trendable: row.get::<bool>("trendable"), // Option<boolean>,
                reviewed_at: DateTime::from_timestamp(
                    row.get::<i64>("reviewedAt").unwrap(),
                    0,
                ), // Option<DateTime<Utc>>,
                requested_review_at: DateTime::from_timestamp(
                    row.get::<i64>("requestedReviewAt").unwrap(),
                    0,
                ), // Option<DateTime<Utc>>,
                indexable: Some(row.get::<bool>("indexable").unwrap()), // Boolean, // default(FALSE), not null
            };
        }

        Ok(Some(rtn))
    }

    pub async fn get_with_account(account: String) -> Self {
        let u = account.split("@").collect::<Vec<&str>>();
        let username = u[0];
        let domain = u[1];

        todo!()
    }

    pub async fn get_userid(username: String) -> Option<u64> {
        let qr = crate::db::Connection::builder()
            .await
            .execute(
                "SELECT id FROM account WHERE username = ?",
                &[SV::Text(username)],
            )
            .await
            .to_owned();
        let row = qr.rows().next().unwrap();
        row.get::<u64>("id")
    }

    pub async fn federation_id(self: &Self) -> Result<String> {
        let username = self.username.clone();
        let domain = self.domain.as_ref().unwrap();
        Ok(format!("{}@{}", username, domain))
    }
}
