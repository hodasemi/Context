use crate::prelude::*;
use std::fmt;

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct VkGeometryInstanceNV {
    pub transform: [f32; 12],
    instance_id_mask: u32,
    instance_offset_flags: u32,
    pub accelerationStructureHandle: u64,
}

impl VkGeometryInstanceNV {
    pub fn new(
        transform: [f32; 12],
        instance_id: u32,
        mask: u8,
        instance_offset: u32,
        flags: impl Into<VkGeometryInstanceFlagBitsNV>,
        acceleration_tructure_handle: u64,
    ) -> Self {
        let instance_id = Self::u24_to_u32(instance_id);
        let instance_offset = Self::u24_to_u32(instance_offset);
        let flags: u32 = flags.into().into();
        let flags = Self::u32_to_u8(flags);
        let mask = Self::u32_to_u8(mask as u32);

        VkGeometryInstanceNV {
            transform,
            instance_id_mask: (instance_id | mask),
            instance_offset_flags: (instance_offset | flags),
            accelerationStructureHandle: acceleration_tructure_handle,
        }
    }

    pub fn instance_id(&self) -> u32 {
        Self::u32_to_u24(self.instance_id_mask)
    }

    pub fn mask(&self) -> u32 {
        Self::u8_to_u32(self.instance_id_mask)
    }

    pub fn instance_offset(&self) -> u32 {
        Self::u32_to_u24(self.instance_offset_flags)
    }

    pub fn flags(&self) -> VkGeometryInstanceFlagBitsNV {
        Self::u8_to_u32(self.instance_offset_flags).into()
    }

    #[inline]
    fn u32_to_u24(bits: u32) -> u32 {
        bits & 0x00FF_FFFF
    }

    #[inline]
    fn u24_to_u32(bits: u32) -> u32 {
        bits & 0x00FF_FFFF
    }

    #[inline]
    fn u32_to_u8(bits: u32) -> u32 {
        (bits & 0x0000_00FF) << 24
    }

    #[inline]
    fn u8_to_u32(bits: u32) -> u32 {
        (bits & 0xFF00_0000) >> 24
    }
}

impl fmt::Debug for VkGeometryInstanceNV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "VkGeometryInstanceNV {{ transform: {:?}, instanceID: {}, mask: {}, instanceOffset: {}, flags: {:?}, accelerationStructureHandle {} }}",
            self.transform,
            self.instance_id(),
            self.mask(),
            self.instance_offset(),
            self.flags(),
            self.accelerationStructureHandle
        )
    }
}
