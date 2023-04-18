// use crate::db::connection::DbPool;
use crate::db::query;
use crate::routes::DbPool;
use crate::utils::parser::{
    library_parse, load_courses, meal_parse, people_parse, weather_parse, NAVER_WEATHER,
};
use crate::CARD_IMAGES;

use crate::routes::Kakao;
use actix_web::{post, web, Responder};
use chrono::prelude::Local;
use kakao_rs::prelude::*;
use rand::Rng;
use serde_json::Value;

const INTEL: &str = "031-219-";

#[post("/weather")]
pub async fn get_weather() -> impl Responder {
    let weather = weather_parse().await.unwrap();
    let description = format!("현재 날씨는 {}, {}\n최저기온 {}, 최고기온은 {}\n\n낮, 밤 강수 확률은 {}, {}\n미세먼지 농도는 {}\n자외선은 {}, 일몰은 {}\n내일은 {}/{}",
        weather.current_status, weather.current_temp,
        weather.min_temp, weather.max_temp,
        weather.rain_day, weather.rain_night,
        weather.fine_dust, weather.uv, weather.sunset,
        weather.tmrw_min_temp, weather.tmrw_max_temp);

    let mut result = Template::new();

    let basic_card = BasicCard::new()
        .set_title("[수원 영통구 기준]")
        .set_desc(description)
        .set_thumbnail(weather.icon)
        .add_button(
            Button::new(ButtonType::Link)
                .set_label("자세히")
                .set_link(NAVER_WEATHER),
        );

    result.add_output(basic_card.build());

    Kakao { template: result }
}

#[post("/schedule")]
pub async fn get_schedule(conn: web::Data<DbPool>) -> impl Responder {
    let mut result = Template::new();
    let mut carousel = Carousel::new().set_type(BasicCard::id());

    let mut rng = rand::thread_rng();

    let db = &conn;

    for sched in query::show_scheds(db).await.unwrap() {
        // println!("id: {}, content: {}", sched.id, sched.content);

        let basic_card = BasicCard::new()
            .set_title(sched.content)
            .set_desc(format!("{} ~ {}", sched.start_date, sched.end_date))
            .set_thumbnail(format!(
                "https://raw.githubusercontent.com/Alfex4936/kakaoChatbot-Ajou/main/imgs/{}.png",
                CARD_IMAGES[rng.gen_range(0..CARD_IMAGES.len())]
            ));

        carousel.add_card(basic_card.build_card());
    }

    result.add_output(carousel.build());

    Kakao { template: result }
}

#[post("/library")]
pub async fn get_library() -> impl Responder {
    let mut result = Template::new();
    let library = match library_parse().await {
        Ok(yes) => yes,
        _ => {
            result.add_qr(QuickReply::new("도서관 좌석", "ㄷ"));
            result.add_output(SimpleText::new("홈페이지 반응이 늦습니다. :(").build());

            return Kakao { template: result };
        }
    };

    let mut description = vec![];
    for data in &library.data.list {
        description.push(format!(
            "{}: {}/{} (잔여/전체)",
            data.name, data.available, data.active_total
        ));
    }

    let basic_card = BasicCard::new()
        .set_title("[중앙도서관]")
        .set_desc(description.join("\n"))
        .add_button(
            Button::new(ButtonType::Link)
                .set_label("중앙도서관 홈페이지")
                .set_link("https://library.ajou.ac.kr/#/"),
        );

    result.add_output(SimpleText::new("현재 중앙 도서관 좌석 현황입니다!").build());
    result.add_output(basic_card.build());

    Kakao { template: result }
}

