use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::lich::lich_quirks::LichQuirkI18N;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SisterQuirk {
    pub id: String,
    pub slug: String,
    pub group: Option<String>,
    pub i18n: Option<HashMap<String, LichQuirkI18N>>,
}
