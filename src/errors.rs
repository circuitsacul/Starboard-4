use thiserror::Error;

pub type StarboardResult<T> = Result<T, StarboardError>;

#[derive(Error, Debug)]
pub enum StarboardError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    TwilightHttp(#[from] twilight_http::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}
