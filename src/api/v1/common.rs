use rocket::{
    catch,
    http::{ContentType, Status},
    request::{FromRequest, Outcome},
    response::{self, Responder, Response},
    Request,
};
use serde::Serialize;
use serde_json::json;
use std::io::Cursor;

use crate::config::Config;

#[derive(Debug, Serialize)]
pub enum GenericResponse<R> {
    Failure,
    Success(R),
    Forbidden,
    Unauthorized,
    NotFound,
    BadRequest,
    Conflict(Option<R>),
}

impl<R: Serialize> Responder<'_, 'static> for GenericResponse<R> {
    fn respond_to(self, _: &Request) -> response::Result<'static> {
        let (status, reason, response) = match self {
            GenericResponse::Failure => (0, "failure", None),
            GenericResponse::Success(r) => (1, "success", Some(r)),
            GenericResponse::Forbidden => (2, "forbidden", None),
            GenericResponse::Unauthorized => (3, "unauthorized", None),
            GenericResponse::NotFound => (4, "not found", None),
            GenericResponse::BadRequest => (5, "bad request", None),
            GenericResponse::Conflict(r) => (6, "conflict", r),
        };

        let json = json!({
            "status": status,
            "reason": reason,
            "response": response,
        })
        .to_string();

        let status = match status {
            0 => Status::InternalServerError,
            1 => Status::Ok,
            2 => Status::Forbidden,
            3 => Status::Unauthorized,
            4 => Status::NotFound,
            5 => Status::BadRequest,
            6 => Status::Conflict,
            _ => unreachable!(),
        };

        Response::build()
            .status(status)
            .header(ContentType::JSON)
            .sized_body(json.len(), Cursor::new(json))
            .ok()
    }
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
        Status { code: 409 } => GenericResponse::Conflict(None),
        Status { code: 404 } => GenericResponse::NotFound,
        Status { code: 403 } => GenericResponse::Forbidden,
        Status { code: 401 } => GenericResponse::Unauthorized,
        Status { code: 400 } => GenericResponse::BadRequest,
        _ => GenericResponse::Failure,
    }
}
