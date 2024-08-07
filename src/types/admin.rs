use rocket::serde::Serialize;
use rocket::FromForm;

#[derive(FromForm, Serialize)]
pub struct AdminLoginRequestBody {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AdminLoginResponseBody {
    pub id: i64,
    pub token: String,
}
