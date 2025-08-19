use serde::{Deserialize, Serialize};

use crate::orders_data::{order::Order, user_short::UserShort};

/// This is a typical order model found in most requests, including a record
/// about an owner.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderWithUser {
    #[serde(flatten)]
    pub order: Order,
    pub user: UserShort,
}
