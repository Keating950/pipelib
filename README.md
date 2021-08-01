# pipelib
Pipelib is a thin Rust wrapper over non-blocking Unix pipes and polling. It is intended to provide
an interface more similar to libc's than alternatives like [mio](https://crates.io/crates/mio)
without sacrificing ergonomics or useful features. It's also very small, having only two
dependencies other than libc.

## Example
```rust
use pipelib::Events; 
use pipelib::Poll;
use std::{
    io::{self, prelude::*},
    str,
};

fn main() -> io::Result<()> {
    const READER_TOKEN: usize = 0;
    const WRITER_TOKEN: usize = 1;
    let (mut reader, mut writer) = pipelib::new()?;
    let mut poll = Poll::new();
    poll.register(&mut reader, READER_TOKEN, Events::POLLIN | Events::POLLERR);
    poll.register(&mut writer, WRITER_TOKEN, Events::POLLOUT | Events::POLLERR);
    let mut buf = Vec::new();
    'outer: loop {
        poll.poll(None)?;
        for (tok, ev) in poll.events() {
            if tok == WRITER_TOKEN && ev.is_writable() {
                writer.write(b"Hello, world")?;
            } else if tok == READER_TOKEN && ev.is_readable() {
                reader.read_to_end(&mut buf)?;
                match str::from_utf8(&buf) {
                    Ok(s) => println!("{}", s),
                    Err(_) => panic!("Invalid unicode"),
                }
                break 'outer;
            }
        }
    }
    Ok(())
}
```

## Compatibility
Pipelib should work on any Unix-like OS, and is actively developed and tested on both Linux and
MacOS. Windows is not supported.

