use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Reqwest(#[from] reqwest::Error),
    SerdeJson(#[from] serde_json::Error),
    #[error("discord api error")]
    Discord(u32),
}
