use crate::{SetupU64Conv, SetupUSizeConv};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkBuffer(u64);
SetupU64Conv!(VkBuffer);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkBufferView(u64);
SetupU64Conv!(VkBufferView);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkCommandBuffer(usize);
SetupUSizeConv!(VkCommandBuffer);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkCommandPool(u64);
SetupU64Conv!(VkCommandPool);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkDescriptorPool(u64);
SetupU64Conv!(VkDescriptorPool);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkDescriptorSet(u64);
SetupU64Conv!(VkDescriptorSet);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkDescriptorSetLayout(u64);
SetupU64Conv!(VkDescriptorSetLayout);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkDevice(usize);
SetupUSizeConv!(VkDevice);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkDeviceMemory(u64);
SetupU64Conv!(VkDeviceMemory);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkEvent(u64);
SetupU64Conv!(VkEvent);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkFence(u64);
SetupU64Conv!(VkFence);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkFramebuffer(u64);
SetupU64Conv!(VkFramebuffer);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkImage(u64);
SetupU64Conv!(VkImage);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkImageView(u64);
SetupU64Conv!(VkImageView);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkInstance(usize);
SetupUSizeConv!(VkInstance);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkPhysicalDevice(usize);
SetupUSizeConv!(VkPhysicalDevice);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkPipeline(u64);
SetupU64Conv!(VkPipeline);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkPipelineCache(u64);
SetupU64Conv!(VkPipelineCache);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkPipelineLayout(u64);
SetupU64Conv!(VkPipelineLayout);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkQueryPool(u64);
SetupU64Conv!(VkQueryPool);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkQueue(usize);
SetupUSizeConv!(VkQueue);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkRenderPass(u64);
SetupU64Conv!(VkRenderPass);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkSampler(u64);
SetupU64Conv!(VkSampler);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkSemaphore(u64);
SetupU64Conv!(VkSemaphore);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VkShaderModule(u64);
SetupU64Conv!(VkShaderModule);
