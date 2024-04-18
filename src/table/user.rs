use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use spin_sdk::sqlite::Value as SV;

// https://github.com/mastodon/mastodon/blob/main/app/models/user.rb
//
// == Schema Information
//
// Table name: users
//
//  id                        :bigint(8)        not null, primary key
//  email                     :string           default(""), not null
//  created_at                :datetime         not null
//  updated_at                :datetime         not null
//  encrypted_password        :string           default(""), not null
//  reset_password_token      :string
//  reset_password_sent_at    :datetime
//  sign_in_count             :integer          default(0), not null
//  current_sign_in_at        :datetime
//  last_sign_in_at           :datetime
//  confirmation_token        :string
//  confirmed_at              :datetime
//  confirmation_sent_at      :datetime
//  unconfirmed_email         :string
//  locale                    :string
//  encrypted_otp_secret      :string
//  encrypted_otp_secret_iv   :string
//  encrypted_otp_secret_salt :string
//  consumed_timestep         :integer
//  otp_required_for_login    :boolean          default(FALSE), not null
//  last_emailed_at           :datetime
//  otp_backup_codes          :string           is an Array
//  account_id                :bigint(8)        not null
//  disabled                  :boolean          default(FALSE), not null
//  invite_id                 :bigint(8)
//  chosen_languages          :string           is an Array
//  created_by_application_id :bigint(8)
//  approved                  :boolean          default(TRUE), not null
//  sign_in_token             :string
//  sign_in_token_sent_at     :datetime
//  webauthn_id               :string
//  sign_up_ip                :inet
//  role_id                   :bigint(8)
//  settings                  :text
//  time_zone                 :string
//

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: u64,                    // not null, primary key
    pub email: String,              // default(""), not null
    pub created_at: DateTime<Utc>,  // not null
    pub updated_at: DateTime<Utc>,  // not null
    pub encrypted_password: String, // default(""), not null
    pub reset_password_token: Option<String>,
    pub reset_password_sent_at: DateTime<Utc>,
    pub sign_in_count: u64, // default(0), not null
    pub current_sign_in_at: Option<DateTime<Utc>>,
    pub last_sign_in_at: Option<DateTime<Utc>>,
    pub confirmation_token: Option<String>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub confirmation_sent_at: Option<DateTime<Utc>>,
    pub unconfirmed_email: Option<String>,
    pub locale: Option<String>,
    pub encrypted_otp_secret: Option<String>,
    pub encrypted_otp_secret_iv: Option<String>,
    pub encrypted_otp_secret_salt: Option<String>,
    pub consumed_timestep: Option<i64>,
    pub otp_required_for_login: bool, // default(FALSE), not null
    pub last_emailed_at: Option<DateTime<Utc>>,
    pub otp_backup_codes: Option<String>, // is an Array
    pub account_id: u64,                  // not null
    pub disabled: bool,                   // default(FALSE), not null
    pub invite_id: u64,
    pub chosen_languages: Option<String>, // is an Array
    pub created_by_application_id: Option<i8>,
    pub approved: bool, // default(TRUE), not null
    pub sign_in_token: Option<String>,
    pub sign_in_token_sent_at: Option<DateTime<Utc>>,
    pub webauthn_id: Option<String>,
    pub sign_up_ip: String,
    pub role_id: Option<i8>,
    pub settings: Option<String>,
    pub time_zone: Option<String>,
}

impl User {
    pub async fn create_table() -> Result<()> {
        let create_stmt = r#"CREATE TABLE IF NOT EXISTS user (
            id                       INTEGER PRIMARY KEY AUTOINCREMENT,
            email                    TEXT DEFAULT "" NOT NULL,
            createdAt                INTEGER NOT NULL,
            updatedAt                INTEGER NOT NULL,
            encryptedPassword        TEXT DEFAULT "" NOT NULL,
            resetPpasswordToken      TEXT, 
            resetPasswordSentAt      INTEGER,
            signInCount              INTEGER DEFAULT 0 NOT NULL,
            currentSignInAt          INTEGER,
            lastSignInAt             INTEGER,
            confirmationToken        TEXT,
            confirmedAt              INTEGER,
            confirmationSentAt       INTEGER,
            unconfirmedEmail         TEXT, 
            locale                   TEXT,
            encryptedOtpSecret       TEXT,
            encryptedOtpSecretIv     TEXT,
            encryptedOtpSecretSalt   TEXT,
            consumedTimestep         INTEGER,
            otpRequiredForLogin      BOOLEAN DEFAULT FALSE NOT NULL,
            lastEmailedAt            INTEGER,
            otpBackupCodes           TEXT,
            accountId                INTEGER NOT NULL,
            disabled                 BOOLEAN DEFAULT FALSE NOT NULL,
            inviteId                 INTEGER,
            chosenLanguages          TEXT,
            createdByApplicationId   INTEGER,
            approved                 BOOLEAN DEFAULT TRUE NOT NULL,
            signInToken              TEXT,
            signInTokenSentAt        INTEGER,
            webauthnId               TEXT,
            signUpIp                 STRING,
            roleId                   INTEGER,
            settings                 TEXT,
            timeZone                 TEXT,
            FOREIGN KEY (accountId)  REFERENCES account (id))"#;
        let _create_table = crate::db::Connection::builder()
            .await
            .execute(create_stmt, &[])
            .await;

