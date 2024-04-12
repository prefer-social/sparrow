use chrono::format::strftime::StrftimeItems;
use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};
use spin_sdk::{
    http::{IntoResponse, Method, Params, Request, Response},
    sqlite::{QueryResult, Value as SV},
};

use crate::mastodon::account::Account;

// https://docs.joinmastodon.org/methods/instance/#v1

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    uri: String,
    title: String,
    short_description: String,
    email: String,
    version: String,
    urls: Urls,
    stats: Stats,
    thumbnail: String,
    languages: Vec<String>,
    registration: bool,
    apporval_required: bool,
    invites_enabled: bool,
    configuration: Configuration,
    contact_account: Account,
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
struct Configuration {
    statuses: Statuses,
    media_attachments: MediaAttachment,
    polls: Polls,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Statuses {
    max_characters: u64,
    max_meda_attachments: u8,
    chracters_reserved_per_url: u8,
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
