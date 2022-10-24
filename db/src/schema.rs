// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
        mobile -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
