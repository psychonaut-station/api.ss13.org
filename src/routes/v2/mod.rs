use std::sync::Arc;

use poem::web::Data;
use poem_openapi::{OpenApi, payload::Json};

use crate::{byond::status::ServerStatus, config::Config};

use super::BaseApi;

mod server;

#[derive(Default)]
pub(super) struct ApiV2;

impl BaseApi for ApiV2 {
    #[inline(always)]
    fn title() -> &'static str {
        "Psychonaut Station API"
    }

    #[inline(always)]
    fn version() -> &'static str {
        "2.0"
    }

    #[inline(always)]
    fn route() -> &'static str {
        "/v2"
    }

    #[inline(always)]
    fn ui_route() -> &'static str {
        "/"
    }
}

#[OpenApi]
impl ApiV2 {
    #[inline]
    #[oai(path = "/server", method = "get")]
    async fn server_(&self, config: Data<&Arc<Config>>) -> Json<Vec<ServerStatus>> {
        self.server(config).await
    }
}
