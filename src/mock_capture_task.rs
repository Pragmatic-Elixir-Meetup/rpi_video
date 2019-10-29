use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::capture::capture_task::CaptureTask;

pub struct MockCaptureTask {
}

impl MockCaptureTask {
    pub fn new() -> Self {
        Self {
        }
    }

    fn gen_filename(&self) -> String {
        let time_now = SystemTime::now();

        let mut rand_filename = time_now
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        rand_filename.push_str(".h264");
        rand_filename
    }
}

impl CaptureTask for MockCaptureTask {
    fn run(&self) -> Result<String, String> {
        // Simply sleeps five seconds for emulating record process.
        let seconds = Duration::new(5, 0);
        sleep(seconds);

        Ok(self.gen_filename())
    }
}
