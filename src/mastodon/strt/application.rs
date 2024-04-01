use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub website: Option<String>,
}
