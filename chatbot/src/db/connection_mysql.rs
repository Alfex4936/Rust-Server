use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};

use dotenv::dotenv;
use std::env;

// pub const DATABASE_FILE: &'static str = env!("DATABASE_URL");
pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn init_pool() -> DbPool {
    dotenv().ok(); // Grabbing ENV vars

    // Pull DATABASE_URL env var
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
