use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Sqlx(#[from] sqlx::Error),
    Reqwest(#[from] reqwest::Error),
    SerdeJson(#[from] serde_json::Error),
    Http(#[from] crate::http::Error),
    #[error("Player not found")]
    PlayerNotFound,
    #[error("Invalid arguments provided")]
    InvalidArguments,
    #[error("Discord account is already linked to {0}")]
    AlreadyLinked(String),
    #[error("Account is not linked")]
    NotLinked,
    #[error("Token does not exist or is invalid")]
    TokenInvalid,
    #[error("Token is already in use by {0}")]
    TokenInUse(i64),
}
