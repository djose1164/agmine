use dotenv::dotenv;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use  diesel::mysql::MysqlConnection;
use std::env;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

fn init_pool(database_url: &str) -> Result<DbPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn establish_connection() -> Result<DbPool, PoolError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("Couldn't find the database's url.");
    init_pool(&database_url)
}