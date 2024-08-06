use rocket::serde::Serialize;
use rocket::FromForm;

#[derive(FromForm, Serialize)]
pub struct AdminLogin {
    pub email: String,
    pub password: String,
}
