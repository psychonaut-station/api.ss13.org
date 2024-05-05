use rocket::{http::Status, post, serde::json, State};
use serde::Deserialize;

use crate::{
    database::{error::Error, *},
    Database,
};

use super::{common::ApiKey, Json};

#[derive(Deserialize)]
pub struct VerifyData<'r> {
    discord_id: &'r str,
    one_time_token: Option<&'r str>,
    ckey: Option<&'r str>,
}

#[post("/verify", data = "<data>")]
pub async fn index(
    data: json::Json<VerifyData<'_>>,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<Json<Option<String>>, Status> {
    if let Some(one_time_token) = data.one_time_token {
        match verify_discord(data.discord_id, one_time_token, &database.pool).await {
            Ok(ckey) => Ok(Json::Ok(Some(ckey))),
            Err(Error::DiscordAlreadyLinked(ckey)) => Ok(Json::Conflict(Some(ckey))),
            Err(Error::CkeyAlreadyLinked(discord_id)) => {
                Ok(Json::Conflict(Some(format!("@{discord_id}"))))
            }
            Err(Error::TokenInvalid) => Err(Status::NotFound),
            Err(_) => Err(Status::InternalServerError),
        }
    } else if let Some(ckey) = data.ckey {
        match force_verify_discord(data.discord_id, ckey, &database.pool).await {
            Ok(_) => Ok(Json::Ok(None)),
            Err(Error::DiscordAlreadyLinked(ckey)) => Ok(Json::Conflict(Some(ckey))),
            Err(Error::CkeyAlreadyLinked(discord_id)) => {
                Ok(Json::Conflict(Some(format!("@{discord_id}"))))
            }
            Err(Error::PlayerNotFound) => Err(Status::NotFound),
            Err(_) => Err(Status::InternalServerError),
        }
    } else {
        Err(Status::BadRequest)
    }
}

#[derive(Deserialize)]
pub struct UnverifyData<'r> {
    discord_id: Option<&'r str>,
    ckey: Option<&'r str>,
}

#[post("/unverify", data = "<data>")]
pub async fn unverify(
    data: json::Json<UnverifyData<'_>>,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<Json<String>, Status> {
    if data.discord_id.is_some() ^ data.ckey.is_none() {
        return Err(Status::BadRequest);
    }

    match unverify_discord(data.discord_id, data.ckey, &database.pool).await {
        Ok(account) => Ok(Json::Ok(account)),
        Err(Error::PlayerNotFound) => Err(Status::NotFound),
        Err(Error::NotLinked) => Err(Status::Conflict),
        Err(_) => Err(Status::InternalServerError),
    }
}
