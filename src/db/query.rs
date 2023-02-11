#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::db::models::Notice;
use crate::db::models::Schedule;

use crate::db::schema::ajou_notices;
use crate::db::schema::ajou_notices::dsl::*;
use crate::db::schema::ajou_sched::dsl::*;

pub fn show_scheds(conn: &MysqlConnection) -> QueryResult<Vec<Schedule>> {
    //posts.filter(published.eq(true))
    ajou_sched.limit(5).load::<Schedule>(&*conn)
}

// Load notices from MySQL db not from homepage
pub fn get_notices(conn: &MysqlConnection, _date: String) -> QueryResult<Vec<Notice>> {
    // let query = format!(
    //     "SELECT * FROM ajou_notices WHERE date = {} ORDER BY id DESC",
    //     _date
    // );

    ajou_notices
        .filter(date.eq(_date))
        .order(ajou_notices::id.desc()) // becuz of ambiguous
        .load::<Notice>(&*conn)
}
