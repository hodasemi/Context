use utilities::prelude::*;

use crate::impl_vk_handle;
use crate::prelude::*;

use std::sync::Arc;

#[derive(Debug)]
pub struct PhysicalDevice {
    instance: Arc<Instance>,
    physical_device: VkPhysicalDevice,
    properties: VkPhysicalDeviceProperties,
    features: VkPhysicalDeviceFeatures,
    memory_properties: VkPhysicalDeviceMemoryProperties,
    supported_extensions: Vec<VkString>,

    // extension info
    ray_tracing_properties: VkPhysicalDeviceRayTracingPropertiesNV,
    descriptor_indexing_features: VkPhysicalDeviceDescriptorIndexingFeaturesEXT,
    descriptor_indexing_properties: VkPhysicalDeviceDescriptorIndexingPropertiesEXT,
}

unsafe impl Sync for PhysicalDevice {}
unsafe impl Send for PhysicalDevice {}

impl PhysicalDevice {
    pub fn new(instance: Arc<Instance>) -> VerboseResult<Arc<PhysicalDevice>> {
        let physical_devices = instance.enumerate_physical_devices()?;

        let (mut physical_device, mut device_properties) = PhysicalDevice::find_phys_dev(
            &instance,
            &physical_devices,
            VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU,
        );

        if physical_device.is_none() {
            let (_physical_device, _device_properties) = PhysicalDevice::find_phys_dev(
                &instance,
                &physical_devices,
                VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU,
            );

            if _physical_device.is_none() {
                create_error!("error finding gpu, neither discrete nor integrated".to_string());
            }

            physical_device = _physical_device;
            device_properties = _device_properties;
        }

        let exported_device = physical_device.unwrap();
        let device_props = device_properties.unwrap();

        Self::internal_new(instance, exported_device, device_props)
    }

    pub fn from_raw(
        instance: Arc<Instance>,
        physical_device: VkPhysicalDevice,
    ) -> VerboseResult<Arc<PhysicalDevice>> {
        let properties = instance.physical_device_properties(physical_device);

        Self::internal_new(instance, physical_device, properties)
    }

    fn internal_new(
        instance: Arc<Instance>,
        physical_device: VkPhysicalDevice,
        properties: VkPhysicalDeviceProperties,
    ) -> VerboseResult<Arc<PhysicalDevice>> {
        let device_features = instance.physical_device_features(physical_device);

        let device_memory_properties = instance.physical_device_memory_properties(physical_device);

        let extensions = Self::query_extensions(&instance, physical_device)?;

        // get extension properties
        let mut device_properties2 = VkPhysicalDeviceProperties2KHR::default();

        // get ray tracing properties
        let ray_tracing_properties = VkPhysicalDeviceRayTracingPropertiesNV::default();
        device_properties2.chain(&ray_tracing_properties);
        instance.physical_device_properties2(physical_device, &mut device_properties2);

        // get descriptor indexing properties
        let descriptor_indexing_properties =
            VkPhysicalDeviceDescriptorIndexingPropertiesEXT::default();
        device_properties2.chain(&descriptor_indexing_properties);
        instance.physical_device_properties2(physical_device, &mut device_properties2);

        // get extension features
        let mut device_features2 = VkPhysicalDeviceFeatures2KHR::default();

        // get descriptor indexing features
        let descriptor_indexing_features = VkPhysicalDeviceDescriptorIndexingFeaturesEXT::default();
        device_features2.chain(&descriptor_indexing_features);
        instance.physical_device_features2(physical_device, &mut device_features2);

        Ok(Arc::new(PhysicalDevice {
            instance,
            physical_device: physical_device,
            properties,
            features: device_features,
            memory_properties: device_memory_properties,
            supported_extensions: extensions,

            ray_tracing_properties,
            descriptor_indexing_properties,
            descriptor_indexing_features,
        }))
    }
}

// getter
impl PhysicalDevice {
    pub fn instance(&self) -> &Arc<Instance> {
        &self.instance
    }

    pub fn features(&self) -> VkPhysicalDeviceFeatures {
        self.features
    }

    pub fn memory_properties(&self) -> &VkPhysicalDeviceMemoryProperties {
        &self.memory_properties
    }

