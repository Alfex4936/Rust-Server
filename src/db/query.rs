#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::db::models::Notice;
use crate::db::models::Schedule;

use crate::db::schema::ajou_sched::dsl::*;

pub fn show_scheds(conn: &MysqlConnection) -> QueryResult<Vec<Schedule>> {
    //posts.filter(published.eq(true))
    ajou_sched.limit(5).load::<Schedule>(&*conn)
}

pub fn get_notices(conn: &MysqlConnection) -> QueryResult<Vec<Notice>> {
    unimplemented!()
}
