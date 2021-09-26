#[cfg(feature = "mongo")]
pub mod connection_mongo;
#[cfg(not(feature = "mongo"))]
pub mod connection_mysql;
pub mod models;
pub mod query;
pub mod schema;
