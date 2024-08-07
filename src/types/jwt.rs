use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Claims {
  pub id: i64,
  pub exp: i64,
  pub user_type: String,
}

impl Claims {
  pub fn encode(&self) -> String {
    let secret_key = env::var("SECRET_KEY").unwrap();

    encode(&Header::default(), self, &EncodingKey::from_secret(secret_key.as_ref())).unwrap()
  }
}

pub fn create_claims(id: i64, user_type: &str) -> Claims {
  Claims {
    id,
    exp: (chrono::Utc::now() + chrono::Duration::days(90)).timestamp(),
    user_type: user_type.to_string(),
  }
}
