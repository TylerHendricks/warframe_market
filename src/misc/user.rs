use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{misc::achievement::Achievement, orders_data::user_short::Activity};

/// Supported platforms.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Mobile,
    #[default]
    Pc,
    Ps4,
    Switch,
    Xbox,
}

/// Online status of user.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    #[default]
    Invisible,
    Offline,
    Online,
    Ingame,
}

/// Public user profile.
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
    pub platform: Platform,
    pub crossplay: bool,
    pub locale: String,
    pub achievement_showcase: Option<Vec<Achievement>>,
    pub status: Status,
    pub activity: Activity,
    pub last_seen: String,
    pub banned: Option<bool>,
    pub ban_until: Option<String>,
    pub warned: Option<bool>,
    pub warned_message: Option<String>,
    pub ban_message: Option<String>,
}

/// Role on the site.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    #[default]
    Anonymous,
    User,
    Moderator,
    Admin,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename = "user")]
pub struct CurrentUserContainer {
    pub user: CurrentUser,
}

/// The user that is currently signed in.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CurrentUser {
    pub anonymous: bool,
    pub avatar: Option<String>,
    pub background: Option<String>,
    pub ban_reason: Option<String>,
    pub banned: bool,
    pub check_code: String,
    pub crossplay: bool,
    pub has_mail: bool,
    pub id: String,
    pub ingame_name: String,
    pub linked_accounts: HashMap<String, bool>,
    pub locale: String,
    pub platform: Platform,
    pub region: String,
    pub reputation: i16,
    pub role: Role,
    pub slug: String,
    pub unread_messages: i16,
    pub verification: bool,
    pub written_reviews: i16,
}

/// Subscription tier.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    #[default]
    None,
    Bronze,
    Silver,
    Gold,
    Diamond,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPrivate {
    pub about: Option<String>,
    pub about_raw: Option<String>,
    pub achievement_showcase: Option<Vec<Achievement>>,
    pub avatar: Option<String>,
    pub background: Option<String>,
    pub ban_message: Option<String>,
    pub ban_until: Option<String>,
    pub banned: Option<bool>,
    pub check_code: String,
    pub created_at: String,
    pub credits: i32,
    pub crossplay: bool,
    pub delete_at: Option<String>,
    pub delete_in_progress: Option<bool>,
    pub has_email: bool,
    pub id: String,
    pub ignore_list: Option<Vec<String>>,
    pub ingame_name: String,
    pub last_seen: String,
    pub linked_accounts: HashMap<String, bool>,
    pub locale: String,
    pub mastery_rank: i8,
    pub platform: Platform,
    pub reputation: i16,
    pub reviews_left: i16,
    pub role: Role,
    pub slug: String,
    pub subscription: bool,
    pub sync_locale: bool,
    pub sync_theme: bool,
    pub theme: Option<String>,
    pub tier: Tier,
    pub unread_messages: Option<i16>,
    pub unread_notifications: i16,
    pub verification: bool,
    pub warn_message: Option<String>,
    pub warned: Option<bool>,
}
