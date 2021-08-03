use libc::{self, c_int, c_void, size_t};
use std::{
    io::{self, prelude::*},
    os::unix::prelude::{AsRawFd, FromRawFd, IntoRawFd, RawFd},
};

#[derive(Debug)]
pub(crate) struct Pipe(pub(crate) c_int);

impl Pipe {
    fn write_from_ptr(&mut self, buf: *const c_void, len: usize) -> io::Result<usize> {
        let written = unsafe { libc::write(self.0, buf, len) };
        if written < 0 {
            Err(oserr!())
        } else {
            Ok(written as usize)
        }
    }

    fn read_to_ptr(&self, buf: *mut c_void, len: usize) -> io::Result<usize> {
        let bytes_read = unsafe { libc::read(self.0, buf, len as size_t) };
        if bytes_read < 0 {
            let e = oserr!();
            match e.kind() {
                io::ErrorKind::WouldBlock => Ok(0),
                _ => Err(e),
            }
        } else {
            Ok(bytes_read as usize)
        }
    }
}

impl Write for Pipe {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let ptr = buf.as_ptr() as *const c_void;
        self.write_from_ptr(ptr, buf.len())
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        let mut to_write = buf.len();
        let ptr = buf.as_ptr() as *const c_void;
        while to_write > 0 {
            let written = self.write_from_ptr(ptr, to_write)?;
            to_write -= written;
            unsafe { ptr.add(written) };
        }
        Ok(())
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Read for Pipe {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let ptr = buf.as_mut_ptr() as *mut c_void;
        self.read_to_ptr(ptr, buf.len())
    }
}

impl FromRawFd for Pipe {
    #[inline]
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Pipe(fd)
    }
}

impl AsRawFd for Pipe {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0
    }
}

impl IntoRawFd for Pipe {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        self.as_raw_fd()
    }
}

impl Drop for Pipe {
    fn drop(&mut self) {
        unsafe { libc::close(self.0) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_ok!(crate::new());
    }

    #[test]
    fn test_read_write() {
        let test_msg = *b"Hello, world";
        let mut buf: [u8; 12] = [0; 12];
        let (mut reader, mut writer) = crate::new().unwrap();
        assert_ok!(writer.write(&test_msg));
        assert_ok!(reader.read(&mut buf));
        assert_eq!(test_msg, buf);
    }

    #[test]
    fn test_read_to_end() {
        let test_msg = b"Hello, world".to_vec();
        let mut buf: Vec<u8> = Vec::with_capacity(6);
        let (mut reader, mut writer) = crate::new().unwrap();
        assert_ok!(writer.write(&test_msg));
        let res = reader.read_to_end(&mut buf);
        assert_ok!(res);
        let bytes_read = res.unwrap();
        assert_eq!(bytes_read, test_msg.len());
        assert_eq!(buf, test_msg);
    }
}
