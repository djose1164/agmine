use actix_web::{get, web, HttpResponse, Error};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use crate::{repository::database::DbPool, models::user::User};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[get("/users")]
pub async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let users = web::block(move || {
        let conn = &mut pool.get()?;
        find_all(conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError);

    Ok(HttpResponse::Ok().json(users?))
}

fn find_all(conn: &mut MysqlConnection) -> Result<Vec<User>, DbError> {
    use crate::models::schema::users::dsl::*;

    let fetched = users.load::<User>(conn)?;
    Ok(fetched)
}