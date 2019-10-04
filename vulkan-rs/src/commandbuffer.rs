use utilities::prelude::*;

use super::pipeline::PipelineType;
use crate::prelude::*;

use crate::impl_vk_handle;

use std::cell::RefCell;
use std::sync::Arc;

pub struct QueryEnable {
    pub query_flags: VkQueryControlFlagBits,
    pub pipeline_statistics: VkQueryPipelineStatisticFlagBits,
}

pub struct CommandBufferBuilder {
    buffer_level: VkCommandBufferLevel,
}

impl CommandBufferBuilder {
    pub fn build(
        self,
        device: Arc<Device>,
        command_pool: &Arc<CommandPool>,
    ) -> VerboseResult<Arc<CommandBuffer>> {
        let command_buffer_ci =
            VkCommandBufferAllocateInfo::new(command_pool.vk_handle(), self.buffer_level, 1);

        let command_buffer = device.allocate_command_buffers(&command_buffer_ci)?[0];

        Ok(Arc::new(CommandBuffer {
            device,
            pool: command_pool.clone(),
            pipeline: RefCell::new(None),

            buffer: command_buffer,
        }))
    }
}

#[derive(Debug)]
pub struct CommandBuffer {
    device: Arc<Device>,
    pool: Arc<CommandPool>,
    pipeline: RefCell<Option<Arc<Pipeline>>>,

    buffer: VkCommandBuffer,
}

unsafe impl Send for CommandBuffer {}
unsafe impl Sync for CommandBuffer {}

impl_vk_handle!(CommandBuffer, VkCommandBuffer, buffer);

impl CommandBuffer {
    pub(crate) fn primary() -> CommandBufferBuilder {
        CommandBufferBuilder {
            buffer_level: VK_COMMAND_BUFFER_LEVEL_PRIMARY,
        }
    }

    pub(crate) fn secondary() -> CommandBufferBuilder {
        CommandBufferBuilder {
            buffer_level: VK_COMMAND_BUFFER_LEVEL_SECONDARY,
        }
    }

    pub fn reset(&self, flags: impl Into<VkCommandBufferResetFlagBits>) -> VerboseResult<()> {
        self.device.reset_command_buffer(self.buffer, flags)
    }

    pub fn begin(&self, begin_info: VkCommandBufferBeginInfo) -> VerboseResult<()> {
        self.device.begin_command_buffer(self.buffer, &begin_info)
    }

    pub fn end(&self) -> VerboseResult<()> {
        self.device.end_command_buffer(self.buffer)
    }

    pub fn pipeline_barrier(
        &self,
        src_stage_mask: impl Into<VkPipelineStageFlagBits>,
        dst_stage_mask: impl Into<VkPipelineStageFlagBits>,
        dependency_flags: impl Into<VkDependencyFlagBits>,
        memory_barriers: &[VkMemoryBarrier],
        buffer_memory_barriers: &[VkBufferMemoryBarrier],
        image_memory_barriers: &[VkImageMemoryBarrier],
    ) {
        self.device.cmd_pipeline_barrier(
            self.buffer,
            src_stage_mask,
            dst_stage_mask,
            dependency_flags,
            memory_barriers,
            buffer_memory_barriers,
            image_memory_barriers,
        )
    }

    pub fn memory_barrier(
        &self,
        src_access_mask: impl Into<VkAccessFlagBits>,
        src_stage: VkPipelineStageFlags,
        dst_access_mask: impl Into<VkAccessFlagBits>,
        dst_stage: VkPipelineStageFlags,
    ) {
        self.pipeline_barrier(
            src_stage,
            dst_stage,
            0,
            &[VkMemoryBarrier::new(src_access_mask, dst_access_mask)],
            &[],
            &[],
        );
    }

