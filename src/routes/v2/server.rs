use rocket::{get, State};

use crate::{
    byond::{get_server_status, Status},
    config::Config,
};

use super::Json;

#[get("/server")]
pub async fn index(config: &State<Config>) -> Json<Vec<Status>> {
    let status = get_server_status(config).await;

    Json::Ok(status)
}
