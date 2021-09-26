pub mod info;
pub mod notice;
pub mod route;

#[cfg(feature = "mongo")]
pub use crate::db::connection_mongo::DbPool;
#[cfg(not(feature = "mongo"))]
pub use crate::db::connection_mysql::DbPool;
