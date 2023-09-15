use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = super::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    id: u16,
    username: String,
    password: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}
