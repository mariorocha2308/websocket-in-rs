// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Text,
        username -> Text,
        phone -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}
