#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod routes;

pub fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/api", routes![routes::notice::hello])
}
