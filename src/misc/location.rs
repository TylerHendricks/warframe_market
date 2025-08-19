use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub id: String,
    pub slug: String,
    pub game_ref: String,
    pub faction: Option<String>,
    pub min_level: Option<i32>,
    pub max_level: Option<i32>,
    pub i18n: Option<HashMap<String, LocationI18N>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationI18N {
    pub node_name: String,
    pub system_name: Option<String>,
    pub icon: String,
    pub thumb: String,
}
