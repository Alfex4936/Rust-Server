#![allow(proc_macro_derive_resolution_fallback)]
// use crate::db::connection::DbPool;

use crate::routes::DbPool;

use crate::db::models::Notice;

use crate::db::query;
use crate::utils::parser::{notice_parse, AJOU_LINK};

use kakao_rs::components::basics::*;
use kakao_rs::components::buttons::*;
use kakao_rs::components::cards::*;

use actix_web::{post, web, HttpResponse, Responder};
use chrono::prelude::Local;
use chrono::Duration;
use lazy_static::lazy_static;
use serde_json::Value;
// use unicode_segmentation::UnicodeSegmentation;

use std::collections::HashMap;

lazy_static! {
    static ref CATEGORIES: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("학사", 1);
        m.insert("비교과", 2);
        m.insert("장학", 3);
        m.insert("학술", 4);
        m.insert("입학", 5);
        m.insert("취업", 6);
        m.insert("사무", 7);
        m.insert("기타", 8);
        m.insert("행사", 166);
        m.insert("파란학기제", 167);
        m.insert("파란학기", 167);
        m.insert("학사일정", 168);
        m.insert("학사 일정", 168);
        m
    };
}

#[post("/notice/last")]
pub async fn get_last_notice() -> impl Responder {
    let mut result = Template::new();
    result.add_qr(QuickReply::new("오늘", "오늘 공지 보여줘"));
    result.add_qr(QuickReply::new("어제", "어제 공지 보여줘"));

    let notice = match notice_parse("ajou", Some(1)).await {
        Ok(yes) => yes,
        _ => {
            result.add_output(SimpleText::new("홈페이지 반응이 늦습니다. :(").build());

            return HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&result).unwrap());
        }
    };

    let notice = notice.first().unwrap().to_owned();

    let mut list_card = ListCard::new(format!("{} 공지", notice.date));
    list_card.add_button(Button::Share(ShareButton::new("공유하기")));

    // if notice.title.graphemes(true).count() > 35 {
    //     notice.title = UnicodeSegmentation::grapheme_indices(notice.title.as_str(), true)
    //         .enumerate()
    //         .filter(|&(i, _)| i < 32)
    //         .map(|(_, (_, s))| s)
    //         .collect::<Vec<&str>>()
    //         .join("")
    //         + "...";
    // }
    let description = format!(
        "[{}] {} {}",
        notice.category,
        notice.writer,
        notice.date[notice.date.len() - 5..].to_string()
    );

    list_card.add_item(
        ListItem::new((*notice.title).to_string())
            .set_desc(description)
            .set_link((*notice.link).to_string()),
    );

    result.add_output(list_card.build());

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&result).unwrap())
}

#[post("/notice/today")]
pub async fn get_today_notice(_: web::Json<Value>) -> impl Responder {
    // println!("{:#?}", kakao);
    let mut result = Template::new();
    result.add_qr(QuickReply::new("오늘", "오늘 공지 보여줘"));
    result.add_qr(QuickReply::new("어제", "어제 공지 보여줘"));

    let mut notices = match notice_parse("ajou", Some(30)).await {
        Ok(yes) => yes,
        _ => {
            result.add_output(SimpleText::new("홈페이지 반응이 늦습니다. :(").build());

            return HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&result).unwrap());
        }
    };
    let today = Local::now().format("%y.%m.%d").to_string(); // "21.07.20"

    let mut list_card = ListCard::new(format!("{}) 오늘 공지", today));

    // notices.iter().position(|&n| n.date.ne(&today)).unwrap();

    notices = notices
        .into_iter()
        .filter(|notice| notice.date.eq(&today))
        .collect();

    // let length = notices.len();

    if !notices.is_empty() {
        result.add_output(SimpleText::new(format!("오늘 공지 총 {}개", notices.len())).build());
    }

    if notices.len() > 5 {
        let label = format!("{}개 더보기", notices.len() - 5);
        list_card.add_button(Button::Msg(MsgButton::new(label).set_msg("더보기")));
        notices.resize(5, Notice::default());
    } else {
        list_card.add_button(Button::Link(
            LinkButton::new("아주대 공지").set_link(AJOU_LINK),
        ));
    }

    if notices.is_empty() {
        list_card.add_item(ListItem::new("공지가 없습니다!").set_image(
            "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg",
        ));
    } else {
        list_card.add_button(Button::Share(ShareButton::new("공유하기")));
        for notice in notices.iter_mut() {
            // if notice.title.graphemes(true).count() > 35 {
            //     notice.title = UnicodeSegmentation::grapheme_indices(notice.title.as_str(), true)
            //         .enumerate()
            //         .filter(|&(i, _)| i < 32)
            //         .map(|(_, (_, s))| s)
            //         .collect::<Vec<&str>>()
            //         .join("")
            //         + "...";
            // }
            let description = format!(
                "[{}] {} {}",
                notice.category,
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

#[post("/notice/today2")]
pub async fn get_more_today_notice(_: web::Json<Value>) -> impl Responder {
    // println!("{:#?}", kakao);
    let mut result = Template::new();

    let mut notices = notice_parse("ajou", Some(30)).await.unwrap();
    let today_desc = Local::now().format("%m월 %d일)").to_string(); // "21.07.20"
    let today = Local::now().format("%y.%m.%d").to_string(); // "21.07.20"

    // notices.iter().position(|&n| n.date.ne(&today)).unwrap();

    notices = notices
        .into_iter()
        .filter(|notice| notice.date.eq(&today))
        .collect();

    // let length = notices.len();

    if notices.len() < 5 {
        result.add_qr(QuickReply::new("오늘 공지", "ㅗ"));
        result.add_output(SimpleText::new("공지가 5개 이하 입니다!").build());

        return HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&result).unwrap());
    }

    let mut carousel = Carousel::new();
    carousel.set_header(
        format!("오늘 공지 총 {}개", notices.len()),
        format!("{}개를 더 불러왔습니다", notices.len() - 5),
        "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg".to_string(),
    );

    for notice in notices[5..].iter_mut() {
        let description = format!("{} {}", today_desc, notice.writer);
        let basic_card = BasicCard::new()
            .set_title(description)
            .set_desc((*notice.title).to_string())
            .add_button(Button::Link(
                LinkButton::new("공지 보기").set_link((*notice.link).to_string()),
            ));

        carousel.add_card(basic_card.build_card());
    }

    // list_card.add_item(ListItem::new("제목".to_string()).set_desc("설명".to_string()));

    result.add_output(carousel.build()); // moved list_card's ownership

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&result).unwrap())
}

