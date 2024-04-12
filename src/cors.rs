use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use std::collections::HashSet;

pub fn cors() -> Result<Cors, rocket_cors::Error> {
    let allowed_origins = AllowedOrigins::some_regex(&["^https?://.+"]);
    let allowed_methods = HashSet::from([Method::Get.into()]);
    let allowed_headers =
        AllowedHeaders::some(&["Accept", "Authorization", "Content-Type", "X-CSRF-Token"]);
    let expose_headers = HashSet::from(["Link".to_string()]);

    CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers,
        expose_headers,
        allow_credentials: false,
        max_age: Some(300),
        ..Default::default()
    }
    .to_cors()
}
