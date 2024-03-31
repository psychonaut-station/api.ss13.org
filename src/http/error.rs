use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Reqwest(#[from] reqwest::Error),
}
