#![allow(proc_macro_derive_resolution_fallback)]

use crate::utils::parser::notice_parse;
use chrono::prelude::*;
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
    let notices = notice_parse(Some(15)).unwrap();

    let context = json!({
        "notices": notices,
    });

    Template::render("ajou", context)
}

#[get("/kakao")]
pub fn kakao_test() -> Template {
    let notices = notice_parse(Some(15)).unwrap();

    let current = Local::now();

    let context = json!({
        "notices": notices,
        "date": current.format("%Y년 %m월 %d일").to_string(),
        "time": current.format("%I:%M %p").to_string(),
    });

    Template::render("kakao", context)
}
