use std::collections::HashMap;
use std::ops::Deref;

use anyhow::Result;
use r2d2::ManageConnection;

use crate::cache::sqlite::SqliteCache;
use crate::config::Config;
use crate::model::session::Session;

#[cfg(feature = "cache-sqlite")]
pub mod sqlite;

pub trait Cache {
    /// Fetches the session with the given ID from the database.
    fn session_get(&self, id: &str) -> Result<Option<Session>>;

    /// Creates a new session, or replaces an old session with the given session in the database.
    fn session_put(&self, session: &Session) -> Result<()>;

    /// Deletes the session with the given ID from the database.
    fn session_delete(&self, id: &str) -> Result<()>;
}

/// Creates, initializes and returns a new manager for pooled Sqlite connections.
pub fn init_cache(config: &Config) -> Result<Box<dyn Cache>> {
    #[cfg(feature = "cache-sqlite")]
        return sqlite::init(config);

    bail!("Compiled without any cache feature.")
}