    pub fn buffer_barrier<T>(
        &self,
        buffer: &Arc<Buffer<T>>,
        src_access_mask: impl Into<VkAccessFlagBits>,
        src_stage: impl Into<VkPipelineStageFlagBits>,
        dst_access_mask: impl Into<VkAccessFlagBits>,
        dst_stage: impl Into<VkPipelineStageFlagBits>,
    ) {
        self.pipeline_barrier(
            src_stage,
            dst_stage,
            0,
            &[],
            &[VkBufferMemoryBarrier::new(
                src_access_mask,
                dst_access_mask,
                VK_QUEUE_FAMILY_IGNORED,
                VK_QUEUE_FAMILY_IGNORED,
                buffer.vk_handle(),
                0,
                buffer.byte_size(),
            )],
            &[],
        );
    }

    pub fn image_barrier(
        &self,
        image: &Arc<Image>,
        old_image_layout: VkImageLayout,
        src_stage: impl Into<VkPipelineStageFlagBits>,
        new_image_layout: VkImageLayout,
        dst_stage: impl Into<VkPipelineStageFlagBits>,
    ) {
        let src_access_mask = Self::src_layout_to_access(old_image_layout);
        let dst_access_mask = Self::dst_layout_to_access(new_image_layout);

        self.pipeline_barrier(
            src_stage,
            dst_stage,
            0,
            &[],
            &[],
            &[VkImageMemoryBarrier::new(
                src_access_mask,
                dst_access_mask,
                old_image_layout,
                new_image_layout,
                VK_QUEUE_FAMILY_IGNORED,
                VK_QUEUE_FAMILY_IGNORED,
                image.vk_handle(),
                image.full_resource_range(),
            )],
        );

        image.image_layout.set(new_image_layout);
    }

    pub fn begin_render_pass(
        &self,
        renderpass_begin_info: VkRenderPassBeginInfo,
        subpass_contents: VkSubpassContents,
    ) {
        self.device
            .cmd_begin_render_pass(self.buffer, &renderpass_begin_info, subpass_contents);
    }

    pub fn begin_render_pass_full(
        &self,
        render_pass: &Arc<RenderPass>,
        framebuffer: &Arc<Framebuffer>,
        clear_values: &[VkClearValue],
        subpass_contents: VkSubpassContents,
    ) {
        let render_pass_begin_info = VkRenderPassBeginInfo::new(
            render_pass.vk_handle(),
            framebuffer.vk_handle(),
            VkRect2D {
                offset: VkOffset2D { x: 0, y: 0 },
                extent: VkExtent2D {
                    width: framebuffer.width(),
                    height: framebuffer.height(),
                },
            },
            clear_values,
        );

        self.device
            .cmd_begin_render_pass(self.buffer, &render_pass_begin_info, subpass_contents);
    }

    pub fn next_subpass(&self, subpass_contents: VkSubpassContents) {
        self.device.cmd_next_subpass(self.buffer, subpass_contents);
    }

    pub fn end_render_pass(&self) {
        self.device.cmd_end_render_pass(self.buffer);
    }

    pub fn bind_pipeline(&self, pipeline: &Arc<Pipeline>) -> VerboseResult<()> {
        match pipeline.pipeline_type() {
            PipelineType::None => create_error!("PipelineType was None"),
            PipelineType::Graphics => self.device.cmd_bind_pipeline(
                self.buffer,
                VK_PIPELINE_BIND_POINT_GRAPHICS,
                pipeline.vk_handle(),
            ),
            PipelineType::Compute => self.device.cmd_bind_pipeline(
                self.buffer,
                VK_PIPELINE_BIND_POINT_COMPUTE,
                pipeline.vk_handle(),
            ),
            PipelineType::RayTracing => self.device.cmd_bind_pipeline(
                self.buffer,
                VK_PIPELINE_BIND_POINT_RAY_TRACING_NV,
                pipeline.vk_handle(),
            ),
        }

        *self.pipeline.try_borrow_mut()? = Some(pipeline.clone());

        Ok(())
    }

