use std::collections::HashMap;

use crate::{
    MarketClient,
    market_error::MarketError,
    orders_data::{order::Order, order_with_user::OrderWithUser},
};

impl MarketClient {
    /// Get the most recent orders.
    ///
    /// 500 max, for the last 4 hours, sorted by `createdAt`
    ///
    /// Cached, with 1min refresh interval.
    pub async fn get_orders_recent(&self) -> Result<Vec<Order>, MarketError> {
        self.get("orders/recent").await
    }

    /// Get a list of all orders for an item from users who was online within
    /// the last 7 days.
    pub async fn get_orders_item(&self, slug: &str) -> Result<Vec<OrderWithUser>, MarketError> {
        self.get(&format!("orders/item/{slug}")).await
    }

    /// This endpoint is designed to fetch the top 5 buy and top 5 sell orders
    /// for a specific item, exclusively from online users.
    ///
    /// Orders are sorted by price.
    ///
    /// Query Parameters:
    ///
    /// rank `int` - Filters orders by the exact rank specified.
    ///
    /// rankLt `int` - Filters orders with a rank less than the specified value.
    ///
    /// charges `int` - Filters orders by the exact number of charges left.
    ///
    /// chargesLt `int` - Filters orders where the number of charges left is
    /// less than the specified value.
    ///
    /// amberStars `int` - Filters orders by the exact number of amber stars.
    ///
    /// amberStarsLt `int` - Filters orders where the number of amber stars is
    /// less than the specified value.
    ///
    /// cyanStars `int` - Filters orders by the exact number of cyan stars.
    ///
    /// cyanStarsLt `int` - Filters orders where the number of cyan stars is
    /// less than the specified value.
    ///
    /// subtype `string` - controls the filtering of orders based on item
    /// subtype field.
    pub async fn get_orders_item_top(
        &self,
        slug: &str,
        query: &HashMap<String, String>,
    ) -> Result<HashMap<String, Vec<OrderWithUser>>, MarketError> {
        self.get_with_query(&format!("orders/item/{slug}/top"), query)
            .await
    }

    /// Get public orders from specified username.
    pub async fn get_orders_user(&self, slug: &str) -> Result<Vec<Order>, MarketError> {
        self.get(&format!("orders/user/{slug}")).await
    }

    /// Get public orders from specified user id.
    pub async fn get_orders_user_id(&self, user_id: &str) -> Result<Vec<Order>, MarketError> {
        self.get(&format!("orders/userId/{user_id}")).await
    }

    /// Get one particular order.
    pub async fn get_order(&self, id: &str) -> Result<Order, MarketError> {
        self.get(&format!("order/{id}")).await
    }
}
