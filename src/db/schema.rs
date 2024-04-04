// @generated automatically by Diesel CLI.

diesel::table! {
    connections (conn_id) {
        conn_id -> Text,
        nickname -> Text,
        user_ref -> Uuid,
    }
}

diesel::table! {
    users (_id) {
        _id -> Uuid,
        nickname -> Text,
        telephone -> Text,
        keypass -> Text,
        created_at -> Timestamptz,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    connections,
    users,
);
