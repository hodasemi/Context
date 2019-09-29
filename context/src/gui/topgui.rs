/// A trait that is used by the gui handler as the target for input
use utilities::prelude::*;

pub trait TopGui {
    /// Decline method which is executed on `InputMap::B` press
    fn decline(&self) -> VerboseResult<()>;

    /// Method which is executed on `InputMap::RightButton` press
    fn next_tab(&self) -> VerboseResult<()>;

    /// Method which is executed on `InputMap::LeftButton` press
    fn previous_tab(&self) -> VerboseResult<()>;
}
