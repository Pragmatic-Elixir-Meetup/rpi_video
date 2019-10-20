use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;

use crate::capture::capture_main_loop_injector::CaptureMainLoopInjector;
use crate::capture::capture_task::CaptureTask;
use crate::capture::mock_capture_task::MockCaptureTask;
use crate::capture::real_capture_task::RealCaptureTask;

pub struct CaptureState {
    channel_receiver: Option<mpsc::Receiver<String>>,
    main_loop_injector: Option<CaptureMainLoopInjector>,
    pending_tasks: VecDeque<Box<dyn CaptureTask + Send>>,
}

impl CaptureState {
    pub fn new() -> Self {
        Self {
            channel_receiver: None,
            main_loop_injector: None,
            pending_tasks: VecDeque::new(),
        }
    }

    pub fn add_mock_task(&mut self) {
        let task = Box::new(MockCaptureTask::new());
        self.pending_tasks.push_back(task);

        self.schedule();
    }

    pub fn add_real_task(&mut self) {
        let task = Box::new(RealCaptureTask::new());
        self.pending_tasks.push_back(task);

        self.schedule();
    }

    pub fn recv(&self) -> String {
        if self.idle() {
            return "".to_string();
        }

        self.channel_receiver.as_ref().unwrap().recv().unwrap()
    }

    pub fn set_main_loop_injector(
        &mut self,
        main_loop_injector: Option<CaptureMainLoopInjector>) {
        self.main_loop_injector = main_loop_injector;
    }

    fn idle(&self) -> bool {
        self.channel_receiver.is_none()
    }

    fn schedule(&mut self) {
        if !self.idle() {
            return;
        }

        let front_item = self.pending_tasks.pop_front();
        if front_item.is_none() {
            return;
        }

        let task = front_item.unwrap();
        let injector = self.main_loop_injector.as_ref().unwrap().clone();

        let (sender, receiver) = mpsc::channel::<String>();
        self.channel_receiver = Some(receiver);

        thread::spawn(move || {
            let filename = (*task).run();
            injector.activate();
            sender.send(filename).unwrap();
        });
    }
}
