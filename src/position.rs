/// The position of a cell on the terminal.
///
/// The top-left cell is at row 0 column 0.
#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Position {
    row: usize,
    col: usize,
}
