use serde::Deserialize;
use std::{fs::read_to_string, net::IpAddr};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub address: IpAddr,
    pub port: u16,
    pub secret: String,
    pub cli_colors: bool,
    pub database: Database,
    pub servers: Vec<Server>,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub user: String,
    pub password: String,
    pub host: IpAddr,
    pub port: u16,
    pub database: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub name: String,
    pub address: String,
    pub connection_address: String,
    pub error_message: String,
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        Ok(toml::from_str(&read_to_string("Config.toml")?)?)
    }
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    Io(#[from] std::io::Error),
    Toml(#[from] toml::de::Error),
}
