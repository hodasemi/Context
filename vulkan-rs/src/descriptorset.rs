use utilities::prelude::*;

use crate::impl_vk_handle;
use crate::prelude::*;

use std::slice;
use std::sync::Arc;

#[derive(Debug)]
pub struct DescriptorWrite {
    binding: u32,
    descriptor_type: VkDescriptorType,
    inner: InnerWrite,
}

#[derive(Debug)]
enum InnerWrite {
    Buffers(Vec<VkDescriptorBufferInfo>),
    Images(Vec<VkDescriptorImageInfo>),
    AS(
        (
            VkWriteDescriptorSetAccelerationStructureNV,
            Vec<VkAccelerationStructureNV>,
        ),
    ),
}

impl DescriptorWrite {
    pub fn uniform_buffers<T>(binding: u32, buffers: &[&Arc<Buffer<T>>]) -> Self {
        DescriptorWrite {
            binding,
            descriptor_type: VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
            inner: InnerWrite::Buffers(
                buffers
                    .iter()
                    .map(|buffer| VkDescriptorBufferInfo {
                        buffer: buffer.vk_handle(),
                        offset: 0,
                        range: buffer.byte_size(),
                    })
                    .collect(),
            ),
        }
    }

    pub fn storage_buffers<T>(binding: u32, buffers: &[&Arc<Buffer<T>>]) -> Self {
        DescriptorWrite {
            binding,
            descriptor_type: VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
            inner: InnerWrite::Buffers(
                buffers
                    .iter()
                    .map(|buffer| VkDescriptorBufferInfo {
                        buffer: buffer.vk_handle(),
                        offset: 0,
                        range: buffer.byte_size(),
                    })
                    .collect(),
            ),
        }
    }

    pub fn combined_samplers(binding: u32, images: &[&Arc<Image>]) -> Self {
        DescriptorWrite {
            binding,
            descriptor_type: VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
            inner: InnerWrite::Images(
                images
                    .iter()
                    .map(|image| VkDescriptorImageInfo {
                        sampler: image.vk_handle(),
                        imageView: image.vk_handle(),
                        imageLayout: image.image_layout().expect("image layout lock error"),
                    })
                    .collect(),
            ),
        }
    }

    pub fn storage_images(binding: u32, images: &[&Arc<Image>]) -> Self {
        DescriptorWrite {
            binding,
            descriptor_type: VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
            inner: InnerWrite::Images(
                images
                    .iter()
                    .map(|image| VkDescriptorImageInfo {
                        sampler: image.vk_handle(),
                        imageView: image.vk_handle(),
                        imageLayout: image.image_layout().expect("image layout lock error"),
                    })
                    .collect(),
            ),
        }
    }

    pub fn acceleration_structures(
        binding: u32,
        acceleration_structures: &[&Arc<AccelerationStructure>],
    ) -> Self {
        let vk_as: Vec<VkAccelerationStructureNV> = acceleration_structures
            .iter()
            .map(|a| a.vk_handle())
            .collect();

        let mut write = DescriptorWrite {
            binding,
            descriptor_type: VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV,
            inner: InnerWrite::AS((
                VkWriteDescriptorSetAccelerationStructureNV::default(),
                vk_as,
            )),
        };

        if let InnerWrite::AS((vk_write_as, vk_as)) = &mut write.inner {
            vk_write_as.set_acceleration_structures(&vk_as);
        }

        write
    }

    pub fn change_image_layout(mut self, image_layout: VkImageLayout) -> Self {
        if let InnerWrite::Images(ref mut infos) = self.inner {
            for info in infos {
                info.imageLayout = image_layout;
            }
        }

        self
    }

    fn vk_write(&self, write: &mut VkWriteDescriptorSet) {
        match &self.inner {
            InnerWrite::Buffers(buffer_infos) => {
                write.set_buffer_infos(buffer_infos);
            }
            InnerWrite::Images(image_infos) => {
                write.set_image_infos(image_infos);
            }
            InnerWrite::AS((as_write, _)) => {
                write.descriptorCount = as_write.accelerationStructureCount;
                write.chain(as_write);
            }
        }
    }
}

