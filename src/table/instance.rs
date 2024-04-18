use anyhow::Result;
use chrono::offset::Utc;
use chrono::DateTime;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::sqlite::{QueryResult, Value as SV};
use url::Url;

pub enum InstanceType {
    Local,
    Remote(String),
}

// https://docs.joinmastodon.org/entities/Instance/
// https://docs.joinmastodon.org/entities/V1_Instance/
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    pub id: u32,
    pub domain: String,
    pub title: String,
    pub platform: String,
    pub version: String,
    pub source_url: String,
    pub description: String,
    pub usage: Value,
    pub thumbnail: Value,
    pub languages: Value,
    pub configuration: Value,
    pub registrations: Value,
    pub contact: Value,
    pub rules: Value,
}

impl Instance {
    async fn create_table() {
        let create_statement = r#"""
            CREATE TABLE IF NOT EXISTS instance 
            (
                id                  INTEGER PRIMARY KEY AUTOINCREMENT,
                domain              TEXT NOT
                 NULL ,
                title               TEXT,
                platform            TEXT, 
                version             TEXT,
                sourceUrl           TEXT, 
                description         TEXT, 
                usage               TEXT, 
                thumbnail           TEXT,
                languages           TEXT,
                configuration       TEXT,
                registrations       TEXT,
                contact             TEXT,
                rules               TEXT,
                createdAt           TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                UNIQUE (id, domain)
            );"""#;
    }

    pub async fn get(it: InstanceType) -> Self {
        match it {
            InstanceType::Local => {
                let iqr = crate::db::Connection::builder()
                    .await
                    .execute("SELECT * FROM instance WHERE id = 1", &[])
                    .await;
                let ir = iqr.rows().next().unwrap();
                let instance = Instance {
                    id: u32::try_from(ir.get::<i32>("id").unwrap()).unwrap(),
                    domain: ir.get::<&str>("domain").unwrap().to_string(),
                    title: ir.get::<&str>("title").unwrap().to_string(),
                    platform: ir.get::<&str>("platform").unwrap().to_string(),
                    version: ir.get::<&str>("version").unwrap().to_string(),
                    source_url: ir
                        .get::<&str>("sourceUrl")
                        .unwrap()
                        .to_string(),
                    description: ir
                        .get::<&str>("description")
                        .unwrap()
                        .to_string(),
                    usage: serde_json::from_str(
                        ir.get::<&str>("usage").unwrap(),
                    )
                    .unwrap(),
                    thumbnail: serde_json::from_str(
                        ir.get::<&str>("thumbnail").unwrap(),
                    )
                    .unwrap(),
                    languages: serde_json::from_str(
                        ir.get::<&str>("languages").unwrap(),
                    )
                    .unwrap(),
                    configuration: serde_json::from_str(
                        ir.get::<&str>("configuration").unwrap(),
                    )
                    .unwrap(),
                    registrations: serde_json::from_str(
                        ir.get::<&str>("registrations").unwrap(),
                    )
                    .unwrap(),
                    contact: serde_json::from_str(
                        ir.get::<&str>("contact").unwrap(),
                    )
                    .unwrap(),
                    rules: serde_json::from_str(
                        ir.get::<&str>("rules").unwrap(),
                    )
                    .unwrap(),
                };
                return instance;
            }
            InstanceType::Remote(url) => todo!(),
        }
    }
}
