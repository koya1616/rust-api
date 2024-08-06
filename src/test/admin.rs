#[cfg(test)]
mod tests {
    use bcrypt::{hash, DEFAULT_COST};
    use chrono::{FixedOffset, Utc};
    use ctor::*;
    use diesel::insert_into;
    use diesel::prelude::*;
    use diesel::sql_query;
    use rocket::http::ContentType;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::routes;
    use std::env;

    use crate::controller::admin;
    use crate::db::test_establish_connection;
    use crate::model::admin::AdminUser;
    use crate::schema::admin_users::dsl::*;

    #[ctor]
    fn setup() {
        env::set_var("RUST_ENV", "test");

        let mut conn = test_establish_connection();

        let jst_offset = FixedOffset::east_opt(9 * 3600).unwrap();
        let now = Utc::now().with_timezone(&jst_offset).naive_local();
        insert_into(admin_users)
            .values(vec![
                (
                    email.eq(Some("test1@test.com")),
                    password_digest.eq(Some(hash("test1234", DEFAULT_COST).unwrap())),
                    created_at.eq(now),
                    updated_at.eq(now),
                ),
                (
                    email.eq(Some("test2@test.com")),
                    password_digest.eq(Some(hash("test5678", DEFAULT_COST).unwrap())),
                    created_at.eq(now),
                    updated_at.eq(now),
                ),
            ])
            .execute(&mut conn)
            .expect("Error inserting admin user");
    }

    struct Cleanup;

    impl Drop for Cleanup {
        fn drop(&mut self) {
            let mut conn = test_establish_connection();

            sql_query("SET FOREIGN_KEY_CHECKS = 0")
                .execute(&mut conn)
                .expect("Error disabling foreign key checks");

            sql_query("TRUNCATE TABLE admin_users")
                .execute(&mut conn)
                .expect("Error truncating admin_users table");
        }
    }

    #[test]
    fn index_test() {
        let client = Client::tracked(rocket::build().mount("/", routes![admin::index])).unwrap();
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);

        let response_body = response.into_string().unwrap();
        let parsed_response: Vec<AdminUser> = serde_json::from_str(&response_body).unwrap();
        assert_eq!(parsed_response.len(), 2);
        assert_eq!(parsed_response[0].id, 1);
        assert_eq!(parsed_response[0].email, Some("test1@test.com".to_string()));
        assert_eq!(parsed_response[0].verify_password("test1234"), true);
        let _cleanup = Cleanup;
    }

    #[test]
    fn get_admin_user_test() {
        let client =
            Client::tracked(rocket::build().mount("/", routes![admin::get_admin_user])).unwrap();
        let response = client.get("/1").dispatch();

        assert_eq!(response.status(), Status::Ok);

        let response_body = response.into_string().unwrap();
        let parsed_response: AdminUser = serde_json::from_str(&response_body).unwrap();
        assert_eq!(parsed_response.id, 1);
        assert_eq!(parsed_response.email, Some("test1@test.com".to_string()));
        assert_eq!(parsed_response.verify_password("test1234"), true);

        let response = client.get("/99").dispatch();
        assert_eq!(response.status(), Status::NotFound);

        let _cleanup = Cleanup;
    }

    #[test]
    fn login_test() {
        let client = Client::tracked(rocket::build().mount("/", routes![admin::login])).unwrap();

        let response = client
            .post("/admin/login")
            .header(ContentType::Form)
            .body("email=test1@test.com&password=test1234")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let response = client
            .post("/admin/login")
            .header(ContentType::Form)
            .body("email=test1@test.com&password=wrongpassword")
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);

        let response = client
            .post("/admin/login")
            .header(ContentType::Form)
            .body("email=unexies@test.com&password=test1234")
            .dispatch();
        assert_eq!(response.status(), Status::NotFound);

        let _cleanup = Cleanup;
    }
}
