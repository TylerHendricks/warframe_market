use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::lich::lich_ephemera::LichEphemeraI18N;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SisterEphemera {
    pub id: String,
    pub slug: String,
    pub game_ref: String,
    pub animation: String,
    pub element: String,
    pub i18n: Option<HashMap<String, LichEphemeraI18N>>,
}
