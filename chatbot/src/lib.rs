#![feature(proc_macro_hygiene, decl_macro)]

extern crate actix_http;
extern crate actix_rt;
extern crate actix_web;

extern crate r2d2;
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;
// #[macro_use]
extern crate serde_json;

extern crate chrono;
#[cfg(feature = "mongo")]
extern crate mongodb;
extern crate rand;

mod db;
mod routes;
pub mod utils;

#[cfg(feature = "mongo")]
pub use db::connection_mongo;
#[cfg(not(feature = "mongo"))]
pub use db::connection_mysql;
pub use routes::info;
pub use routes::notice;
pub use routes::route;

pub const SERVER: &str = "0.0.0.0:8008";
pub const CARD_IMAGES: [&str; 2] = ["ajou_carousel", "ajou_carousel_1"];
pub const MY_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.127 Safari/537.36";
