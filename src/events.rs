use bitflags::bitflags;
use libc::{
    POLLERR, POLLHUP, POLLIN, POLLNVAL, POLLOUT, POLLPRI, POLLRDBAND, POLLRDNORM, POLLWRBAND,
    POLLWRNORM,
};
use std::{convert::TryFrom, io};

bitflags! {
    /// `Events` provides a more type-safe interface for [libc]'s poll flags \([POLLIN](libc::POLLIN), [POLLOUT](libc::POLLOUT), etc\).
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

impl TryFrom<i16> for Events {
    type Error = io::Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Events::ALL_EVENTS
            .binary_search_by(|ev| i16::from(*ev).cmp(&value))
            .map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Received unknown or unsupported revent ({}) from poll",
                        value
                    ),
                )
            })
            .map(|idx| Events::ALL_EVENTS[idx])
    }
}

impl Events {
    const ALL_EVENTS: [Events; 10] = [
        Events::POLLIN,
        Events::POLLPRI,
        Events::POLLOUT,
        Events::POLLERR,
        Events::POLLHUP,
        Events::POLLNVAL,
        Events::POLLRDNORM,
        Events::POLLRDBAND,
        Events::POLLWRNORM,
        Events::POLLWRBAND,
    ];

    pub fn iter() -> impl Iterator<Item = Events> {
        Events::ALL_EVENTS.iter().copied()
    }

    pub fn is_writable(self) -> bool {
        const WRITABLE_MASK: i16 =
            Events::POLLOUT.bits | Events::POLLWRNORM.bits | Events::POLLWRBAND.bits;
        self.bits & WRITABLE_MASK != 0
    }

    pub fn is_readable(self) -> bool {
        const READABLE_MASK: i16 = Events::POLLIN.bits
            | Events::POLLRDNORM.bits
            | Events::POLLRDBAND.bits
            | Events::POLLPRI.bits;
        self.bits & READABLE_MASK != 0
    }

    pub fn is_error(self) -> bool {
        const ERROR_MASK: i16 = Events::POLLERR.bits | Events::POLLNVAL.bits;
        self.bits & ERROR_MASK != 0
    }
}