    pub fn execute_commands(&self, command_buffers: &[&impl VkHandle<VkCommandBuffer>]) {
        let buffers: Vec<VkCommandBuffer> =
            command_buffers.iter().map(|cb| cb.vk_handle()).collect();

        self.device
            .cmd_execute_commands(self.buffer, buffers.as_slice());
    }

    pub fn bind_descriptor_sets_minimal(
        &self,
        descriptor_sets: &[&dyn VkHandle<VkDescriptorSet>],
    ) -> VerboseResult<()> {
        let (pipeline_bind_point, vk_layout) = {
            let opt_borrow = self.pipeline.try_borrow()?;
            let pipeline = match opt_borrow.as_ref() {
                Some(pipeline) => pipeline,
                None => create_error!("no pipeline in command buffer"),
            };

            let pipe_type = match pipeline.pipeline_type() {
                PipelineType::None => create_error!("PipelineType was None"),
                PipelineType::Graphics => VK_PIPELINE_BIND_POINT_GRAPHICS,
                PipelineType::Compute => VK_PIPELINE_BIND_POINT_COMPUTE,
                PipelineType::RayTracing => VK_PIPELINE_BIND_POINT_RAY_TRACING_NV,
            };

            (pipe_type, pipeline.pipeline_layout().vk_handle())
        };

        let vk_descriptor_sets: Vec<VkDescriptorSet> =
            descriptor_sets.iter().map(|ds| ds.vk_handle()).collect();

        self.device.cmd_bind_descriptor_sets(
            self.buffer,
            pipeline_bind_point,
            vk_layout,
            0,
            vk_descriptor_sets.as_slice(),
            &[],
        );

        Ok(())
    }

    pub fn bind_vertex_buffer<T>(&self, buffer: &Arc<Buffer<T>>) {
        self.device
            .cmd_bind_vertex_buffers(self.buffer, 0, &[buffer.vk_handle()], &[0]);
    }

    pub fn bind_vertex_buffers_minimal<T>(&self, buffers: &[&Arc<Buffer<T>>]) {
        let vk_buffers: Vec<VkBuffer> = buffers.iter().map(|b| b.vk_handle()).collect();

        let offsets = vec![0; vk_buffers.len()];

        self.device.cmd_bind_vertex_buffers(
            self.buffer,
            0,
            vk_buffers.as_slice(),
            offsets.as_slice(),
        );
    }

    pub fn bind_index_buffer<T>(
        &self,
        buffer: &Arc<Buffer<T>>,
        offset: VkDeviceSize,
        index_type: VkIndexType,
    ) {
        self.device
            .cmd_bind_index_buffer(self.buffer, buffer.vk_handle(), offset, index_type);
    }

    pub fn set_viewport(&self, viewports: &[VkViewport]) {
        self.device.cmd_set_viewport(self.buffer, 0, viewports);
    }

    pub fn set_scissor(&self, scissors: &[VkRect2D]) {
        self.device.cmd_set_scissor(self.buffer, 0, scissors);
    }

    pub fn draw(
        &self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        self.device.cmd_draw(
            self.buffer,
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        );
    }

    pub fn draw_complete_single_instance(&self, vertex_count: u32) {
        self.device.cmd_draw(self.buffer, vertex_count, 1, 0, 0);
    }

    pub fn draw_indexed(
        &self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        self.device.cmd_draw_indexed(
            self.buffer,
            index_count,
            instance_count,
            first_index,
            vertex_offset,
            first_instance,
        );
    }

    pub fn draw_indexed_complete_single_instance(&self, index_count: u32) {
        self.device
            .cmd_draw_indexed(self.buffer, index_count, 1, 0, 0, 0);
    }

    pub fn push_constants<U>(
        &self,
        stage_flags: impl Into<VkShaderStageFlagBits>,
        data: &U,
    ) -> VerboseResult<()> {
        let opt_borrow = self.pipeline.try_borrow()?;
        let pipeline = match opt_borrow.as_ref() {
            Some(pipeline) => pipeline,
            None => create_error!("no pipeline in command buffer"),
        };

        let layout = pipeline.pipeline_layout();

        self.device
            .cmd_push_constants(self.buffer, layout.vk_handle(), stage_flags, 0, data);

        Ok(())
    }

