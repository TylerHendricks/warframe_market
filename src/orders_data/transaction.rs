use serde::{Deserialize, Serialize};

use crate::{Type, UserShort};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionItem {
    pub id: Option<String>,
    pub rank: Option<i32>,
    pub charges: Option<i32>,
    pub subtype: Option<String>,
    pub cyan_stars: Option<i32>,
    pub amber_stars: Option<i32>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: String,
    pub r#type: Type,
    pub origin_id: String,
    pub platinum: i32,
    pub quantity: i32,
    pub created_at: String,
    pub updated_at: String,
    pub item: Option<TransactionItem>,
    pub user: Option<UserShort>,
}
