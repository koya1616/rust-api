use crate::db::establish_connection;
use crate::model::admin::AdminUser;
use crate::schema::admin_users::dsl::*;
use crate::types::admin::AdminLogin;
use diesel::prelude::*;
use rocket::form::Form;
use rocket::get;
use rocket::post;
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

#[post("/admin/login", data = "<login>")]
pub fn login(login: Form<AdminLogin>) -> String {
    let result = admin_users
        .filter(email.eq(&login.email))
        .first::<AdminUser>(&mut establish_connection())
        .expect("Error loading admin users");
    println!("{:?}", result);
    format!("Admin user: {:?}", result)
}
