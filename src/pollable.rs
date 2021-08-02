use libc::c_int;
use std::{
    io,
    os::unix::prelude::{AsRawFd, FromRawFd},
};

/// An interface for pollable types, i.e., those that wrap an underlying file descriptor.
pub trait Pollable: Sized + AsRawFd + FromRawFd {
    /// Duplicates the underlying file descriptor and sets the [`FD_CLOEXEC`](libc::FD_CLOEXEC)
    /// flag on it. This can be useful for combining a child process's [`stdout` and `stderr`
    /// streams](std::process::Stdio). For further information, see your platform's [man
    /// pages](https://www.freebsd.org/cgi/man.cgi?query=dup2&sektion=2&manpath=FreeBSD+9.0-RELEASE).
    fn dup(&self) -> io::Result<Self> {
        unsafe {
            let new_fd = libc::dup(self.as_raw_fd());
            if new_fd == -1 {
                return Err(oserr!());
            }
            if libc::fcntl(new_fd, libc::FD_CLOEXEC) != 0 {
                libc::close(new_fd);
                return Err(oserr!());
            }
            Ok(Self::from_raw_fd(new_fd))
        }
    }

    /// Equivalent to [`dup`](Pollable::dup), but duplicates the underlying file descriptor into
    /// `dst` instead of the lowest-numbered available file descriptor.
    fn dup2(&self, dst: c_int) -> io::Result<Self> {
        unsafe {
            let new_fd = libc::dup2(self.as_raw_fd(), dst);
            if new_fd == -1 {
                return Err(oserr!());
            }
            if libc::fcntl(new_fd, libc::FD_CLOEXEC) != 0 {
                libc::close(new_fd);
                return Err(oserr!());
            }
            Ok(Self::from_raw_fd(new_fd))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dup() {
        let (rd, _) = crate::new().unwrap();
        let duped = rd.dup();
        assert_ok!(duped);
    }

    #[test]
    fn test_dup2() {
        const TEST_FD: c_int = 10;
        let (rd, _) = crate::new().unwrap();
        let duped = rd.dup2(TEST_FD);
        assert_ok!(duped);
        assert_eq!(duped.unwrap().as_raw_fd(), TEST_FD);
    }
}
