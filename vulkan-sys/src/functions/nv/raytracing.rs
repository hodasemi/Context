use crate::load_function_ptrs;
use crate::prelude::*;

use std::mem::transmute;
use std::os::raw::c_void;

load_function_ptrs!(NVRayTracingFunctions, {
    vkCreateAccelerationStructureNV(
        device: VkDevice,
        pCreateInfo: *const VkAccelerationStructureCreateInfoNV,
        pAllocator: *const VkAllocationCallbacks,
        pAccelerationStructure: *mut VkAccelerationStructureNV
    ) -> VkResult,

    vkDestroyAccelerationStructureNV(
        device: VkDevice,
        accelerationStructure: VkAccelerationStructureNV,
        pAllocator: *const VkAllocationCallbacks
    ) -> VkResult,

    vkGetAccelerationStructureMemoryRequirementsNV(
        device: VkDevice,
        pInfo: *const VkAccelerationStructureMemoryRequirementsInfoNV,
        pMemoryRequirements: *mut VkMemoryRequirements2KHR
    ) -> (),

    vkBindAccelerationStructureMemoryNV(
        device: VkDevice,
        bindInfoCount: u32,
        pBindInfos: *const VkBindAccelerationStructureMemoryInfoNV
    ) -> VkResult,

    vkCmdBuildAccelerationStructureNV(
        commandBuffer: VkCommandBuffer,
        pInfo: *const VkAccelerationStructureInfoNV,
        instanceData: VkBuffer,
        instanceOffset: VkDeviceSize,
        update: VkBool32,
        dst: VkAccelerationStructureNV,
        src: VkAccelerationStructureNV,
        scratch: VkBuffer,
        scratchOffset: VkDeviceSize
    ) -> (),

    vkCmdCopyAccelerationStructureNV(
        commandBuffer: VkCommandBuffer,
        dst: VkAccelerationStructureNV,
        src: VkAccelerationStructureNV,
        mode: VkCopyAccelerationStructureModeNV
    ) -> (),

    vkCmdTraceRaysNV(
        commandBuffer: VkCommandBuffer,
        raygenShaderBindingTableBuffer: VkBuffer,
        raygenShaderBindingOffset: VkDeviceSize,
        missShaderBindingTableBuffer: VkBuffer,
        missShaderBindingOffset: VkDeviceSize,
        missShaderBindingStride: VkDeviceSize,
        hitShaderBindingTableBuffer: VkBuffer,
        hitShaderBindingOffset: VkDeviceSize,
        hitShaderBindingStride: VkDeviceSize,
        callableShaderBindingTableBuffer: VkBuffer,
        callableShaderBindingOffset: VkDeviceSize,
        callableShaderBindingStride: VkDeviceSize,
        width: u32,
        height: u32,
        depth: u32
    ) -> (),

    vkCreateRayTracingPipelinesNV(
        device: VkDevice,
        pipelineCache: VkPipelineCache,
        createInfoCount: u32,
        pCreateInfos: *const VkRayTracingPipelineCreateInfoNV,
        pAllocator: *const VkAllocationCallbacks,
        pPipelines: *mut VkPipeline
    ) -> VkResult,

    vkGetRayTracingShaderGroupHandlesNV(
        device: VkDevice,
        pipeline: VkPipeline,
        firstGroup: u32,
        groupCount: u32,
        dataSize: usize,
        pData: *mut c_void
    ) -> VkResult,

    vkGetAccelerationStructureHandleNV(
        device: VkDevice,
        accelerationStructure: VkAccelerationStructureNV,
        dataSize: usize,
        pData: *mut c_void
    ) -> VkResult,

    vkCmdWriteAccelerationStructurePropertiesNV(
        commandBuffer: VkCommandBuffer,
        accelerationStructureCount: u32,
        pAccelerationStructures: *const VkAccelerationStructureNV,
        queryType: VkQueryType,
        queryPool: VkQueryPool,
        firstQuery: u32
    ) -> (),

    vkCompileDeferredNV(
        device: VkDevice,
        pipeline: VkPipeline,
        shader: u32
    ) -> VkResult,
});
