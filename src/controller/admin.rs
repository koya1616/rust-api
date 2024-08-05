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

    Json(results)
}

#[get("/<admin_id>")]
pub fn get_admin_user(admin_id: i64) -> Option<Json<AdminUser>> {
    admin_users
        .find(admin_id)
        .first::<AdminUser>(&mut establish_connection())
        .map(Json)
        .ok()
}
