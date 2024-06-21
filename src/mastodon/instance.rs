use chrono::format::strftime::StrftimeItems;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response},
    sqlite::{QueryResult, Value as SV},
};

use crate::mastodon::account::Account;

// https://docs.joinmastodon.org/methods/instance/#v1
// https://docs.joinmastodon.org/entities/V1_Instance

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct V1Instance {
    uri: String,
    title: String,
    short_description: String,
    description: String,
    email: String,
    version: String,
    urls: Urls,
    stats: Stats,
    thumbnail: Option<String>,
    languages: Value,
    registration: bool,
    apporval_required: bool,
    invites_enabled: bool,
    configuration: Value,
    contact_account: Option<Account>,
    rules: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Urls {
    streaming_api: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Stats {
    user_count: u64,
    status_count: u64,
    domain_count: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
struct MediaAttachment {
    supported_mime_types: Vec<String>,
    image_size_limit: u64,
    image_matrix_limit: u64,
    video_size_limit: u64,
    video_frame_rate_limit: u32,
    video_matrix_limit: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
struct Polls {
    max_options: usize,
    max_characters_per_option: usize,
    min_expiration: usize,
    max_expiration: usize,
}

impl V1Instance {
    pub async fn get() -> Self {
        // let local_instance = crate::table::instance::InstanceType::Local;
        // let table =
        //     crate::table::instance::Instance::get(local_instance).await;

        // let stats = Stats {
        //     user_count: 1,
        //     status_count: 1,
        //     domain_count: 1,
        // };

        // let urls = Urls {
        //     streaming_api: format!("wss://{}", table.domain),
        // };

        // let instance = V1Instance {
        //     uri: table.domain,
        //     title: table.title,
        //     short_description: table.description,
        //     email: "".to_string(),
        //     version: table.version,
        //     urls: urls,
        //     stats: stats,
        //     thumbnail: None,
        //     languages: None,
        //     registration: false,
        //     apporval_required: false,
        //     invites_enabled: false,
        //     configuration: Configuration,
        //     contact_account: Account,
        //     rules: None,
        // };

        todo!();
    }
}
