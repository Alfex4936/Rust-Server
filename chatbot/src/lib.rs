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

mod db;
mod routes;
mod utils;

pub use db::connection;
pub use routes::chatbot;
pub use routes::test;