pub struct DescriptorSetBuilder {
    device: Arc<Device>,
    descriptor_pool: Arc<DescriptorPool>,
    variable_desc_counts: Vec<u32>,
    variable_descriptor_count: VkDescriptorSetVariableDescriptorCountAllocateInfoEXT,
}

impl DescriptorSetBuilder {
    pub fn set_variable_descriptor_counts(mut self, descriptor_counts: &[u32]) -> Self {
        self.variable_desc_counts = descriptor_counts.to_vec();

        self
    }

    pub fn allocate(mut self) -> VerboseResult<Arc<DescriptorSet>> {
        let layout = self.descriptor_pool.vk_handle();

        let mut descriptor_set_ci = VkDescriptorSetAllocateInfo::new(
            self.descriptor_pool.vk_handle(),
            slice::from_ref(&layout),
        );

        if !self.variable_desc_counts.is_empty() {
            self.variable_descriptor_count
                .set_descriptor_counts(&self.variable_desc_counts);
            descriptor_set_ci.chain(&self.variable_descriptor_count);
        }

        let descriptor_set = self.device.allocate_descriptor_sets(&descriptor_set_ci)?[0];

        Ok(Arc::new(DescriptorSet {
            device: self.device,
            pool: self.descriptor_pool,
            descriptor_set,
        }))
    }
}

#[derive(Debug)]
pub struct DescriptorSet {
    device: Arc<Device>,
    pool: Arc<DescriptorPool>,
    descriptor_set: VkDescriptorSet,
}

impl DescriptorSet {
    pub(crate) fn builder(
        device: Arc<Device>,
        descriptor_pool: Arc<DescriptorPool>,
    ) -> DescriptorSetBuilder {
        DescriptorSetBuilder {
            device,
            descriptor_pool,
            variable_desc_counts: Vec::new(),
            variable_descriptor_count: VkDescriptorSetVariableDescriptorCountAllocateInfoEXT::new(
                &[],
            ),
        }
    }

    // TODO: add update function for VkCopyDescriptorSet
    pub fn update(&self, writes: &[DescriptorWrite]) {
        debug_assert!(!writes.is_empty());

        let mut vk_writes = Vec::new();

        for write in writes {
            let mut write_desc = VkWriteDescriptorSet::new(
                self.descriptor_set,
                write.binding,
                0,
                write.descriptor_type,
            );

            write.vk_write(&mut write_desc);

            vk_writes.push(write_desc);
        }

        self.device
            .update_descriptor_sets(vk_writes.as_slice(), &[]);
    }
}

impl_vk_handle!(DescriptorSet, VkDescriptorSet, descriptor_set);

impl VkHandle<VkDescriptorSetLayout> for DescriptorSet {
    fn vk_handle(&self) -> VkDescriptorSetLayout {
        self.pool.vk_handle()
    }
}

impl<'a> VkHandle<VkDescriptorSetLayout> for &'a DescriptorSet {
    fn vk_handle(&self) -> VkDescriptorSetLayout {
        self.pool.vk_handle()
    }
}

impl VkHandle<VkDescriptorSetLayout> for Arc<DescriptorSet> {
    fn vk_handle(&self) -> VkDescriptorSetLayout {
        self.pool.vk_handle()
    }
}

impl<'a> VkHandle<VkDescriptorSetLayout> for &'a Arc<DescriptorSet> {
    fn vk_handle(&self) -> VkDescriptorSetLayout {
        self.pool.vk_handle()
    }
}

impl Drop for DescriptorSet {
    fn drop(&mut self) {
        if let Err(error) = self
            .device
            .free_descriptor_sets(self.pool.vk_handle(), &[self.descriptor_set])
        {
            println!("{}", error);
        }
    }
}
