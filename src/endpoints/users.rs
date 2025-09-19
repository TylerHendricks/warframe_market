use crate::{MarketClient, UserPrivate, market_error::MarketError, misc::user::User};

impl MarketClient {
    /// Getting information about current authenticated user.
    pub async fn get_me(&self) -> Result<UserPrivate, MarketError> {
        self.get("me", None, true).await
    }

    /// Get information about particular user.
    pub async fn get_user(&self, slug: &str) -> Result<User, MarketError> {
        self.get(&format!("user/{slug}"), None, false).await
    }
}
