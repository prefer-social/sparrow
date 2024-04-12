use anyhow::Result;
use chrono::offset::Utc;
use chrono::DateTime;
use serde_derive::{Deserialize, Serialize};
use spin_sdk::sqlite::{QueryResult, Value as SV};
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: u32,
    pub federation_id: String,
    pub name: String,
    pub display_name: String,
    pub email: Option<String>,
    pub url: Url,
    pub summary: Option<String>,
    pub local: bool,
    pub discoverable: bool,
    pub manually_approves_followers: bool,
    pub indexable: bool,
    pub inbox: Url,
    pub outbox: Url,
    pub following: Url,
    pub followers: Url,
    pub featured: Url,
    pub featured_tags: Url,
    pub image_location: Url,
    pub icon_location: Url,
    pub published: DateTime<Utc>,
    pub last_refreshed_at: DateTime<Utc>,
}

impl User {
    pub async fn get_with_id(id: usize) -> Result<User> {
        todo!()
    }

    pub async fn get_with_fed_id() -> User {
        todo!()
    }

    pub async fn get_with_name(name: &str) -> Result<Self> {
        let user_rowset = crate::db::Connection::builder()
            .await
            .execute(
                "SELECT * FROM user WHERE name = ?",
                &[SV::Text(name.to_string())],
            )
            .await;

        if user_rowset.rows().count() == 0 {
            return Err(anyhow::Error::msg("No user found"));
        }

        let row = user_rowset.rows().next().unwrap();

        let user = User {
            id: row.get::<u32>("id").unwrap(),
            federation_id: row
                .get::<&str>("federationId")
                .unwrap()
                .to_string(),
            name: row.get::<&str>("name").unwrap().to_owned(),
            display_name: row.get::<&str>("displayName").unwrap().to_owned(),
            email: Some(row.get::<&str>("email").unwrap().to_owned()),
            url: Url::parse(row.get::<&str>("url").unwrap()).unwrap(),
            summary: Some(row.get::<&str>("summary").unwrap().to_owned()),
            local: row.get::<bool>("local").unwrap(),
            discoverable: row.get::<bool>("discoverable").unwrap(),
            manually_approves_followers: row
                .get::<bool>("manuallyApprovesFollowers")
                .unwrap(),
            indexable: row.get::<bool>("indexable").unwrap(),
            inbox: Url::parse(
                row.get::<&str>("inbox").unwrap().to_owned().as_str(),
            )
            .unwrap(),
            outbox: Url::parse(
                row.get::<&str>("outbox").unwrap().to_owned().as_str(),
            )
            .unwrap(),
            following: Url::parse(
                row.get::<&str>("following").unwrap().to_owned().as_str(),
            )
            .unwrap(),
            followers: Url::parse(
                row.get::<&str>("followers").unwrap().to_owned().as_str(),
            )
            .unwrap(),
            featured: Url::parse(
                row.get::<&str>("featured").unwrap().to_owned().as_str(),
            )
            .unwrap(),
            featured_tags: Url::parse(
                row.get::<&str>("featuredTags").unwrap().to_owned().as_str(),
            )
            .unwrap(),
            image_location: Url::parse(
                row.get::<&str>("imageLocation")
                    .unwrap()
                    .to_owned()
                    .as_str(),
            )
            .unwrap(),
            icon_location: Url::parse(
                row.get::<&str>("iconLocation").unwrap().to_owned().as_str(),
            )
            .unwrap(),
            published: DateTime::from_timestamp(
                row.get::<i64>("published").unwrap(),
                0,
            )
            .unwrap(),
            last_refreshed_at: DateTime::from_timestamp(
                row.get::<i64>("lastRefreshedAt").unwrap(),
                0,
            )
            .unwrap(),
        };

        Ok(user)
    }
}

struct Users;

impl Users {
    pub fn build(rowset: QueryResult) -> Vec<User> {
        fn get_fed_id(u: &str) -> String {
            let a = Url::parse(u).expect("Not Valid Url");
            if a.path() != "/" {
                return a.to_string();
            }

            format!("{}://{}", a.scheme(), a.host().unwrap())
        }

        let users: Vec<User> = rowset
            .rows()
            .map(|row| User {
                id: row.get::<u32>("id").unwrap(),
                federation_id: get_fed_id(
                    row.get::<&str>("federationId").unwrap(),
                ),
                name: row.get::<&str>("name").unwrap().to_owned(),
                display_name: row
                    .get::<&str>("displayName")
                    .unwrap()
                    .to_owned(),
                email: Some(row.get::<&str>("email").unwrap().to_owned()),
                url: Url::parse(row.get::<&str>("url").unwrap()).unwrap(),
                summary: Some(row.get::<&str>("summary").unwrap().to_owned()),
                local: row.get::<bool>("local").unwrap(),
                discoverable: row.get::<bool>("discoverable").unwrap(),
                manually_approves_followers: row
                    .get::<bool>("manuallyApprovesFollowers")
                    .unwrap(),
                indexable: row.get::<bool>("indexable").unwrap(),
                inbox: Url::parse(
                    row.get::<&str>("inbox").unwrap().to_owned().as_str(),
                )
                .unwrap(),
                outbox: Url::parse(
                    row.get::<&str>("outbox").unwrap().to_owned().as_str(),
                )
                .unwrap(),
                following: Url::parse(
                    row.get::<&str>("following").unwrap().to_owned().as_str(),
                )
                .unwrap(),
                followers: Url::parse(
                    row.get::<&str>("followers").unwrap().to_owned().as_str(),
                )
                .unwrap(),
                featured: Url::parse(
                    row.get::<&str>("featured").unwrap().to_owned().as_str(),
                )
                .unwrap(),
                featured_tags: Url::parse(
                    row.get::<&str>("featuredTags")
                        .unwrap()
                        .to_owned()
                        .as_str(),
                )
                .unwrap(),
                image_location: Url::parse(
                    row.get::<&str>("imageLocation")
                        .unwrap()
                        .to_owned()
                        .as_str(),
                )
                .unwrap(),
                icon_location: Url::parse(
                    row.get::<&str>("iconLocation")
                        .unwrap()
                        .to_owned()
                        .as_str(),
                )
                .unwrap(),
                published: DateTime::from_timestamp(
                    row.get::<i64>("published").unwrap(),
                    0,
                )
                .unwrap(),
                last_refreshed_at: DateTime::from_timestamp(
                    row.get::<i64>("lastRefreshedAt").unwrap(),
                    0,
                )
                .unwrap(),
            })
            .collect();
        users
    }
}

pub async fn foo() {
    println!("rasasarars");
}

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
        let u = User::get_with_name("seungjin").await.unwrap();
        println!("{:?}", u);
    }
}
