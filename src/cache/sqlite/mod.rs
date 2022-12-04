use std::collections::HashMap;
use std::io::ErrorKind;

use anyhow::{Context, Result};
use rusqlite::Connection;

use crate::cache::Cache;
use crate::config::Config;
use crate::model::session::Session;

mod session;

/// Holds the Sqlite connection
#[derive(Debug)]
pub struct SqliteCache {
    client: Connection
}

impl SqliteCache {
    pub fn new(url: &str) -> Result<Self> {
        let conn = rusqlite::Connection::open(url)
            .context("Could not connect to Sqlite cache")?;

        Ok(SqliteCache {
            client: conn
        })
    }

    pub fn validate(&self) -> Result<()> {
        self.client.execute("SELECT 1", params![])
            .map(|_| ())
            .map_err(anyhow::Error::msg)
    }
}

impl Cache for SqliteCache {
    fn session_get(&self, id: &str) -> Result<Option<Session>> {
        session::get(&self.client, id)
    }

    fn session_put(&self, session: &Session) -> Result<()> {
        session::put(&self.client, session)
    }

    fn session_delete(&self, id: &str) -> Result<()> {
        session::delete(&self.client, id)
    }
}

/// Creates, initializes and returns a new DynamoDB database object.
pub fn init(config: &Config) -> Result<Box<dyn Cache>> {
    let cache = SqliteCache::new(&config.cache.sqlite.path)?;
    Ok(Box::new(cache))
}
