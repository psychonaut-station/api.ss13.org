use std::collections::HashSet;

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

pub fn cors() -> Result<Cors, rocket_cors::Error> {
    let allowed_methods = [Method::Get.into(), Method::Options.into()];
    let allowed_headers = ["Accept", "Authorization", "X-API-KEY", "X-DEV-KEY"];
    let expose_headers = ["Content-Type".to_string(), "Content-Length".to_string()];

    CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: HashSet::from(allowed_methods),
        allowed_headers: AllowedHeaders::some(&allowed_headers),
        expose_headers: HashSet::from(expose_headers),
        send_wildcard: true,
        allow_credentials: false,
        max_age: Some(300),
        ..Default::default()
    }
    .to_cors()
}
