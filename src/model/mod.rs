use chrono::Utc;
use uuid::Uuid;

pub mod session;

/// Generates and returns a new table id.
///
/// UUIDs v4 are used to generate new IDs randomly. Keep in mind, UUIDs are
/// highly unlikely to clash, however its not impossible!
pub fn generate_id() -> String {
    Uuid::new_v4().to_simple().to_string()
}

/// Returns the current UNIX timestamp
fn get_timestamp_now() -> i64 {
    Utc::now().timestamp()
}