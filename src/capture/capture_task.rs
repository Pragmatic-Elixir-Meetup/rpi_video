pub trait CaptureTask {
    fn run(&self);
    fn complete_message(&self) -> String;
}
