use std::collections::HashMap;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum MarketError {
    NoData(HashMap<String, Vec<String>>),
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    TooManyRequests,
}

impl fmt::Display for MarketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MarketError::NoData(_) => {
                write!(f, "api request returned no data")
            }
            MarketError::Reqwest(_) => {
                write!(f, "could not process request")
            }
            MarketError::SerdeJson(_) => {
                write!(f, "the string could not be deserialized")
            }
            MarketError::TooManyRequests => {
                write!(f, "too many requests; rate limit exceeded")
            }
        }
    }
}

impl error::Error for MarketError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            MarketError::NoData(_) => None,
            MarketError::Reqwest(ref error) => Some(error),
            MarketError::SerdeJson(ref error) => Some(error),
            MarketError::TooManyRequests => None,
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
