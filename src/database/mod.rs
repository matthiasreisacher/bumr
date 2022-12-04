use anyhow::Result;
use async_trait::async_trait;

use crate::config::Config;
use crate::model::session::Session;

#[cfg(feature = "db-dynamodb")]
pub mod dynamodb;

#[async_trait]
pub trait Database {
    /// Fetches the session with the given ID from the database.
    async fn session_get(&self, id: &str) -> Result<Option<Session>>;

    /// Creates a new session, or replaces an old session with the given session in the database.
    async fn session_put(&self, session: &Session) -> Result<()>;

    /// Deletes the session with the given ID from the database.
    async fn session_delete(&self, id: &str) -> Result<()>;
}

/// Creates, initializes and returns a new database object.
pub fn init(config: &Config) -> Result<Box<dyn Database>> {
    #[cfg(feature = "db-dynamodb")]
        return Ok(dynamodb::init(config));

    bail!("Compiled without any database feature.")
}