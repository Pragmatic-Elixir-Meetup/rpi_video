use crate::capture::capture_task::CaptureTask;

pub struct CaptureParam<T>
    where T: Fn() -> Box<dyn CaptureTask + Send>
{
    pub create_task: T,
}

impl<T> CaptureParam<T>
    where T: Fn() -> Box<dyn CaptureTask + Send>
{
    pub fn new(create_task: T) -> Self {
        Self {
            create_task: create_task,
        }
    }
}
