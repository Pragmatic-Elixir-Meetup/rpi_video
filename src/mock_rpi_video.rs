mod capture;
mod mock_capture_task;

use capture::capture_main_loop::CaptureMainLoop;
use capture::capture_param::CaptureParam;

use crate::mock_capture_task::MockCaptureTask;

fn main() {
    let param = CaptureParam::new(|| { Box::new(MockCaptureTask::new()) });

    let mut main_loop = CaptureMainLoop::new(param);
    main_loop.run();
}
