use crate::capture::capture_task::CaptureTask;

pub struct RealCaptureTask {
}

impl RealCaptureTask {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl CaptureTask for RealCaptureTask {
    fn run(&self) -> String {

        // TODO

        String::new()

    }
}
