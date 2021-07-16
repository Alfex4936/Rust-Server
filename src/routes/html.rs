#![allow(proc_macro_derive_resolution_fallback)]

use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct Context {
    first_name: String,
    last_name: String,
}

#[get("/front")]
pub fn front_test() -> Template {
    let context = Context {
        first_name: String::from("First"),
        last_name: String::from("Last"),
    };

    Template::render("home", context)
}
