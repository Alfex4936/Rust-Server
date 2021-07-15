#![allow(proc_macro_derive_resolution_fallback)]

use crate::db::schema::ajou_sched;
// use diesel::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Notice {
    pub id: u64,
    pub title: String,
    pub date: String,
    pub link: String,
    pub writer: String,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "ajou_sched"]
pub struct Schedule {
    pub id: i32,
    pub content: String,
    pub start_date: String,
    pub end_date: String,
}

// impl Schedule {
//     pub fn read(conn: &MysqlConnection) -> Result<Vec<Schedule>, Error> {
//         ajou_sched::table
//             .order(ajou_sched::id.asc())
//             .load::<Schedule>(conn)
//     }
// }
