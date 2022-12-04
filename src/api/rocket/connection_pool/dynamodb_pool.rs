use std::io::{Error, ErrorKind};

use r2d2::{ManageConnection, Pool};
use rocket_contrib::databases::{DatabaseConfig, DbError, Poolable};

use crate::config::Config;
use crate::database::dynamodb::DynamoDbDatabase;

/// CacheManger to pool Sqlite connections via r2d2
pub struct DynamoDbDatabaseManager {
    name: String,
    endpoint: String,
}

impl DynamoDbDatabaseManager {
    pub fn new(config: &DatabaseConfig) -> Result<Self, Error> {
        Ok(DynamoDbDatabaseManager {
            name: config.extras.get("name")
                .ok_or(Error::new(ErrorKind::NotFound, "Could not find DynamoDB name in config"))?
                .to_string(),
            endpoint: config.url.to_owned(),
        })
    }
}

impl ManageConnection for DynamoDbDatabaseManager {
    type Connection = DynamoDbDatabase;
    type Error = Error;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Ok(DynamoDbDatabase::new(&self.name, &self.endpoint))
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.validate()
            .map_err(|e| Error::new(ErrorKind::Interrupted, e))
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

impl Poolable for DynamoDbDatabase {
    type Manager = DynamoDbDatabaseManager;
    type Error = DbError<std::io::Error>;

    fn pool(config: DatabaseConfig) -> Result<Pool<Self::Manager>, Self::Error> {
        let manager = DynamoDbDatabaseManager::new(&config)
            .map_err(DbError::Custom)?;

        r2d2::Pool::builder()
            .max_size(config.pool_size)
            .build(manager)
            .map_err(DbError::PoolError)
    }
}
