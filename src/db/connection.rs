use std::ops::Deref;

use r2d2;
use r2d2::PooledConnection;
//use diesel::mysql::MysqlConnection;
use diesel::mysql::*;
use r2d2_diesel::ConnectionManager;

use dotenv::dotenv;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::env;
pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

// pub const DATABASE_FILE: &'static str = env!("DATABASE_URL");

pub fn init_pool() -> Pool {
    dotenv().ok(); // Grabbing ENV vars

    // Pull DATABASE_URL env var
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // let config = Config::default();
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::new(manager).expect("db pool")
}

// #[derive(ConnectionPool)]
pub struct Conn(pub PooledConnection<ConnectionManager<MysqlConnection>>);

impl Deref for Conn {
    type Target = MysqlConnection;

    // #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
