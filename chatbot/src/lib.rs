#![feature(proc_macro_hygiene, decl_macro)]

extern crate actix_http;
extern crate actix_rt;
extern crate actix_web;

extern crate r2d2;
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate chrono;
extern crate rand;

mod db;
mod routes;
mod utils;

pub use db::connection;
pub use routes::info;
pub use routes::notice;
pub use routes::route;

pub const SERVER: &str = "0.0.0.0:8008";
pub const CARD_IMAGES: [&str; 2] = ["ajou_carousel", "ajou_carousel_1"];
