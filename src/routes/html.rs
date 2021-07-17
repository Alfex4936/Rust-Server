#![allow(proc_macro_derive_resolution_fallback)]

use crate::utils::parse::notice_parse;
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
