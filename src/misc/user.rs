use serde::{Deserialize, Serialize};

use crate::{misc::achievement::Achievement, orders_data::user_short::Activity};

/// Public user profile
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub ingame_name: String,
    pub avatar: Option<String>,
    pub background: Option<String>,
    pub about: Option<String>,
    pub reputation: i16,
    pub mastery_level: Option<i8>,
    pub platform: String,
    pub crossplay: bool,
    pub locale: String,
    pub achievement_showcase: Option<Vec<Achievement>>,
    pub status: String,
    pub activity: Activity,
    pub last_seen: String,
    pub banned: Option<bool>,
    pub ban_until: Option<String>,
    pub warned: Option<bool>,
    pub warned_message: Option<String>,
    pub ban_message: Option<String>,
}
