mod capture;

use capture::capture_main_loop::CaptureMainLoop;
use capture::capture_param::CaptureParam;

fn main() {
    let mut param = CaptureParam::default();
    param.mock = false;

    let main_loop = CaptureMainLoop::new(param);
    main_loop.run();
}
