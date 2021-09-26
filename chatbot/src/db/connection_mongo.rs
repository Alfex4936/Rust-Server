const MONGO_URL: &str = env!("MONGODB_URL");

use mongodb::{options::ClientOptions, Client};
use std::sync::Mutex;
pub type DbPool = Mutex<Client>;

pub async fn init_mongo() -> DbPool {
    let client_options = ClientOptions::parse(MONGO_URL).await.unwrap();
    Mutex::new(Client::with_options(client_options).unwrap())
}
