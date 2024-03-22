use rocket::{get, routes, Config as RocketConfig};
use thiserror::Error;

use crate::config::Config;

mod api;
mod byond;
mod config;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), Error> {
    let config = Config::load()?;

    let rocket_config = RocketConfig {
        address: config.address,
        port: config.port,
        ..RocketConfig::default()
    };

    let rocket = rocket::custom(rocket_config)
        .manage(config)
        .mount("/", routes![index]);

    let rocket = api::mount(rocket);

    rocket.launch().await?;

    Ok(())
}

#[derive(Debug, Error)]
#[error(transparent)]
enum Error {
    Config(#[from] config::Error),
    Rocket(#[from] rocket::Error),
}
