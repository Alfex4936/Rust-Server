#![allow(proc_macro_derive_resolution_fallback)]

use crate::utils::parser::notice_parse;
use rocket_contrib::templates::Template;

#[get("/front/<nums>")]
pub fn front_test(nums: usize) -> Template {
    let notices = notice_parse(Some(nums)).unwrap();

    let context = json!({
        "notices": notices,
        "first": "first",
        "last": "last",
    });

    Template::render("home", context)
}

#[get("/test")]
pub fn just_test() -> Template {
    let notices = notice_parse(Some(1)).unwrap();

    let context = json!({
        "notices": notices,
        "first": "first",
        "last": "last",
    });

    Template::render("test", context)
}