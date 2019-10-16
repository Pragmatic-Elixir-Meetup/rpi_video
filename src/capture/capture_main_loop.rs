extern crate eetf;
extern crate libc;
extern crate nix;

use std::io::{Cursor, Read, Stdin, stdin};
use std::os::unix::io::{AsRawFd, RawFd};
use std::str;

use self::eetf::Term;

use self::nix::errno::Errno;
use self::nix::sys::select::{FdSet, select};
use self::nix::sys::time::{TimeVal, TimeValLike};
use self::nix::unistd::{close, pipe, read};
use self::nix::{Error, fcntl};

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
        // Initializes a `stdin` handle. It is used to receive commands from Elixir.
        let mut stdin_handle = stdin();
        let stdin_fd = stdin_handle.as_raw_fd();

        // Initializes a `pipe`. It is used to activate main loop from `capture_tasks`.
        self.init_pipe().unwrap();
        let pipe_read_fd = self.pipe_read_fd();
        let pipe_write_fd = self.pipe_write_fd();

        loop {
            let mut read_fds = FdSet::new();
            read_fds.insert(stdin_fd);
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

                eprintln!("\nReturns an unexpected error from `select` - {}\n", errno.desc());
                break;
            }

            if select_err == 0 {
                continue;
            }

            if read_fds.contains(libc::STDOUT_FILENO) {
                // Receives the `close` flag.
                break;
            }

            if read_fds.contains(stdin_fd) {
              self.handle_port_commands(&mut stdin_handle);
            }

            if read_fds.contains(pipe_read_fd) {
              self.handle_record_complete();

              // Flushes the pipe.
              let mut buf = [0u8; 1];
              read(pipe_read_fd, &mut buf).unwrap();
            }

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

    fn dispatch_command(&self, command: &str) {
        match command.as_ref() {
            "start_record" => self.run_command_start_record(),
            _ => eprintln!("\nUnsupported command `{}`\n", command),
        }
    }

    fn handle_port_commands(&self, stdin: &mut Stdin) {
        let mut handle = stdin.lock();

        let mut len_buf = [0; 8];
        handle.read_exact(&mut len_buf).unwrap();


        let len = u64::from_be_bytes(len_buf);
        let mut term_buf = Vec::new();
        term_buf.resize(len as usize, 0);
        handle.read_exact(&mut term_buf).unwrap();

        let term = Term::decode(Cursor::new(&term_buf)).unwrap();
        if let Term::Binary(binary) = term {
            let command = str::from_utf8(&binary.bytes).unwrap();
            eprintln!("\nReceives command `{}`\n", command);

            self.dispatch_command(&command);
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

    fn run_command_start_record(&self) {

      // TODO

    }
}

impl Drop for CaptureMainLoop {
    fn drop(&mut self) {
        self.destroy_all();
    }
}
