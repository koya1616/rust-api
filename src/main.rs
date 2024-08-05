#[macro_use]
extern crate rocket;

use app::controller::admin;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![admin::index, admin::get_admin_user])
}
