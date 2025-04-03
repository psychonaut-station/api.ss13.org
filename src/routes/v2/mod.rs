use poem_openapi::{OpenApi, param::Query, payload::PlainText};

use super::BaseApi;

#[derive(Default)]
pub(crate) struct ApiV2;

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
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }
}
