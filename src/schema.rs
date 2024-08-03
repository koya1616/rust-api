// @generated automatically by Diesel CLI.

diesel::table! {
    admin_users (id) {
        id -> Bigint,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        password_digest -> Nullable<Varchar>,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    admin_users,
);
