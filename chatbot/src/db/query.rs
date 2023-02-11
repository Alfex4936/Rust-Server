#![allow(proc_macro_derive_resolution_fallback)]

use crate::routes::DbPool;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;

use crate::db::models::Notice;
use crate::db::models::Schedule;

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
