use libc::c_int;

use crate::{pipe::Pipe, Pollable};
use std::{
    io::{self, prelude::*},
    os::unix::{
        io::{AsRawFd, FromRawFd, RawFd},
        prelude::IntoRawFd,
    },
};

/// The read end of a Unix pipe. Like [`Writer`](crate::Writer), Reader is non-blocking, and the
/// [`CLOEXEC`](libc::FD_CLOEXEC) flag is set.
#[derive(Debug)]
pub struct Reader(Pipe);

impl Pollable for Reader {}

impl FromRawFd for Reader {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Reader(Pipe(fd))
    }
}

impl AsRawFd for Reader {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0.0
    }
}

impl IntoRawFd for Reader {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.as_raw_fd()
    }
}

impl Reader {
    pub(crate) fn new(n: c_int) -> Reader {
        Reader(Pipe(n))
    }
}

impl Read for Reader {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}
