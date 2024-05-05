use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use once_cell::sync::Lazy;
use rocket::{get, State};
use serde::Serialize;
use serde_json::{json, Value};
use tokio::sync::RwLock;

use crate::{
    byond::{self, ServerStatus},
    config::{Config, Server},
};

use super::GenericResponse;

type ServerStatusCache = Option<(Instant, Vec<Status>)>;

static LAST_SERVER_STATUS: Lazy<Arc<RwLock<ServerStatusCache>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

#[derive(Debug, Clone, Serialize)]
pub struct Status(Value);

impl Status {
    fn new(server: &Server, status: Option<ServerStatus>) -> Self {
        Self(match status {
            Some(status) => json!({
                "server_status": 1,
                "name": server.name,
                "round_id": status.round_id,
                "players": status.players,
                "map": status.map_name,
                "security_level": status.security_level,
                "round_duration": status.round_duration,
                "gamestate": status.gamestate,
                "connection_info": server.connection_address,
            }),
            None => json!({
                "server_status": 0,
                "name": server.name,
                "err_str": server.error_message,
            }),
        })
    }
}

#[get("/server")]
pub async fn index(config: &State<Config>) -> GenericResponse<Vec<Status>> {
    {
        let last_server_status = LAST_SERVER_STATUS.read().await;
        if let Some((last_update, server_status)) = &*last_server_status {
            if last_update.elapsed() < Duration::from_secs(30) {
                return GenericResponse::Success(server_status.clone());
            }
        }
    }

    let mut should_cache = false;

    let mut response = Vec::new();

    for server in config.servers.iter() {
        let status = byond::status(&server.address).await.ok();

        if !should_cache && status.is_some() {
            should_cache = true;
        }

        response.push(Status::new(server, status));
    }

    if should_cache {
        let mut last_server_status = LAST_SERVER_STATUS.write().await;
        *last_server_status = Some((Instant::now(), response.clone()));
    }

    GenericResponse::Success(response)
}
