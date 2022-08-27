use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_epoch_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get duration since Unix epoch")
        .as_millis() as u64
}
