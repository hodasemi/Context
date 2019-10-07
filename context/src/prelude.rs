// structures
pub use crate::core::{
    context::{Context, Event, GameObject},
    osspecific::osspecific::OsSpecificConfig,
};

#[cfg(feature = "user_interface")]
pub use crate::core::guihandler::{Color, GuiHandler, GuiHandlerCreateInfo};

#[cfg(feature = "audio")]
pub use crate::core::soundhandler::{Music, Sound, SoundHandler, VolumeInfo};

// reexport ears presets (OpenAL)
#[cfg(feature = "audio")]
pub use ears::{ReverbEffect, ReverbPreset};

// gui elements
#[cfg(feature = "user_interface")]
pub use crate::gui::{
    clickable::Clickable,
    colorable::Colorable,
    displayable::Displayable,
    executable::Executable,
    frameable::*,
    hoverable::Hoverable,
    iconizable::Iconizable,
    selectable::Selectable,
    textable::{TextAlignment, Textable},
    topgui::TopGui,
    writeable::Writeable,
};

// crate rexports
pub use utilities::prelude::*;
pub use vulkan_rs::prelude::*;

// render target
pub use crate::utils::rendertarget::{ClearValue, CustomTarget, RenderTarget};

pub use presentation::prelude::*;
