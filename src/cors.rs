use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

pub fn cors() -> Result<Cors, rocket_cors::Error> {
    let allowed_origins = AllowedOrigins::some_regex(&["^https?://.+"]);
    let allowed_methods = vec![Method::Get].into_iter().map(From::from).collect();
    let allowed_headers =
        AllowedHeaders::some(&["Accept", "Authorization", "Content-Type", "X-CSRF-Token"]);
    let expose_headers = vec!["Link".to_string()].into_iter().collect();

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
