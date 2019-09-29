//! Enum for gui direction input abstraction

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum GuiDirection {
    Left,
    Right,
    Up,
    Down,
    None,
}
