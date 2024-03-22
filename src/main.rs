use rocket::{get, http::Method, routes, Config as RocketConfig};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions, Error as CorsError};
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
        ..Default::default()
    };

    let rocket = rocket::custom(rocket_config)
        .attach(cors()?)
        .manage(config)
        .mount("/", routes![index]);

    let rocket = api::mount(rocket);

    rocket.launch().await?;

    Ok(())
}

fn cors() -> Result<Cors, CorsError> {
    let allowed_origins = AllowedOrigins::some_regex(&["^https?://.+"]);
    let allowed_methods = vec![Method::Get].into_iter().map(From::from).collect();
    let allowed_headers =
        AllowedHeaders::some(&["Accept", "Authorization", "Content-Type", "X-CSRF-Token"]);
    let expose_headers = vec!["Link".to_string()].into_iter().collect();

    CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers,
        expose_headers,
        allow_credentials: false,
        max_age: Some(300),
        ..Default::default()
    }
    .to_cors()
}

#[derive(Debug, Error)]
#[error(transparent)]
enum Error {
    Config(#[from] config::Error),
    Cors(#[from] rocket_cors::Error),
    Rocket(#[from] rocket::Error),
}
