use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct TimeManager {
    pub start_time: u64,
}

impl TimeManager {
    pub fn new() -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Время пошло назад")
            .as_secs();
        Self {
            start_time: since_the_epoch,
        }
    }

    pub fn get_start_time(&self) -> u64 {
        self.start_time
    }

    fn get_current_time(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    pub fn is_expire_time(&self, time: i32) -> bool {
        if time < -0 {
            return false;
        }
        (self.get_start_time() + time as u64) < self.get_current_time()
    }

    pub fn get_expire_time(&self, time: i32) -> i32 {
        if time < 0 {
            return -1;
        }
        (self.get_current_time() - self.get_start_time()) as i32 + time
    }
}