        // todo: do a proper thing with create_table QueryResult
        Ok(())
    }

    pub async fn get(account_id: u64) -> Result<Vec<Self>> {
        let user_rowset = crate::db::Connection::builder()
            .await
            .execute(
                "SELECT * FROM user WHERE accountId = ?",
                &[SV::Text(account_id.to_string())],
            )
            .await;

        if user_rowset.rows().count() == 0 {
            return Err(anyhow::Error::msg("No user found"));
        }

        let row = user_rowset.rows().next().unwrap();

        let user = User {
            id: row.get::<u64>("id").unwrap(), // not null, primary key
            email: row.get::<&str>("email").unwrap_or("").to_owned(), // default(""), not null
            created_at: DateTime::from_timestamp(
                row.get::<i64>("createdAt").unwrap(),
                0,
            )
            .unwrap(),
            updated_at: DateTime::from_timestamp(
                row.get::<i64>("updatedAt").unwrap(),
                0,
            )
            .unwrap(), // not null
            encrypted_password: row
                .get::<&str>("encryptedPassword")
                .unwrap_or("")
                .to_owned(),
            reset_password_token: row
                .get::<&str>("encryptedPassword")
                .map(str::to_string),
            reset_password_sent_at: DateTime::from_timestamp(
                row.get::<i64>("resetPasswordSentAt").unwrap(),
                0,
            )
            .unwrap(),
            sign_in_count: row.get::<u64>("signInCount").unwrap_or(0), // default(0), not null
            current_sign_in_at: DateTime::from_timestamp(
                row.get::<i64>("currentSignInAt").unwrap(),
                0,
            ),
            last_sign_in_at: DateTime::from_timestamp(
                row.get::<i64>("lastSignInAt").unwrap(),
                0,
            ),
            confirmation_token: row
                .get::<&str>("confirmationToken")
                .map(str::to_string),
            confirmed_at: DateTime::from_timestamp(
                row.get::<i64>("confirmedAt").unwrap(),
                0,
            ),
            confirmation_sent_at: DateTime::from_timestamp(
                row.get::<i64>("confirmationSentAt").unwrap(),
                0,
            ),
            unconfirmed_email: row
                .get::<&str>("unconfirmedEmail")
                .map(str::to_string),
            locale: row.get::<&str>("locale").map(str::to_string),
            encrypted_otp_secret: row
                .get::<&str>("encryptedOtpSecret")
                .map(str::to_string),
            encrypted_otp_secret_iv: row
                .get::<&str>("encryptedOtpSecretIv")
                .map(str::to_string),
            encrypted_otp_secret_salt: row
                .get::<&str>("encryptedOtpSecretSalt")
                .map(str::to_string),
            consumed_timestep: row.get::<i64>("consumedTimestep"),
            otp_required_for_login: row
                .get::<bool>("otpRequiredForLogin")
                .unwrap(),
            last_emailed_at: DateTime::from_timestamp(
                row.get::<i64>("lastEmailedAt").unwrap(),
                0,
            ),
            otp_backup_codes: row
                .get::<&str>("otpBackupCodes")
                .map(str::to_string),
            account_id: row.get::<u64>("accountId").unwrap(),
            disabled: row.get::<bool>("disabled").unwrap_or(false), // default(FALSE), not null
            invite_id: row.get::<u64>("inviteId").unwrap(),
            chosen_languages: row
                .get::<&str>("chosenLanguage")
                .map(str::to_string),
            created_by_application_id: row.get::<i8>("createdByApplicationId"),
            approved: row.get::<bool>("approved").unwrap_or(true),
            sign_in_token: row.get::<&str>("signInToken").map(str::to_string),
            sign_in_token_sent_at: DateTime::from_timestamp(
                row.get::<i64>("signInTokenSentAt").unwrap(),
                0,
            ),
            webauthn_id: row.get::<&str>("webauthnId").map(str::to_string),
            sign_up_ip: row
                .get::<&str>("signUpIp")
                .map(str::to_string)
                .unwrap(),
            role_id: row.get::<i8>("roleId"),
            settings: row.get::<&str>("settings").map(str::to_string),
            time_zone: row.get::<&str>("timeZone").map(str::to_string),
        };

        Ok(vec![user])
    }
}

struct Users;

impl Users {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn foo2() {
        let u = User::get(1).await.unwrap();
        println!("{:?}", u);
    }
}
