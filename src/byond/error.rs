use thiserror::Error;

use super::Response;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    AddrParse(#[from] std::net::AddrParseError),
    Timeout(#[from] tokio::time::error::Elapsed),
    Io(#[from] std::io::Error),
    ParseInt(#[from] std::num::ParseIntError),
    ParseFloat(#[from] std::num::ParseFloatError),
    #[error("invalid response")]
    InvalidResponse,
    #[error("the response was not the expected type: {0:?}")]
    UnexpectedType(Response),
    #[error("failed to parse key: {0} {1}")]
    ParseKey(&'static str, String),
    #[error("unknown key: {0}")]
    UnknownKey(String),
}
