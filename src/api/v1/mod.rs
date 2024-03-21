use rocket::{routes, Build, Rocket};

mod common;
mod server;

pub use common::*;

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let servers = Servers(vec![Server {
        name: "Psychonaut Station",
        address: "185.198.75.209:3131",
        connection_address: "turkb.us:3131",
        error_message: "Rebooting",
    }]);

    rocket.manage(servers).mount("/v1", routes![server::route])
}
