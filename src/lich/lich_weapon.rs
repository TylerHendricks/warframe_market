use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LichWeapon {
    pub id: String,
    pub slug: String,
    pub game_ref: String,
    pub req_mastery_rank: i8,
    pub i18n: Option<HashMap<String, LichWeaponI18N>>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LichWeaponI18N {
    pub name: String,
    pub wiki_link: Option<String>,
    pub icon: String,
    pub thumb: String,
}
