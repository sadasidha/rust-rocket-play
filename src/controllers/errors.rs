use crate::utils::error_message::{ErrorMessage, ErrorResponse};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{catch, Request};

#[catch(404)]
pub fn not_found(req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: Status::NotFound,
        message: format!("page not found: {}", req.uri().path()),
    })
}

#[catch(422)]
pub fn invalid_inputs(req: &Request) -> Json<ErrorMessage> {
    Json(ErrorMessage::NotFound(
        Status::BadRequest, "Invalid input format".to_string(),
    ))
}


#[catch(500)]
pub fn internal_error_500(req: &Request) -> Json<ErrorMessage> {
    Json(ErrorMessage::Internal(Status::InternalServerError,"Internal Server Error".to_string()))
}

#[catch(501)]
pub fn internal_error_501(req: &Request) -> Json<ErrorMessage> {
    Json(ErrorMessage::Internal(Status::InternalServerError,"Internal Server Error".to_string()))
}

#[catch(502)]
pub fn internal_error_502(req: &Request) -> Json<ErrorMessage> {
    Json(ErrorMessage::Internal(Status::InternalServerError,"Internal Server Error".to_string()))
}

#[catch(503,)]
pub fn internal_error_503(req: &Request) -> Json<ErrorMessage> {
    Json(ErrorMessage::Internal(Status::InternalServerError,"Internal Server Error".to_string()))
}


#[catch(504)]
pub fn internal_error_504(req: &Request) -> Json<ErrorMessage> {
    Json(ErrorMessage::Internal(Status::InternalServerError,"Internal Server Error".to_string()))
}


