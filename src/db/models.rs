#![allow(proc_macro_derive_resolution_fallback)]

use crate::db::schema::ajou_notices;
use crate::db::schema::ajou_sched;
// use diesel::prelude::*;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug, Clone)]
#[table_name = "ajou_notices"]
pub struct Notice {
    pub id: i32,
    pub title: String,
    pub date: String,
    pub link: String,
    pub writer: String,
}

impl Default for Notice {
    fn default() -> Notice {
        Notice {
            id: 0,
            title: "".to_string(),
            date: "".to_string(),
            link: "".to_string(),
            writer: "".to_string(),
        }
    }
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
