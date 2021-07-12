#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;

mod db;
mod routes;

pub fn rocket() -> rocket::Rocket {
    let pool = db::db::init_pool();
    let conn = if cfg!(test) {
        Some(db::db::Conn(
            pool.get().expect("database connection for testing"),
        ))
    } else {
        None
    };
    rocket::ignite()
        .manage(pool)
        // .attach(conn::fairing())
        // .attach(Template::fairing())
        .mount("/api", routes![routes::notice::hello])
}
