use crate::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub struct VkMVKPhysicalDeviceMetalFeatures {
    pub depthClipMode: VkBool32,
    pub indirectDrawing: VkBool32,
    pub baseVertexInstanceDrawing: VkBool32,
    pub maxVertexBufferCount: u32,
    pub maxFragmentBufferCount: u32,
    pub bufferAlignment: VkDeviceSize,
    pub pushConstantsAlignment: VkDeviceSize,
}
