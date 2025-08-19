use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::lich::lich_weapon::LichWeaponI18N;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SisterWeapon {
    pub id: String,
    pub slug: String,
    pub game_ref: String,
    pub req_mastery_rank: i8,
    pub i18n: Option<HashMap<String, LichWeaponI18N>>,
}
