use utilities::prelude::*;

use crate::pipeline::PipelineType;
use crate::prelude::*;

use std::sync::Arc;

pub struct ComputePipelineBuilder<'a> {
    shader_module: Option<&'a Arc<ShaderModule>>,
    pipeline_cache: Option<&'a Arc<PipelineCache>>,
    flags: VkPipelineCreateFlagBits,
    pipeline_layout_builder: PipelineLayoutBuilder,
}

impl<'a> ComputePipelineBuilder<'a> {
    // TODO: add support for specialization constants
    pub fn set_shader_module(mut self, shader_module: &'a Arc<ShaderModule>) -> Self {
        if cfg!(debug_assertions) {
            if self.shader_module.is_some() {
                panic!("shader already set!");
            }

            if shader_module.shader_type() != ShaderType::Compute {
                panic!("shader has wrong type!");
            }
        }

        self.shader_module = Some(shader_module);

        self
    }

    pub fn set_pipeline_cache(mut self, pipeline_cache: &'a Arc<PipelineCache>) -> Self {
        self.pipeline_cache = Some(pipeline_cache);

        self
    }

    pub fn set_flags(mut self, flags: impl Into<VkPipelineCreateFlagBits>) -> Self {
        self.flags = flags.into();

        self
    }

    pub fn add_descriptor_set_layout(
        mut self,
        descriptor_set_layout: &dyn VkHandle<VkDescriptorSetLayout>,
    ) -> Self {
        self.pipeline_layout_builder = self
            .pipeline_layout_builder
            .add_descriptor_set_layout(descriptor_set_layout);

        self
    }

    pub fn add_push_constant_range(mut self, push_constant_range: VkPushConstantRange) -> Self {
        self.pipeline_layout_builder = self
            .pipeline_layout_builder
            .add_push_constant(push_constant_range);

        self
    }

    pub fn build(self, device: &Arc<Device>) -> VerboseResult<Arc<Pipeline>> {
        let pipeline_layout = self.pipeline_layout_builder.build(device.clone())?;

        let pipeline_ci = match self.shader_module {
            Some(module) => VkComputePipelineCreateInfo::new(
                self.flags,
                module.pipeline_stage_info(),
                pipeline_layout.vk_handle(),
            ),
            None => create_error!("no shader module set!"),
        };

        let pipeline = device.create_compute_pipelines(
            match self.pipeline_cache {
                Some(cache) => Some(cache.vk_handle()),
                None => None,
            },
            &[pipeline_ci],
        )?[0];

        Ok(Arc::new(Pipeline::new(
            device.clone(),
            pipeline_layout.clone(),
            PipelineType::Compute,
            pipeline,
        )))
    }
}

impl<'a> Default for ComputePipelineBuilder<'a> {
    fn default() -> Self {
        ComputePipelineBuilder {
            shader_module: None,
            pipeline_cache: None,
            flags: 0.into(),
            pipeline_layout_builder: PipelineLayout::builder(),
        }
    }
}
