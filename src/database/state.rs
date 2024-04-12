use std::time::Duration;

use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use urlencoding::encode;

use crate::config;

pub struct Database {
    pub pool: MySqlPool,
}

impl Database {
    pub fn new(config: &config::Database) -> Result<Self, sqlx::Error> {
        let options = MySqlPoolOptions::new()
            .max_connections(1)
            .idle_timeout(Duration::from_secs(30));

        let pool = options.connect_lazy(&format!(
            "mysql://{}:{}@{}:{}/{}",
            config.user,
            encode(&config.password),
            config.host,
            config.port,
            config.database
        ))?;

        Ok(Self { pool })
    }
}
