use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RivenAttribute {
    pub id: String,
    pub slug: String,
    pub game_ref: String,
    pub group: Option<String>,
    pub prefix: String,
    pub suffix: String,
    pub exlusive_to: Option<Vec<String>>,
    pub positive_is_negative: Option<bool>,
    pub unit: Option<String>,
    pub positive_only: Option<bool>,
    pub negative_only: Option<bool>,
    pub i18n: Option<HashMap<String, RivenAttributeI18N>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RivenAttributeI18N {
    pub name: String,
    pub icon: String,
    pub thumb: String,
}
