use rocket::{catchers, routes, Build, Rocket};
use std::sync::Arc;
use tokio::sync::Mutex;

mod autocomplete;
mod common;
mod player;
mod server;
mod verify;

pub use common::*;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let cache = Arc::new(Mutex::new(Cache::default()));

    rocket
        .manage(cache)
        .register("/v1", catchers![common::default_catcher])
        .mount(
            "/v1",
            routes![
                player::index,
                player::ban,
                player::roletime,
                player::top,
                server::index,
                verify::index,
                verify::unverify,
                autocomplete::job,
                autocomplete::ckey
            ],
        )
}
