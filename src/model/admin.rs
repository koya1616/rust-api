use bcrypt::verify;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::*;

#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(table_name = admin_users)]
pub struct AdminUser {
    pub id: i64,
    pub email: Option<String>,
    pub password_digest: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl AdminUser {
    pub fn verify_password(&self, password: &str) -> bool {
        match &self.password_digest {
            Some(digest) => verify(password, digest).unwrap_or(false),
            None => false,
        }
    }
}
