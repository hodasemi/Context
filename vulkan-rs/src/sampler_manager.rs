use crate::impl_vk_handle;
use crate::prelude::*;

use utilities::prelude::*;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct SamplerBuilder {
    create_info: VkSamplerCreateInfo,
}

impl SamplerBuilder {
    pub fn min_mag_filter(mut self, min_filter: VkFilter, mag_filter: VkFilter) -> Self {
        self.create_info.minFilter = min_filter;
        self.create_info.magFilter = mag_filter;

        self
    }

    pub fn map_map_mode(mut self, mode: VkSamplerMipmapMode) -> Self {
        self.create_info.mipmapMode = mode;

        self
    }

    pub fn address_mode(
        mut self,
        u: VkSamplerAddressMode,
        v: VkSamplerAddressMode,
        w: VkSamplerAddressMode,
    ) -> Self {
        self.create_info.addressModeU = u;
        self.create_info.addressModeV = v;
        self.create_info.addressModeW = w;

        self
    }

    pub fn min_load_bias(mut self, bias: f32) -> Self {
        self.create_info.mipLodBias = bias;

        self
    }

    pub fn anisotropy(mut self, anisotropy: f32) -> Self {
        self.create_info.anisotropyEnable = VK_TRUE;
        self.create_info.maxAnisotropy = anisotropy;

        self
    }

    pub fn compare(mut self, compare_op: VkCompareOp) -> Self {
        self.create_info.compareEnable = VK_TRUE;
        self.create_info.compareOp = compare_op;

        self
    }

    pub fn min_max_lod(mut self, min_lod: f32, max_lod: f32) -> Self {
        self.create_info.minLod = min_lod;
        self.create_info.maxLod = max_lod;

        self
    }

    pub fn border_color(mut self, border_color: VkBorderColor) -> Self {
        self.create_info.borderColor = border_color;

        self
    }

    pub fn coordinates<T>(mut self, unnormalized_coordinates: T) -> Self
    where
        T: Into<VkBool32>,
    {
        self.create_info.unnormalizedCoordinates = unnormalized_coordinates.into();

        self
    }

    pub fn build(self, device: &Device) -> VerboseResult<Arc<Sampler>> {
        device.create_sampler_from_manager(self.create_info)
    }
}

#[derive(Debug)]
pub struct Sampler {
    sampler: VkSampler,
}

impl Sampler {
    pub fn nearest_sampler() -> SamplerBuilder {
        SamplerBuilder {
            create_info: VkSamplerCreateInfo::new(
                0,
                VK_FILTER_NEAREST,
                VK_FILTER_NEAREST,
                VK_SAMPLER_MIPMAP_MODE_NEAREST,
                VK_SAMPLER_ADDRESS_MODE_REPEAT,
                VK_SAMPLER_ADDRESS_MODE_REPEAT,
                VK_SAMPLER_ADDRESS_MODE_REPEAT,
                0.0,
                false,
                1.0,
                false,
                VK_COMPARE_OP_NEVER,
                0.0,
                0.0,
                VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE,
                false,
            ),
        }
    }

    pub fn pretty_sampler() -> SamplerBuilder {
        SamplerBuilder {
            create_info: VkSamplerCreateInfo::new(
                0,
                VK_FILTER_LINEAR,
                VK_FILTER_LINEAR,
                VK_SAMPLER_MIPMAP_MODE_LINEAR,
                VK_SAMPLER_ADDRESS_MODE_REPEAT,
                VK_SAMPLER_ADDRESS_MODE_REPEAT,
                VK_SAMPLER_ADDRESS_MODE_REPEAT,
                0.0,
                true,
                8.0,
                false,
                VK_COMPARE_OP_NEVER,
                0.0,
                0.0,
                VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE,
                false,
            ),
        }
    }
}

impl_vk_handle!(Sampler, VkSampler, sampler);

pub struct SamplerManager {
    samplers: HashMap<VkSamplerCreateInfo, Arc<Sampler>>,
}

unsafe impl Sync for SamplerManager {}
unsafe impl Send for SamplerManager {}

impl SamplerManager {
    pub fn new() -> Mutex<Self> {
        Mutex::new(SamplerManager {
            samplers: HashMap::new(),
        })
    }

    pub fn create_sampler(
        &mut self,
        create_info: VkSamplerCreateInfo,
        device: &Device,
    ) -> VerboseResult<Arc<Sampler>> {
        match self.samplers.get(&create_info) {
            Some(sampler) => Ok(sampler.clone()),
            None => {
                let new_sampler = Arc::new(Sampler {
                    sampler: device.create_sampler(&create_info)?,
                });

                self.samplers.insert(create_info, new_sampler.clone());

                Ok(new_sampler)
            }
        }
    }

    /// This will destroy all VkSampler handles, no matter if they are in use or not
    pub unsafe fn clear(&mut self, device: &Device) {
        self.samplers
            .iter()
            .for_each(|(_, sampler)| device.destroy_sampler(sampler.vk_handle()));

        self.samplers.clear();
    }
}
