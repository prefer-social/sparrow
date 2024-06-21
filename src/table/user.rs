use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use spin_sdk::sqlite::Value as SV;

#[derive(
    Clone, Debug, Deserialize, Serialize, PartialEq, Default, sqlx::FromRow,
)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub rowid: Option<i64>,
    pub uuid: Option<String>,  // not null, primary key
    pub email: Option<String>, // default(""), not null
    pub created_at: Option<i64>, // not null
    pub updated_at: Option<i64>, // not null
    pub encrypted_password: Option<String>, // default(""), not null
    pub reset_password_token: Option<String>,
    pub reset_password_sent_at: Option<i64>,
    pub sign_in_count: Option<i64>, // default(0), not null
    pub current_sign_in_at: Option<i64>,
    pub last_sign_in_at: Option<i64>,
    pub confirmation_token: Option<String>,
    pub confirmed_at: Option<i64>,
    pub confirmation_sent_at: Option<i64>,
    pub unconfirmed_email: Option<String>,
    pub locale: Option<String>,
    pub encrypted_otp_secret: Option<String>,
    pub encrypted_otp_secret_iv: Option<String>,
    pub encrypted_otp_secret_salt: Option<String>,
    pub consumed_timestep: Option<i64>,
    pub otp_required_for_login: Option<bool>, // default(FALSE), not null
    pub last_emailed_at: Option<i64>,
    pub otp_backup_codes: Option<String>, // is an Array
    pub account_id: Option<i64>,
    pub disabled: Option<bool>,
    pub invite_id: Option<i64>,
    pub chosen_languages: Option<String>, // is an Array
    pub created_by_application_id: Option<i64>,
    pub approved: Option<bool>, // default(TRUE), not null
    pub sign_in_token: Option<String>,
    pub sign_in_token_sent_at: Option<i64>,
    pub webauthn_id: Option<String>,
    pub sign_up_ip: Option<String>,
    pub role_id: Option<i64>,
    pub settings: Option<String>,
    pub time_zone: Option<String>,
}

impl User {
    pub async fn all() -> Result<Vec<Self>> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let users: Vec<User> = sqlx::query_as("SELECT rowid, * FROM user")
            .fetch_all(&sqlx_conn)
            .await?;
        Ok(users)
    }

    pub async fn get(uuid: &str) -> Result<Vec<Self>> {
        let sqlx_conn = spin_sqlx::Connection::open_default()?;
        let users: Vec<User> =
            sqlx::query_as("SELECT rowid, * FROM user WHERE uresr.uuid = ?")
                .bind(uuid)
                .fetch_all(&sqlx_conn)
                .await?;
        Ok(users)
    }
}
