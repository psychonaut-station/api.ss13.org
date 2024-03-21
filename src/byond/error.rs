use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    AddrParse(#[from] std::net::AddrParseError),
    Timeout(#[from] tokio::time::error::Elapsed),
    Io(#[from] std::io::Error),
    ParseInt(#[from] std::num::ParseIntError),
    ParseFloat(#[from] std::num::ParseFloatError),
    #[error("response type mismatch")]
    ResponseTypeMismatch(Vec<u8>),
    #[error("invalid response")]
    InvalidResponse(String),
    #[error("failed to parse key: {0} {1}")]
    ParseKey(String, String),
    #[error("unknown key: {0}")]
    UnknownKey(String),
}
