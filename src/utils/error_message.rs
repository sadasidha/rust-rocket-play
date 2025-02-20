use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: Status,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorMessage {
    //#[resp("{0}")]
    Internal(Status, String),

    //#[resp("{0}")]
    NotFound(Status, String),

    //#[resp("{0}")]
    BadRequest(Status, String),
}

impl ErrorMessage {
    fn get_error_message(&self) -> &String {
        match self {
            ErrorMessage::Internal(_, v2) => v2,
            ErrorMessage::NotFound(_, v2) => v2,
            ErrorMessage::BadRequest(_, v2) => v2,
        }
    }
    fn get_http_status(&self) -> Status {
        match self {
            ErrorMessage::Internal(_, _) => Status::InternalServerError,
            ErrorMessage::NotFound(_, _) => Status::NotFound,
            _ => Status::BadRequest,
        }
    }
}

impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Error {}.", self.get_error_message())
    }
}

impl<'r> Responder<'r, 'static> for ErrorMessage {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        // serialize struct into json string
        let err_response = serde_json::to_string(&ErrorResponse {
            status: self.get_http_status(),
            message: self.get_error_message().clone(),
        })
        .unwrap();

        Response::build()
            .status(self.get_http_status())
            .header(ContentType::JSON)
            .sized_body(err_response.len(), Cursor::new(err_response))
            .ok()
    }
}
