use rocket::{get, http::Status};
use serde_json::{json, Value};

use crate::http::byond;

use super::{common::ApiKey, Json};

#[get("/byond/member?<ckey>")]
pub async fn member(ckey: &str, _api_key: ApiKey) -> Result<Json<Value>, Status> {
    let Ok(member) = byond::is_member(ckey).await else {
        return Err(Status::InternalServerError);
    };

    Ok(Json::Ok(json!({ "member": member })))
}
