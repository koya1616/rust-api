#[macro_use]
extern crate rocket;

use crate::controller::admin;

pub mod controller;
pub mod db;
pub mod model;
pub mod schema;
pub mod test;
pub mod types;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![admin::index, admin::get_admin_user, admin::login],
    )
}
