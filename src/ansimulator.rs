use crate::Ansimulation;
use std::io::{self, Write};

/// Double buffered ANSI code simulator, allowing for writes to be optimized before being relayed
/// to an actual output device.
///
/// Writing to this will modify it as if it were a terminal, and not as a normal byte buffer.
///
/// # Notes
///
/// Not all ANSI codes are supported. Only codes that modify the contents of the buffer
/// can be simulated. Actions such as "switch to alternate buffer" or "scroll down" will have
/// no effect and will be lost.
#[derive(Clone, Default, Debug)]
pub struct Ansimulator {
    /// The internal buffer, used for diffs and reading.
    inner: Ansimulation,
    /// The outer buffer, used as a scratchpad for writing.
    outer: Ansimulation,
}

impl Ansimulator {
    /// Flips the simulation buffers, returning the optimized ANSI codes required to convert
    /// from one to the other.
    pub fn flip(&mut self) -> Vec<u8> {
        self.inner.copy_from(&self.outer);
        vec![]
    }
}

impl Write for Ansimulator {
    fn flush(&mut self) -> io::Result<()> {
        self.outer.flush()
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.outer.write(buf)
    }
}
