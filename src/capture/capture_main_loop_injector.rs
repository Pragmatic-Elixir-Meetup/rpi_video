extern crate nix;

use std::os::unix::io::RawFd;

use self::nix::unistd::write;

#[derive(Clone)]
pub struct CaptureMainLoopInjector {
  pub write_fd: RawFd,
}

impl CaptureMainLoopInjector {
  pub fn activate(&self) {
    write(self.write_fd, &[1; 1]).unwrap();
  }
}
