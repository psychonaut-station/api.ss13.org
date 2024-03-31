use rocket::{get, http::Status, State};

use crate::{
    config::Config,
    database::{error::Error as DatabaseError, *},
    http::discord::User,
    Database,
};

use super::{common::ApiKey, GenericResponse};

#[get("/player?<ckey>")]
pub async fn index(
    ckey: &str,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<GenericResponse<Player>, Status> {
    match get_player(ckey, &database.pool).await {
        Ok(player) => Ok(GenericResponse::Success(player)),
        Err(DatabaseError::PlayerNotFound) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/player/ban?<ckey>")]
pub async fn ban(
    ckey: &str,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<GenericResponse<Vec<Ban>>, Status> {
    match get_ban(ckey, &database.pool).await {
        Ok(bans) => Ok(GenericResponse::Success(bans)),
        Err(DatabaseError::PlayerNotFound) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/player/roletime?<ckey>")]
pub async fn roletime(
    ckey: &str,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<GenericResponse<Vec<PlayerRoletime>>, Status> {
    match get_roletime(ckey, &database.pool).await {
        Ok(roletimes) => Ok(GenericResponse::Success(roletimes)),
        Err(DatabaseError::PlayerNotFound) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/player/roletime/top?<job>")]
pub async fn top(
    job: &str,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<GenericResponse<Vec<JobRoletime>>, Status> {
    let Ok(roletimes) = get_top_roletime(job, &database.pool).await else {
        return Err(Status::InternalServerError);
    };

    Ok(GenericResponse::Success(roletimes))
}

#[get("/player/discord?<ckey>")]
pub async fn discord(
    ckey: &str,
    database: &State<Database>,
    config: &State<Config>,
    _api_key: ApiKey,
) -> Result<GenericResponse<User>, Status> {
    match fetch_discord_by_ckey(ckey, &config.discord_token, &database.pool).await {
        Ok(user) => Ok(GenericResponse::Success(user)),
        Err(DatabaseError::PlayerNotFound) => Err(Status::NotFound),
        Err(DatabaseError::NotLinked) => Err(Status::Conflict),
        Err(_) => Err(Status::InternalServerError),
    }
}
