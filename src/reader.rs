use libc::c_int;

use crate::pipe::Pipe;
use std::{
    io::{self, prelude::*},
    os::unix::io::{FromRawFd, IntoRawFd, RawFd},
};

#[derive(Debug)]
pub struct Reader(Pipe);

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

    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.0.read_to_end(buf)
    }
}

impl FromRawFd for Reader {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Reader(Pipe(fd))
    }
}

impl IntoRawFd for Reader {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.0.0
    }
}
