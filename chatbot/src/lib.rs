#![feature(proc_macro_hygiene, decl_macro)]

extern crate actix_http;
extern crate actix_rt;
extern crate actix_web;

#[macro_use]
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
pub use routes::chatbot;
pub use routes::test;

pub const SERVER: &str = "127.0.0.1:8000";
pub const CARD_IMAGES: [&str; 2] = ["ajou_carousel", "ajou_carousel_1"];
