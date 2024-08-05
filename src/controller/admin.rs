use crate::db::establish_connection;
use crate::model::admin::AdminUser;
use crate::schema::admin_users::dsl::*;
use diesel::prelude::*;
use rocket::get;
use rocket::serde::json::Json;

#[get("/")]
pub fn index() -> Json<Vec<AdminUser>> {
    let results = admin_users
        .load::<AdminUser>(&mut establish_connection())
        .expect("Error loading admin users");

    const PASSWORD: &str = "";

    let matching_users: Vec<AdminUser> = results
        .into_iter()
        .filter(|admin_user| admin_user.verify_password(PASSWORD))
        .collect();

    Json(matching_users)
}
