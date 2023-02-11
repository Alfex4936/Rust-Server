use crate::db::models::{Notice, Schedule};
use reqwest::header::AUTHORIZATION;
use serde_json::json;

pub const HARDER_URL: &str = env!("HARPERDB_URL");
pub const HARDER_AUTH: &str = env!("HARPERDB_AUTH");

pub async fn insert_notice() -> Result<(), reqwest::Error> {
    let notice = Notice::default();

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let payload = json!({"operation": "insert","schema": "ajou","table": "notice","records": [{"id": 1,"title": "Harper","date": "2021.08.17","link": "https:", "writer": "csw"}]});
    client
        .post(HARDER_URL)
        .header(AUTHORIZATION, HARDER_AUTH)
        .json(&payload)
        .send()
        .await?;

    Ok(())
}

pub async fn get_sched() -> Result<Vec<Schedule>, reqwest::Error> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let payload = json!({"operation": "sql", "sql": "SELECT * FROM ajou.sched"});
    let result = client
        .post(HARDER_URL)
        .header(AUTHORIZATION, HARDER_AUTH)
        .json(&payload)
        .send()
        .await?;
    let response = result.text().await?;

    let scheds: Vec<Schedule> = serde_json::from_str(&response).unwrap();

    for sched in scheds.iter() {
        println!("{}", sched.content);
    }

    Ok(scheds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn harper_insert_test() {
        insert_notice().await.unwrap();
    }

    #[actix_rt::test]
    async fn harper_sql_test() {
        use std::time::Instant;
        let now = Instant::now();
        {
            get_sched().await.unwrap();
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
        // println!("{:#}", result);
    }
}