#[post("/prof")]
pub async fn get_people(kakao: web::Json<Value>) -> impl Responder {
    let mut keyword = kakao["action"]["params"]["search"]
        .as_str()
        .unwrap()
        .to_string();
    keyword.retain(|c| !c.is_whitespace());

    let mut result = Template::new();

    result.add_qr(QuickReply::new("인물 검색", "인물"));

    let mut carousel = Carousel::new();

    let mut people = match people_parse(&keyword).await {
        Ok(yes) => yes,
        _ => {
            result.add_output(SimpleText::new("홈페이지 반응이 늦습니다. :(").build());

            return Kakao { template: result };
        }
    };

    // result.add_qr(QuickReply::new("인물", "인물"));
    if people.phone_number.is_empty() {
        result.add_output(SimpleText::new(format!("{} 검색 결과가 없습니다.", keyword)).build());
        return Kakao { template: result };
    } else if people.phone_number.len() > 10 {
        result.add_output(SimpleText::new("10개의 검색 결과만 불러왔습니다.").build());
        people.phone_number.resize(10, Default::default());
    } // if greater than 10

    for person in people.phone_number.iter().take(10) {
        let basic_card = BasicCard::new()
            .set_title(format!(
                "{} ({})",
                person.kor_nm.as_ref().unwrap_or(&"X".to_string()),
                person.email.as_ref().unwrap_or(&"X".to_string())
            ))
            .set_desc(format!(
                "전화번호: {}\n부서명: {}",
                INTEL.to_string() + person.tel_no.as_ref().unwrap_or(&"X".to_string()),
                person.dept_nm.as_ref().unwrap_or(&"X".to_string())
            ))
            .add_button(Button::new(ButtonType::Call).set_label("전화").set_number(
                &(INTEL.to_string() + person.tel_no.as_ref().unwrap_or(&"X".to_string())),
            ))
            .add_button(
                Button::new(ButtonType::Link)
                    .set_label("이메일")
                    .set_link(format!(
                        "mailto:{}?subject=안녕하세요",
                        person.email.as_ref().unwrap_or(&"X".to_string())
                    )),
            );

        carousel.add_card(basic_card.build_card());
    }

    result.add_output(carousel.build());

    Kakao { template: result }
}

#[post("/map")]
pub async fn get_map() -> impl Responder {
    let mut result = Template::new();
    result.add_output(SimpleText::new("[아주대 지도 (Ajou Map)]").build());

    result.add_output(SimpleText::new("https://www.ajou.ac.kr/en/intro/way01.do").build());

    result.add_output(
        SimpleImage::new(
            "https://raw.githubusercontent.com/Alfex4936/Rust-Server/main/imgs/map.jpg",
            "https://www.ajou.ac.kr/en/intro/way01.do",
        )
        .build(),
    );

    Kakao { template: result }
}

#[post("/meal")]
pub async fn get_meal_today() -> impl Responder {
    let today = Local::now().format("%Y%m%d").to_string(); // "20230410"
    let meal = meal_parse(today.to_owned(), 63).await.unwrap();

    // 기숙사 식당(63)
    let mut result = Template::new();
    let text = format!(
        "점심: {}\n저녁: {}",
        meal.data.lunch.unwrap_or("없음".to_string()),
        meal.data.dinner.unwrap_or("없음".to_string())
    );
    result.add_output(SimpleText::new(format!("[아주대 기숙사 식당]\n{text}")).build());

    // 교직원 식당(221)
    let meal = meal_parse(today.to_owned(), 221).await.unwrap();

    let text = format!(
        "점심: {}\n저녁: {}",
        meal.data.lunch.unwrap_or("없음".to_string()),
        meal.data.dinner.unwrap_or("없음".to_string())
    );
    result.add_output(SimpleText::new(format!("[아주대 교직원 식당]\n{text}")).build());

    result.add_qr(QuickReply::new("ㅂ", "ㅂ"));
    result.add_qr(QuickReply::new("학식", "학식"));

    Kakao { template: result }
}

#[post("/course")]
pub async fn get_courses(kakao: web::Json<Value>) -> impl Responder {
    let mut keyword = kakao["action"]["params"]["course"]
        .as_str()
        .unwrap()
        .to_string();
    keyword.retain(|c| !c.is_whitespace());

    let mut result = Template::new();
    let mut carousel = Carousel::new().set_type(BasicCard::id());

    for course in load_courses(&keyword).await.unwrap() {
        let basic_card = BasicCard::new()
            .set_title(course.subject_korean_name)
            .set_desc(format!(
                "{} (장소: {})",
                course.main_lecturer_name, course.class_time_korean
            ));

        carousel.add_card(basic_card.build_card());
    }

    result.add_output(carousel.build());

    Kakao { template: result }
}

pub fn init_info(cfg: &mut web::ServiceConfig) {
    cfg.service(get_weather);
    cfg.service(get_schedule);
    cfg.service(get_people);
    cfg.service(get_library);
    cfg.service(get_map);
    cfg.service(get_meal_today);
    cfg.service(get_courses);
}
