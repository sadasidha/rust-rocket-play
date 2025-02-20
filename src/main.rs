use crate::controllers::{aggregator_ctrl, errors, index_ctrl};
use rocket::{catchers, launch, routes};

mod controllers;
mod db;
mod utils;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![errors::not_found, errors::invalid_inputs, errors::internal_error_500, errors::internal_error_501, errors::internal_error_502, errors::internal_error_503, errors::internal_error_504, errors::internal_error_505])
        .mount("/", routes![index_ctrl::index])
        .mount("/v1.0", routes![aggregator_ctrl::aggregator])
}
