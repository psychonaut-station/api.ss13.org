use rocket::{catchers, routes, Build, Rocket};

mod autocomplete;
mod common;
mod player;
mod server;
mod verify;

pub use common::*;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .register("/v1", catchers![common::default_catcher])
        .mount(
            "/v1",
            routes![
                player::index,
                player::ban,
                player::roletime,
                player::top,
                player::discord,
                server::index,
                verify::index,
                verify::unverify,
                autocomplete::job,
                autocomplete::ckey,
                autocomplete::ic_name
            ],
        )
}
