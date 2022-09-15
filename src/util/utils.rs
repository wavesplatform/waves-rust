use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_current_epoch_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get duration since Unix epoch")
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use crate::util::get_current_epoch_millis;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test() {
        let current_epoch_millis1 = get_current_epoch_millis();
        thread::sleep(Duration::from_secs(1));
        let current_epoch_millis2 = get_current_epoch_millis();
        assert!(
            current_epoch_millis2 > current_epoch_millis1,
            "now1 = {}, now2 = {}",
            current_epoch_millis1,
            current_epoch_millis2
        );
    }
}
