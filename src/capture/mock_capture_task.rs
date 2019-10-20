use crate::capture::capture_task::CaptureTask;

pub struct MockCaptureTask {
}

impl MockCaptureTask {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl CaptureTask for MockCaptureTask {
    fn run(&self) {

        // TODO

    }

    fn complete_message(&self) -> String {
        String::new()
    }
}
