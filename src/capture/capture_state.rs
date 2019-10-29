use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;

use crate::capture::capture_main_loop_injector::CaptureMainLoopInjector;
use crate::capture::capture_param::CaptureParam;
use crate::capture::capture_task::CaptureTask;

pub struct CaptureState<T>
    where T: Fn() -> Box<dyn CaptureTask + Send>
{
    param: CaptureParam<T>,
    channel_receiver: Option<mpsc::Receiver<Result<String, String>>>,
    main_loop_injector: Option<CaptureMainLoopInjector>,
    pending_tasks: VecDeque<Box<dyn CaptureTask + Send>>,
}

impl<T> CaptureState<T>
    where T: Fn() -> Box<dyn CaptureTask + Send>
{
    pub fn new(param: CaptureParam<T>) -> Self {
        Self {
            param: param,
            channel_receiver: None,
            main_loop_injector: None,
            pending_tasks: VecDeque::new(),
        }
    }


    pub fn add_task(&mut self) {
        let task = (self.param.create_task)();
        self.pending_tasks.push_back(task);

        self.schedule();
    }

    pub fn recv(&mut self) -> Result<String, String> {
        if self.idle() {
            return Ok("".to_string());
        }

        let result = self.channel_receiver.as_ref().unwrap().recv().unwrap();

        // Schedules for another task after receiving.
        self.channel_receiver = None;
        self.schedule();

        result
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

        let (sender, receiver) = mpsc::channel::<Result<String, String>>();
        self.channel_receiver = Some(receiver);

        thread::spawn(move || {
            let filename = (*task).run();
            injector.activate();
            sender.send(filename).unwrap();
        });
    }
}
