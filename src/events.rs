use bitflags;
use libc::{
    POLLERR, POLLHUP, POLLIN, POLLNVAL, POLLOUT, POLLPRI, POLLRDBAND, POLLRDNORM, POLLWRBAND,
    POLLWRNORM,
};

bitflags::bitflags! {
    /// `Events` is a [bitflags] struct provides a more type-safe interface for [libc]'s poll flags
    /// \([POLLIN](libc::POLLIN), [POLLOUT](libc::POLLOUT), etc\).
    pub struct Events: i16 {
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

impl From<Events> for i16 {
    fn from(e: Events) -> Self {
        e.bits
    }
}

impl Events {
    /// Returns a bitmask of all events indicating that a pipe is readable.
    #[inline]
    pub fn readable_mask() -> Events {
        Events::POLLIN | Events::POLLRDNORM | Events::POLLRDBAND | Events::POLLPRI
    }

    /// Returns a bitmask of all events indicating that a pipe is writable.
    #[inline]
    pub fn writable_mask() -> Events {
        Events::POLLOUT | Events::POLLWRNORM | Events::POLLWRBAND
    }

    /// Returns a bitmask of all events indicating an error state.
    #[inline]
    pub fn error_mask() -> Events {
        Events::POLLERR | Events::POLLNVAL
    }

    /// Whether a particular event indicates that a pipe is readable.
    #[inline]
    pub fn is_readable(self) -> bool {
        self.intersects(Events::readable_mask())
    }

    /// Whether a particular event indicates that a pipe is writable.
    #[inline]
    pub fn is_writable(self) -> bool {
        self.intersects(Events::writable_mask())
    }

    /// Whether a particular event indicates an error.
    #[inline]
    pub fn is_error(self) -> bool {
        self.intersects(Events::error_mask())
    }

    /// Whether an event is `Events::POLLHUP`.
    #[inline]
    pub fn is_hangup(self) -> bool {
        self.intersects(Events::POLLHUP)
    }
}
