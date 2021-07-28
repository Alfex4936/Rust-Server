#![allow(proc_macro_derive_resolution_fallback)]
use crate::db::connection::Conn;
use crate::db::models::Notice;
use crate::db::models::Schedule;
use crate::db::query;
use crate::kakao_json::basics::Template;
use crate::utils::parser::{check_type, notice_parse};

use chrono::prelude::*;
use chrono::Duration;
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

#[get("/notice/<nums>")]
pub fn get_notices(nums: usize) -> Result<Json<Vec<Notice>>, Status> {
    // println!("{}", kakao["userRequest"]["utterance"].as_str().unwrap()); // 발화문
    let result = notice_parse(Some(nums)).unwrap();
    Ok(Json(result))
}

#[post("/notice", format = "json", data = "<_kakao>")]
pub fn notice_test(_kakao: Json<Value>) -> Result<Json<Vec<Notice>>, Status> {
    // println!("{}", kakao["userRequest"]["utterance"].as_str().unwrap()); // 발화문
    let result = notice_parse(Some(7)).unwrap();
    Ok(Json(result))
}

#[post("/yesterday", format = "json", data = "<_kakao>")]
pub fn last_notice_test(_kakao: Json<Value>, conn: Conn) -> Result<Json<Vec<Notice>>, Status> {
    // println!("{}", kakao["userRequest"]["utterance"].as_str().unwrap()); // 발화문
    let date = Local::now() - Duration::days(1);
    let date_str = date.format("%y.%m.%d").to_string();
    // %y : The proleptic Gregorian year modulo 100, zero-padded to 2 digits.

    // println!("What is {}", date_str);

    let result = query::get_notices(&conn, date_str)
        .map(|notice| Json(notice))
        .map_err(|error| crate::error_status(error));
    result
}

#[post("/json", format = "json", data = "<kakao>")]
pub fn json_test(kakao: String) -> Result<Json<Value>, Status> {
    // println!("what is {:#?}", kakao);
    let mut vec = vec![];
    let json: Template = match serde_json::from_str(&kakao) {
        Ok(t) => t,
        Err(e) => {
            return Ok(Json(
                json!({"type": "알 수 없음", "error": "SimpleText, BasicCard, ListCard, BasicCard 중 매치되는 데이터가 없습니다.</br>필드를 다시 확인해주세요."}),
            ))
        }
    };

    // let json: Template = serde_json::from_str(&kakao).map_err(|error| crate::error_status(error));
    for output in &json.template.outputs {
        // println!("{:#?}", output);
        // println!("Key: {}", check_type(output).unwrap());
        match check_type(output) {
            Some(t) => vec.push(t),
            _ => {
                // deny unknown fields 때문에 unreachable
                return Ok(Json(
                    json!({"type": "알 수 없음", "error": format!("couldn't identify {:?}", output)}),
                ));
            }
        }
    }

    let context = json!({
        "type": vec,
        "json": json,
    });

    Ok(Json(context))
}
