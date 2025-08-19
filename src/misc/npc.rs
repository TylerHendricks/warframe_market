use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Npc {
    pub id: String,
    pub slug: String,
    pub game_ref: String,
    pub i18n: Option<HashMap<String, NpcI18N>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NpcI18N {
    pub name: String,
    pub icon: String,
    pub thumb: String,
}
