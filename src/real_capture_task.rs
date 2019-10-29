extern crate rpi_video_rs;

use self::rpi_video_rs::recorder::Recorder;

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
    fn run(&self) -> Result<String, String> {
        let mut recorder = Recorder::new(None);

        match recorder.run() {
            Ok(res) => Ok(res.output_file_path),
            Err(error) => Err(error.message),
        }
    }
}
