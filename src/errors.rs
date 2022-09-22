use std::{error::Error, fmt::Display};

pub type StarboardResult<T> = Result<T, StarboardError>;

#[derive(Debug)]
pub enum StarboardError {
    Sqlx(sqlx::Error),
    Serde(serde_json::Error),
    TwilightHttp(twilight_http::Error),
    Reqwest(reqwest::Error),
}

impl Display for StarboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sqlx(err) => err.fmt(f),
            Self::Serde(err) => err.fmt(f),
            Self::TwilightHttp(err) => err.fmt(f),
            Self::Reqwest(err) => err.fmt(f),
        }
    }
}

impl Error for StarboardError {}

impl From<sqlx::Error> for StarboardError {
    fn from(err: sqlx::Error) -> Self {
        Self::Sqlx(err)
    }
}

impl From<serde_json::Error> for StarboardError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serde(err)
    }
}

impl From<twilight_http::Error> for StarboardError {
    fn from(err: twilight_http::Error) -> Self {
        Self::TwilightHttp(err)
    }
}

impl From<reqwest::Error> for StarboardError {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}
