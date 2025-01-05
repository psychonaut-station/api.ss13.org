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
    #[error("failed to parse param: {0} {1}")]
    ParseParam(&'static str, String),
    #[allow(dead_code)]
    #[error("unknown param: {0}")]
    UnknownParam(String),
}
