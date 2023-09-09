use crate::{pipe::Pipe, Pollable};
use libc::c_int;
use std::{
    io::{self, prelude::*},
    os::unix::{
        io::{AsRawFd, FromRawFd, RawFd},
        prelude::IntoRawFd,
    },
};

/// The write end of a Unix pipe. Like [`Reader`](crate::Reader), Writer is non-blocking, and the
/// [`CLOEXEC`](libc::FD_CLOEXEC) flag is set.
#[derive(Debug)]
pub struct Writer(Pipe);

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

impl AsRawFd for Writer {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl IntoRawFd for Writer {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.0.into_raw_fd()
    }
}

impl Pollable for Writer {}

impl Writer {
    pub(crate) fn new(n: c_int) -> Writer {
        Writer(Pipe(n))
    }
}
