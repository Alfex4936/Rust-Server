extern crate serde_derive;
// #[macro_use]
extern crate chrono;
extern crate chrono_tz;
extern crate serde_json;

use serde::{Deserialize, Serialize};

pub const AJOU_LINK: &str = "https://www.ajou.ac.kr/kr/ajou/notice.do";
pub const MY_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/100.0.4896.127 Safari/537.36";
pub const MONGO_URL: &str = "mongo";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Notice {
    pub id: i32,
    pub category: String,
    pub title: String,
    pub date: String,
    pub link: String,
    pub writer: String,
}
