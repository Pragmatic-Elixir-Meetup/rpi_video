pub trait CaptureTask {
    fn run(&self) -> Result<String, String>;
}
