use rocket::{get, http::Status, State};

use crate::{database::*, Database};

use super::{common::ApiKey, GenericResponse};

#[get("/autocomplete/job")]
pub async fn job(
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<GenericResponse<Vec<String>>, Status> {
    let Ok(jobs) = get_jobs(&database.pool).await else {
        return Err(Status::InternalServerError);
    };

    Ok(GenericResponse::Success(jobs))
}

#[get("/autocomplete/ckey?<ckey>")]
pub async fn ckey(
    ckey: &str,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<GenericResponse<Vec<String>>, Status> {
    let Ok(ckeys) = get_ckeys(ckey, &database.pool).await else {
        return Err(Status::InternalServerError);
    };

    Ok(GenericResponse::Success(ckeys))
}

#[get("/autocomplete/ic_name?<ic_name>")]
pub async fn ic_name(
    ic_name: &str,
    database: &State<Database>,
    _api_key: ApiKey,
) -> Result<GenericResponse<Vec<IcName>>, Status> {
    let Ok(ic_names) = get_ic_names(ic_name, &database.pool).await else {
        return Err(Status::InternalServerError);
    };

    Ok(GenericResponse::Success(ic_names))
}
