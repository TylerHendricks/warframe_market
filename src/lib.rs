use std::collections::HashMap;

use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::market_error::MarketError;

mod endpoints;
mod lich;
mod market_error;
mod misc;
mod orders_data;
mod riven;
mod sister;

pub use lich::lich_ephemera::*;
pub use lich::lich_quirks::*;
pub use lich::lich_weapon::*;
pub use misc::achievement::*;
pub use misc::dashboard_showcase::*;
pub use misc::item::*;
pub use misc::item_set::*;
pub use misc::location::*;
pub use misc::mission::*;
pub use misc::npc::*;
pub use misc::user::*;
pub use misc::versions::*;
pub use orders_data::order::*;
pub use orders_data::order_with_user::*;
pub use orders_data::user_short::*;
pub use riven::riven_attribute::*;
pub use riven::riven_item::*;
pub use sister::sister_ephemera::*;
pub use sister::sister_quirk::*;
pub use sister::sister_weapon::*;

pub const BASE_URL: &str = "https://api.warframe.market/v2/";
pub const ASSETS_URL: &str = "https://warframe.market/static/assets/";

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MarketResponse<T> {
    api_version: String,
    data: Option<T>,
    error: Option<HashMap<String, Vec<String>>>,
}

#[derive(Default)]
pub struct MarketClient {
    headers: HeaderMap,
    reqwest_client: reqwest::Client,
}

impl MarketClient {
    pub fn new() -> MarketClient {
        Self {
            reqwest_client: reqwest::Client::new(),
            headers: HeaderMap::new(),
        }
    }

    pub fn headers(&mut self, headers: HeaderMap) {
        self.headers = headers;
    }

    async fn get<T>(&self, endpoint: &str) -> Result<T, MarketError>
    where
        T: DeserializeOwned,
    {
        self.get_helper(endpoint, None).await
    }

    async fn get_with_query<T>(
        &self,
        endpoint: &str,
        query: &HashMap<String, String>,
    ) -> Result<T, MarketError>
    where
        T: DeserializeOwned,
    {
        self.get_helper(endpoint, Some(query)).await
    }

    async fn get_helper<T>(
        &self,
        endpoint: &str,
        query: Option<&HashMap<String, String>>,
    ) -> Result<T, MarketError>
    where
        T: DeserializeOwned,
    {
        let url = format!("{BASE_URL}{endpoint}");

        let mut request_builder = self.reqwest_client.get(url).headers(self.headers.clone());

        if let Some(value) = query {
            request_builder = request_builder.query(value);
        }

        let json = request_builder.send().await?.text().await?;

        let response: MarketResponse<T> = serde_json::from_str(&json)?;

        match response.data {
            Some(data) => Ok(data),
            None => Err(MarketError::Api(
                response.error.expect("should be Some if data is None"),
            )),
        }
    }
}
