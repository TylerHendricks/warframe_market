use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LichQuirk {
    pub id: String,
    pub slug: String,
    pub group: Option<String>,
    pub i18n: Option<HashMap<String, LichQuirkI18N>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LichQuirkI18N {
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub thumb: Option<String>,
}
