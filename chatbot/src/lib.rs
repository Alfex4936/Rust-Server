extern crate actix_http;
extern crate actix_rt;
extern crate actix_web;

#[macro_use]
extern crate serde_derive;
// #[macro_use]
extern crate serde_json;

extern crate chrono;
extern crate mongodb;
extern crate rand;

extern crate argon2;

mod db;
mod routes;
// mod service;
pub mod utils;

pub use db::connection_mongo;
pub use routes::info;
pub use routes::notice;
pub use routes::user;
// pub use routes::route;

pub const SERVER: &str = env!("SERVER");
pub const CARD_IMAGES: [&str; 2] = ["ajou_carousel", "ajou_carousel_1"];
pub const MY_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36";
pub const MONGO_URL: &str = env!("MONGODB_URL");
