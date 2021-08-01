# pipelib
A thin Rust wrapper over Unix pipes and polling.

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
