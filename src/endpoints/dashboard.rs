use crate::{MarketClient, market_error::MarketError, misc::dashboard_showcase::DashboardShowcase};

impl MarketClient {
    /// Get mobile app main screen dashboard with featured items.
    pub async fn get_dashboard_showcase(&self) -> Result<DashboardShowcase, MarketError> {
        self.get("dashboard/showcase", None, false).await
    }
}
