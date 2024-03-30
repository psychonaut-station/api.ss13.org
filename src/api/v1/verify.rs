use rocket::{http::Status, post, serde::json::Json, State};
use serde::Deserialize;

use crate::{
    database::{error::Error as DatabaseError, *},
    Database,
};

use super::{common::ApiKey, GenericResponse};

#[derive(Deserialize)]
pub struct VerifyData<'r> {
    discord_id: &'r str,
    one_time_token: &'r str,
}

#[post("/verify", data = "<data>")]
pub async fn index(
    data: Json<VerifyData<'_>>,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<GenericResponse<String>, Status> {
    match verify_discord(data.discord_id, data.one_time_token, &database.pool).await {
        Ok(ckey) => Ok(GenericResponse::Success(ckey)),
        Err(DatabaseError::AlreadyLinked(ckey)) => Ok(GenericResponse::Conflict(Some(ckey))),
        Err(DatabaseError::TokenInUse(discord_id)) => {
            Ok(GenericResponse::Conflict(Some(format!("@{discord_id}"))))
        }
        Err(DatabaseError::TokenInvalid) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[derive(Deserialize)]
pub struct UnverifyData<'r> {
    discord_id: Option<&'r str>,
    ckey: Option<&'r str>,
}

#[post("/unverify", data = "<data>")]
pub async fn unverify(
    data: Json<UnverifyData<'_>>,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<GenericResponse<String>, Status> {
    if data.discord_id.is_some() ^ data.ckey.is_none() {
        return Err(Status::BadRequest);
    }

    match unverify_discord(data.discord_id, data.ckey, &database.pool).await {
        Ok(account) => Ok(GenericResponse::Success(account)),
        Err(DatabaseError::NotLinked) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}
