use std::mem;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VkClearColorValue([u32; 4]);

impl VkClearColorValue {
    #[inline]
    pub fn as_float32(&self) -> &[f32; 4] {
        unsafe { mem::transmute(&self.0) }
    }

    #[inline]
    pub fn as_int32(&self) -> &[i32; 4] {
        unsafe { mem::transmute(&self.0) }
    }

    #[inline]
    pub fn as_uint32(&self) -> &[u32; 4] {
        &self.0
    }

    #[inline]
    pub fn float32(val: [f32; 4]) -> VkClearColorValue {
        VkClearColorValue(unsafe { mem::transmute(val) })
    }
    #[inline]
    pub fn int32(val: [i32; 4]) -> VkClearColorValue {
        VkClearColorValue(unsafe { mem::transmute(val) })
    }
    #[inline]
    pub fn uint32(val: [u32; 4]) -> VkClearColorValue {
        VkClearColorValue(val)
    }
}
