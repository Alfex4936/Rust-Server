#![allow(proc_macro_derive_resolution_fallback)]

#[cfg(feature = "mongo")]
use crate::routes::DbPool;
#[cfg(not(feature = "mongo"))]
use diesel;
#[cfg(not(feature = "mongo"))]
use diesel::prelude::*;
#[cfg(feature = "mongo")]
use futures::stream::TryStreamExt;
#[cfg(feature = "mongo")]
use mongodb::{bson::doc, options::FindOptions};

use crate::db::models::Notice;
use crate::db::models::Schedule;

#[cfg(not(feature = "mongo"))]
use crate::db::schema::ajou_notices;
#[cfg(not(feature = "mongo"))]
use crate::db::schema::ajou_notices::dsl::*;
#[cfg(not(feature = "mongo"))]
use crate::db::schema::ajou_sched::dsl::*;

#[cfg(not(feature = "mongo"))]
pub async fn show_scheds(conn: &MysqlConnection) -> QueryResult<Vec<Schedule>> {
    //posts.filter(published.eq(true))
    ajou_sched.load::<Schedule>(&*conn)
}

#[cfg(not(feature = "mongo"))]
// Load notices from MySQL db not from homepage
pub async fn get_notices_by_date(
    conn: &MysqlConnection,
    _date: String,
) -> QueryResult<Vec<Notice>> {
    // let query = format!(
    //     "SELECT * FROM ajou_notices WHERE date = {} ORDER BY id DESC",
    //     _date
    // );

    ajou_notices
        .filter(date.eq(_date))
        .order(ajou_notices::id.desc()) // becuz of ambiguous
        .load::<Notice>(&*conn)
}

/************* MONGO *************/
#[cfg(feature = "mongo")]
pub async fn show_scheds(conn: &DbPool) -> Result<Vec<Schedule>, ()> {
    let sched_collection = conn
        .lock()
        .unwrap()
        .database("ajou")
        .collection::<Schedule>("schedule");

    let mut scheds = sched_collection.find(doc! {}, None).await.unwrap();
    let mut result: Vec<Schedule> = Vec::new();
    while let Some(sched) = scheds.try_next().await.unwrap() {
        result.push(sched);
    }

    Ok(result)
}

#[cfg(feature = "mongo")]
// Load notices from Mongo db not from homepage
pub async fn get_notices_by_date(conn: &DbPool, _date: String) -> Result<Vec<Notice>, ()> {
    let notice_collection = conn
        .lock()
        .unwrap()
        .database("ajou")
        .collection::<Notice>("notice");

    let mut notices = notice_collection
        .find(doc! {"date": {"$eq": _date}}, None)
        .await
        .unwrap();

    let mut result: Vec<Notice> = Vec::new();
    while let Some(notice) = notices.try_next().await.unwrap() {
        result.push(notice);
    }

    Ok(result)
}
