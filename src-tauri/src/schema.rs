// @generated automatically by Diesel CLI.

diesel::table! {
    songs (id) {
        id -> Integer,
        file_path -> Text,
        file_hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}
