#[derive(Debug, Clone)]
pub struct CaptureParam {
    pub mock: bool,
}

impl Default for CaptureParam {
    fn default() -> Self {
        Self {
            mock: false,
        }
    }
}
