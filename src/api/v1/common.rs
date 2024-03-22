use rocket::{
    http::ContentType,
    response::{self, Responder, Response},
    Request,
};
use serde::Serialize;
use serde_json::json;
use std::{io::Cursor, time::SystemTime};

use super::server::Status;

#[allow(dead_code)]
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
    pub server: Option<CacheEntry<Vec<Status>>>,
}

#[derive(Debug)]
pub struct CacheEntry<T> {
    pub data: T,
    pub expires: SystemTime,
}
