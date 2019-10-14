extern crate libc;
extern crate nix;

use std::os::unix::io::RawFd;

use self::nix::Error;
use self::nix::errno::{Errno, errno};
use self::nix::fcntl;
use self::nix::sys::select::{FdSet, select};
use self::nix::sys::time::{TimeVal, TimeValLike};
use self::nix::unistd::{close, pipe, read};

use crate::capture::capture_param::CaptureParam;
use crate::capture::capture_state::CaptureState;

pub struct CaptureMainLoop {
    param: CaptureParam,
    state: CaptureState,
    pipe_fds: Option<(RawFd, RawFd)>,
}

impl CaptureMainLoop {
    pub fn new(param: CaptureParam) -> Self {
        Self {
            param: param,
            state: CaptureState::new(),
            pipe_fds: None,
        }
    }

    pub fn run(&mut self) {
        self.init_pipe().unwrap();

        let pipe_read_fd = self.pipe_read_fd();
        let pipe_write_fd = self.pipe_write_fd();

        loop {
            let mut read_fds = FdSet::new();
            read_fds.insert(pipe_read_fd);

            // Monitors the `close` flag (0 byte).
            read_fds.insert(libc::STDOUT_FILENO);

            let mut timeout = TimeVal::seconds(30);
            let select_err = select(None, &mut read_fds, None, None, &mut timeout).unwrap();

            if select_err < 0 {
                let errno = Errno::last();
                if errno == Errno::EINTR {
                    continue;
                }

                println!("Returns an unexpected error from `select` - {}", errno.desc());
                break;
            }

            if select_err == 0 {
                continue;
            }

            if read_fds.contains(libc::STDOUT_FILENO) {
                // Receives the `close` flag.
                break;
            }

            if !read_fds.contains(pipe_read_fd) {
                continue;
            }

            self.handle_record_complete();

            // Flushes the pipe.
            let mut buf = [0u8; 1];
            read(pipe_read_fd, &mut buf).unwrap();
        }
    }

    fn destroy_all(&mut self) {
        self.destroy_pipe();
    }

    fn destroy_pipe(&mut self) {
        if let Some((pipe_read_fd, pipe_write_fd)) = self.pipe_fds {
            close(pipe_read_fd).unwrap();
            close(pipe_write_fd).unwrap();

            self.pipe_fds = None;
        }
    }

    fn handle_record_complete(&self) {

        // TODO

    }

    fn init_pipe(&mut self) -> Result<(), Error> {
        self.destroy_pipe();

        let (pipe_read_fd, pipe_write_fd) = pipe()?;
        self.pipe_fds = Some((pipe_read_fd, pipe_write_fd));

        let flags = fcntl::fcntl(pipe_write_fd, fcntl::F_GETFL)?;
        let new_flags = fcntl::OFlag::from_bits_truncate(flags) | fcntl::OFlag::O_NONBLOCK;
        fcntl::fcntl(pipe_write_fd, fcntl::F_SETFL(new_flags))?;

        Ok(())
    }

    fn pipe_read_fd(&self) -> RawFd {
        if let Some((pipe_read_fd, _pipe_write_fd)) = self.pipe_fds {
            return pipe_read_fd;
        }

        panic!("`pipe_fds` has not been initialized")
    }

    fn pipe_write_fd(&self) -> RawFd {
        if let Some((_pipe_read_fd, pipe_write_fd)) = self.pipe_fds {
            return pipe_write_fd;
        }

        panic!("`pipe_fds` has not been initialized")
    }
}

impl Drop for CaptureMainLoop {
    fn drop(&mut self) {
        self.destroy_all();
    }
}
