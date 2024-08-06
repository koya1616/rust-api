use crate::create_error;
use crate::db::establish_connection;
use crate::model::admin::AdminUser;
use crate::schema::admin_users::dsl::*;
use crate::types::admin::AdminLogin;
use crate::ErrorResponse;
use diesel::prelude::*;
use rocket::form::Form;
use rocket::get;
use rocket::http::Status;
use rocket::post;
use rocket::response::status::Custom;
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
pub fn login(login: Form<AdminLogin>) -> Result<Json<AdminUser>, Custom<Json<ErrorResponse>>> {
    admin_users
        .filter(email.eq(&login.email))
        .first::<AdminUser>(&mut establish_connection())
        .map_err(|error| match error {
            diesel::result::Error::NotFound => {
                create_error(Status::NotFound, "ユーザーが見つかりません")
            }
            _ => create_error(Status::InternalServerError, "予期せぬエラーが発生しました"),
        })
        .and_then(|user| {
            if user.verify_password(&login.password) {
                Ok(Json(user))
            } else {
                Err(create_error(Status::Unauthorized, "ログインに失敗しました"))
            }
        })
}
