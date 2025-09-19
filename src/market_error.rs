use std::collections::HashMap;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum MarketError {
    NoData(HashMap<String, Vec<String>>),
    Reqwest(reqwest::Error),
    ReqwestHeaderToStr(reqwest::header::ToStrError),
    SerdeJson(serde_json::Error),
    SignInFailed,
    TooManyRequests,
    Unauthorized,
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
            MarketError::ReqwestHeaderToStr(_) => {
                write!(f, "reqwest header could not be converted to str")
            }
            MarketError::SerdeJson(_) => {
                write!(f, "the string could not be deserialized")
            }
            MarketError::SignInFailed => {
                write!(f, "sign in failed")
            }
            MarketError::TooManyRequests => {
                write!(f, "too many requests; rate limit exceeded")
            }
            MarketError::Unauthorized => {
                write!(f, "missing authorization header and token")
            }
        }
    }
}

impl error::Error for MarketError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            MarketError::Reqwest(ref error) => Some(error),
            MarketError::ReqwestHeaderToStr(ref error) => Some(error),
            MarketError::SerdeJson(ref error) => Some(error),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for MarketError {
    fn from(value: reqwest::Error) -> Self {
        MarketError::Reqwest(value)
    }
}

impl From<reqwest::header::ToStrError> for MarketError {
    fn from(value: reqwest::header::ToStrError) -> Self {
        MarketError::ReqwestHeaderToStr(value)
    }
}

impl From<serde_json::Error> for MarketError {
    fn from(value: serde_json::Error) -> Self {
        MarketError::SerdeJson(value)
    }
}
