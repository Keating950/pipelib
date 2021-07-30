use libc::c_int;

use crate::pipe::Pipe;
use std::{
    io::{self, prelude::*},
    os::unix::io::{FromRawFd, IntoRawFd, RawFd},
};

#[derive(Debug)]
pub struct Writer(Pipe);

impl Writer {
    pub(crate) fn new(n: c_int) -> Writer {
        Writer(Pipe(n))
    }
}

impl Write for Writer {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}

impl FromRawFd for Writer {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Writer(Pipe(fd))
    }
}

impl IntoRawFd for Writer {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.0.0
    }
}
