#![allow(unused)]
mod pipe;
mod reader;
mod writer;

use crate::{pipe::Pipe, reader::Reader, writer::Writer};
use libc::c_int;

pub(crate) mod crate_macros {
    #[macro_export]
    macro_rules! oserr {
        () => {
            std::io::Error::last_os_error()
        };
    }

    #[macro_export]
    macro_rules! assert_ok {
        ($val:ident) => {
            assert!($val.is_ok(), "{:?}", $val.unwrap_err())
        };
        ($e:expr) => {
            let tmp = $e;
            assert_ok!(tmp)
        };
    }
}

pub fn new() -> std::io::Result<(Reader, Writer)> {
    let mut fds: [c_int; 2] = [-1, -1];
    unsafe {
        if libc::pipe(fds.as_mut_ptr()) != 0 {
            return Err(oserr!());
        }
        for fd in fds {
            if libc::fcntl(fd, libc::FD_CLOEXEC) != 0
                || libc::fcntl(fd, libc::F_SETFL, libc::O_NONBLOCK) != 0
            {
                return Err(oserr!());
            }
        }
    }
    debug_assert_ne!(fds[0], -1);
    debug_assert_ne!(fds[1], -1);
    Ok((Reader::new(fds[0]), Writer::new(fds[1])))
}
