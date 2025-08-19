use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Versions {
    pub apps: HashMap<String, String>,
    pub collections: HashMap<String, String>,
    pub updated_at: String,
}
