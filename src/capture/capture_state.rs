use crate::capture::capture_task::CaptureTask;

pub struct CaptureState {
    tasks: Vec<Box<CaptureTask>>,
}

impl CaptureState {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
        }
    }
}
