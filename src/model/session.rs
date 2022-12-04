use crate::model::get_timestamp_now;

use super::generate_id;

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub created_at: i64,
}

impl Session {
    /// Create a new Session with a randomly generated id and the current timestamp.
    pub fn new(user_id: &str) -> Self {
        Session {
            id: generate_id(),
            user_id: user_id.to_owned(),
            created_at: get_timestamp_now(),
        }
    }
}