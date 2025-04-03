use std::{
    str::FromStr,
    sync::{Arc, LazyLock},
    time::{Duration, Instant},
};

use color_eyre::eyre::eyre;
use poem_openapi::{Enum, Object, Union};
use tokio::sync::RwLock;
use tracing::warn;

use crate::config::{Config, Server};

use super::{Response, topic};

#[derive(Clone, Default, Enum)]
pub(crate) enum GameState {
    #[default]
    #[oai(rename = "0")]
    Startup,
    #[oai(rename = "1")]
    Pregame,
    #[oai(rename = "2")]
    SettingUp,
    #[oai(rename = "3")]
    Playing,
    #[oai(rename = "4")]
    Finished,
}

impl FromStr for GameState {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(GameState::Startup),
            "1" => Ok(GameState::Pregame),
            "2" => Ok(GameState::SettingUp),
            "3" => Ok(GameState::Playing),
            "4" => Ok(GameState::Finished),
            _ => Err(eyre!("Unknown game state: {s}")),
        }
    }
}

#[derive(Clone, Default, Enum)]
#[oai(rename_all = "snake_case")]
pub(crate) enum SecurityLevel {
    #[default]
    Green,
    Blue,
    Red,
    Delta,
}

impl FromStr for SecurityLevel {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "green" => Ok(SecurityLevel::Green),
            "blue" => Ok(SecurityLevel::Blue),
            "red" => Ok(SecurityLevel::Red),
            "delta" => Ok(SecurityLevel::Delta),
            _ => Err(eyre!("Unknown security level: {s}")),
        }
    }
}

#[derive(Default, Enum)]
#[oai(rename_all = "snake_case")]
enum ShuttleMode {
    #[default]
    Idle,
    Igniting,
    Recallled,
    Called,
    Docked,
    Stranded,
    Disabled,
    Escape,
    #[oai(rename = "endgame: game over")]
    Endgame,
    Recharging,
    Landing,
}

impl FromStr for ShuttleMode {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "idle" => Ok(ShuttleMode::Idle),
            "igniting" => Ok(ShuttleMode::Igniting),
            "recallled" => Ok(ShuttleMode::Recallled),
            "called" => Ok(ShuttleMode::Called),
            "docked" => Ok(ShuttleMode::Docked),
            "stranded" => Ok(ShuttleMode::Stranded),
            "disabled" => Ok(ShuttleMode::Disabled),
            "escape" => Ok(ShuttleMode::Escape),
            "endgame%3a+game+over" => Ok(ShuttleMode::Endgame),
            "recharging" => Ok(ShuttleMode::Recharging),
            "landing" => Ok(ShuttleMode::Landing),
            _ => Err(eyre!("Unknown shuttle mode: {s}")),
        }
    }
}

#[derive(Default)]
struct FullStatus {
    pub version: String,
    pub respawn: bool,
    pub enter: bool,
    pub ai: bool,
    pub host: String,
    pub round_id: u32,
    pub players: u32,
    pub revision: String,
    pub revision_data: String,
    pub hub: bool,
    pub identifier: bool,
    pub admins: u32,
    pub gamestate: GameState,
    pub map_name: String,
    pub security_level: SecurityLevel,
    pub round_duration: u32,
    pub time_dilation_current: f32,
    pub time_dilation_avg: f32,
    pub time_dilation_avg_slow: f32,
    pub time_dilation_avg_fast: f32,
    pub soft_popcap: u32,
    pub hard_popcap: u32,
    pub extreme_popcap: u32,
    pub popcap: bool,
    pub bunkered: bool,
    pub interviews: bool,
    pub shuttle_mode: ShuttleMode,
    pub shuttle_timer: u32,
}

