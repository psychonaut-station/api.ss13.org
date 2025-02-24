use rocket::{get, State};

use crate::{
    byond::{get_server_status, Status},
    config::Config,
};

use super::Json;

#[get("/server")]
pub async fn index(config: &State<Config>) -> Json<Vec<Status>> {
    Json::Ok(get_server_status(config).await)
}
