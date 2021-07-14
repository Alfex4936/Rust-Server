#![allow(proc_macro_derive_resolution_fallback)]

use crate::db::schema::ajou_sched;
use diesel::prelude::*;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
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
