const MONGO_URL: &str = env!("MONGODB_URL");
const MONGO_DB_NAME: &str = env!("MONGO_DB_NAME");

use actix_web::web;
use mongodb::{options::ClientOptions, Client, Collection};

use super::models::{Notice, Schedule, User};

pub type DbPool = web::Data<Database>;

pub struct Database {
    client: Client,
    db_name: String,
}

impl Database {
    pub async fn new() -> Self {
        let client_options = ClientOptions::parse(MONGO_URL).await.unwrap();
        let client = Client::with_options(client_options).unwrap();

        Self {
            client,
            db_name: MONGO_DB_NAME.to_string(),
        }
    }

    pub fn notice_collection(&self) -> Collection<Notice> {
        self.client.database(&self.db_name).collection("notice")
    }

    pub fn user_collection(&self) -> Collection<User> {
        self.client.database(&self.db_name).collection("user")
    }

    pub fn sched_collection(&self) -> Collection<Schedule> {
        self.client.database(&self.db_name).collection("schedule")
    }
}
