use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Riven {
    pub id: String,
    pub slug: String,
    pub game_ref: String,
    pub group: Option<String>,
    pub riven_type: Option<String>,
    pub disposition: f64,
    pub req_mastery_rank: i8,
    pub i18n: Option<HashMap<String, RivenI18NJson>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RivenI18NJson {
    pub name: Option<String>,
    pub wiki_link: Option<String>,
    pub icon: String,
    pub thumb: String,
}
