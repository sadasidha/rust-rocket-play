use crate::utils::error_message::{ErrorMessage, ErrorResponse};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{catch, get, Request};
use serde::Serialize;

#[get("/")]
pub fn index() -> String {
    "aggregate-data-v1.0".to_string()
}
