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

trait Korean {
    fn kweek(&self) -> String;
    fn kday(&self) -> String;
}

impl Korean for chrono::DateTime<chrono::Local> {
    fn kweek(&self) -> String {
        match self.weekday() {
            Weekday::Mon => return "월요일".to_string(),
            Weekday::Tue => return "화요일".to_string(),
            Weekday::Wed => return "수요일".to_string(),
            Weekday::Thu => return "목요일".to_string(),
            Weekday::Fri => return "금요일".to_string(),
            Weekday::Sat => return "토요일".to_string(),
            Weekday::Sun => return "일요일".to_string(),
        };
    }

    fn kday(&self) -> String {
        match self.hour() {
            h if h > 12 => return "오후".to_string(),
            _ => return "오전".to_string(),
        };
    }
}

#[get("/kakao")]
pub fn kakao_test() -> Template {
    let notices = notice_parse(Some(15)).unwrap();

    let current = Local::now();
    let ap = current.kday(); // 오전/오후
    let week = current.kweek(); // N요일

    let context = json!({
        "notices": notices,
        "date": format!("{} {}", current.format("%Y년 %-m월 %d일"), week),
        "time": format!("{} {}", ap, current.format("%-I:%M")),
    });

    Template::render("kakao", context)
}
