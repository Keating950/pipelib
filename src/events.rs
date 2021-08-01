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
    /// Returns a bitmask of all events indicating that a pipe is writable.
    pub const fn writable_mask() -> i16 {
        Events::POLLOUT.bits | Events::POLLWRNORM.bits | Events::POLLWRBAND.bits
    }

    /// Whether a particular event indicates that a pipe is writable.
    pub const fn is_writable(self) -> bool {
        self.bits & Events::writable_mask() != 0
    }

    /// Returns a bitmask of all events indicating that a pipe is readable.
    pub const fn readable_mask() -> i16 {
        Events::POLLIN.bits
            | Events::POLLRDNORM.bits
            | Events::POLLRDBAND.bits
            | Events::POLLPRI.bits
    }

    /// Whether a particular event indicates that a pipe is readable.
    pub const fn is_readable(self) -> bool {
        self.bits & Events::readable_mask() != 0
    }

    /// Returns a bitmask of all events indicating an error state.
    pub const fn error_mask() -> i16 {
        Events::POLLERR.bits | Events::POLLNVAL.bits
    }

    /// Whether a particular event indicates an error.
    pub const fn is_error(self) -> bool {
        self.bits & Events::error_mask() != 0
    }

    /// Whether an event is `Events::POLLHUP`.
    pub const fn is_hangup(self) -> bool {
        self.bits & Events::POLLHUP.bits != 0
    }
}
