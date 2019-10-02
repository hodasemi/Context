use openxr::{
    vulkan::SessionCreateInfo, ApplicationInfo, EnvironmentBlendMode, ExtensionSet, FormFactor,
    FrameStream, FrameWaiter, Instance as OpenXRInstance, Session, SystemId, SystemProperties,
    ViewConfigurationProperties, ViewConfigurationType, ViewConfigurationView, Vulkan,
};

use utilities::prelude::*;
use vulkan_rs::prelude::*;

use crate::p_try;

use std::mem;
use std::sync::Arc;

pub struct OpenXRIntegration {
    instance: Arc<OpenXRInstance>,
    system_id: SystemId,
}

impl OpenXRIntegration {
    pub fn new<'a>(app_info: ApplicationInfo<'a>) -> VerboseResult<OpenXRIntegration> {
        let entry = openxr::Entry::linked();

        let supported_extensions = p_try!(entry.enumerate_extensions());
        println!("supported extensions: {:#?}", supported_extensions);

        let mut extensions = ExtensionSet::default();

        if !supported_extensions.khr_vulkan_enable {
            create_error!("vulkan not available for OpenXR implementation");
        }

        extensions.khr_vulkan_enable = true;

        if supported_extensions.ext_debug_utils {
            // extensions.ext_debug_utils = true;
        }

        let instance = p_try!(entry.create_instance(&app_info, &extensions));
        let system_id = p_try!(instance.system(FormFactor::HEAD_MOUNTED_DISPLAY));

        Ok(OpenXRIntegration {
            instance: Arc::new(instance),
            system_id,
        })
    }

    pub fn activate_vulkan_instance_extensions(
        &self,
        extensions: &mut InstanceExtensions,
    ) -> VerboseResult<()> {
        let extension_names: Vec<String> =
            p_try!(self.instance.vulkan_instance_extensions(self.system_id))
                .split(" ")
                .map(|extension_name| extension_name.to_string())
                .collect();

        for extension_name in extension_names {
            extensions.activate(&extension_name)?;
        }

        Ok(())
    }

    pub fn activate_vulkan_device_extensions(
        &self,
        extensions: &mut DeviceExtensions,
    ) -> VerboseResult<()> {
        let extension_names: Vec<String> =
            p_try!(self.instance.vulkan_device_extensions(self.system_id))
                .split(" ")
                .map(|s| s.to_string())
                .collect();

        for extension_name in extension_names {
            extensions.activate(&extension_name)?;
        }

        Ok(())
    }

    pub fn physical_device(&self, instance: &Arc<Instance>) -> VerboseResult<VkPhysicalDevice> {
        unsafe {
            let phys_dev = p_try!(self
                .instance
                .vulkan_graphics_device(self.system_id, mem::transmute(instance.vk_handle())));

            Ok(mem::transmute(phys_dev))
        }
    }

    pub(crate) fn instance(&self) -> &Arc<OpenXRInstance> {
        &self.instance
    }

    pub(crate) fn create_session(
        &self,
        device: &Arc<Device>,
        queue_family_index: u32,
        queue_index: u32,
    ) -> VerboseResult<(Session<Vulkan>, FrameWaiter, FrameStream<Vulkan>)> {
        let session_ci = unsafe {
            SessionCreateInfo {
                instance: mem::transmute(device.physical_device().instance().vk_handle()),
                device: mem::transmute(device.vk_handle()),
                physical_device: mem::transmute(device.physical_device().vk_handle()),
                queue_family_index,
                queue_index,
            }
        };

        Ok(p_try!(unsafe {
            self.instance
                .create_session::<Vulkan>(self.system_id, &session_ci)
        }))
    }

    pub(crate) fn view_configs(&self) -> VerboseResult<Vec<ViewConfigurationType>> {
        Ok(p_try!(self
            .instance
            .enumerate_view_configurations(self.system_id)))
    }

    pub(crate) fn view_config_properties(
        &self,
        view_config_type: ViewConfigurationType,
    ) -> VerboseResult<ViewConfigurationProperties> {
        Ok(p_try!(self.instance.view_configuration_properties(
            self.system_id,
            view_config_type
        )))
    }

    pub(crate) fn view_config_views(
        &self,
        view_config_type: ViewConfigurationType,
    ) -> VerboseResult<Vec<ViewConfigurationView>> {
        Ok(p_try!(self.instance.enumerate_view_configuration_views(
            self.system_id,
            view_config_type
        )))
    }

    pub(crate) fn enumerate_environment_blend_modes(
        &self,
        view_config_type: ViewConfigurationType,
    ) -> VerboseResult<Vec<EnvironmentBlendMode>> {
        Ok(p_try!(self.instance.enumerate_environment_blend_modes(
            self.system_id,
            view_config_type
        )))
    }

    pub(crate) fn system_properties(&self) -> VerboseResult<SystemProperties> {
        Ok(p_try!(self.instance.system_properties(self.system_id)))
    }

    pub(crate) fn print_system_properties(system_properties: &SystemProperties) {
        println!("OpenXR System Properties:");
        println!("vendor_id: {}", system_properties.vendor_id);
        println!("system_name: {}", system_properties.system_name);
        println!("graphics_properties: {{");
        println!(
            "\tmax_swapchain_image_width: {}",
            system_properties
                .graphics_properties
                .max_swapchain_image_width
        );
        println!(
            "\tmax_swapchain_image_height: {}",
            system_properties
                .graphics_properties
                .max_swapchain_image_height
        );
        println!(
            "\tmax_layer_count: {}",
            system_properties.graphics_properties.max_layer_count,
        );
        println!("}}");
        println!("{:#?}", system_properties.tracking_properties);
    }
}
