use rocket::{catch, catchers, http::Status, Config as RocketConfig, Request};
use thiserror::Error;

use crate::{config::Config, cors::cors, database::Database};

mod byond;
mod config;
mod cors;
mod database;
mod http;
mod routes;
mod serde;

#[rocket::main]
async fn main() -> Result<(), Error> {
    let config = Config::read_from_file()?;
    let database = Database::new(&config.database)?;

    let provider = RocketConfig {
        address: config.address,
        port: config.port,
        cli_colors: config.cli_colors,
        log_level: config.log_level,
        ..Default::default()
    };

    let rocket = rocket::custom(provider)
        .attach(cors()?)
        .manage(config)
        .manage(database)
        .register("/", catchers![empty_catcher]);

    let rocket = routes::mount(rocket);

    rocket.launch().await?;

    Ok(())
}

#[catch(default)]
fn empty_catcher(_: Status, _: &Request) {}

#[derive(Debug, Error)]
#[error(transparent)]
enum Error {
    Config(#[from] config::Error),
    Cors(#[from] rocket_cors::Error),
    Rocket(#[from] rocket::Error),
    Sqlx(#[from] sqlx::Error),
}
