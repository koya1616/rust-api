#[macro_use]
extern crate rocket;

pub mod controller;
pub mod db;
pub mod model;
pub mod schema;
pub mod test;
pub mod types;

use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use serde::Serialize;

use crate::controller::admin;

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![admin::index, admin::get_admin_user, admin::login])
}

#[derive(Serialize)]
pub struct ErrorResponse {
  message: String,
}

impl ErrorResponse {
  fn new(message: &str) -> Json<Self> {
    Json(Self {
      message: message.to_string(),
    })
  }
}

pub fn create_error(status: Status, message: &str) -> Custom<Json<ErrorResponse>> {
  Custom(status, ErrorResponse::new(message))
}
