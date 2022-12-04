// use anyhow::Result;
//
// use crate::cache;
// use crate::database;
// use crate::model::session::Session;
// use crate::repository::Repository;
//
// pub fn get_session(repository: &Repository, id: &str) -> Result<Option<Session>> {
//     let mut session = None;
//
//     // Fetch session from cache is available
//     if let Some(cache) = repository.cache.as_ref() {
//         session = cache.session_get(id)?;
//     }
//
//     // Fetch session from database if cache is unavailable or session was not found
//     if session.is_none() {
//         session = repository.database.session_get(id)?;
//     }
//
//     Ok(session)
// }