use rocket::{
    catch,
    http::{ContentType, Status},
    request::{FromRequest, Outcome},
    response::{self, Responder, Response},
    Request,
};
use serde::Serialize;
use serde_json::json;
use std::{io::Cursor, time::SystemTime};

use crate::config::Config;

use super::server::Status as ServerStatus;

#[derive(Debug, Serialize)]
pub enum GenericResponse<R> {
    Success(R),
    Failure,
    Denied,
    BadAuth,
}

impl<R: Serialize> Responder<'_, 'static> for GenericResponse<R> {
    fn respond_to(self, _: &Request) -> response::Result<'static> {
        let (status, reason, response) = match self {
            GenericResponse::Success(r) => (1, "success", Some(r)),
            GenericResponse::Failure => (0, "failure", None),
            GenericResponse::Denied => (2, "denied", None),
            GenericResponse::BadAuth => (3, "bad auth", None),
        };

        let json = json!({
            "status": status,
            "reason": reason,
            "response": response,
        })
        .to_string();

        Response::build()
            .header(ContentType::JSON)
            .sized_body(json.len(), Cursor::new(json))
            .ok()
    }
}

#[derive(Debug, Default)]
pub struct Cache {
    pub server: Option<CacheEntry<Vec<ServerStatus>>>,
}

#[derive(Debug)]
pub struct CacheEntry<T> {
    pub data: T,
    pub expires: SystemTime,
}

pub struct ApiKey;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let Some(config) = request.rocket().state::<Config>() else {
            return Outcome::Error((Status::InternalServerError, ()));
        };

        if request.headers().get_one("X-API-KEY") == Some(&config.secret) {
            Outcome::Success(ApiKey)
        } else {
            Outcome::Error((Status::Unauthorized, ()))
        }
    }
}

#[catch(default)]
pub fn default_catcher(status: Status, _: &Request) -> GenericResponse<()> {
    match status {
        Status { code: 404 } => GenericResponse::Failure,
        Status { code: 403 } => GenericResponse::Denied,
        Status { code: 401 } => GenericResponse::BadAuth,
        _ => GenericResponse::Failure,
    }
}
