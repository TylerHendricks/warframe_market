use serde::{Deserialize, Serialize};

use crate::misc::item::Item;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemSet {
    pub id: String,
    pub items: Vec<Item>,
}
