use std::collections::HashMap;

use crate::{MarketClient, market_error::MarketError, misc::achievement::Achievement};

impl MarketClient {
    /// Get list of all available achievements, except secret ones.
    pub async fn get_achievements(&self) -> Result<Vec<Achievement>, MarketError> {
        self.get("achievements", None, false).await
    }

    /// Get a list of all achievements for username.
    ///
    /// Query Parameters:
    ///
    /// featured `bool` - Return only featured: true achievements.
    pub async fn get_achievements_user(
        &self,
        slug: &str,
        query: Option<&HashMap<String, String>>,
    ) -> Result<Vec<Achievement>, MarketError> {
        if let Some(q) = query {
            self.get(&format!("achievements/user/{slug}"), Some(q), false)
                .await
        } else {
            self.get(&format!("achievements/user/{slug}"), None, false)
                .await
        }
    }

    /// Get a list of all achievements for user id.
    ///
    /// Query Parameters:
    ///
    /// featured `bool` - Return only featured: true achievements.
    pub async fn get_achievements_user_id(
        &self,
        user_id: &str,
        query: Option<&HashMap<String, String>>,
    ) -> Result<Vec<Achievement>, MarketError> {
        if let Some(q) = query {
            self.get(&format!("achievements/userId/{user_id}"), Some(q), false)
                .await
        } else {
            self.get(&format!("achievements/userId/{user_id}"), None, false)
                .await
        }
    }
}
