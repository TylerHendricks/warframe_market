use serde::{Deserialize, Serialize};

/// Order type
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Type {
    #[default]
    Buy,
    Sell,
}

/// Just an order, that’s it.
///
/// Without specifying the owner, used in cases where you already know who the
/// owner of the order is, such as in a user profile or your own profile.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub id: String,
    pub r#type: Type,
    pub platinum: i32,
    pub quantity: i32,
    pub per_trade: Option<i8>,
    pub rank: Option<i8>,
    pub charges: Option<i8>,
    pub sub_type: Option<String>,
    pub amber_stars: Option<i8>,
    pub cyan_stars: Option<i8>,
    pub visible: bool,
    pub created_at: String,
    pub updated_at: String,
    pub item_id: String,
    pub group: Option<String>,
}

/// The struct required to create a new order
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostOrder {
    pub item_id: String,
    pub r#type: Type,
    pub platinum: i32,
    pub quantity: i32,
    pub per_trade: Option<i8>,
    pub rank: Option<i8>,
    pub charges: Option<i8>,
    pub sub_type: Option<String>,
    pub amber_stars: Option<i8>,
    pub cyan_stars: Option<i8>,
    pub visible: bool,
}
