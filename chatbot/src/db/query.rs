use crate::routes::DbPool;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;

use crate::db::models::{Notice, Schedule, User};

/************* MONGO *************/
pub async fn show_scheds(conn: &DbPool) -> Result<Vec<Schedule>, ()> {
    let sched_collection = conn.sched_collection();

    let mut scheds = sched_collection.find(doc! {}, None).await.unwrap();
    let mut result: Vec<Schedule> = Vec::new();
    while let Some(sched) = scheds.try_next().await.unwrap() {
        result.push(sched);
    }

    Ok(result)
}

// Load notices from Mongo db not from homepage
pub async fn get_notices_by_date(conn: &DbPool, _date: String) -> Result<Vec<Notice>, ()> {
    let notice_collection = conn.notice_collection();

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

// Load notices from Mongo db not from homepage
pub async fn save_user(conn: &DbPool, user: &User) -> Result<User, String> {
    let user_collection = conn.user_collection();

    let filter = doc! { "id": user.id.clone() };
    let update = doc! { "$set": { "openai_key": user.openai_key.clone() } };
    let options = mongodb::options::FindOneAndUpdateOptions::builder()
        .upsert(true)
        .return_document(mongodb::options::ReturnDocument::After)
        .build();

    let result = user_collection
        .find_one_and_update(filter, update, options)
        .await
        .unwrap();

    match result {
        Some(updated_user) => Ok(updated_user),
        None => Err("User not found and upsert failed".to_string()),
    }
}

// Check if user already exists
pub async fn get_user(conn: &DbPool, user_id: &str) -> Result<Option<User>, String> {
    let user_collection = conn.user_collection();

    let filter = doc! { "id": user_id.clone() };

    match user_collection.find_one(filter, None).await {
        Ok(found_user) => Ok(found_user),
        Err(e) => Err(format!("Error checking for user existence: {}", e)),
    }
}

// Update user's mode
pub async fn update_user_mode(conn: &DbPool, user: &User) -> Result<(), String> {
    let user_collection = conn.user_collection();

    let filter = doc! { "id": user.id.clone() };
    let update = doc! { "$set": { "mode": user.mode.clone() } };

    match user_collection.update_one(filter, update, None).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error updating user mode: {}", e)),
    }
}
