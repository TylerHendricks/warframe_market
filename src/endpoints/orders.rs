use std::collections::HashMap;

use crate::{
    MarketClient, PostOrder, Transaction,
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
        self.get("orders/recent", None, false).await
    }

    /// Get a list of all orders for an item from users who was online within
    /// the last 7 days.
    pub async fn get_orders_item(&self, slug: &str) -> Result<Vec<OrderWithUser>, MarketError> {
        self.get(&format!("orders/item/{slug}"), None, false).await
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
        query: Option<&HashMap<String, String>>,
    ) -> Result<HashMap<String, Vec<OrderWithUser>>, MarketError> {
        if let Some(q) = query {
            self.get(&format!("orders/item/{slug}/top"), Some(q), false)
                .await
        } else {
            self.get(&format!("orders/item/{slug}/top"), None, false)
                .await
        }
    }

    /// Get public orders from specified username.
    pub async fn get_orders_user(&self, slug: &str) -> Result<Vec<Order>, MarketError> {
        self.get(&format!("orders/user/{slug}"), None, false).await
    }

    /// Get public orders from specified user id.
    pub async fn get_orders_user_id(&self, user_id: &str) -> Result<Vec<Order>, MarketError> {
        self.get(&format!("orders/userId/{user_id}"), None, false)
            .await
    }

    /// Get orders for the account that is signed in.
    pub async fn get_orders_my(&self) -> Result<Vec<Order>, MarketError> {
        self.get("orders/my", None, true).await
    }

    /// Get one particular order.
    pub async fn get_order(&self, id: &str) -> Result<Order, MarketError> {
        self.get(&format!("order/{id}"), None, false).await
    }

    /// Create a new order.
    pub async fn post_order(&self, order: &PostOrder) -> Result<Order, MarketError> {
        self.post("order", Some(order)).await
    }

    /// Close a portion or all of an existing order.
    ///
    /// Allows you to close part of an open order by specifying a quantity to
    /// reduce.
    ///
    /// For example, if your order was initially created with a quantity of 20,
    /// and you send a request to close 8 units, the remaining quantity will be
    /// 12.
    ///
    /// If you close the entire remaining quantity, the order will be
    /// considered fully closed and removed.
    ///
    /// Request Fields:
    ///
    /// quantity `i32` - The number of units to close (subtract) from the order's
    /// current quantity
    pub async fn post_order_close(
        &self,
        order_id: &str,
        quantity: i32,
    ) -> Result<Transaction, MarketError> {
        let mut map = HashMap::new();
        map.insert("Quantity", quantity);
        self.post(&format!("order/{order_id}/close"), Some(&map))
            .await
    }
}
