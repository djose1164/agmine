// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Unsigned<Smallint>,
        #[max_length = 32]
        username -> Varchar,
        #[max_length = 32]
        password -> Varchar,
        created_at -> Datetime,
        updated_at -> Timestamp,
    }
}
