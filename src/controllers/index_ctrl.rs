use rocket::get;

#[get("/")]
pub fn index() -> String {
    "aggregate-data-v1.0".to_string()
}
