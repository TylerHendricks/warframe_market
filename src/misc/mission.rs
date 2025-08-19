use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mission {
    pub id: String,
    pub slug: String,
    pub game_ref: String,
    pub i18n: Option<HashMap<String, MissionI18N>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MissionI18N {
    pub name: String,
    pub icon: Option<String>,
    pub thumb: Option<String>,
}
