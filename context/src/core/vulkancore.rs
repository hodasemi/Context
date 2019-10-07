use crate::prelude::*;
use presentation::prelude::*;

use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct VulkanCore {
    device: Arc<Device>,
    queue: Arc<Mutex<Queue>>,
}

impl VulkanCore {
    pub fn new(
        presentation_core: &PresentationCore,
        vulkan_debug_info: &VulkanDebugInfo,
        app_info: &ApplicationInfo,
    ) -> VerboseResult<VulkanCore> {
        // --- create instance ---
        // application info
        let app_name = VkString::new(&app_info.application_name);
        let engine_name = VkString::new(&app_info.engine_name);

        let info = VkApplicationInfo::new(
            &app_name,
            app_info.application_version,
            &engine_name,
            app_info.engine_version,
            VK_MAKE_VERSION(1, 0, 0),
        );

        // instance extensions
        let mut instance_extensions = InstanceExtensions::default();
        presentation_core.activate_vulkan_instance_extensions(&mut instance_extensions)?;
        instance_extensions.physical_device_properties2 = true;

        // create instance
        let instance = Instance::new(info, *vulkan_debug_info, instance_extensions)?;

        let (queue_info, physical_device) = match presentation_core.backend() {
            PresentationBackend::Window(wsi) => {
                wsi.create_vulkan_surface(&instance)?;

                // create physical device
                let physical_device = PhysicalDevice::new(instance.clone())?;

                let queue_info = Queue::create_presentable_request_info(
                    &physical_device,
                    &wsi.surface()?,
                    VK_QUEUE_GRAPHICS_BIT | VK_QUEUE_COMPUTE_BIT,
                )?;

                (queue_info, physical_device)
            }
            PresentationBackend::OpenXR(xri) => {
                let physical_device =
                    PhysicalDevice::from_raw(instance.clone(), xri.physical_device(&instance)?)?;

                let queue_info = Queue::create_non_presentable_request_info(
                    &physical_device,
                    VK_QUEUE_GRAPHICS_BIT | VK_QUEUE_COMPUTE_BIT,
                )?;

                (queue_info, physical_device)
            }
            PresentationBackend::OpenVR(vri) => {
                let physical_device =
                    PhysicalDevice::from_raw(instance.clone(), vri.physical_device(&instance)?)?;

                let queue_info = Queue::create_non_presentable_request_info(
                    &physical_device,
                    VK_QUEUE_GRAPHICS_BIT | VK_QUEUE_COMPUTE_BIT,
                )?;

                (queue_info, physical_device)
            }
        };

        // device extensions
        let mut dev_exts = DeviceExtensions::default();
        presentation_core.activate_vulkan_device_extensions(&mut dev_exts, &physical_device)?;
        dev_exts.memory_requirements2 = true;
        dev_exts.nv_ray_tracing = true;
        dev_exts.amd_rasterization_order = true;
        dev_exts.descriptor_indexing = true;
        dev_exts.maintenance3 = true;

        if vulkan_debug_info.debugging && vulkan_debug_info.renderdoc {
            dev_exts.debug_marker = true;
        }

        let device = Device::new(physical_device, dev_exts, &[queue_info.queue_create_info])?;

        let queue = Device::get_queue(
            &device,
            queue_info.queue_family_index,
            queue_info.queue_index,
        );

        Ok(VulkanCore { device, queue })
    }

    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }

    pub fn queue(&self) -> &Arc<Mutex<Queue>> {
        &self.queue
    }
}
