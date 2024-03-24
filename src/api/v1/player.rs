use rocket::{get, http::Status, State};
use serde_json::{json, Value};

use crate::{
    database::{error::Error as DatabaseError, get_player},
    Database,
};

use super::common::ApiKey;

#[get("/player?<ckey>")]
pub async fn index(
    ckey: &str,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<Value, Status> {
    let player = match get_player(ckey, &database.pool).await {
        Ok(player) => player,
        Err(err) => match err {
            DatabaseError::PlayerNotFound => return Err(Status::NotFound),
            _ => return Err(Status::InternalServerError),
        },
    };

    Ok(json!({
        "ckey": player.ckey,
        "byond_key": player.byond_key,
        "first_seen": player.first_seen.to_string(),
        "last_seen": player.last_seen.to_string(),
        "first_seen_round": player.first_seen_round,
        "last_seen_round": player.last_seen_round,
        "cid": player.cid,
        "byond_age": player.byond_age.to_string(),
    }))
}
