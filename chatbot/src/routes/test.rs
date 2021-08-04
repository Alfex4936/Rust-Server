#![allow(proc_macro_derive_resolution_fallback)]
use crate::db::models::Notice;
use crate::db::query;
use crate::utils::parser::notice_parse;
use kakao_rs::components::basics::*;
use kakao_rs::components::buttons::*;

use actix_web::{get, post, web, Error, HttpResponse, Responder, Result};
use serde_json::Value;

use crate::db::connection::DbPool;

#[get("/")]
pub async fn hello() -> impl Responder {
    let notice = Notice {
        id: 12345,
        title: "공지1".to_string(),
        date: "2021-07-09".to_string(),
        link: "https://".to_string(),
        writer: "CSW".to_string(),
    };

    HttpResponse::Ok().json(notice)
}

#[get("/db")]
pub async fn db_test(conn: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let result = query::show_scheds(&conn.get().unwrap())
        .await
        .map(|sched| HttpResponse::Ok().json(sched))
        .map_err(|_| HttpResponse::InternalServerError());

    for row in query::show_scheds(&conn.get().unwrap()).await.unwrap() {
        println!("id: {}, content: {}", row.id, row.content);
    }

    Ok(result.ok().unwrap())
}

#[get("/notice/{nums}")]
pub async fn get_notices(nums: web::Path<usize>) -> impl Responder {
    // println!("{}", kakao["userRequest"]["utterance"].as_str().unwrap()); // 발화문
    let result = notice_parse(Some(nums.into_inner())).await.unwrap();
    HttpResponse::Ok().json(result)
}

#[post("/json")]
pub async fn json_test(kakao: web::Json<Value>) -> impl Responder {
    println!("{:#?}", kakao);
    let mut result = Template::new();

    // 빠른 응답
    result.add_qr(QuickReply::new(
        "오늘".to_string(),
        "오늘 공지 보여줘".to_string(),
    ));
    result.add_qr(QuickReply::new(
        "어제".to_string(),
        "어제 공지 보여줘".to_string(),
    ));

    let mut list_card = ListCard::new("리스트 카드 제목!".to_string()); // 제목

    list_card.add_button(Button::Msg(MsgButton::new("그냥 텍스트 버튼".to_string())));

    list_card.add_button(Button::Link(
        LinkButton::new("link label".to_string()).set_link("https://google.com".to_string()),
    ));
    list_card.add_button(Button::Share(
        ShareButton::new("share label".to_string()).set_msg("카톡에 보이는 메시지".to_string()),
    ));

    list_card.add_item(
        ListItem::new("title".to_string())
            .set_desc("description".to_string())
            .set_link("https://naver.com".to_string()),
    );

    result.add_output(list_card.build()); // moved list_card's ownership

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&result).unwrap())
}