async fn status(address: &str) -> color_eyre::Result<FullStatus> {
    let response = topic(address, "?status").await?;

    if let Response::String(response) = response {
        let mut status = FullStatus::default();

        for params in response.split('&') {
            let mut split = params.splitn(2, '=');
            let key = split
                .next()
                .ok_or(eyre!("Missing key in status response"))?;
            let value = split.next().unwrap_or("");

            match key {
                "version" => status.version = value.to_string(),
                "respawn" => status.respawn = value == "1",
                "enter" => status.enter = value == "1",
                "ai" => status.ai = value == "1",
                "host" => status.host = value.to_string(),
                "round_id" => status.round_id = value.parse()?,
                "players" => status.players = value.parse()?,
                "revision" => status.revision = value.to_string(),
                "revision_date" => status.revision_data = value.to_string(),
                "hub" => status.hub = value == "1",
                "identifier" => status.identifier = value == "1",
                "admins" => status.admins = value.parse()?,
                "gamestate" => status.gamestate = value.parse()?,
                "map_name" => status.map_name = value.replace('+', " "),
                "security_level" => status.security_level = value.parse()?,
                "round_duration" => status.round_duration = value.parse()?,
                "time_dilation_current" => status.time_dilation_current = value.parse()?,
                "time_dilation_avg" => status.time_dilation_avg = value.parse()?,
                "time_dilation_avg_slow" => status.time_dilation_avg_slow = value.parse()?,
                "time_dilation_avg_fast" => status.time_dilation_avg_fast = value.parse()?,
                "soft_popcap" => status.soft_popcap = value.parse()?,
                "hard_popcap" => status.hard_popcap = value.parse()?,
                "extreme_popcap" => status.extreme_popcap = value.parse()?,
                "popcap" => status.popcap = value == "1",
                "bunkered" => status.bunkered = value == "1",
                "interviews" => status.interviews = value == "1",
                "shuttle_mode" => status.shuttle_mode = value.parse()?,
                "shuttle_timer" => status.shuttle_timer = value.parse()?,
                _ => {
                    warn!("Status topic responsed with unknown param: {key} = {value} ({address})");
                }
            }
        }

        return Ok(status);
    }

    Err(eyre!("Unexpected response type: {response:?}"))
}

type ServerStatusCache = Option<(Instant, Vec<ServerStatus>)>;

static LAST_SERVER_STATUS: LazyLock<Arc<RwLock<ServerStatusCache>>> =
    LazyLock::new(|| Arc::new(RwLock::new(None)));

#[derive(Clone, Union)]
pub(crate) enum ServerStatus {
    Good(GoodServerStatus),
    Bad(BadServerStatus),
}

impl ServerStatus {
    fn new(server: &Server, status: Option<FullStatus>) -> Self {
        match status {
            Some(status) => Self::Good(GoodServerStatus {
                server_status: 1,
                name: server.name.clone(),
                round_id: status.round_id,
                players: status.players,
                map: status.map_name,
                security_level: status.security_level,
                round_duration: status.round_duration,
                gamestate: status.gamestate,
                connection_info: server.connection_address.clone(),
            }),
            None => Self::Bad(BadServerStatus {
                server_status: 0,
                name: server.name.clone(),
                err_str: server.error_message.clone(),
            }),
        }
    }
}

#[derive(Clone, Object)]
pub(crate) struct GoodServerStatus {
    #[oai(validator(minimum(value = "0"), maximum(value = "1")))]
    server_status: i8,
    name: String,
    round_id: u32,
    players: u32,
    map: String,
    security_level: SecurityLevel,
    round_duration: u32,
    gamestate: GameState,
    connection_info: String,
}

#[derive(Clone, Object)]
pub(crate) struct BadServerStatus {
    #[oai(validator(minimum(value = "0"), maximum(value = "1")))]
    server_status: i8,
    name: String,
    err_str: String,
}

pub(crate) async fn get_server_status(config: &Config) -> Vec<ServerStatus> {
    {
        let last_server_status = LAST_SERVER_STATUS.read().await;
        if let Some((last_update, server_status)) = &*last_server_status {
            if last_update.elapsed() < Duration::from_secs(30) {
                return server_status.clone();
            }
        }
    }

    let mut should_cache = false;
    let mut response = Vec::new();

    for server in config.servers.iter() {
        let status = status(&server.address).await.ok();

        if !should_cache && status.is_some() {
            should_cache = true;
        }

        response.push(ServerStatus::new(server, status));
    }

    if should_cache {
        let mut last_server_status = LAST_SERVER_STATUS.write().await;
        *last_server_status = Some((Instant::now(), response.clone()));
    }

    response
}
