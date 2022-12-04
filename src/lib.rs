use std::io::Write;

/// Style data that is attributed to a Cell.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Style {}

/// Represents one cell on the terminal.
///
/// Each cell contains exactly one character which may be styled.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Cell {
    contents: char,
    style: Style,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            contents: ' ',
            style: Default::default(),
        }
    }
}

/// The position of a cell on the terminal.
///
/// The top-left cell is at row 0 column 0.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Position {
    row: usize,
    col: usize,
}

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
    fn copy_from(&mut self, other: &Self) {
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
    fn flush(&mut self) -> std::io::Result<()> {
        // TODO: do all the work here.
        self.queue.clear();
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.queue.write(buf)
    }
}

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
    fn flush(&mut self) -> std::io::Result<()> {
        self.outer.flush()
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.outer.write(buf)
    }
}
