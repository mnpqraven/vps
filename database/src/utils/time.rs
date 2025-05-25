use chrono::Utc;

// TODO: better cast
pub fn now() -> i32 {
    Utc::now().timestamp_millis() as i32
}
