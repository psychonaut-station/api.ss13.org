use rocket::{routes, Build, Rocket};

mod common;
mod server;

pub use common::*;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/v1", routes![self::server::route])
}
