use std::collections::HashMap;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum MarketError {
    Api(HashMap<String, Vec<String>>),
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
}

impl fmt::Display for MarketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MarketError::Api(_) => {
                write!(f, "api request returned no data")
            }
            MarketError::Reqwest(_) => {
                write!(f, "could not process request")
            }
            MarketError::SerdeJson(_) => {
                write!(f, "the string could not be deserialized")
            }
        }
    }
}

impl error::Error for MarketError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            MarketError::Api(_) => None,
            MarketError::Reqwest(ref error) => Some(error),
            MarketError::SerdeJson(ref error) => Some(error),
        }
    }
}

impl From<reqwest::Error> for MarketError {
    fn from(value: reqwest::Error) -> Self {
        MarketError::Reqwest(value)
    }
}

impl From<serde_json::Error> for MarketError {
    fn from(value: serde_json::Error) -> Self {
        MarketError::SerdeJson(value)
    }
}
