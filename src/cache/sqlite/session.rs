use anyhow::{Context, Result};
use rusqlite::{Connection, Error};
use tokio::task;

use crate::model::session::Session;

static TABLE_NAME: &str = "session";

static ATTRIBUTE_ID: &str = "id";
static ATTRIBUTE_USER_ID: &str = "user_id";
static ATTRIBUTE_CREATED_AT: &str = "created_at";

/// Fetches the session with the given id.
pub fn get(client: &Connection, id: &str) -> Result<Option<Session>> {
    let mut stmt = client.prepare(
        &format!("SELECT {0}, {1}, {2} FROM {3} WHERE {0} = ?1",
                 ATTRIBUTE_ID, ATTRIBUTE_USER_ID, ATTRIBUTE_CREATED_AT, TABLE_NAME))?;

    let mut session_iter = stmt.query_map(
        params![id],
        |row| {
            Ok(Session {
                id: row.get(0)?,
                user_id: row.get(1)?,
                created_at: row.get(2)?,
            })
        })?;

    let session = match session_iter.next() {
        Some(session) => Some(session?),
        None => None
    };

    // If more than one item was fetched, return an error
    if session_iter.next().is_none() {
        Ok(session)
    } else {
        bail!("Found more than one session for {} = {}", ATTRIBUTE_ID, id)
    }
}

/// Creates a new session, or replaces and existing one with the given session.
pub fn put(client: &Connection, session: &Session) -> Result<()> {
    client
        .execute(&format!("INSERT INTO {} ({}, {}, {}) VALUES (?1, ?2, ?3)",
                          TABLE_NAME, ATTRIBUTE_ID, ATTRIBUTE_USER_ID, ATTRIBUTE_CREATED_AT),
                 params![session.id, session.user_id, session.created_at])
        .and_then(|_| {
            debug!("Cached session {}", session.id);
            Ok(())
        })
        .or_else(|error| bail!("{}", error))
}

/// Deletes the session with the given id.
pub fn delete(client: &Connection, id: &str) -> Result<()> {
    client
        .execute(&format!("DELETE FROM {} WHERE {} = ?1",
                          TABLE_NAME, ATTRIBUTE_ID),
                 params![id])
        .and_then(|_| {
            debug!("Deleted session {}", id);
            Ok(())
        })
        .or_else(|error| bail!("{}", error))
}