#[post("/notice/yesterday")]
pub async fn get_yesterday_notice(conn: web::Data<DbPool>) -> impl Responder {
    let mut result = Template::new();
    result.add_qr(QuickReply::new("오늘", "오늘 공지 보여줘"));
    result.add_qr(QuickReply::new("어제", "어제 공지 보여줘"));

    let date = Local::now() - Duration::days(1);
    let yesterday = date.format("%y.%m.%d").to_string();

    #[cfg(not(feature = "mongo"))]
    let db = &conn.get().unwrap();
    #[cfg(feature = "mongo")]
    let db = &conn;

    let mut notices = query::get_notices_by_date(db, yesterday.to_owned())
        .await
        .unwrap();

    let mut list_card = ListCard::new(format!("{}) 어제 공지", yesterday));

    // let length = notices.len();

    let label: String;
    if notices.len() > 5 {
        label = format!("{}개 더보기", notices.len() - 5);
        notices.resize(5, Notice::default());
    } else {
        label = "아주대 공지".to_string();
    }
    list_card.add_button(Button::Link(LinkButton::new(label).set_link(AJOU_LINK)));

    if notices.is_empty() {
        list_card.add_item(ListItem::new("공지가 없습니다!").set_image(
            "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg",
        ));
    } else {
        list_card.add_button(Button::Share(ShareButton::new("공유하기")));
        for notice in notices.iter_mut() {
            // if notice.title.graphemes(true).count() > 35 {
            //     notice.title = UnicodeSegmentation::grapheme_indices(notice.title.as_str(), true)
            //         .enumerate()
            //         .filter(|&(i, _)| i < 32)
            //         .map(|(_, (_, s))| s)
            //         .collect::<Vec<&str>>()
            //         .join("")
            //         + "...";
            // }
            let description = format!(
                "[{}] {} {}",
                notice.category,
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

    result.add_output(list_card.build());

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&result).unwrap())
}

#[post("/notice/search")]
pub async fn get_keyword_notice(kakao: web::Json<Value>) -> impl Responder {
    let kakao_keyword = &kakao["action"]["params"];
    let mut result = Template::new();

    let keyword = match kakao_keyword.get("search") {
        Some(v) => v.as_str().unwrap(),
        _ => {
            result.add_qr(QuickReply::new("검색", "검색"));
            result.add_output(SimpleText::new("검색어를 다시 입력하세요.").build());

            return HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&result).unwrap());
        }
    };

    result.add_qr(QuickReply::new("검색", "검색"));

    let mut notices = match notice_parse(keyword, Some(7)).await {
        Ok(yes) => yes,
        _ => {
            result.add_output(SimpleText::new("홈페이지 반응이 늦습니다. :(").build());

            return HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&result).unwrap());
        }
    };

    // let mut notices = notice_parse(keyword, Some(7)).await.unwrap();
    if notices.is_empty() {
        result.add_output(SimpleText::new(format!("{}에 관한 글이 없어요.", keyword)).build());

        return HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&result).unwrap());
    }

    let mut list_card = ListCard::new(format!("{} 결과", keyword));
    list_card.add_button(Button::Share(ShareButton::new("공유하기")));

    // if keyword.to_string().graphemes(true).count() > 12 {}

    let label;
    if notices.len() > 5 {
        label = "더보기";
    } else {
        label = "홈페이지 열기";
    }

    list_card.add_button(Button::Link(LinkButton::new(label).set_link(format!(
        "{}?mode=list&srSearchKey=&srSearchVal={}",
        AJOU_LINK, keyword
    ))));

    for notice in notices.iter_mut() {
        // if notice.title.graphemes(true).count() > 35 {
        //     notice.title = UnicodeSegmentation::grapheme_indices(notice.title.as_str(), true)
        //         .enumerate()
        //         .filter(|&(i, _)| i < 32)
        //         .map(|(_, (_, s))| s)
        //         .collect::<Vec<&str>>()
        //         .join("")
        //         + "...";
        // }  // 원래는 안되었는데 카카오 챗봇이 알아서 짜르는 느낌?
        let description = format!(
            "[{}] {} {}",
            notice.category,
            notice.writer,
            notice.date[notice.date.len() - 5..].to_string()
        );
        list_card.add_item(
            ListItem::new((*notice.title).to_string())
                .set_desc(description)
                .set_link((*notice.link).to_string()),
        );
    }
    result.add_output(list_card.build());
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&result).unwrap())
}