    pub fn set_image_layout(
        &self,
        image: &Image,
        new_image_layout: VkImageLayout,
        subresource_range: VkImageSubresourceRange,
    ) {
        let src_access = Self::src_layout_to_access(image.image_layout.get());
        let dst_access = Self::dst_layout_to_access(new_image_layout);

        self.pipeline_barrier(
            Self::access_to_stage(src_access),
            Self::access_to_stage(dst_access),
            0,
            &[],
            &[],
            &[VkImageMemoryBarrier::new(
                src_access,
                dst_access,
                image.image_layout.get(),
                new_image_layout,
                VK_QUEUE_FAMILY_IGNORED,
                VK_QUEUE_FAMILY_IGNORED,
                image.vk_handle(),
                subresource_range,
            )],
        );

        image.image_layout.set(new_image_layout);
    }

    pub fn set_full_image_layout(&self, image: &Arc<Image>, new_image_layout: VkImageLayout) {
        let src_access = Self::src_layout_to_access(image.image_layout.get());
        let dst_access = Self::dst_layout_to_access(new_image_layout);

        self.pipeline_barrier(
            Self::access_to_stage(src_access),
            Self::access_to_stage(dst_access),
            0,
            &[],
            &[],
            &[VkImageMemoryBarrier::new(
                src_access,
                dst_access,
                image.image_layout.get(),
                new_image_layout,
                VK_QUEUE_FAMILY_IGNORED,
                VK_QUEUE_FAMILY_IGNORED,
                image.vk_handle(),
                image.full_resource_range(),
            )],
        );

        image.image_layout.set(new_image_layout);
    }

    fn access_to_stage(access_mask: VkAccessFlagBits) -> VkPipelineStageFlags {
        if access_mask == 0 {
            VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT
        } else if access_mask == VK_ACCESS_HOST_WRITE_BIT {
            VK_PIPELINE_STAGE_HOST_BIT
        } else if access_mask == VK_ACCESS_TRANSFER_WRITE_BIT {
            VK_PIPELINE_STAGE_TRANSFER_BIT
        } else if access_mask == VK_ACCESS_TRANSFER_READ_BIT {
            VK_PIPELINE_STAGE_TRANSFER_BIT
        } else if access_mask == VK_ACCESS_SHADER_READ_BIT {
            VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT
        } else if access_mask == VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT {
            VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT
        } else if access_mask == VK_ACCESS_MEMORY_READ_BIT {
            VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT
        } else if access_mask == VK_ACCESS_MEMORY_READ_BIT | VK_ACCESS_MEMORY_WRITE_BIT {
            VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT
        } else if access_mask
            == VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT
                | VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT
        {
            VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT
        } else {
            unimplemented!("access mask not supported {:?}", access_mask)
        }
    }

    fn src_layout_to_access(image_layout: VkImageLayout) -> VkAccessFlagBits {
        match image_layout {
            VK_IMAGE_LAYOUT_UNDEFINED => 0u32.into(),
            VK_IMAGE_LAYOUT_PREINITIALIZED => VK_ACCESS_HOST_WRITE_BIT.into(),
            VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL => VK_ACCESS_TRANSFER_WRITE_BIT.into(),
            VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL => VK_ACCESS_TRANSFER_READ_BIT.into(),
            VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL => VK_ACCESS_SHADER_READ_BIT.into(),
            VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL => VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT.into(),
            VK_IMAGE_LAYOUT_PRESENT_SRC_KHR => VK_ACCESS_MEMORY_READ_BIT.into(),
            VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL => {
                VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT
                    | VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT
            }
            VK_IMAGE_LAYOUT_GENERAL => VK_ACCESS_MEMORY_READ_BIT | VK_ACCESS_MEMORY_WRITE_BIT,
            _ => unimplemented!("source image layout ({:?})", image_layout),
        }
    }

