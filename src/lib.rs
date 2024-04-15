pub mod account; 
pub mod status;
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

use regex::Regex;

/// Identification
/// Username: Local user name. ie: seungjin, seungjin.kim
/// Account: seungjin@prefer.social, @seungjin@prefer.social
/// ActorUrl: https://seungjin.prefer.social, https:://seungjin.prefer.social/users/seungjin

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Ord)]
pub enum Identification {
    Username(String),
    Account(String),
    ActorUrl(String),
}

impl Identification {
    pub async fn get(a: &str) -> Identification {
    
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
            Identification::Account(a) => Identification::Account(a),
            Identification::Username(a) => Identification::Username(a),
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


#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn handler_type_test_1() {
        let a = "google.com";
        assert_eq!(
            Identification::get(a).await,
            Identification::Username(a.to_string())
        );
    }
    #[tokio::test]
    async fn handler_type_test_2() {
        let a = "https://google.com/foo/bar";
        assert_eq!(
            Identification::get(a).await,
            Identification::ActorUrl(a.to_string())
        );
    }
    #[tokio::test]
    async fn handler_type_test_3() {
        let a = "foo@google.com";
        assert_eq!(
            Identification::get(a).await,
            Identification::Account(a.to_string())
        );
    }
    #[tokio::test]
    async fn handler_type_test_4() {
        let a = "foo@google";
        assert_eq!(
            Identification::get(a).await,
            Identification::Username(a.to_string())
        );
    }
 }
