use openvr::{init, ApplicationType, Compositor, Context, System};

use utilities::prelude::*;
use vulkan_rs::prelude::*;

use crate::p_try;

use std::ffi::CString;
use std::mem;
use std::sync::Arc;

pub struct OpenVRIntegration {
    _context: Context,
    system: Arc<System>,
    compositor: Arc<Compositor>,
}

impl OpenVRIntegration {
    pub fn new() -> VerboseResult<Self> {
        let context = p_try!(unsafe { init(ApplicationType::Scene) });
        let system = p_try!(context.system());
        let compositor = p_try!(context.compositor());

        Ok(OpenVRIntegration {
            _context: context,
            system: Arc::new(system),
            compositor: Arc::new(compositor),
        })
    }

    pub fn activate_vulkan_instance_extensions(
        &self,
        extensions: &mut InstanceExtensions,
    ) -> VerboseResult<()> {
        let extension_names: Vec<CString> = self.compositor.vulkan_instance_extensions_required();

        for extension_name in extension_names {
            let string = p_try!(extension_name.into_string());
            extensions.activate(&string)?;
        }

        Ok(())
    }

    pub fn activate_vulkan_device_extensions(
        &self,
        extensions: &mut DeviceExtensions,
        physical_device: &Arc<PhysicalDevice>,
    ) -> VerboseResult<()> {
        let extension_names: Vec<CString> = unsafe {
            self.compositor
                .vulkan_device_extensions_required(mem::transmute(physical_device.vk_handle()))
        };

        for extension_name in extension_names {
            let string = p_try!(extension_name.into_string());
            extensions.activate(&string)?;
        }

        Ok(())
    }

    pub fn physical_device(&self, instance: &Arc<Instance>) -> VerboseResult<VkPhysicalDevice> {
        unsafe {
            let phys_dev = self
                .system
                .vulkan_output_device(mem::transmute(instance.vk_handle()))
                .ok_or("no physical device found")?;

            Ok(mem::transmute(phys_dev))
        }
    }

    pub(crate) fn image_size(&self) -> (u32, u32) {
        self.system.recommended_render_target_size()
    }

    pub(crate) fn compositor(&self) -> &Arc<Compositor> {
        &self.compositor
    }

    pub(crate) fn system(&self) -> &Arc<System> {
        &self.system
    }
}
