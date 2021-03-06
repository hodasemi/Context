use crate::prelude::*;

use std::hash::{Hash, Hasher};
use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct VkSamplerCreateInfo {
    pub sType: VkStructureType,
    pub pNext: *const c_void,
    pub flags: VkSamplerCreateFlagBits,
    pub magFilter: VkFilter,
    pub minFilter: VkFilter,
    pub mipmapMode: VkSamplerMipmapMode,
    pub addressModeU: VkSamplerAddressMode,
    pub addressModeV: VkSamplerAddressMode,
    pub addressModeW: VkSamplerAddressMode,
    pub mipLodBias: f32,
    pub anisotropyEnable: VkBool32,
    pub maxAnisotropy: f32,
    pub compareEnable: VkBool32,
    pub compareOp: VkCompareOp,
    pub minLod: f32,
    pub maxLod: f32,
    pub borderColor: VkBorderColor,
    pub unnormalizedCoordinates: VkBool32,
}

impl VkSamplerCreateInfo {
    pub fn new<T>(
        flags: T,
        mag_filter: VkFilter,
        min_filter: VkFilter,
        mipmap_mode: VkSamplerMipmapMode,
        address_mode_u: VkSamplerAddressMode,
        address_mode_v: VkSamplerAddressMode,
        address_mode_w: VkSamplerAddressMode,
        mip_lod_bias: f32,
        anisotropy_enable: bool,
        max_anisotropy: f32,
        compare_enable: bool,
        compare_op: VkCompareOp,
        min_lod: f32,
        max_lod: f32,
        border_color: VkBorderColor,
        unnormalized_coordinates: bool,
    ) -> Self
    where
        T: Into<VkSamplerCreateFlagBits>,
    {
        VkSamplerCreateInfo {
            sType: VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
            pNext: ptr::null(),
            flags: flags.into(),
            magFilter: mag_filter,
            minFilter: min_filter,
            mipmapMode: mipmap_mode,
            addressModeU: address_mode_u,
            addressModeV: address_mode_v,
            addressModeW: address_mode_w,
            mipLodBias: mip_lod_bias,
            anisotropyEnable: anisotropy_enable.into(),
            maxAnisotropy: max_anisotropy,
            compareEnable: compare_enable.into(),
            compareOp: compare_op,
            minLod: min_lod,
            maxLod: max_lod,
            borderColor: border_color,
            unnormalizedCoordinates: unnormalized_coordinates.into(),
        }
    }
}

impl Eq for VkSamplerCreateInfo {}

impl Hash for VkSamplerCreateInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // ignore pNext and structure type

        self.flags.hash(state);
        self.magFilter.hash(state);
        self.minFilter.hash(state);
        self.mipmapMode.hash(state);
        self.addressModeU.hash(state);
        self.addressModeV.hash(state);
        self.addressModeW.hash(state);
        self.anisotropyEnable.hash(state);
        self.compareEnable.hash(state);
        self.compareOp.hash(state);
        self.borderColor.hash(state);
        self.unnormalizedCoordinates.hash(state);

        // cast f32 into native endian byte slice
        self.mipLodBias.to_ne_bytes().hash(state);
        self.maxAnisotropy.to_ne_bytes().hash(state);
        self.minLod.to_ne_bytes().hash(state);
        self.maxLod.to_ne_bytes().hash(state);
    }
}
