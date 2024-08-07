use rocket::serde::{Deserialize, Serialize};
use rocket::FromForm;

#[derive(FromForm, Serialize)]
pub struct AdminLoginRequestBody {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct AdminLoginResponseBody {
    pub id: i64,
    pub token: String,
}
