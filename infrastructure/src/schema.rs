// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
