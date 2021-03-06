// structures
pub use crate::core::{
    configs::WindowConfig, context::Context, osspecific::osspecific::OsSpecificConfig,
};

#[cfg(feature = "audio")]
pub use crate::core::soundhandler::{Music, Sound, SoundHandler, VolumeInfo};

// reexport ears presets (OpenAL)
#[cfg(feature = "audio")]
pub use ears::{ReverbEffect, ReverbPreset};

// render target
pub use crate::utils::{
    render_target::{ClearValue, CustomTarget, RenderTarget},
    single_submit::SingleSubmit,
};

pub use presentation::{input::eventsystem::PresentationEventType, prelude::*};
