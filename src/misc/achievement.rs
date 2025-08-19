use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Achievement {
    pub id: String,
    pub slug: String,
    pub r#type: String,
    pub secret: Option<bool>,
    pub reputation_bonus: Option<i32>,
    pub goal: Option<i32>,
    pub i18n: Option<HashMap<String, AchievementI18N>>,
    pub state: Option<AchievementState>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AchievementI18N {
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub thumb: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AchievementState {
    pub featured: Option<bool>,
    pub hidden: Option<bool>,
    pub progress: Option<i32>,
    pub completed_at: Option<String>,
}
