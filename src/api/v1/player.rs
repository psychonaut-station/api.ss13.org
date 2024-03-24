use rocket::{get, http::Status, State};
use serde_json::{json, Value};

use crate::{
    database::{error::Error as DatabaseError, get_player, get_top_roletime},
    Database,
};

use super::{common::ApiKey, GenericResponse};

#[get("/player?<ckey>")]
pub async fn index(
    ckey: &str,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<GenericResponse<Value>, Status> {
    let player = match get_player(ckey, &database.pool).await {
        Ok(player) => player,
        Err(err) => match err {
            DatabaseError::PlayerNotFound => return Err(Status::NotFound),
            _ => return Err(Status::InternalServerError),
        },
    };

    Ok(GenericResponse::Success(json!({
        "ckey": player.ckey,
        "byond_key": player.byond_key,
        "first_seen": player.first_seen.to_string(),
        "last_seen": player.last_seen.to_string(),
        "first_seen_round": player.first_seen_round,
        "last_seen_round": player.last_seen_round,
        "ip": player.ip,
        "cid": player.cid,
        "byond_age": player.byond_age.to_string(),
    })))
}

#[get("/player/top?<job>")]
pub async fn top(
    job: &str,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<GenericResponse<Value>, Status> {
    let roletimes = match get_top_roletime(job, &database.pool).await {
        Ok(roletimes) => roletimes,
        Err(_) => return Err(Status::InternalServerError),
    };

    Ok(GenericResponse::Success(json!(roletimes)))
}
