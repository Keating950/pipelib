use bitflags;
use libc::{
    POLLERR, POLLHUP, POLLIN, POLLNVAL, POLLOUT, POLLPRI, POLLRDBAND, POLLRDNORM, POLLWRBAND,
    POLLWRNORM,
};
use std::mem;

bitflags::bitflags! {
    /// `Events` is a [bitflags] struct provides a more type-safe interface for [libc]'s poll flags
    /// \([POLLIN](libc::POLLIN), [POLLOUT](libc::POLLOUT), etc.\).
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
    #[inline]
    fn from(e: Events) -> Self {
        e.bits
    }
}

impl Events {
    /// Returns a bitmask of all events indicating that a pipe is readable.
    // For whatever, reason the BitOr operation for a bitflags struct is not considered const,
    // despite the operation on its bits being so. This is why transmute is used here and in the
    // other masking functions.
    #[inline]
    pub const fn all_readable() -> Events {
        unsafe {
            mem::transmute(
                Events::POLLIN.bits
                    | Events::POLLRDNORM.bits
                    | Events::POLLRDBAND.bits
                    | Events::POLLPRI.bits,
            )
        }
    }

    /// Returns a bitmask of all events indicating that a pipe is writable.
    #[inline]
    pub const fn all_writable() -> Events {
        unsafe {
            mem::transmute(Events::POLLOUT.bits | Events::POLLWRNORM.bits | Events::POLLWRBAND.bits)
        }
    }

    /// Returns a bitmask of all events indicating an error state.
    #[inline]
    pub const fn all_error() -> Events {
        unsafe { mem::transmute(Events::POLLERR.bits | Events::POLLNVAL.bits) }
    }

    /// Whether a particular event indicates that a pipe is readable.
    #[inline]
    pub const fn is_readable(self) -> bool {
        self.intersects(Events::all_readable())
    }

    /// Whether a particular event indicates that a pipe is writable.
    #[inline]
    pub const fn is_writable(self) -> bool {
        self.intersects(Events::all_writable())
    }

    /// Whether a particular event indicates an error.
    #[inline]
    pub const fn is_error(self) -> bool {
        self.intersects(Events::all_error())
    }

    /// Whether an event is `Events::POLLHUP`.
    #[inline]
    pub const fn is_hangup(self) -> bool {
        self.intersects(Events::POLLHUP)
    }
}