    fn dst_layout_to_access(image_layout: VkImageLayout) -> VkAccessFlagBits {
        match image_layout {
            VK_IMAGE_LAYOUT_UNDEFINED => {
                panic!("target image layout must not be VK_IMAGE_LAYOUT_UNDEFINED")
            }
            VK_IMAGE_LAYOUT_PREINITIALIZED => {
                panic!("target image layout must not be VK_IMAGE_LAYOUT_PREINITIALIZED")
            }
            VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL => VK_ACCESS_TRANSFER_WRITE_BIT.into(),
            VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL => VK_ACCESS_TRANSFER_READ_BIT.into(),
            VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL => VK_ACCESS_SHADER_READ_BIT.into(),
            VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL => VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT.into(),
            VK_IMAGE_LAYOUT_PRESENT_SRC_KHR => VK_ACCESS_MEMORY_READ_BIT.into(),
            VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL => {
                VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT
                    | VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT
            }
            VK_IMAGE_LAYOUT_GENERAL => VK_ACCESS_MEMORY_READ_BIT | VK_ACCESS_MEMORY_WRITE_BIT,
            VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL => VK_ACCESS_SHADER_READ_BIT.into(),
        }
    }

    // TODO:
    pub fn set_line_width(&self) {
        unimplemented!();
    }

    pub fn set_depth_bias(&self) {
        unimplemented!();
    }

    pub fn set_blend_constants(&self) {
        unimplemented!();
    }

    pub fn ste_depth_bounds(&self) {
        unimplemented!();
    }

    pub fn set_stencil_compare_mask(&self) {
        unimplemented!();
    }

    pub fn set_stencil_write_mask(&self) {
        unimplemented!();
    }

    pub fn set_stencil_reference(&self) {
        unimplemented!();
    }

    pub fn draw_indirect(&self) {
        unimplemented!();
    }

    pub fn draw_indexed_indirect(&self) {
        unimplemented!();
    }

    pub fn dispatch(&self, x: u32, y: u32, z: u32) {
        self.device.cmd_dispatch(self.buffer, x, y, z);
    }

    pub fn dispatch_indirect(&self) {
        unimplemented!();
    }

    pub fn copy_buffer<T, U>(
        &self,
        src_buffer: &Arc<Buffer<T>>,
        dst_buffer: &Arc<Buffer<U>>,
        regions: &[VkBufferCopy],
    ) {
        self.device.cmd_copy_buffer(
            self.buffer,
            src_buffer.vk_handle(),
            dst_buffer.vk_handle(),
            regions,
        );
    }

    pub fn copy_image(
        &self,
        src_image: &Arc<Image>,
        dst_image: &Arc<Image>,
        src_layout: VkImageLayout,
        dst_layout: VkImageLayout,
        regions: &[VkImageCopy],
    ) {
        self.device.cmd_copy_image(
            self.buffer,
            src_image.vk_handle(),
            src_layout,
            dst_image.vk_handle(),
            dst_layout,
            regions,
        );
    }

    pub fn blit_complete(&self, src_image: &Arc<Image>, dst_image: &Arc<Image>, filter: VkFilter) {
        let image_blit = VkImageBlit {
            srcSubresource: src_image.full_resource_layers(),
            srcOffsets: [
                VkOffset3D { x: 0, y: 0, z: 0 },
                VkOffset3D {
                    x: src_image.width() as i32,
                    y: src_image.height() as i32,
                    z: 1,
                },
            ],
            dstSubresource: dst_image.full_resource_layers(),
            dstOffsets: [
                VkOffset3D { x: 0, y: 0, z: 0 },
                VkOffset3D {
                    x: dst_image.width() as i32,
                    y: dst_image.height() as i32,
                    z: 1,
                },
            ],
        };

        self.blit_image(
            src_image,
            dst_image,
            src_image.image_layout.get(),
            dst_image.image_layout.get(),
            &[image_blit],
            filter,
        );
    }

