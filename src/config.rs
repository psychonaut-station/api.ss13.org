use std::{collections::HashSet, fs::read_to_string, net::IpAddr};

use color_eyre::eyre::Context;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct Config {
    pub address: IpAddr,
    pub port: u16,
    pub public_address: String,
    pub secret: String,
    pub dev_secret: String,
    pub dev_routes: HashSet<String>,
    pub exposed_secret: String,
    pub exposed_routes: HashSet<String>,
    pub discord: Discord,
    pub database: Database,
    pub servers: Vec<Server>,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct Discord {
    pub token: String,
    pub guild: i64,
    pub patreon_role: i64,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct Database {
    pub user: String,
    pub password: String,
    pub host: IpAddr,
    pub port: u16,
    pub database: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
pub struct Server {
    pub name: String,
    pub address: String,
    pub connection_address: String,
    pub error_message: String,
}

impl Config {
    pub fn read_from_file() -> color_eyre::Result<Self> {
        toml::from_str(&read_to_string("config.toml").context("Config file not found")?)
            .context("Failed to parse config file")
    }
}
