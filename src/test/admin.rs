#[cfg(test)]
mod tests {
    use crate::db::test_establish_connection;
    use crate::schema::admin_users::dsl::*;
    use bcrypt::{hash, DEFAULT_COST};
    use chrono::{FixedOffset, Utc};
    use diesel::insert_into;
    use diesel::prelude::*;
    use diesel::sql_query;

    fn before_each() {
        let mut conn = test_establish_connection();

        let jst_offset = FixedOffset::east_opt(9 * 3600).unwrap();
        let now = Utc::now().with_timezone(&jst_offset).naive_local();
        insert_into(admin_users)
            .values((
                email.eq(Some("test@test.com")),
                password_digest.eq(Some(hash("test1234", DEFAULT_COST).unwrap())),
                created_at.eq(now),
                updated_at.eq(now),
            ))
            .execute(&mut conn)
            .expect("Error inserting admin user");
    }

    fn after_each() {
        let mut conn = test_establish_connection();

        sql_query("SET FOREIGN_KEY_CHECKS = 0")
            .execute(&mut conn)
            .expect("Error disabling foreign key checks");

        sql_query("TRUNCATE TABLE admin_users")
            .execute(&mut conn)
            .expect("Error truncating admin_users table");
    }
    #[test]
    fn add_test() {
        before_each();

        after_each();
    }
}
