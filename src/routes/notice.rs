#![allow(proc_macro_derive_resolution_fallback)]
use crate::db::connection::Conn;
use crate::db::models::Notice;
use crate::db::models::Schedule;
use crate::db::query;
use crate::utils::parse::html_parse;

use rocket::http::Status;
use rocket_contrib::json::Json;
use serde_json::Value;

#[get("/hello")]
pub fn hello() -> Json<Notice> {
    let notice = Notice {
        id: 12345,
        title: "공지1".to_string(),
        date: "2021-07-09".to_string(),
        link: "https://".to_string(),
        writer: "CSW".to_string(),
    };
    Json(notice)
}

#[get("/db")]
pub fn db_test(conn: Conn) -> Result<Json<Vec<Schedule>>, Status> {
    let result = query::show_scheds(&conn)
        .map(|sched| Json(sched))
        .map_err(|error| crate::error_status(error));

    // for row in query::show_scheds(&conn).unwrap() {
    //     println!("id: {}, content: {}", row.id, row.content);
    // }

    result
}

#[post("/notice", format = "json", data = "<kakao>")]
pub fn notice_test(kakao: Json<Value>) -> Result<Json<Vec<Notice>>, Status> {
    // println!("{}", kakao["userRequest"]["utterance"].as_str().unwrap()); // 발화문
    let result = html_parse(Some(7)).unwrap();
    Ok(Json(result))
}
