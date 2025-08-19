use serde::{Deserialize, Serialize};

/// This is a shortened model of a user, which can be found in chats, messages,
/// orders, bids, auctions, etc.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserShort {
    pub id: String,
    pub ingame_name: String,
    pub avatar: Option<String>,
    pub reputation: i16,
    pub locale: String,
    pub platform: String,
    pub crossplay: bool,
    pub status: String,
    pub activity: Activity,
    pub last_seen: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Activity {
    pub r#type: ActivityType,
    pub details: Option<String>,
    pub started_at: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityType {
    #[default]
    Unknown,
    Idle,
    OnMission,
    InDojo,
    InOrbiter,
    InRelay,
}
