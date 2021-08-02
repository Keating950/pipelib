use crate::Events;
use libc::{c_int, nfds_t, pollfd};
use smallvec::SmallVec;
use std::{fmt, io, iter, mem, os::unix::prelude::AsRawFd};

/// `Poll` provides an interface for [libc::poll] that allows the monitoring of registered
/// [Reader](crate::Reader) and [Writer](crate::Writer) instances.
#[derive(Debug, Default)]
pub struct Poll {
    fds: SmallVec<[PollFd; 16]>,
    tokens: SmallVec<[usize; 16]>,
}

impl Poll {
    #[inline]
    pub fn new() -> Poll {
        Default::default()
    }

    /// Register a [Reader](crate::Reader) or [Writer](crate::Writer) for polling. `token` is later
    /// yielded by [Poll::events] along with each event to indicate which Reader/Writer the event
    /// applies to. Note that a caller may register multiple different pollable objects with
    /// the same token.
    #[inline]
    pub fn register<T: Pollable>(&mut self, fd: &T, token: usize, events: Events) {
        self.fds.push(PollFd::new(fd.as_raw_fd(), events));
        self.tokens.push(token);
    }

    /// Polls the registered pipes. If the value of `timeout` is `None`, the call will return
    /// immediately.
    pub fn poll(&mut self, timeout: Option<u32>) -> io::Result<usize> {
        let timeout = timeout.unwrap_or(0) as i32;
        unsafe {
            let ptr = mem::transmute::<_, *mut pollfd>(self.fds.as_mut_ptr());
            match libc::poll(ptr, self.fds.len() as nfds_t, timeout) {
                n if n < 0 => Err(oserr!()),
                n => Ok(n as usize),
            }
        }
    }

    /// Iterates over events received in the last call to (poll)[Poll::poll]. Each event
    /// is yielded along with the token that the [Reader](crate::Reader)/[Writer](crate::Writer)
    /// was registered with.
    pub fn events(&mut self) -> impl Iterator<Item = (usize, Events)> + '_ {
        self.fds
            .iter_mut()
            .zip(&self.tokens)
            .flat_map(|(pfd, tok)| pfd.events().map(move |ev| (*tok, ev)))
    }
}

pub trait Pollable: AsRawFd {}

/* -------------------------------------------------------------- */

#[repr(transparent)]
pub(crate) struct PollFd(pollfd);

impl fmt::Debug for PollFd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            r#"PollFd {{ fd: {}, events: {}, revents: {} }}"#,
            self.0.fd, self.0.events, self.0.revents
        )
    }
}

impl PollFd {
    pub fn new(fd: c_int, events: Events) -> PollFd {
        PollFd(pollfd {
            fd,
            events: events.into(),
            revents: 0,
        })
    }

    pub fn events(&mut self) -> impl Iterator<Item = Events> {
        let revents = self.0.revents;
        self.0.revents = 0;
        let mut shift = 0;
        iter::from_fn(move || {
            loop {
                if shift < 16 {
                    let event = revents & (1 << shift);
                    shift += 1;
                    if event != 0 {
                        return Some(Events::from_bits(event).unwrap());
                    } else {
                        continue;
                    }
                } else {
                    return None;
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;

    #[test]
    fn test_pollfd_events() {
        let events = Events::POLLIN | Events::POLLERR | Events::POLLHUP;
        let mut pfd = PollFd::new(100, events);
        pfd.0.revents = events.into();
        let ev_vec: Vec<Events> = pfd.events().collect();
        assert_eq!(ev_vec.len(), 3);
        assert_eq!(
            ev_vec,
            vec![Events::POLLIN, Events::POLLERR, Events::POLLHUP]
        );
    }

    #[test]
    fn test_poll_events() {
        let mut poll = Poll::new();
        let (reader, mut writer) = crate::new().unwrap();
        poll.register(&reader, 0, Events::all_readable() | Events::all_error());
        poll.register(&writer, 1, Events::all_writable() | Events::all_error());
        assert_ok!(poll.poll(None));
        let (_, ev) = poll.events().nth(0).unwrap();
        assert!(ev.is_writable());
        writer.write(b"Hello").unwrap();
        assert_ok!(poll.poll(None));
        let (_, ev) = poll.events().nth(0).unwrap();
        assert!(ev.is_readable());
    }
}
