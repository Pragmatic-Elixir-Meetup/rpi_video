mod capture;
mod real_capture_task;

use capture::capture_main_loop::CaptureMainLoop;
use capture::capture_param::CaptureParam;

use crate::real_capture_task::RealCaptureTask;

fn main() {
    let param = CaptureParam::new(|| { Box::new(RealCaptureTask::new()) });

    let mut main_loop = CaptureMainLoop::new(param);
    main_loop.run();
}