    pub fn blit_image(
        &self,
        src_image: &Arc<Image>,
        dst_image: &Arc<Image>,
        src_layout: VkImageLayout,
        dst_layout: VkImageLayout,
        regions: &[VkImageBlit],
        filter: VkFilter,
    ) {
        self.device.cmd_blit_image(
            self.buffer,
            src_image.vk_handle(),
            src_layout,
            dst_image.vk_handle(),
            dst_layout,
            regions,
            filter,
        );
    }

    pub fn copy_buffer_to_image<T>(
        &self,
        src_buffer: &Arc<Buffer<T>>,
        dst_image: &Arc<Image>,
        image_layout: VkImageLayout,
        regions: &[VkBufferImageCopy],
    ) {
        self.device.cmd_copy_buffer_to_image(
            self.buffer,
            src_buffer.vk_handle(),
            dst_image.vk_handle(),
            image_layout,
            regions,
        );
    }

    pub fn copy_image_to_buffer<T>(
        &self,
        src_image: &Arc<Image>,
        image_layout: VkImageLayout,
        dst_buffer: &Arc<Buffer<T>>,
        regions: &[VkBufferImageCopy],
    ) {
        self.device.cmd_copy_image_to_buffer(
            self.buffer,
            src_image.vk_handle(),
            image_layout,
            dst_buffer.vk_handle(),
            regions,
        )
    }

    pub fn update_buffer(&self) {
        unimplemented!();
    }

    pub fn fill_buffer(&self) {
        unimplemented!();
    }

    pub fn clear_color_image(&self, image: &Arc<Image>, clear_color: VkClearColorValue) {
        self.device.cmd_clear_color_image(
            self.buffer,
            image.vk_handle(),
            image.image_layout.get(),
            clear_color,
            &[image.full_resource_range()],
        );
    }

    pub fn clear_depth_stencil_image(&self) {
        unimplemented!();
    }

    pub fn clear_attachments(&self) {
        unimplemented!();
    }

    pub fn resolve_image(&self) {
        unimplemented!();
    }

    pub fn set_event(&self) {
        unimplemented!();
    }

    pub fn reset_event(&self) {
        unimplemented!();
    }

    pub fn wait_events(&self) {
        unimplemented!();
    }

    pub fn begin_query(&self) {
        unimplemented!();
    }

    pub fn end_query(&self) {
        unimplemented!();
    }

    pub fn reset_query_pool(&self) {
        unimplemented!();
    }

    pub fn write_timestamp(
        &self,
        query_pool: &Arc<QueryPool>,
        query: u32,
        pipeline_stage: impl Into<VkPipelineStageFlagBits>,
    ) {
        self.device.cmd_write_timestamp(
            self.buffer,
            pipeline_stage,
            query_pool.vk_query_pool(),
            query,
        );
    }

    pub fn build_acceleration_structure<T>(
        &self,
        info: &VkAccelerationStructureInfoNV,
        instance_data: &Option<Arc<Buffer<VkGeometryInstanceNV>>>,
        dst: &AccelerationStructure,
        src: Option<&AccelerationStructure>,
        scratch: &Arc<Buffer<T>>,
    ) {
        self.device.cmd_build_acceleration_structure(
            self.buffer,
            info,
            match instance_data {
                Some(instance) => instance.vk_handle(),
                None => VkBuffer::NULL_HANDLE,
            },
            0,
            src.is_some(),
            dst.vk_handle(),
            match src {
                Some(src) => src.vk_handle(),
                None => VkAccelerationStructureNV::NULL_HANDLE,
            },
            scratch.vk_handle(),
            0,
        )
    }

