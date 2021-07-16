#![allow(proc_macro_derive_resolution_fallback)]

use crate::utils::parse::html_parse;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[get("/front/<nums>")]
pub fn front_test(nums: usize) -> Template {
    let notices = html_parse(Some(nums)).unwrap();

    let context = json!({
        "notices": notices,
        "first": "first",
        "last": "last",
    });

    Template::render("home", context)
}
