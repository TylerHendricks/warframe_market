use std::collections::HashMap;

use reqwest::Response;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::market_error::MarketError;

mod endpoints;
mod lich;
pub mod market_error;
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
pub use orders_data::transaction::*;
pub use orders_data::user_short::*;
pub use riven::riven_attribute::*;
pub use riven::riven_item::*;
pub use sister::sister_ephemera::*;
pub use sister::sister_quirk::*;
pub use sister::sister_weapon::*;

pub const BASE_URL: &str = "https://api.warframe.market/v2/";
pub const V1_URL: &str = "https://api.warframe.market/v1/";
pub const ASSETS_URL: &str = "https://warframe.market/static/assets/";

#[derive(Serialize, Deserialize)]
struct MarketResponseV1<T> {
    payload: T,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MarketResponse<T> {
    api_version: String,
    data: Option<T>,
    error: Option<HashMap<String, Vec<String>>>,
}

#[derive(Default)]
pub struct MarketClient {
    auth_token: Option<String>,
    headers: HeaderMap,
    reqwest_client: reqwest::Client,
}

impl MarketClient {
    pub fn new() -> MarketClient {
        Self {
            auth_token: None,
            reqwest_client: reqwest::Client::new(),
            headers: HeaderMap::new(),
        }
    }

    pub fn headers(&mut self, headers: HeaderMap) {
        self.headers = headers;
    }

    async fn get<T>(
        &self,
        endpoint: &str,
        query: Option<&HashMap<String, String>>,
        needs_authorization: bool,
    ) -> Result<T, MarketError>
    where
        T: DeserializeOwned,
    {
        let url = format!("{BASE_URL}{endpoint}");

        let mut request_builder = self.reqwest_client.get(url).headers(self.headers.clone());
        if needs_authorization {
            if let Some(auth_token) = &self.auth_token {
                request_builder = request_builder.bearer_auth(auth_token);
            } else {
                return Err(MarketError::Unauthorized);
            }
        }

        if let Some(value) = query {
            request_builder = request_builder.query(value);
        }

        let response = request_builder.send().await?;

        parse_response(response).await
    }

    async fn post<T, U>(&self, endpoint: &str, json: Option<&U>) -> Result<T, MarketError>
    where
        T: DeserializeOwned,
        U: Serialize,
    {
        let url = format!("{BASE_URL}{endpoint}");

        let mut request_builder = self
            .reqwest_client
            .post(url)
            .headers(self.headers.clone())
            .json(&json);

        if let Some(auth_token) = &self.auth_token {
            request_builder = request_builder.bearer_auth(auth_token);
        } else {
            return Err(MarketError::Unauthorized);
        }

        let response = request_builder.send().await?;

        parse_response(response).await
    }
}

async fn parse_response<T>(response: Response) -> Result<T, MarketError>
where
    T: DeserializeOwned,
{
    if response.status() == 429 {
        return Err(MarketError::TooManyRequests);
    }

    let json = response.text().await?;

    let market_response: MarketResponse<T> = serde_json::from_str(&json)?;

    match market_response.data {
        Some(data) => Ok(data),
        None => Err(MarketError::NoData(
            market_response
                .error
                .expect("should be Some if data is None"),
        )),
    }
}