    pub fn copy_acceleration_structure(
        &self,
        dst: &Arc<AccelerationStructure>,
        src: &Arc<AccelerationStructure>,
        mode: VkCopyAccelerationStructureModeNV,
    ) {
        self.device.cmd_copy_acceleration_structure(
            self.buffer,
            dst.vk_handle(),
            src.vk_handle(),
            mode,
        )
    }

    pub fn write_acceleration_structure_properties(
        &self,
        acceleration_structures: &[&Arc<AccelerationStructure>],
        query_type: VkQueryType,
        query_pool: &Arc<QueryPool>,
        first_query: u32,
    ) {
        let vk_handles: Vec<VkAccelerationStructureNV> = acceleration_structures
            .iter()
            .map(|a| a.vk_handle())
            .collect();

        self.device.cmd_write_acceleration_structure_properties(
            self.buffer,
            &vk_handles,
            query_type,
            query_pool.vk_query_pool(),
            first_query,
        )
    }

    // TODO: callable shader binding !?!?!?
    pub fn trace_rays_sbt(&self, sbt: &ShaderBindingTable, width: u32, height: u32, depth: u32) {
        self.device.cmd_trace_rays(
            self.buffer,
            sbt.sbt_buffer().vk_handle(),
            sbt.ray_gen_offset(),
            sbt.sbt_buffer().vk_handle(),
            sbt.miss_offset(),
            sbt.miss_stride(),
            sbt.sbt_buffer().vk_handle(),
            sbt.hit_group_offset(),
            sbt.hit_group_stride(),
            VkBuffer::NULL_HANDLE,
            0,
            0,
            width,
            height,
            depth,
        )
    }

    pub fn trace_rays(
        &self,
        raygen_shader_binding_table: &Arc<Buffer<impl Copy>>,
        raygen_shader_binding_offset: VkDeviceSize,
        miss_shader_binding_table: &Arc<Buffer<impl Copy>>,
        miss_shader_binding_offset: VkDeviceSize,
        miss_shader_binding_stride: VkDeviceSize,
        hit_shader_binding_table: &Arc<Buffer<impl Copy>>,
        hit_shader_binding_offset: VkDeviceSize,
        hit_shader_binding_stride: VkDeviceSize,
        callable_shader_binding_table: &Arc<Buffer<impl Copy>>,
        callable_shader_binding_offset: VkDeviceSize,
        callable_shader_binding_stride: VkDeviceSize,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        self.device.cmd_trace_rays(
            self.buffer,
            raygen_shader_binding_table.vk_handle(),
            raygen_shader_binding_offset,
            miss_shader_binding_table.vk_handle(),
            miss_shader_binding_offset,
            miss_shader_binding_stride,
            hit_shader_binding_table.vk_handle(),
            hit_shader_binding_offset,
            hit_shader_binding_stride,
            callable_shader_binding_table.vk_handle(),
            callable_shader_binding_offset,
            callable_shader_binding_stride,
            width,
            height,
            depth,
        )
    }

    pub fn copy_query_pool_results(&self) {
        unimplemented!();
    }

    pub fn inheritance_info(
        render_pass: Option<&Arc<RenderPass>>,
        sub_pass: Option<u32>,
        framebuffer: Option<&Arc<Framebuffer>>,
        query_enable: Option<QueryEnable>,
    ) -> VkCommandBufferInheritanceInfo {
        let mut info = VkCommandBufferInheritanceInfo::new(
            match render_pass {
                Some(render_pass) => render_pass.vk_handle(),
                None => VkRenderPass::NULL_HANDLE,
            },
            match sub_pass {
                Some(sub_pass) => sub_pass,
                None => 0,
            },
            match framebuffer {
                Some(framebuffer) => framebuffer.vk_handle(),
                None => VkFramebuffer::NULL_HANDLE,
            },
        );

        if let Some(query) = query_enable {
            info.set_query(true, query.query_flags, query.pipeline_statistics);
        }

        info
    }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        self.device
            .free_command_buffers(self.pool.vk_handle(), &[self.buffer]);
    }
}
