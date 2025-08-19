use crate::{MarketClient, market_error::MarketError, misc::user::User};

impl MarketClient {
    /// Get information about particular user.
    pub async fn get_user(&self, slug: &str) -> Result<User, MarketError> {
        self.get(&format!("user/{slug}")).await
    }
}
