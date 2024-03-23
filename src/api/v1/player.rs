use rocket::get;

use super::common::ApiKey;

#[get("/player")]
pub fn index(_api_key: ApiKey) -> &'static str {
    "Hello, world!"
}
