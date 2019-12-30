use utilities::prelude::*;

use crate::impl_vk_handle;
use crate::prelude::*;

use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PipelineType {
    None,
    Graphics,
    Compute,
    RayTracing,
}

impl Default for PipelineType {
    fn default() -> Self {
        PipelineType::None
    }
}

#[derive(Debug)]
pub struct Pipeline {
    device: Arc<Device>,
    pipeline_layout: Arc<PipelineLayout>,

    pipeline_type: PipelineType,

    pipeline: VkPipeline,
}

impl Pipeline {
    pub(crate) fn new(
        device: Arc<Device>,
        pipeline_layout: Arc<PipelineLayout>,
        pipeline_type: PipelineType,
        pipeline: VkPipeline,
    ) -> Self {
        Pipeline {
            device,
            pipeline_layout,
            pipeline_type,
            pipeline,
        }
    }

    pub fn new_graphics() -> GraphicsPipelineBuilder {
        GraphicsPipelineBuilder::default()
    }

    pub fn new_compute<'a>() -> ComputePipelineBuilder<'a> {
        ComputePipelineBuilder::default()
    }

    pub fn new_ray_tracing() -> RayTracingPipelineBuilder {
        RayTracingPipelineBuilder::default()
    }

    pub fn ray_tracing_shader_group_handles(
        &self,
        group_count: u32,
        handle_size: u32,
    ) -> VerboseResult<Vec<u8>> {
        if self.pipeline_type != PipelineType::RayTracing {
            create_error!("wrong pipeline type");
        }

        self.device
            .ray_tracing_shader_group_handles(self.pipeline, 0, group_count, handle_size)
    }

    pub fn compile_deferred(&self, shader_index: u32) -> VerboseResult<()> {
        if self.pipeline_type != PipelineType::RayTracing {
            create_error!("wrong pipeline type");
        }

        self.device.compile_deferred(self.pipeline, shader_index)
    }

    pub fn pipeline_layout(&self) -> &Arc<PipelineLayout> {
        &self.pipeline_layout
    }

    pub fn pipeline_type(&self) -> PipelineType {
        self.pipeline_type
    }
}

impl_vk_handle!(Pipeline, VkPipeline, pipeline);

impl Drop for Pipeline {
    fn drop(&mut self) {
        self.device.destroy_pipeline(self.pipeline);
    }
}
