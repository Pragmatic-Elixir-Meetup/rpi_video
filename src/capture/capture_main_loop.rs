extern crate mio;

use crate::capture::capture_param::CaptureParam;
use crate::capture::capture_state::CaptureState;

pub struct CaptureMainLoop {
    param: CaptureParam,
    state: CaptureState,
}

impl CaptureMainLoop {
    pub fn new(param: CaptureParam) -> Self {
        Self {
            param: param,
            state: CaptureState::new(),
        }
    }

    pub fn run(&self) {
        let poll = mio::Poll::new().unwrap();
        let mut events = mio::Events::with_capacity(4);

        loop {
            poll.poll(&mut events, None).unwrap();

        }
    }
}
