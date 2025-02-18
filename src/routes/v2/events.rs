use rocket::{get, http::Status, State};

use crate::{database::*, Database};

use super::{common::ApiKey, Json};

#[get("/events/deaths?<since>")]
pub async fn deaths(
    since: Option<&str>,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<Json<Vec<Death>>, Status> {
    match get_deaths(since, &database.pool).await {
        Ok(deaths) => Ok(Json::Ok(deaths)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/events/citations?<since>")]
pub async fn citations(
    since: Option<&str>,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<Json<Vec<Citation>>, Status> {
    match get_citations(since, &database.pool).await {
        Ok(citations) => Ok(Json::Ok(citations)),
        Err(_) => Err(Status::InternalServerError),
    }
}