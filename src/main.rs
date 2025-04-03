use poem::{Server as PoemServer, listener::TcpListener};
use tracing::Level;

use crate::config::Config;

mod config;
mod routes;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let subscriber = tracing_subscriber::fmt();
    #[cfg(debug_assertions)]
    let subscriber = subscriber.with_max_level(Level::DEBUG);
    tracing::subscriber::set_global_default(subscriber.finish())?;

    color_eyre::install()?;

    let config = Config::read_from_file()?;

    let route = routes::route(&config);

    PoemServer::new(TcpListener::bind((config.address, config.port)))
        .run(route)
        .await?;

    Ok(())
}
