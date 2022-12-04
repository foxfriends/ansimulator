use crate::{Cell, Position};
use std::io::{self, Write};

/// A single-buffered simulated terminal.
///
/// Writing to this will modify it as if it were a terminal, and not as a normal byte buffer.
///
/// # Notes
///
/// Not all ANSI codes are supported. Only codes that modify the contents of the buffer
/// can be simulated. Actions such as "switch to alternate buffer" or "scroll down" will have
/// no effect and will be lost.
#[derive(Clone, Default, Debug)]
pub struct Ansimulation {
    buffer: Vec<Cell>,
    saved_cursor: Position,
    current_cursor: Position,

    // Queue of unprocessed input. Will be processed on call to `flush`.
    queue: Vec<u8>,
}

impl Ansimulation {
    pub(crate) fn copy_from(&mut self, other: &Self) {
        self.buffer
            .resize_with(other.buffer.len(), Default::default);
        self.buffer.copy_from_slice(&other.buffer);
        self.saved_cursor = other.saved_cursor;
        self.current_cursor = other.current_cursor;
        // Assume that the other queue was flushed before copying.
        self.queue.clear();
    }
}

impl Write for Ansimulation {
    fn flush(&mut self) -> io::Result<()> {
        // TODO: do all the work here.
        self.queue.clear();
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.queue.write(buf)
    }
}
