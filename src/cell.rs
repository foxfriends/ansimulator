use crate::Style;

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
