use std::io::{Error, ErrorKind};

use r2d2::{ManageConnection, Pool};
use rocket_contrib::databases::{DatabaseConfig, DbError, Poolable};

use crate::cache::sqlite::SqliteCache;
use crate::config::Config;

/// CacheManger to pool Sqlite connections via r2d2
pub struct SqliteCacheManager {
    url: String,
}

impl SqliteCacheManager {
    pub fn new(config: &DatabaseConfig) -> Self {
        SqliteCacheManager {
            url: config.url.to_owned(),
        }
    }
}

impl ManageConnection for SqliteCacheManager {
    type Connection = SqliteCache;
    type Error = Error;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        SqliteCache::new(&self.url)
            .map_err(|e| Error::new(ErrorKind::ConnectionRefused, e))
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.validate()
            .map_err(|e| Error::new(ErrorKind::Interrupted, e))
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

impl Poolable for SqliteCache {
    type Manager = SqliteCacheManager;
    type Error = DbError<std::io::Error>;

    fn pool(config: DatabaseConfig) -> Result<Pool<Self::Manager>, Self::Error> {
        let manager = SqliteCacheManager::new(&config);

        r2d2::Pool::builder()
            .max_size(config.pool_size)
            .build(manager)
            .map_err(DbError::PoolError)
    }
}
