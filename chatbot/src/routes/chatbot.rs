#![allow(proc_macro_derive_resolution_fallback)]
use crate::db::connection::DbPool;
use crate::db::models::Notice;

use crate::db::query;
use crate::utils::parser::{notice_parse, weather_parse, AJOU_LINK, NAVER_WEATHER};
use crate::CARD_IMAGES;

use kakao_rs::components::basics::*;
use kakao_rs::components::buttons::*;
use kakao_rs::components::cards::*;

use actix_web::{post, web, HttpResponse, Responder};
use chrono::prelude::Local;
use rand::Rng;
use serde_json::Value;
use unicode_segmentation::UnicodeSegmentation;

#[post("/today")]
pub async fn get_today_notice(_: web::Json<Value>) -> impl Responder {
    // println!("{:#?}", kakao);
    let mut result = Template::new();
    result.add_qr(QuickReply::new(
        "오늘".to_string(),
        "오늘 공지 보여줘".to_string(),
    ));
    result.add_qr(QuickReply::new(
        "어제".to_string(),
        "어제 공지 보여줘".to_string(),
    ));

    let mut notices = notice_parse(Some(30)).await.unwrap();
    let today = Local::now().format("%y.%m.%d").to_string(); // "21.07.20"

    let mut list_card = ListCard::new(format!("{}) 오늘 공지", today));

    list_card.add_button(Button::Share(ShareButton::new("공유하기".to_string())));

    // notices.iter().position(|&n| n.date.ne(&today)).unwrap();

    notices = notices
        .into_iter()
        .filter(|notice| notice.date.eq(&today))
        .collect();

    if notices.len() > 5 {
        let label = format!("{}개 더보기", notices.len() - 5);
        list_card.add_button(Button::Msg(MsgButton::new(label)));
        notices.resize(5, Notice::default());
    } else {
        list_card.add_button(Button::Link(
            LinkButton::new("아주대 공지".to_string()).set_link(AJOU_LINK.to_string()),
        ));
    }

    if notices.is_empty() {
        list_card.add_item(ListItem::new("공지가 없습니다!".to_string()).set_image(
            "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg".to_string(),
        ));
    } else {
        for notice in notices.iter_mut() {
            if notice.title.graphemes(true).count() > 35 {
                notice.title = UnicodeSegmentation::grapheme_indices(notice.title.as_str(), true)
                    .enumerate()
                    .filter(|&(i, _)| i < 32)
                    .map(|(_, (_, s))| s)
                    .collect::<Vec<&str>>()
                    .join("")
                    + "...";
            }
            let description = format!(
                "{} {}",
                notice.writer,
                notice.date[notice.date.len() - 5..].to_string()
            );

            list_card.add_item(
                ListItem::new((*notice.title).to_string())
                    .set_desc(description)
                    .set_link((*notice.link).to_string()),
            );
        }
    }

    // list_card.add_item(ListItem::new("제목".to_string()).set_desc("설명".to_string()));

    result.add_output(list_card.build()); // moved list_card's ownership

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&result).unwrap())
}

#[post("/weather")]
pub async fn ask_weather(_: web::Json<Value>) -> impl Responder {
    let weather = weather_parse().await.unwrap();
    let description = format!("현재 날씨는 {}, {} (체감 {})\n최저기온 {}, 최고기온은 {}\n\n낮, 밤 강수 확률은 {}, {}\n미세먼지 농도는 {}\n자외선 지수는 {}",
        weather.current_status, weather.current_temp, weather.wind_chill,
        weather.min_temp, weather.max_temp,
        weather.rain_day, weather.rain_night,
        weather.fine_dust, weather.uv);

    let mut result = Template::new();

    let basic_card = BasicCard::new()
        .set_title("[수원 영통구 기준]".to_string())
        .set_desc(description)
        .set_thumbnail(weather.icon)
        .add_button(Button::Link(
            LinkButton::new("자세히".to_string()).set_link(NAVER_WEATHER.to_string()),
        ));

    result.add_output(basic_card.build());

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&result).unwrap())
}

#[post("/schedule")]
pub async fn get_schedule(conn: web::Data<DbPool>) -> impl Responder {
    let mut result = Template::new();
    let mut carousel = Carousel::new().set_type(BasicCard::id());

    let mut rng = rand::thread_rng();

    for sched in query::show_scheds(&conn.get().unwrap()).await.unwrap() {
        // println!("id: {}, content: {}", sched.id, sched.content);

        let basic_card = BasicCard::new()
            .set_title(sched.content.to_string())
            .set_desc(format!("{} ~ {}", sched.start_date, sched.end_date))
            .set_thumbnail(format!(
                "https://raw.githubusercontent.com/Alfex4936/kakaoChatbot-Ajou/main/imgs/{}.png",
                CARD_IMAGES[rng.gen_range(0..CARD_IMAGES.len())]
            ));

        carousel.add_card(basic_card.build_card());
    }

    result.add_output(carousel.build());

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&result).unwrap())
}
