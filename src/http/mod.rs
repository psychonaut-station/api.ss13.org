use std::sync::Arc;

use once_cell::sync::Lazy;
use reqwest::Client;

pub mod byond;
pub mod discord;
mod error;

pub use error::Error;

pub static REQWEST_CLIENT: Lazy<Arc<Client>> = Lazy::new(|| Arc::new(Client::new()));
