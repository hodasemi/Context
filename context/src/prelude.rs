// structures
pub use crate::core::{
    configs::WindowConfig, context::Context, osspecific::osspecific::OsSpecificConfig,
};

#[cfg(feature = "audio")]
pub use crate::core::soundhandler::{Music, Sound, SoundHandler, VolumeInfo};

// reexport ears presets (OpenAL)
#[cfg(feature = "audio")]
pub use ears::{ReverbEffect, ReverbPreset};

// crate rexports
pub use utilities::prelude::*;
pub use vulkan_rs::prelude::*;

// render target
pub use crate::utils::rendertarget::{ClearValue, CustomTarget, RenderTarget};

pub use presentation::{input::eventsystem::PresentationEventType, prelude::*};
