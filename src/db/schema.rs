// @generated automatically by Diesel CLI.

diesel::table! {
    users (_id) {
        _id -> Uuid,
        nickname -> Text,
        telephone -> Text,
        keypass -> Text,
        created_at -> Timestamptz,
    }
}
