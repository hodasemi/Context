use crate::impl_vk_handle;
use crate::prelude::*;

use utilities::prelude::*;

use std::sync::Arc;

#[derive(Debug)]
pub struct Surface {
    external_source: bool,
    instance: Arc<Instance>,
    surface: VkSurfaceKHR,
}

impl Surface {
    pub fn from_vk_surface(surface: VkSurfaceKHR, instance: &Arc<Instance>) -> Arc<Surface> {
        Arc::new(Surface {
            external_source: true,
            instance: instance.clone(),
            surface,
        })
    }

    pub fn capabilities(&self, device: &Arc<Device>) -> VerboseResult<VkSurfaceCapabilitiesKHR> {
        self.instance.physical_device_surface_capabilities(
            device.physical_device().vk_handle(),
            self.surface,
        )
    }

    pub fn format_colorspace(
        &self,
        device: &Arc<Device>,
    ) -> VerboseResult<(VkFormat, VkColorSpaceKHR)> {
        let surface_formats = self
            .instance
            .physical_device_surface_formats(device.physical_device().vk_handle(), self.surface)?;

        // if there is a single undefined format, assume the preferred mode
        if (surface_formats.len() == 1) && (surface_formats[0].format == VK_FORMAT_UNDEFINED) {
            return Ok((VK_FORMAT_R8G8B8A8_UNORM, VK_COLOR_SPACE_SRGB_NONLINEAR_KHR));
        }

        // look for VK_FORMAT_R8G8B8A8_UNORM
        for surface_format in &surface_formats {
            if surface_format.format == VK_FORMAT_R8G8B8A8_UNORM {
                return Ok((surface_format.format, surface_format.colorSpace));
            }
        }

        // if nothing was found, take the first one
        Ok((surface_formats[0].format, surface_formats[0].colorSpace))
    }

    pub fn present_modes(&self, device: &Arc<Device>) -> VerboseResult<Vec<VkPresentModeKHR>> {
        self.instance
            .physical_device_present_modes(device.physical_device().vk_handle(), self.surface)
    }
}

unsafe impl Sync for Surface {}
unsafe impl Send for Surface {}

impl_vk_handle!(Surface, VkSurfaceKHR, surface);

impl Drop for Surface {
    fn drop(&mut self) {
        if !self.external_source {
            self.instance.destroy_surface(self.surface)
        }
    }
}
