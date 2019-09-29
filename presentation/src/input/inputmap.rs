//! Enum for button/key input abstraction

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum InputMap {
    A,
    B,
    Y,
    X,

    Start,
    Select,

    RightButton,
    LeftButton,

    RightTrigger,
    LeftTrigger,

    DPadUp,
    DPadDown,
    DPadRight,
    DPadLeft,
}
