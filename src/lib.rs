#![doc = include_str!("../README.md")]
#[macro_use]
mod macros;
mod events;
mod pipe;
mod poll;
mod pollable;
mod reader;
mod writer;

pub use crate::{events::Events, poll::{Poll, Token}, pollable::Pollable, reader::Reader, writer::Writer};
use libc::c_int;

/// Creates a [Reader]/[Writer] pair for a non-blocking Unix pipe. The [FD_CLOEXEC](libc::FD_CLOEXEC)
/// and [O_NONBLOCK](libc::O_NONBLOCK) flags are set for both.
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
