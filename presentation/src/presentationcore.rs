use crate::input::eventsystem::EventSystem;
use crate::prelude::*;
use crate::vri::openvrintegration::OpenVRIntegration;
use crate::wsi::windowsystemintegration::WindowSystemIntegration;
use crate::xri::openxrintegration::OpenXRIntegration;

use utilities::prelude::*;
use vulkan_rs::prelude::*;

use sdl2::Sdl;

use std::sync::Arc;

pub enum PresentationBackend {
    Window(WindowSystemIntegration),
    OpenXR(OpenXRIntegration),
    OpenVR(OpenVRIntegration),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VRMode {
    OpenXR,
    OpenVR,
}

#[derive(Debug, Clone)]
pub struct ApplicationInfo {
    pub application_name: String,
    pub application_version: u32,
    pub engine_name: String,
    pub engine_version: u32,
}

impl Default for ApplicationInfo {
    fn default() -> Self {
        ApplicationInfo {
            application_name: "empty".to_string(),
            application_version: 0,
            engine_name: "empty".to_string(),
            engine_version: 0,
        }
    }
}

#[cfg(feature = "OpenXR")]
impl ApplicationInfo {
    fn as_openxr_info<'a>(&'a self) -> openxr::ApplicationInfo<'a> {
        openxr::ApplicationInfo {
            application_name: &self.application_name,
            application_version: self.application_version,
            engine_name: &self.engine_name,
            engine_version: self.engine_version,
        }
    }
}

#[cfg(not(feature = "OpenXR"))]
impl ApplicationInfo {
    fn as_openxr_info(&self) -> ApplicationInfo {
        ApplicationInfo {
            application_name: self.application_name.clone(),
            application_version: self.application_version,
            engine_name: self.engine_name.clone(),
            engine_version: self.engine_version,
        }
    }
}

pub struct PresentationCore {
    _sdl_context: Sdl,

    event_system: EventSystem,

    backend: PresentationBackend,
}

impl PresentationCore {
    pub fn enabled_vr_modes() -> Vec<VRMode> {
        let mut _modes = Vec::new();

        #[cfg(feature = "OpenXR")]
        _modes.push(VRMode::OpenXR);

        #[cfg(feature = "OpenVR")]
        _modes.push(VRMode::OpenVR);

        _modes
    }

    pub fn new(
        use_vr: Option<VRMode>,
        window_create_info: &WindowCreateInfo,
        appl_info: ApplicationInfo,
    ) -> VerboseResult<PresentationCore> {
        // create sdl2 context
        let context = sdl2::init()?;

        Ok(PresentationCore {
            event_system: EventSystem::new(&context)?,

            backend: match use_vr {
                Some(vr_mode) => match vr_mode {
                    VRMode::OpenXR => PresentationBackend::OpenXR(OpenXRIntegration::new(
                        appl_info.as_openxr_info(),
                    )?),
                    VRMode::OpenVR => PresentationBackend::OpenVR(OpenVRIntegration::new()?),
                },
                None => PresentationBackend::Window(WindowSystemIntegration::new(
                    window_create_info,
                    &context,
                )?),
            },

            _sdl_context: context,
        })
    }

    pub fn event_system(&self) -> &EventSystem {
        &self.event_system
    }

    pub fn activate_vulkan_instance_extensions(
        &self,
        extensions: &mut InstanceExtensions,
    ) -> VerboseResult<()> {
        match &self.backend {
            PresentationBackend::Window(wsi) => {
                wsi.activate_vulkan_instance_extensions(extensions)?;
            }
            PresentationBackend::OpenXR(xri) => {
                xri.activate_vulkan_instance_extensions(extensions)?;
            }
            PresentationBackend::OpenVR(vri) => {
                vri.activate_vulkan_instance_extensions(extensions)?;
            }
        }

        Ok(())
    }

    pub fn activate_vulkan_device_extensions(
        &self,
        extensions: &mut DeviceExtensions,
        physical_device: &Arc<PhysicalDevice>,
    ) -> VerboseResult<()> {
        match &self.backend {
            PresentationBackend::Window(wsi) => {
                wsi.activate_vulkan_device_extensions(extensions)?;
            }
            PresentationBackend::OpenXR(xri) => {
                xri.activate_vulkan_device_extensions(extensions)?;
            }
            PresentationBackend::OpenVR(vri) => {
                vri.activate_vulkan_device_extensions(extensions, physical_device)?;
            }
        }

        Ok(())
    }

    pub fn backend(&self) -> &PresentationBackend {
        &self.backend
    }
}
