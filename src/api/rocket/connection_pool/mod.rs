use crate::cache;
use crate::database;

#[cfg(feature = "db-dynamodb")]
mod dynamodb_pool;
#[cfg(feature = "cache-sqlite")]
mod sqlite_pool;

#[cfg(feature = "db-dynamodb")]
#[database("db")]
pub struct DbPool(database::dynamodb::DynamoDbDatabase);

#[cfg(feature = "cache-sqlite")]
#[database("cache")]
pub struct CachePool(cache::sqlite::SqliteCache);
