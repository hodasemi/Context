#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]

pub mod input;
pub mod presentationcore;
mod renderbackend;
pub mod traits;
pub mod vri;
pub mod wsi;
pub mod xri;

pub mod prelude;

#[macro_export]
macro_rules! p_try {
    ($r:expr) => {
        match $r {
            Ok(t) => t,
            Err(err) => create_error!(format!("{:?}", err)),
        }
    };
}

use traits::RenderCore;
use utilities::prelude::*;
use vulkan_rs::prelude::*;

use crate::prelude::*;

use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy)]
pub struct RenderCoreCreateInfo {
    pub usage: VkImageUsageFlagBits,
    pub format: VkFormat,
    pub vsync: bool,
}

pub fn create_render_core(
    presentation_core: &PresentationCore,
    device: &Arc<Device>,
    queue: &Arc<Mutex<Queue>>,
    create_info: RenderCoreCreateInfo,
) -> VerboseResult<(Box<dyn RenderCore + Send + Sync>, TargetMode<()>)> {
    match presentation_core.backend() {
        PresentationBackend::Window(wsi) => {
            let (render_core, target_mode) =
                wsi::vulkanwindowrendercore::VulkanWindowRenderCore::new(
                    wsi,
                    device,
                    queue,
                    create_info,
                )?;

            Ok((Box::new(render_core), target_mode))
        }
        PresentationBackend::OpenXR(xri) => {
            let (render_core, target_mode) =
                xri::openxrrendercore::OpenXRRenderCore::new(xri, device, queue, create_info)?;

            Ok((Box::new(render_core), target_mode))
        }
        PresentationBackend::OpenVR(vri) => {
            let (render_core, target_mode) =
                vri::openvrrendercore::OpenVRRenderCore::new(vri, device, queue, create_info)?;

            Ok((Box::new(render_core), target_mode))
        }
    }
}
