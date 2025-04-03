use std::sync::Arc;

use poem::web::Data;
use poem_openapi::payload::Json;

use crate::{
    byond::status::{ServerStatus, get_server_status},
    config::Config,
};

use super::ApiV2;

impl ApiV2 {
    pub(super) async fn server(&self, config: Data<&Arc<Config>>) -> Json<Vec<ServerStatus>> {
        Json(get_server_status(*config).await)
    }
}
