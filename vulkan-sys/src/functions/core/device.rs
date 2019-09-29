use crate::load_function_ptrs;
use crate::prelude::*;

use std::os::raw::c_char;
use std::os::raw::c_void;

load_function_ptrs!(DeviceFunctions, {
    vkDestroyDevice(device: VkDevice, pAllocator: *const VkAllocationCallbacks) -> (),

    vkGetDeviceQueue(
        device: VkDevice,
        queueFamilyIndex: u32,
        queueIndex: u32,
        pQueue: *mut VkQueue
    ) -> (),

    vkWaitForFences(
        device: VkDevice,
        fenceCount: u32,
        pFences: *const VkFence,
        waitAll: VkBool32,
        timeout: u64
    ) -> VkResult,

    vkGetQueryPoolResults(
        device: VkDevice,
        queryPool: VkQueryPool,
        firstQuery: u32,
        queryCount: u32,
        dataSize: usize,
        pData: *mut c_void,
        stride: VkDeviceSize,
        flags: VkQueryResultFlagBits
    ) -> VkResult,

    // Queue
    vkQueueSubmit(
        queue: VkQueue,
        submitCount: u32,
        pSubmits: *const VkSubmitInfo,
        fence: VkFence
    ) -> VkResult,

    vkQueueWaitIdle(queue: VkQueue) -> VkResult,

    // buffer
    vkCreateBuffer(
        device: VkDevice,
        pCreateInfo: *const VkBufferCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pBuffer: *mut VkBuffer
    ) -> VkResult,

    vkDestroyBuffer(
        device: VkDevice,
        buffer: VkBuffer,
        pAllocator: *const VkAllocationCallbacks
    ) -> (),

    vkGetBufferMemoryRequirements(
        device: VkDevice,
        buffer: VkBuffer,
        pMemoryRequirements: *mut VkMemoryRequirements
    ) -> (),

    // memory
    vkAllocateMemory(
        device: VkDevice,
        pAllocateInfo: *const VkMemoryAllocateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pMemory: *mut VkDeviceMemory
    ) -> VkResult,

    vkFreeMemory(
        device: VkDevice,
        memory: VkDeviceMemory,
        pAllocator: *const VkAllocationCallbacks
    ) -> (),

    vkMapMemory(
        device: VkDevice,
        memory: VkDeviceMemory,
        offset: VkDeviceSize,
        size: VkDeviceSize,
        flags: VkMemoryMapFlags,
        ppData: *mut *mut c_void
    ) -> VkResult,

    vkUnmapMemory(device: VkDevice, memory: VkDeviceMemory) -> (),

    vkBindBufferMemory(
        device: VkDevice,
        buffer: VkBuffer,
        memory: VkDeviceMemory,
        memoryOffset: VkDeviceSize
    ) -> VkResult,

    // render pass
    vkCreateRenderPass(
        device: VkDevice,
        pCreateInfo: *const VkRenderPassCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pRenderPass: *mut VkRenderPass
    ) -> VkResult,

    vkDestroyRenderPass(
        device: VkDevice,
        renderPass: VkRenderPass,
        pAllocator: *const VkAllocationCallbacks
    ) -> (),

    // image
    vkCreateImage(
        device: VkDevice,
        pCreateInfo: *const VkImageCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pImage: *mut VkImage
    ) -> VkResult,

    vkDestroyImage(
        device: VkDevice,
        image: VkImage,
        pAllocator: *const VkAllocationCallbacks
    ) -> (),

    vkGetImageSubresourceLayout(
        device: VkDevice,
        image: VkImage,
        pSubresource: *const VkImageSubresource,
        pLayout: *mut VkSubresourceLayout
    ) -> (),

    vkCreateImageView(
        device: VkDevice,
        pCreateInfo: *const VkImageViewCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pView: *mut VkImageView
    ) -> VkResult,

    vkDestroyImageView(
        device: VkDevice,
        imageView: VkImageView,
        pAllocator: *const VkAllocationCallbacks
    ) -> (),

    vkGetImageMemoryRequirements(
        device: VkDevice,
        image: VkImage,
        pMemoryRequirements: *mut VkMemoryRequirements
    ) -> (),

    vkGetImageSparseMemoryRequirements(
        device: VkDevice,
        image: VkImage,
        pSparseMemoryRequirementCount: *mut u32,
        pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements
    ) -> (),

    vkBindImageMemory(
        device: VkDevice,
        image: VkImage,
        memory: VkDeviceMemory,
        memoryOffset: VkDeviceSize
    ) -> VkResult,

    vkCreateSampler(
        device: VkDevice,
        pCreateInfo: *const VkSamplerCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pSampler: *mut VkSampler
    ) -> VkResult,

    vkDestroySampler(
        device: VkDevice,
        sampler: VkSampler,
        pAllocator: *const VkAllocationCallbacks
    ) -> (),

    // buffer view
    vkCreateBufferView(
        device: VkDevice,
        pCreateInfo: *const VkBufferViewCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pView: *mut VkBufferView
    ) -> VkResult,

    vkDestroyBufferView(
        device: VkDevice,
        bufferView: VkBufferView,
        pAllocator: *const VkAllocationCallbacks
    ) -> (),

    // fence
    vkCreateFence(
        device: VkDevice,
        pCreateInfo: *const VkFenceCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pFence: *mut VkFence
    ) -> VkResult,

    vkDestroyFence(
        device: VkDevice,
        fence: VkFence,
        pAllocator: *const VkAllocationCallbacks
    ) -> (),

    vkResetFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult,

    // semaphore
    vkCreateSemaphore(
        device: VkDevice,
        pCreateInfo: *const VkSemaphoreCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pSemaphore: *mut VkSemaphore
    ) -> VkResult,

    vkDestroySemaphore(
        device: VkDevice,
        semaphore: VkSemaphore,
        pAllocator: *const VkAllocationCallbacks
    ) -> (),

    // shadermodule
    vkCreateShaderModule(
        device: VkDevice,
        pCreateInfo: *const VkShaderModuleCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pShaderModule: *mut VkShaderModule
    ) -> VkResult,

    vkDestroyShaderModule(
        device: VkDevice,
        shaderModule: VkShaderModule,
        pAllocator: *const VkAllocationCallbacks
    ) -> (),

    // descriptor pool
    vkCreateDescriptorPool(
        device: VkDevice,
        pCreateInfo: *const VkDescriptorPoolCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pDescriptorPool: *mut VkDescriptorPool
    ) -> VkResult,

    vkDestroyDescriptorPool(
        device: VkDevice,
        descriptorPool: VkDescriptorPool,
        pAllocator: *const VkAllocationCallbacks
    ) -> (),

    vkResetDescriptorPool(
        device: VkDevice,
        descriptorPool: VkDescriptorPool,
        flags: VkDescriptorPoolResetFlags
    ) -> VkResult,

    // descriptor set layout
    vkCreateDescriptorSetLayout(
        device: VkDevice,
        pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pSetLayout: *mut VkDescriptorSetLayout
    ) -> VkResult,

    vkDestroyDescriptorSetLayout(
        device: VkDevice,
        descriptorSetLayout: VkDescriptorSetLayout,
        pAllocator: *const VkAllocationCallbacks
    ) -> (),

    // descriptor set
    vkAllocateDescriptorSets(
        device: VkDevice,
        pAllocateInfo: *const VkDescriptorSetAllocateInfo<'_>,
        pDescriptorSets: *mut VkDescriptorSet
    ) -> VkResult,

    vkFreeDescriptorSets(
        device: VkDevice,
        descriptorPool: VkDescriptorPool,
        descriptorSetCount: u32,
        pDescriptorSets: *const VkDescriptorSet
    ) -> VkResult,

    vkUpdateDescriptorSets(
        device: VkDevice,
        descriptorWriteCount: u32,
        pDescriptorWrites: *const VkWriteDescriptorSet,
        descriptorCopyCount: u32,
        pDescriptorCopies: *const VkCopyDescriptorSet
    ) -> (),

    // event
    vkCreateEvent(
        device: VkDevice,
        pCreateInfo: *const VkEventCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pEvent: *mut VkEvent
    ) -> VkResult,

    vkDestroyEvent(
        device: VkDevice,
        event: VkEvent,
        pAllocator: *const VkAllocationCallbacks
    ) ->(),

    vkGetEventStatus(device: VkDevice, event: VkEvent) -> VkResult,

    vkSetEvent(device: VkDevice, event: VkEvent) -> VkResult,

    vkResetEvent(device: VkDevice, event: VkEvent) -> VkResult,

    // command pool
    vkCreateCommandPool(
        device: VkDevice,
        pCreateInfo: *const VkCommandPoolCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pCommandPool: *mut VkCommandPool
    ) -> VkResult,

    vkDestroyCommandPool(
        device: VkDevice,
        commandPool: VkCommandPool,
        pAllocator: *const VkAllocationCallbacks
    ) -> (),

    vkResetCommandPool(
        device: VkDevice,
        commandPool: VkCommandPool,
        flags: VkCommandPoolResetFlags
    ) -> VkResult,

    vkTrimCommandPool(
        device: VkDevice,
        commandPool: VkCommandPool,
        flags: VkCommandPoolTrimFlags
    ) -> (),

    // framebuffer
    vkCreateFramebuffer(
        device: VkDevice,
        pCreateInfo: *const VkFramebufferCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pFramebuffer: *mut VkFramebuffer
    ) -> VkResult,

    vkDestroyFramebuffer(
        device: VkDevice,
        framebuffer: VkFramebuffer,
        pAllocator: *const VkAllocationCallbacks
    ) -> (),

    // command buffer
    vkAllocateCommandBuffers(
        device: VkDevice,
        pAllocateInfo: *const VkCommandBufferAllocateInfo,
        pCommandBuffers: *mut VkCommandBuffer
    ) -> VkResult,

    vkFreeCommandBuffers(
        device: VkDevice,
        commandPool: VkCommandPool,
        commandBufferCount: u32,
        pCommandBuffers: *const VkCommandBuffer
    ) -> (),

    vkBeginCommandBuffer(
        commandBuffer: VkCommandBuffer,
        pBeginInfo: *const VkCommandBufferBeginInfo
    ) -> VkResult,

    vkEndCommandBuffer(commandBuffer: VkCommandBuffer) -> VkResult,

    vkResetCommandBuffer(
        commandBuffer: VkCommandBuffer,
        flags: VkCommandBufferResetFlagBits
    ) -> VkResult,

    vkCmdBindPipeline(
        commandBuffer: VkCommandBuffer,
        pipelineBindPoint: VkPipelineBindPoint,
        pipeline: VkPipeline
    ) -> (),

    vkCmdSetViewport(
        commandBuffer: VkCommandBuffer,
        firstViewport: u32,
        viewportCount: u32,
        pViewports: *const VkViewport
    ) -> (),

    vkCmdSetScissor(
        commandBuffer: VkCommandBuffer,
        firstScissor: u32,
        scissorCount: u32,
        pScissors: *const VkRect2D
    ) -> (),

    vkCmdSetLineWidth(commandBuffer: VkCommandBuffer, lineWidth: f32) -> (),

    vkCmdSetDepthBias(
        commandBuffer: VkCommandBuffer,
        depthBiasConstantFactor: f32,
        depthBiasClamp: f32,
        depthBiasSlopeFactor: f32
    ) -> (),

    vkCmdSetBlendConstants(commandBuffer: VkCommandBuffer, blendConstants: [f32; 4]) -> (),

    vkCmdSetDepthBounds(
        commandBuffer: VkCommandBuffer,
        minDepthBounds: f32,
        maxDepthBounds: f32
    ) -> (),

    vkCmdSetStencilCompareMask(
        commandBuffer: VkCommandBuffer,
        faceMask: VkStencilFaceFlags,
        compareMask: u32
    ) -> (),

    vkCmdSetStencilWriteMask(
        commandBuffer: VkCommandBuffer,
        faceMask: VkStencilFaceFlags,
        writeMask: u32
    ) -> (),

    vkCmdSetStencilReference(
        commandBuffer: VkCommandBuffer,
        faceMask: VkStencilFaceFlags,
        reference: u32
    ) -> (),

    vkCmdBindDescriptorSets(
        commandBuffer: VkCommandBuffer,
        pipelineBindPoint: VkPipelineBindPoint,
        layout: VkPipelineLayout,
        firstSet: u32,
        descriptorSetCount: u32,
        pDescriptorSets: *const VkDescriptorSet,
        dynamicOffsetCount: u32,
        pDynamicOffsets: *const u32
    ) -> (),

    vkCmdBindIndexBuffer(
        commandBuffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        indexType: VkIndexType
    ) -> (),

    vkCmdBindVertexBuffers(
        commandBuffer: VkCommandBuffer,
        firstBinding: u32,
        bindingCount: u32,
        pBuffers: *const VkBuffer,
        pOffsets: *const VkDeviceSize
    ) -> (),

    vkCmdDraw(
        commandBuffer: VkCommandBuffer,
        vertexCount: u32,
        instanceCount: u32,
        firstVertex: u32,
        firstInstance: u32
    ) -> (),

    vkCmdDrawIndexed(
        commandBuffer: VkCommandBuffer,
        indexCount: u32,
        instanceCount: u32,
        firstIndex: u32,
        vertexOffset: i32,
        firstInstance: u32
    ) -> (),

    vkCmdDrawIndirect(
        commandBuffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        drawCount: u32,
        stride: u32
    ) -> (),

    vkCmdDrawIndexedIndirect(
        commandBuffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        drawCount: u32,
        stride: u32
    ) -> (),

    vkCmdDispatch(commandBuffer: VkCommandBuffer, x: u32, y: u32, z: u32) -> (),

    vkCmdDispatchIndirect(
        commandBuffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize
    ) -> (),

    vkCmdCopyBuffer(
        commandBuffer: VkCommandBuffer,
        srcBuffer: VkBuffer,
        dstBuffer: VkBuffer,
        regionCount: u32,
        pRegions: *const VkBufferCopy
    ) -> (),

    vkCmdCopyImage(
        commandBuffer: VkCommandBuffer,
        srcImage: VkImage,
        srcImageLayout: VkImageLayout,
        dstImage: VkImage,
        dstImageLayout: VkImageLayout,
        regionCount: u32,
        pRegions: *const VkImageCopy
    ) -> (),

    vkCmdBlitImage(
        commandBuffer: VkCommandBuffer,
        srcImage: VkImage,
        srcImageLayout: VkImageLayout,
        dstImage: VkImage,
        dstImageLayout: VkImageLayout,
        regionCount: u32,
        pRegions: *const VkImageBlit,
        filter: VkFilter
    ) -> (),

    vkCmdCopyBufferToImage(
        commandBuffer: VkCommandBuffer,
        srcBuffer: VkBuffer,
        dstImage: VkImage,
        dstImageLayout: VkImageLayout,
        regionCount: u32,
        pRegions: *const VkBufferImageCopy
    ) -> (),

    vkCmdCopyImageToBuffer(
        commandBuffer: VkCommandBuffer,
        srcImage: VkImage,
        srcImageLayout: VkImageLayout,
        dstBuffer: VkBuffer,
        regionCount: u32,
        pRegions: *const VkBufferImageCopy
    ) -> (),

    vkCmdUpdateBuffer(
        commandBuffer: VkCommandBuffer,
        dstBuffer: VkBuffer,
        dstOffset: VkDeviceSize,
        dataSize: VkDeviceSize,
        pData: *const u32
    ) -> (),

    vkCmdFillBuffer(
        commandBuffer: VkCommandBuffer,
        dstBuffer: VkBuffer,
        dstOffset: VkDeviceSize,
        size: VkDeviceSize,
        data: u32
    ) -> (),

    vkCmdClearColorImage(
        commandBuffer: VkCommandBuffer,
        image: VkImage,
        imageLayout: VkImageLayout,
        pColor: *const VkClearColorValue,
        rangeCount: u32,
        pRanges: *const VkImageSubresourceRange
    ) -> (),

    vkCmdClearDepthStencilImage(
        commandBuffer: VkCommandBuffer,
        image: VkImage,
        imageLayout: VkImageLayout,
        pDepthStencil: *const VkClearDepthStencilValue,
        rangeCount: u32,
        pRanges: *const VkImageSubresourceRange
    ) -> (),

    vkCmdClearAttachments(
        commandBuffer: VkCommandBuffer,
        attachmentCount: u32,
        pAttachments: *const VkClearAttachment,
        VkRectCount: u32,
        pRects: *const VkClearRect
    ) -> (),

    vkCmdResolveImage(
        commandBuffer: VkCommandBuffer,
        srcImage: VkImage,
        srcImageLayout: VkImageLayout,
        dstImage: VkImage,
        dstImageLayout: VkImageLayout,
        regionCount: u32,
        pRegions: *const VkImageResolve
    ) -> (),

    vkCmdSetEvent(
        commandBuffer: VkCommandBuffer,
        event: VkEvent,
        stageMask: VkPipelineStageFlags
    ) -> (),

    vkCmdResetEvent(
        commandBuffer: VkCommandBuffer,
        event: VkEvent,
        stageMask: VkPipelineStageFlags
    ) -> (),

    vkCmdWaitEvents(
        commandBuffer: VkCommandBuffer,
        eventCount: u32,
        pEvents: *const VkEvent,
        srcStageMask: VkPipelineStageFlags,
        dstStageMask: VkPipelineStageFlags,
        memoryBarrierCount: u32,
        pMemoryBarriers: *const VkMemoryBarrier,
        bufferMemoryBarrierCount: u32,
        pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
        imageMemoryBarrierCount: u32,
        pImageMemoryBarriers: *const VkImageMemoryBarrier
    ) -> (),

    vkCmdPipelineBarrier(
        commandBuffer: VkCommandBuffer,
        srcStageMask: VkPipelineStageFlagBits,
        dstStageMask: VkPipelineStageFlagBits,
        dependencyFlags: VkDependencyFlagBits,
        memoryBarrierCount: u32,
        pMemoryBarriers: *const VkMemoryBarrier,
        bufferMemoryBarrierCount: u32,
        pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
        imageMemoryBarrierCount: u32,
        pImageMemoryBarriers: *const VkImageMemoryBarrier
    ) -> (),

    vkCmdBeginQuery(
        commandBuffer: VkCommandBuffer,
        queryPool: VkQueryPool,
        query: u32,
        flags: VkQueryControlFlagBits
    ) -> (),

    vkCmdEndQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32) -> (),

    vkCmdResetQueryPool(
        commandBuffer: VkCommandBuffer,
        queryPool: VkQueryPool,
        firstQuery: u32,
        queryCount: u32
    ) -> (),

    vkCmdWriteTimestamp(
        commandBuffer: VkCommandBuffer,
        pipelineStage: VkPipelineStageFlagBits,
        queryPool: VkQueryPool,
        query: u32
    ) -> (),

    vkCmdCopyQueryPoolResults(
        commandBuffer: VkCommandBuffer,
        queryPool: VkQueryPool,
        firstQuery: u32,
        queryCount: u32,
        dstBuffer: VkBuffer,
        dstOffset: VkDeviceSize,
        stride: VkDeviceSize,
        flags: VkQueryResultFlags
    ) -> (),

    vkCmdPushConstants(
        commandBuffer: VkCommandBuffer,
        layout: VkPipelineLayout,
        stageFlags: VkShaderStageFlagBits,
        offset: u32,
        size: u32,
        pValues: *const c_void
    ) -> (),

    vkCmdBeginRenderPass(
        commandBuffer: VkCommandBuffer,
        pRenderPassBegin: *const VkRenderPassBeginInfo,
        contents: VkSubpassContents
    ) -> (),

    vkCmdNextSubpass(commandBuffer: VkCommandBuffer, contents: VkSubpassContents) -> (),

    vkCmdEndRenderPass(commandBuffer: VkCommandBuffer) -> (),

    vkCmdExecuteCommands(
        commandBuffer: VkCommandBuffer,
        commandBufferCount: u32,
        pCommandBuffers: *const VkCommandBuffer
    ) -> (),

    // query pool
    vkCreateQueryPool(
        device: VkDevice,
        pCreateInfo: *const VkQueryPoolCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pQueryPool: *mut VkQueryPool
    ) -> VkResult,

    vkDestroyQueryPool(
        device: VkDevice,
        queryPool: VkQueryPool,
        pAllocator: *const VkAllocationCallbacks
    ) -> (),

    // pipeline cache
    vkCreatePipelineCache(
        device: VkDevice,
        pCreateInfo: *const VkPipelineCacheCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pPipelineCache: *mut VkPipelineCache
    ) -> VkResult,

    vkDestroyPipelineCache(
        device: VkDevice,
        pipelineCache: VkPipelineCache,
        pAllocator: *const VkAllocationCallbacks
    ) -> (),

    vkGetPipelineCacheData(
        device: VkDevice,
        pipelineCache: VkPipelineCache,
        pDataSize: *mut usize,
        pData: *mut c_void
    ) -> VkResult,

    vkMergePipelineCaches(
        device: VkDevice,
        dstCache: VkPipelineCache,
        srcCacheCount: u32,
        pSrcCaches: *const VkPipelineCache
    ) -> VkResult,

    // pipeline layout
    vkCreatePipelineLayout(
        device: VkDevice,
        pCreateInfo: *const VkPipelineLayoutCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pPipelineLayout: *mut VkPipelineLayout
    ) -> VkResult,

    vkDestroyPipelineLayout(
        device: VkDevice,
        pipelineLayout: VkPipelineLayout,
        pAllocator: *const VkAllocationCallbacks
    ) -> (),

    // pipeline
    vkCreateGraphicsPipelines(
        device: VkDevice,
        pipelineCache: VkPipelineCache,
        createInfoCount: u32,
        pCreateInfos: *const VkGraphicsPipelineCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pPipelines: *mut VkPipeline
    ) -> VkResult,

    vkCreateComputePipelines(
        device: VkDevice,
        pipelineCache: VkPipelineCache,
        createInfoCount: u32,
        pCreateInfos: *const VkComputePipelineCreateInfo,
        pAllocator: *const VkAllocationCallbacks,
        pPipelines: *mut VkPipeline
    ) -> VkResult,

    vkDestroyPipeline(
        device: VkDevice,
        pipeline: VkPipeline,
        pAllocator: *const VkAllocationCallbacks
    ) -> (),
});
