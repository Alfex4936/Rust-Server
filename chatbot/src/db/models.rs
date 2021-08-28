#![allow(proc_macro_derive_resolution_fallback)]

use crate::db::schema::ajou_notices;
use crate::db::schema::ajou_sched;
// use diesel::prelude::*;

use serde_json::Value;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug, Default, Clone)]
#[table_name = "ajou_notices"]
pub struct Notice {
    pub id: i32,
    pub category: String,
    pub title: String,
    pub date: String,
    pub link: String,
    pub writer: String,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "ajou_sched"]
pub struct Schedule {
    pub id: i32,
    pub start_date: String,
    pub end_date: String,
    pub content: String,
}

#[derive(Debug, Default)]
pub struct Weather {
    pub max_temp: String,
    pub min_temp: String,
    pub current_temp: String,
    pub current_status: String,
    pub wind_chill: String, // 체감온도
    pub rain_day: String,
    pub rain_night: String,
    pub fine_dust: String,
    pub ultra_dust: String,
    pub uv: String,
    pub icon: String,
}

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Meal {
//     pub msg_code: String,
//     #[serde(rename = "p018Text")]
//     pub data: MealContent,
// }

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// /// 63 기숙사, 220 학생, 221 교직원
// pub struct MealContent {
//     pub breakfast: String, //아침
//     pub lunch: String,     // 점심
//     pub dinner: String,    // 저녁
//     pub snack_bar: String, // 분식
//     #[serde(rename = "menuDt")]
//     pub date: String, // 날짜
//     #[serde(rename = "restaurantNm")]
//     pub name: String, // 식당 이름 (교직원식당(생활관 2층))
// }

#[derive(Debug, Deserialize)]
pub struct Library {
    code: String,
    pub data: LibraryData,
    message: String,
    pub success: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryData {
    pub list: Vec<LibraryList>,
    total_count: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryList {
    id: u32,
    pub active_total: u32,
    pub available: u32,
    branch_group: Value,
    disable_period: Value,
    is_active: bool,
    is_reservable: bool,
    pub name: String,
    note: Value,
    occupied: u32,
    room_type_id: u32,
    total: u32,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct People {
    msg_code: String,
    pub phone_number: Vec<PeopleList>,
}

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PeopleList {
    // 전부 None일 수도...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tel_no: Option<String>, // 전화번호: 031-219-"1234"
    #[serde(skip_serializing_if = "Option::is_none")]
    dept_cd: Option<String>, // "DS01234657"
    #[serde(skip_serializing_if = "Option::is_none")]
    user_no: Option<String>, // "201900000"
    #[serde(skip_serializing_if = "Option::is_none")]
    buss_nm: Option<String>, // 업무명: "XXX학과(공학인증)"
    #[serde(skip_serializing_if = "Option::is_none")]
    mdf_line_no: Option<String>, // "289"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_nm: Option<String>, // 부서명: "정보통신대학교학팀(팔달관 777-1)"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>, // 이메일: "example@ajou.ac.kr"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kor_nm: Option<String>, // 이름(신분): "이름1(직원)" | "이름2(교원)"
}
