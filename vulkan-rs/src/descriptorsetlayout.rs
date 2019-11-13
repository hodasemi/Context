use utilities::prelude::*;

use crate::impl_vk_handle;
use crate::prelude::*;

use std::sync::Arc;

pub struct DescriptorSetLayoutBuilder {
    layout_bindings: Vec<VkDescriptorSetLayoutBinding>,
    indexing_flags: Vec<VkDescriptorBindingFlagBitsEXT>,
    flags: VkDescriptorSetLayoutCreateFlagBits,
}

impl DescriptorSetLayoutBuilder {
    pub fn add_layout_binding(
        mut self,
        binding: u32,
        descriptor_type: VkDescriptorType,
        stage_flags: impl Into<VkShaderStageFlagBits>,
        indexing_flags: impl Into<VkDescriptorBindingFlagBitsEXT>,
    ) -> Self {
        self.layout_bindings.push(VkDescriptorSetLayoutBinding::new(
            binding,
            descriptor_type,
            stage_flags,
        ));

        let flags = indexing_flags.into();
        self.indexing_flags.push(flags);

        if (flags & VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT_EXT) != 0 {
            self.flags |= VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT_EXT;
        }

        self
    }

    pub fn change_descriptor_count(mut self, count: u32) -> Self {
        if !self.layout_bindings.is_empty() {
            let length = self.layout_bindings.len();
            self.layout_bindings[length - 1].descriptorCount = count;
        }

        self
    }

    pub fn set_flags(mut self, flags: impl Into<VkDescriptorSetLayoutCreateFlagBits>) -> Self {
        self.flags = flags.into();

        self
    }

    pub fn build(self, device: Arc<Device>) -> VerboseResult<Arc<DescriptorSetLayout>> {
        let mut descriptor_set_ci =
            VkDescriptorSetLayoutCreateInfo::new(self.flags, &self.layout_bindings);
        let binding_flags_ci =
            VkDescriptorSetLayoutBindingFlagsCreateInfoEXT::new(&self.indexing_flags);

        if device.enabled_extensions().descriptor_indexing {
            descriptor_set_ci.chain(&binding_flags_ci);

            /*
            if device.enabled_extensions().maintenance3 {
                let mut layout_support = VkDescriptorSetLayoutSupport::default();
                let variable_support =
                    VkDescriptorSetVariableDescriptorCountLayoutSupportEXT::default();

                layout_support.chain(&variable_support);

                device.descriptor_set_layout_support(&descriptor_set_ci, &mut layout_support);
            }
            */
        }

        let descriptor_set_layout = device.create_descriptor_set_layout(&descriptor_set_ci)?;

        let mut pool_sizes = Vec::new();

        for layout_binding in &self.layout_bindings {
            pool_sizes.push(VkDescriptorPoolSize {
                ty: layout_binding.descriptorType,
                descriptorCount: layout_binding.descriptorCount,
            });
        }

        Ok(Arc::new(DescriptorSetLayout {
            device,
            descriptor_set_layout,
            pool_sizes,
        }))
    }
}

#[derive(Debug)]
pub struct DescriptorSetLayout {
    device: Arc<Device>,
    descriptor_set_layout: VkDescriptorSetLayout,
    pool_sizes: Vec<VkDescriptorPoolSize>,
}

impl DescriptorSetLayout {
    pub fn builder() -> DescriptorSetLayoutBuilder {
        DescriptorSetLayoutBuilder {
            layout_bindings: Vec::new(),
            indexing_flags: Vec::new(),
            flags: 0u32.into(),
        }
    }

    pub fn pool_sizes(&self) -> &[VkDescriptorPoolSize] {
        self.pool_sizes.as_slice()
    }
}

unsafe impl Send for DescriptorSetLayout {}
unsafe impl Sync for DescriptorSetLayout {}

impl_vk_handle!(
    DescriptorSetLayout,
    VkDescriptorSetLayout,
    descriptor_set_layout
);

impl Drop for DescriptorSetLayout {
    fn drop(&mut self) {
        self.device
            .destroy_descriptor_set_layout(self.descriptor_set_layout);
    }
}
