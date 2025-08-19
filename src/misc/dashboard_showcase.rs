use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Mobile app main screen dashboard with featured items.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardShowcase {
    pub i18n: Option<HashMap<String, DashboardShowcaseI18N>>,
    pub items: Vec<DashboardShowcaseItem>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardShowcaseI18N {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardShowcaseItem {
    pub item: String,
    pub background: String,
    pub big_card: bool,
}
