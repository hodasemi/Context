
use crate::prelude::*;
use crate::SetupUSizeConv;

use core::ffi::c_void;
use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkCommandBuffer(usize);
SetupUSizeConv!(VkCommandBuffer);

/*
impl VkCommandBuffer {
    pub fn allocate(
        device: VkDevice,
        command_buffer_allocate_info: &VkCommandBufferAllocateInfo,
    ) -> Result<Vec<VkCommandBuffer>, VkResult> {
        unsafe {
            let count = command_buffer_allocate_info.commandBufferCount as usize;

            let mut command_buffers = Vec::with_capacity(count);
            command_buffers.set_len(count);

            let result = vkAllocateCommandBuffers(
                device,
                command_buffer_allocate_info,
                command_buffers.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(command_buffers)
            } else {
                Err(result)
            }
        }
    }

    pub fn free(
        device: VkDevice,
        command_pool: VkCommandPool,
        command_buffers: &[VkCommandBuffer],
    ) {
        unsafe {
            vkFreeCommandBuffers(
                device,
                command_pool,
                command_buffers.len() as u32,
                command_buffers.as_ptr(),
            )
        }
    }

    pub fn begin(&self, begin_info: &VkCommandBufferBeginInfo) -> Result<(), VkResult> {
        unsafe {
            let result = vkBeginCommandBuffer(*self, begin_info);

            if result == VK_SUCCESS {
                Ok(())
            } else {
                Err(result)
            }
        }
    }

    pub fn end(&self) -> Result<(), VkResult> {
        unsafe {
            let result = vkEndCommandBuffer(*self);

            if result == VK_SUCCESS {
                Ok(())
            } else {
                Err(result)
            }
        }
    }

    pub fn reset<T>(&self, flags: T) -> Result<(), VkResult>
    where
        T: Into<VkCommandBufferResetFlags>,
    {
        unsafe {
            let result = vkResetCommandBuffer(*self, flags.into());

            if result == VK_SUCCESS {
                Ok(())
            } else {
                Err(result)
            }
        }
    }

    pub fn bind_pipeline(&self, pipeline_bind_point: VkPipelineBindPoint, pipeline: VkPipeline) {
        unsafe {
            vkCmdBindPipeline(*self, pipeline_bind_point, pipeline);
        }
    }

    pub fn set_viewport(&self, first: u32, viewports: &[VkViewport]) {
        unsafe { vkCmdSetViewport(*self, first, viewports.len() as u32, viewports.as_ptr()) }
    }

    pub fn set_scissor(&self, first: u32, scissors: &[VkRect2D]) {
        unsafe { vkCmdSetScissor(*self, first, scissors.len() as u32, scissors.as_ptr()) }
    }

    pub fn set_depth_bias(
        &self,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
    ) {
        unsafe {
            vkCmdSetDepthBias(
                *self,
                depth_bias_constant_factor,
                depth_bias_clamp,
                depth_bias_slope_factor,
            )
        }
    }

    pub fn bind_descriptor_sets(
        &self,
        pipeline_bind_point: VkPipelineBindPoint,
        pipeline_layout: VkPipelineLayout,
        first_set: u32,
        descriptor_sets: &[VkDescriptorSet],
        dynamic_offsets: &[u32],
    ) {
        unsafe {
            vkCmdBindDescriptorSets(
                *self,
                pipeline_bind_point,
                pipeline_layout,
                first_set,
                descriptor_sets.len() as u32,
                descriptor_sets.as_ptr(),
                dynamic_offsets.len() as u32,
                dynamic_offsets.as_ptr(),
            )
        }
    }

    pub fn bind_index_buffer(
        &self,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        index_type: VkIndexType,
    ) {
        unsafe { vkCmdBindIndexBuffer(*self, buffer, offset, index_type) }
    }

    pub fn bind_vertex_buffers(
        &self,
        first_binding: u32,
        buffers: &[VkBuffer],
        offsets: &[VkDeviceSize],
    ) {
        // sanity check
        debug_assert!(buffers.len() == offsets.len());

        unsafe {
            vkCmdBindVertexBuffers(
                *self,
                first_binding,
                buffers.len() as u32,
                buffers.as_ptr(),
                offsets.as_ptr(),
            )
        }
    }

    pub fn draw(
        &self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        unsafe {
            vkCmdDraw(
                *self,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            )
        }
    }

    pub fn draw_indexed(
        &self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        unsafe {
            vkCmdDrawIndexed(
                *self,
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            );
        }
    }

    pub fn dispatch(&self, x: u32, y: u32, z: u32) {
        unsafe { vkCmdDispatch(*self, x, y, z) }
    }

    pub fn begin_render_pass(
        &self,
        render_pass_begin: &VkRenderPassBeginInfo,
        contents: VkSubpassContents,
    ) {
        unsafe { vkCmdBeginRenderPass(*self, render_pass_begin, contents) }
    }

    pub fn next_subpass(&self, contents: VkSubpassContents) {
        unsafe { vkCmdNextSubpass(*self, contents) }
    }

    pub fn end_render_pass(&self) {
        unsafe { vkCmdEndRenderPass(*self) }
    }

    pub fn execute_commands(&self, command_buffers: &[VkCommandBuffer]) {
        unsafe {
            vkCmdExecuteCommands(
                *self,
                command_buffers.len() as u32,
                command_buffers.as_ptr(),
            )
        }
    }

    pub fn pipeline_barrier<T, U, V>(
        &self,
        src_stage_mask: T,
        dst_stage_mask: U,
        dependency_flags: V,
        memory_barriers: &[VkMemoryBarrier],
        buffer_memory_barriers: &[VkBufferMemoryBarrier],
        image_memory_barriers: &[VkImageMemoryBarrier],
    ) where
        T: Into<VkPipelineStageFlags>,
        U: Into<VkPipelineStageFlags>,
        V: Into<VkDependencyFlags>,
    {
        unsafe {
            vkCmdPipelineBarrier(
                *self,
                src_stage_mask.into(),
                dst_stage_mask.into(),
                dependency_flags.into(),
                memory_barriers.len() as u32,
                memory_barriers.as_ptr(),
                buffer_memory_barriers.len() as u32,
                buffer_memory_barriers.as_ptr(),
                image_memory_barriers.len() as u32,
                image_memory_barriers.as_ptr(),
            )
        }
    }

    pub fn copy_buffer(
        &self,
        src_buffer: VkBuffer,
        dst_buffer: VkBuffer,
        regions: &[VkBufferCopy],
    ) {
        unsafe {
            vkCmdCopyBuffer(
                *self,
                src_buffer,
                dst_buffer,
                regions.len() as u32,
                regions.as_ptr(),
            )
        }
    }

    pub fn copy_image(
        &self,
        src_image: VkImage,
        src_image_layout: VkImageLayout,
        dst_image: VkImage,
        dst_image_layout: VkImageLayout,
        regions: &[VkImageCopy],
    ) {
        unsafe {
            vkCmdCopyImage(
                *self,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                regions.len() as u32,
                regions.as_ptr(),
            )
        }
    }

    pub fn blit_image(
        &self,
        src_image: VkImage,
        src_image_layout: VkImageLayout,
        dst_image: VkImage,
        dst_image_layout: VkImageLayout,
        regions: &[VkImageBlit],
        filter: VkFilter,
    ) {
        unsafe {
            vkCmdBlitImage(
                *self,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                regions.len() as u32,
                regions.as_ptr(),
                filter,
            )
        }
    }

    pub fn copy_buffer_to_image(
        &self,
        src_buffer: VkBuffer,
        dst_image: VkImage,
        dst_image_layout: VkImageLayout,
        regions: &[VkBufferImageCopy],
    ) {
        unsafe {
            vkCmdCopyBufferToImage(
                *self,
                src_buffer,
                dst_image,
                dst_image_layout,
                regions.len() as u32,
                regions.as_ptr(),
            )
        }
    }

    pub fn copy_image_to_buffer(
        &self,
        src_image: VkImage,
        src_image_layout: VkImageLayout,
        dstBuffer: VkBuffer,
        regions: &[VkBufferImageCopy],
    ) {
        unsafe {
            vkCmdCopyImageToBuffer(
                *self,
                src_image,
                src_image_layout,
                dstBuffer,
                regions.len() as u32,
                regions.as_ptr(),
            )
        }
    }

    pub fn push_constants<T, U>(
        &self,
        pipeline_layout: VkPipelineLayout,
        stage_flags: U,
        offset: u32,
        data: &T,
    ) where
        U: Into<VkShaderStageFlags>,
    {
        unsafe {
            vkCmdPushConstants(
                *self,
                pipeline_layout,
                stage_flags.into(),
                offset,
                mem::size_of::<T>() as u32,
                data as *const T as *const c_void,
            )
        }
    }

    pub fn begin_query<T>(&self, query_pool: VkQueryPool, query: u32, flags: T)
    where
        T: Into<VkQueryControlFlagBits>,
    {
        unsafe { vkCmdBeginQuery(*self, query_pool, query, flags.into()) }
    }

    pub fn end_query(&self, query_pool: VkQueryPool, query: u32) {
        unsafe { vkCmdEndQuery(*self, query_pool, query) }
    }

    pub fn reset_query_pool(&self, query_pool: VkQueryPool, first_query: u32, query_count: u32) {
        unsafe { vkCmdResetQueryPool(*self, query_pool, first_query, query_count) }
    }

    pub fn write_timestamp<T>(&self, pipeline_stage: T, query_pool: VkQueryPool, query: u32)
    where
        T: Into<VkPipelineStageFlagBits>,
    {
        unsafe { vkCmdWriteTimestamp(*self, pipeline_stage.into(), query_pool, query) }
    }

    pub fn build_acceleration_structure<T>(
        &self,
        info: &VkAccelerationStructureInfoNV,
        instance_data: VkBuffer,
        instance_offset: VkDeviceSize,
        update: T,
        dst: VkAccelerationStructureNV,
        src: VkAccelerationStructureNV,
        scratch: VkBuffer,
        scratch_offset: VkDeviceSize,
    ) -> Result<(), VkResult>
    where
        T: Into<VkBool32>,
    {
        unsafe {
            match vkCmdBuildAccelerationStructureNV {
                Some(pfn) => {
                    pfn(
                        *self,
                        info,
                        instance_data,
                        instance_offset,
                        update.into(),
                        dst,
                        src,
                        scratch,
                        scratch_offset,
                    );

                    Ok(())
                }
                None => return Err(VK_ERROR_EXTENSION_NOT_PRESENT),
            }
        }
    }

    pub fn copy_acceleration_structure(
        &self,
        dst: VkAccelerationStructureNV,
        src: VkAccelerationStructureNV,
        mode: VkCopyAccelerationStructureModeNV,
    ) -> Result<(), VkResult> {
        unsafe {
            match vkCopyAccelerationStructureNV {
                Some(pfn) => {
                    pfn(*self, dst, src, mode);

                    Ok(())
                }
                None => return Err(VK_ERROR_EXTENSION_NOT_PRESENT),
            }
        }
    }

    pub fn trace_rays(
        &self,
        raygen_shader_binding_table_buffer: VkBuffer,
        raygen_shader_binding_offset: VkDeviceSize,
        miss_shader_binding_table_buffer: VkBuffer,
        miss_shader_binding_offset: VkDeviceSize,
        miss_shader_binding_stride: VkDeviceSize,
        hit_shader_binding_table_buffer: VkBuffer,
        hit_shader_binding_offset: VkDeviceSize,
        hit_shader_binding_stride: VkDeviceSize,
        callable_shader_binding_table_offset: VkBuffer,
        callable_shader_binding_offset: VkDeviceSize,
        callable_shader_binding_stride: VkDeviceSize,
        width: u32,
        height: u32,
        depth: u32,
    ) -> Result<(), VkResult> {
        unsafe {
            match vkCmdTraceRaysNV {
                Some(pfn) => {
                    pfn(
                        *self,
                        raygen_shader_binding_table_buffer,
                        raygen_shader_binding_offset,
                        miss_shader_binding_table_buffer,
                        miss_shader_binding_offset,
                        miss_shader_binding_stride,
                        hit_shader_binding_table_buffer,
                        hit_shader_binding_offset,
                        hit_shader_binding_stride,
                        callable_shader_binding_table_offset,
                        callable_shader_binding_offset,
                        callable_shader_binding_stride,
                        width,
                        height,
                        depth,
                    );

                    Ok(())
                }
                None => return Err(VK_ERROR_EXTENSION_NOT_PRESENT),
            }
        }
    }

    pub fn write_acceleration_structure_properties(
        &self,
        acceleration_structures: &[VkAccelerationStructureNV],
        query_type: VkQueryType,
        query_pool: VkQueryPool,
        first_query: u32,
    ) -> Result<(), VkResult> {
        unsafe {
            match vkCmdWriteAccelerationStructurePropertiesNV {
                Some(pfn) => {
                    pfn(
                        *self,
                        acceleration_structures.len() as u32,
                        acceleration_structures.as_ptr(),
                        query_type,
                        query_pool,
                        first_query,
                    );

                    Ok(())
                }
                None => return Err(VK_ERROR_EXTENSION_NOT_PRESENT),
            }
        }
    }
}
*/
