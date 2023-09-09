use bitflags::bitflags;
use libc::{
    POLLERR, POLLHUP, POLLIN, POLLNVAL, POLLOUT, POLLPRI, POLLRDBAND, POLLRDNORM, POLLWRBAND,
    POLLWRNORM,
};

bitflags! {
    /// `Events` is a [bitflags] struct provides a more type-safe interface for [libc]'s poll flags
    /// \([POLLIN](libc::POLLIN), [POLLOUT](libc::POLLOUT), etc.\).
    pub struct Event: i16 {
        const POLLIN = POLLIN;
        const POLLPRI = POLLPRI;
        const POLLOUT = POLLOUT;
        const POLLERR = POLLERR;
        const POLLHUP = POLLHUP;
        const POLLNVAL = POLLNVAL;
        const POLLRDNORM = POLLRDNORM;
        const POLLRDBAND = POLLRDBAND;
        const POLLWRNORM = POLLWRNORM;
        const POLLWRBAND = POLLWRBAND;
    }
}

impl From<Event> for i16 {
    #[inline]
    fn from(e: Event) -> Self {
        e.bits
    }
}

impl Event {
    #[inline]
    pub const fn all_readable() -> Event {
        Event::POLLIN
            .union(Event::POLLRDNORM)
            .union(Event::POLLRDBAND)
            .union(Event::POLLPRI)
    }

    /// Returns a bitmask of all events indicating that a pipe is writable.
    #[inline]
    pub const fn all_writable() -> Event {
        Event::POLLOUT
            .union(Event::POLLWRNORM)
            .union(Event::POLLWRBAND)
    }

    /// Returns a bitmask of all events indicating an error state.
    #[inline]
    pub const fn all_error() -> Event {
        Event::POLLERR.union(Event::POLLNVAL)
    }

    /// Whether a particular event indicates that a pipe is readable.
    #[inline]
    pub const fn is_readable(self) -> bool {
        self.intersects(Event::all_readable())
    }

    /// Whether a particular event indicates that a pipe is writable.
    #[inline]
    pub const fn is_writable(self) -> bool {
        self.intersects(Event::all_writable())
    }

    /// Whether a particular event indicates an error.
    #[inline]
    pub const fn is_error(self) -> bool {
        self.intersects(Event::all_error())
    }

    /// Whether an event includes `Events::POLLHUP`.
    #[inline]
    pub const fn is_hangup(self) -> bool {
        self.intersects(Event::POLLHUP)
    }
}
