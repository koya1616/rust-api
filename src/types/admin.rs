use rocket::serde::Serialize;
use rocket::FromForm;

#[derive(FromForm, Serialize)]
pub struct AdminLoginRequestBody {
    pub email: String,
    pub password: String,
}
