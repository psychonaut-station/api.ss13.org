use lazy_static::lazy_static;
use reqwest::Client;

pub mod discord;
mod error;

pub use error::Error;

lazy_static! {
    pub static ref REQWEST_CLIENT: Client = Client::new();
}
