use crate::{Event, Pollable};
use libc::{c_int, nfds_t, pollfd};
use smallvec::SmallVec;
use std::{fmt, io, iter, mem};

/// `Poll` provides an interface for [`libc::poll`] that allows the monitoring of registered
/// [`Reader`](crate::Reader) and [`Writer`](crate::Writer) instances.
#[derive(Debug, Default)]
pub struct Poll {
    fds: SmallVec<[PollFd; Poll::POLL_STACK_CAPACITY]>,
    tokens: SmallVec<[Token; Poll::POLL_STACK_CAPACITY]>,
}

impl Poll {
    // Should be enough for the vast majority of use cases
    const POLL_STACK_CAPACITY: usize = 8;

    #[inline]
    #[must_use]
    pub fn new() -> Poll {
        Default::default()
    }

    /// Register a [Pollable] object for polling. `token` is later yielded by [`Poll::events`] along
    /// with each event to indicate which object the event applies to. Note that a caller may
    /// register multiple different pollable objects with the same token.
    pub fn register<T: Pollable>(&mut self, fd: &T, token: Token, events: Event) {
        self.fds.push(PollFd::new(fd.as_raw_fd(), events));
        self.tokens.push(token);
    }

    /// Polls the registered pipes. If the value of `timeout` is `None`, the call will return
    /// immediately.
    #[allow(clippy::cast_possible_wrap)] // Temporary measure
    pub fn poll(&mut self, timeout: Option<u32>) -> io::Result<usize> {
        let timeout = timeout.unwrap_or(0) as i32;
        unsafe {
            let ptr = self.fds.as_mut_ptr().cast::<pollfd>();
            match libc::poll(ptr, self.fds.len() as nfds_t, timeout) {
                n if n < 0 => Err(oserr!()),
                n => Ok(n as usize),
            }
        }
    }

    /// Iterates over events received in the last call to (poll)[`Poll::poll`]. Each event
    /// is yielded along with the token that the [Pollable] was registered with.
    #[inline]
    pub fn events(&mut self) -> impl Iterator<Item = (Token, Event)> + '_ {
        self.fds
            .iter_mut()
            .zip(&self.tokens)
            .flat_map(|(pfd, tok)| pfd.events().map(move |ev| (*tok, ev)))
    }
}

/* -------------------------------------------------------------- */

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token(pub usize);

impl From<Token> for usize {
    #[inline]
    fn from(tok: Token) -> Self {
        tok.0
    }
}

#[repr(transparent)]
pub(crate) struct PollFd(pollfd);

impl fmt::Debug for PollFd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PollFd")
            .field("fd", &self.0.fd)
            .field("events", &self.0.events)
            .field("revents", &self.0.revents)
            .finish()
    }
}

impl PollFd {
    pub fn new(fd: c_int, events: Event) -> PollFd {
        PollFd(pollfd {
            fd,
            events: events.into(),
            revents: 0,
        })
    }

    pub fn events(&mut self) -> impl Iterator<Item = Event> {
        let revents = self.0.revents;
        self.0.revents = 0;
        let mut shift = 0;
        iter::from_fn(move || {
            loop {
                if shift < mem::size_of::<Event>() * 8 {
                    let event = revents & (1 << shift);
                    shift += 1;
                    if event != 0 {
                        return Some(Event::from_bits(event).unwrap());
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
        let events = Event::POLLIN | Event::POLLERR | Event::POLLHUP;
        let mut pfd = PollFd::new(100, events);
        pfd.0.revents = events.into();
        let ev_vec: Vec<Event> = pfd.events().collect();
        assert_eq!(ev_vec.len(), 3);
        assert_eq!(ev_vec, vec![Event::POLLIN, Event::POLLERR, Event::POLLHUP]);
    }

    #[test]
    fn test_poll_events() {
        let mut poll = Poll::new();
        let (reader, mut writer) = crate::new().unwrap();
        poll.register(
            &reader,
            Token(0),
            Event::all_readable() | Event::all_error(),
        );
        poll.register(
            &writer,
            Token(1),
            Event::all_writable() | Event::all_error(),
        );
        assert_ok!(poll.poll(None));
        let (_, ev) = poll.events().nth(0).unwrap();
        assert!(ev.is_writable());
        writer.write(b"Hello").unwrap();
        assert_ok!(poll.poll(None));
        let (_, ev) = poll.events().nth(0).unwrap();
        assert!(ev.is_readable());
    }
}