    pub fn extensions(&self) -> &Vec<VkString> {
        &self.supported_extensions
    }

    pub fn properties(&self) -> &VkPhysicalDeviceProperties {
        &self.properties
    }

    pub fn ray_tracing_properties(&self) -> &VkPhysicalDeviceRayTracingPropertiesNV {
        &self.ray_tracing_properties
    }

    pub fn descriptor_indexing_properties(
        &self,
    ) -> &VkPhysicalDeviceDescriptorIndexingPropertiesEXT {
        &self.descriptor_indexing_properties
    }

    pub fn descriptor_indexing_features(&self) -> &VkPhysicalDeviceDescriptorIndexingFeaturesEXT {
        &self.descriptor_indexing_features
    }

    pub fn check_optimal_format_features(
        &self,
        format: VkFormat,
        usage: impl Into<VkImageUsageFlagBits>,
    ) -> bool {
        let format_properties = self
            .instance
            .physical_device_format_properties(self.physical_device, format);

        let features = Self::image_usage_into_features(usage.into());

        if (format_properties.optimalTilingFeatures & features) == features {
            return true;
        }

        false
    }

    pub fn check_linear_format_features(
        &self,
        format: VkFormat,
        usage: impl Into<VkImageUsageFlagBits>,
    ) -> bool {
        let format_properties = self
            .instance
            .physical_device_format_properties(self.physical_device, format);

        let features = Self::image_usage_into_features(usage.into());

        if (format_properties.linearTilingFeatures & features) == features {
            return true;
        }

        false
    }

    pub fn check_buffer_format_features(
        &self,
        format: VkFormat,
        features: impl Into<VkFormatFeatureFlagBits>,
    ) -> bool {
        let format_properties = self
            .instance
            .physical_device_format_properties(self.physical_device, format);

        let features = features.into();

        if (format_properties.bufferFeatures & features) == features {
            return true;
        }

        false
    }

    fn image_usage_into_features(usage: VkImageUsageFlagBits) -> VkFormatFeatureFlagBits {
        let mut features = 0u32.into();

        if (usage & VK_IMAGE_USAGE_TRANSFER_SRC_BIT) == VK_IMAGE_USAGE_TRANSFER_SRC_BIT {
            features |= VK_FORMAT_FEATURE_TRANSFER_SRC_BIT_KHR;
        }

        if (usage & VK_IMAGE_USAGE_TRANSFER_DST_BIT) == VK_IMAGE_USAGE_TRANSFER_DST_BIT {
            features |= VK_FORMAT_FEATURE_TRANSFER_DST_BIT_KHR;
        }

        if (usage & VK_IMAGE_USAGE_SAMPLED_BIT) == VK_IMAGE_USAGE_SAMPLED_BIT {
            features |= VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT;
        }

        if (usage & VK_IMAGE_USAGE_STORAGE_BIT) == VK_IMAGE_USAGE_STORAGE_BIT {
            features |= VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT;
        }

        if (usage & VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT) == VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT {
            features |= VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT;
        }

        if (usage & VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT)
            == VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT
        {
            features |= VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT;
        }

        features
    }
}

impl_vk_handle!(PhysicalDevice, VkPhysicalDevice, physical_device);

// private
impl PhysicalDevice {
    fn find_phys_dev(
        instance: &Arc<Instance>,
        physical_devices: &[VkPhysicalDevice],
        device_type: VkPhysicalDeviceType,
    ) -> (Option<VkPhysicalDevice>, Option<VkPhysicalDeviceProperties>) {
        for physical_device in physical_devices {
            let properties = instance.physical_device_properties(*physical_device);

            if properties.deviceType == device_type {
                return (Some(*physical_device), Some(properties));
            }
        }

        (None, None)
    }

    fn query_extensions(
        instance: &Arc<Instance>,
        physical_device: VkPhysicalDevice,
    ) -> VerboseResult<Vec<VkString>> {
        let extensions = instance.enumerate_device_extensions(physical_device)?;

        let mut vkstrings = Vec::new();

        for extension_property in extensions {
            vkstrings.push(extension_property.extension_name()?);
        }

        Ok(vkstrings)
    }
}
