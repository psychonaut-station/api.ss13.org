use rocket::{get, http::Status, State};

use crate::{
    database::{error::Error as DatabaseError, *},
    Database,
};

use super::{common::ApiKey, GenericResponse};

#[get("/player?<ckey>")]
pub async fn index(
    ckey: &str,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<GenericResponse<Player>, Status> {
    let player = match get_player(ckey, &database.pool).await {
        Ok(player) => player,
        Err(DatabaseError::PlayerNotFound) => return Err(Status::NotFound),
        Err(_) => return Err(Status::InternalServerError),
    };

    Ok(GenericResponse::Success(player))
}

#[get("/player/ban?<ckey>&<id>")]
pub async fn ban(
    ckey: Option<&str>,
    id: Option<&str>,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<GenericResponse<Vec<Ban>>, Status> {
    if ckey.is_some() ^ id.is_none() {
        return Err(Status::BadRequest);
    }

    let Ok(bans) = get_ban(ckey, id, &database.pool).await else {
        return Err(Status::InternalServerError);
    };

    Ok(GenericResponse::Success(bans))
}

#[get("/player/roletime?<ckey>")]
pub async fn roletime(
    ckey: &str,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<GenericResponse<Vec<PlayerRoletime>>, Status> {
    let Ok(roletimes) = get_roletime(ckey, &database.pool).await else {
        return Err(Status::InternalServerError);
    };

    Ok(GenericResponse::Success(roletimes))
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
