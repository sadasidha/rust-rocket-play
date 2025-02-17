use rocket::{ launch, routes};
use crate::controllers::{aggregator_ctrl, index_ctrl};

mod db;
mod controllers;
mod utils;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index_ctrl::index, aggregator_ctrl::aggregator])
}