#[post("/notice/ask")]
pub async fn get_category() -> impl Responder {
    // println!("{:#?}", kakao);
    // let mut result = Template::new();
    // result.add_qr(QuickReply::new("학사", "학사"));
    // result.add_qr(QuickReply::new("학사일정", "학사일정"));
    // result.add_qr(QuickReply::new("비교과", "비교과"));
    // result.add_qr(QuickReply::new("장학", "장학"));
    // result.add_qr(QuickReply::new("취업", "취업줘"));
    // result.add_qr(QuickReply::new("사무", "사무"));
    // result.add_qr(QuickReply::new("행사", "행사"));
    // result.add_qr(QuickReply::new("파란학기제", "파란학기제"));
    // result.add_qr(QuickReply::new("학술", "학술"));
    // result.add_qr(QuickReply::new("입학", "입학"));
    // result.add_qr(QuickReply::new("기타", "기타"));

    // result.add_output(SimpleText::new("무슨 공지를 보고 싶으신가요?").build());

    // HttpResponse::Ok()
    //     .content_type("application/json")
    //     .body(serde_json::to_string(&result).unwrap())

    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"template":{"outputs":[{"simpleText":{"text":"무슨 공지를 보고 싶으신가요?"}}],"quickReplies":[{"action":"message","label":"학사","messageText":"학사"},{"action":"message","label":"학사일정","messageText":"학사일정"},{"action":"message","label":"비교과","messageText":"비교과"},{"action":"message","label":"장학","messageText":"장학"},{"action":"message","label":"취업","messageText":"취업줘"},{"action":"message","label":"사무","messageText":"사무"},{"action":"message","label":"행사","messageText":"행사"},{"action":"message","label":"파란학기제","messageText":"파란학기제"},{"action":"message","label":"학술","messageText":"학술"},{"action":"message","label":"입학","messageText":"입학"},{"action":"message","label":"기타","messageText":"기타"}]},"version":"2.0"}"#)
}

#[post("/notice/category")]
pub async fn get_category_notice(kakao: web::Json<Value>) -> impl Responder {
    let kakao_keyword = &kakao["action"]["params"];
    let mut result = Template::new();

    let keyword = match kakao_keyword.get("cate") {
        Some(v) => v.as_str().unwrap(),
        _ => {
            result.add_qr(QuickReply::new("카테", "ㅋㅌ"));
            result.add_output(SimpleText::new("오류가 발생했습니다 :(").build());

            return HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&result).unwrap());
        }
    };

    // let categories: HashMap<&str, i32> = [
    //     ("학사", 1),
    //     ("비교과", 2),
    //     ("장학", 3),
    //     ("학술", 4),
    //     ("입학", 5),
    //     ("취업", 6),
    //     ("사무", 7),
    //     ("기타", 8),
    //     ("행사", 166),
    //     ("파란학기제", 167),
    //     ("파란학기", 167),
    //     ("학사일정", 168),
    //     ("학사 일정", 168),
    // ]
    // .iter()
    // .cloned()
    // .collect();

    result.add_qr(QuickReply::new("카테", "ㅋㅌ"));

    let mut notices = match notice_parse("ajou", Some(5)).await {
        Ok(yes) => yes,
        _ => {
            result.add_output(SimpleText::new("홈페이지 반응이 늦습니다. :(").build());

            return HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&result).unwrap());
        }
    };

    let mut list_card = ListCard::new(format!("{} 공지", keyword));
    list_card.add_button(Button::Share(ShareButton::new("공유하기")));

    list_card.add_button(Button::Link(LinkButton::new(keyword).set_link(format!(
        "{}?mode=list&srCategoryId={}",
        AJOU_LINK,
        CATEGORIES.get(keyword).unwrap()
    ))));

    for notice in notices.iter_mut() {
        // if notice.title.graphemes(true).count() > 35 {
        //     notice.title = UnicodeSegmentation::grapheme_indices(notice.title.as_str(), true)
        //         .enumerate()
        //         .filter(|&(i, _)| i < 32)
        //         .map(|(_, (_, s))| s)
        //         .collect::<Vec<&str>>()
        //         .join("")
        //         + "...";
        // }  // 원래는 안되었는데 카카오 챗봇이 알아서 짜르는 느낌?
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
    result.add_output(list_card.build());
    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&result).unwrap())
}
