use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
  dotenv().ok();

  let database_url = match env::var("RUST_ENV").unwrap_or_else(|_| "dev".to_string()).as_str() {
    "test" => env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set for testing"),
    _ => env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
  };

  MysqlConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn test_establish_connection() -> MysqlConnection {
  dotenv().ok();

  let database_url = env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
  MysqlConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
