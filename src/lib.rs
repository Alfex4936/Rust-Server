#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
// #[macro_use]
// extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate serde_derive;

mod db;
mod routes;

pub fn rocket() -> rocket::Rocket {
    rocket::ignite().manage(db::connection::init_pool()).mount(
        "/api",
        routes![routes::notice::hello, routes::notice::db_test],
    )
}
