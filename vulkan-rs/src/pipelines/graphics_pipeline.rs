use utilities::prelude::*;

use crate::pipeline::PipelineType;
use crate::prelude::*;

use std::sync::Arc;

pub struct GraphicsPipelineBuilder {
    flags: VkPipelineCreateFlagBits,

    pipeline_cache: Option<Arc<PipelineCache>>,

    amd_rasterization_order: Option<VkPipelineRasterizationStateRasterizationOrderAMD>,

    vertex_shader: Option<Arc<ShaderModule>>,
    vertex_binding_description: Vec<VkVertexInputBindingDescription>,
    vertex_attribute_description: Vec<VkVertexInputAttributeDescription>,

    input_assembly: Option<VkPipelineInputAssemblyStateCreateInfo>,

    tesselation_shader: Option<(Arc<ShaderModule>, Arc<ShaderModule>)>,
    patch_control_points: u32,

    geometry_shader: Option<Arc<ShaderModule>>,

    fragment_shader: Option<Arc<ShaderModule>>,

    viewports: Vec<VkViewport>,
    scissors: Vec<VkRect2D>,

    rasterization: Option<VkPipelineRasterizationStateCreateInfo>,
    multisample: Option<VkPipelineMultisampleStateCreateInfo>,
    depth_stencil: Option<VkPipelineDepthStencilStateCreateInfo>,

    blend_attachments: Vec<VkPipelineColorBlendAttachmentState>,
    color_blend: Option<VkPipelineColorBlendStateCreateInfo>,

    dynamic_states: Vec<VkDynamicState>,
}

impl GraphicsPipelineBuilder {
    pub fn set_vertex_shader(
        mut self,
        shader: Arc<ShaderModule>,
        vertex_binding_description: Vec<VkVertexInputBindingDescription>,
        vertex_attribute_description: Vec<VkVertexInputAttributeDescription>,
    ) -> Self {
        if cfg!(debug_assertions) {
            assert_eq!(shader.shader_type(), ShaderType::Vertex);
        }

        self.vertex_shader = Some(shader);
        self.vertex_binding_description = vertex_binding_description;
        self.vertex_attribute_description = vertex_attribute_description;

        self
    }

    pub fn set_tesselation_shader(
        mut self,
        tesselation_control: Arc<ShaderModule>,
        tesselation_evaluation: Arc<ShaderModule>,
        patch_control_points: u32,
    ) -> Self {
        if cfg!(debug_assertions) {
            assert_eq!(
                tesselation_control.shader_type(),
                ShaderType::TesselationControl
            );

            assert_eq!(
                tesselation_evaluation.shader_type(),
                ShaderType::TesselationEvaluation
            );
        }

        self.tesselation_shader = Some((tesselation_control, tesselation_evaluation));
        self.patch_control_points = patch_control_points;

        self
    }

    pub fn set_geometry_shader(mut self, shader: Arc<ShaderModule>) -> Self {
        if cfg!(debug_assertions) {
            assert_eq!(shader.shader_type(), ShaderType::Geometry);
        }

        self.geometry_shader = Some(shader);

        self
    }

    pub fn set_fragment_shader(mut self, shader: Arc<ShaderModule>) -> Self {
        if cfg!(debug_assertions) {
            assert_eq!(shader.shader_type(), ShaderType::Fragment);
        }

        self.fragment_shader = Some(shader);

        self
    }

    pub fn set_flags(mut self, flags: impl Into<VkPipelineCreateFlagBits>) -> Self {
        self.flags = flags.into();

        self
    }

    pub fn enable_rasterization_order(mut self, order: VkRasterizationOrderAMD) -> Self {
        self.amd_rasterization_order = Some(
            VkPipelineRasterizationStateRasterizationOrderAMD::new(order),
        );

        self
    }

    pub fn input_assembly(
        mut self,
        topology: VkPrimitiveTopology,
        primitive_restart_enable: bool,
    ) -> Self {
        self.input_assembly = Some(VkPipelineInputAssemblyStateCreateInfo::new(
            0,
            topology,
            primitive_restart_enable,
        ));

        self
    }

    pub fn default_rasterization(
        mut self,
        cull_mode: VkCullModeFlags,
        front_face: VkFrontFace,
    ) -> Self {
        self.rasterization = Some(VkPipelineRasterizationStateCreateInfo::new(
            0,
            false,
            false,
            VK_POLYGON_MODE_FILL,
            cull_mode,
            front_face,
            false,
            0.0,
            0.0,
            0.0,
            1.0,
        ));

        self
    }

    pub fn custom_rasterization(
        mut self,
        depth_clamp_enable: bool,
        rasterization_discard_enable: bool,
        polygon_mode: VkPolygonMode,
        cull_mode: VkCullModeFlags,
        front_face: VkFrontFace,
        depth_bias_enable: bool,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
        line_width: f32,
    ) -> Self {
        self.rasterization = Some(VkPipelineRasterizationStateCreateInfo::new(
            0,
            depth_clamp_enable,
            rasterization_discard_enable,
            polygon_mode,
            cull_mode,
            front_face,
            depth_bias_enable,
            depth_bias_constant_factor,
            depth_bias_clamp,
            depth_bias_slope_factor,
            line_width,
        ));

        self
    }

    pub fn default_multisample(mut self, sample_count: VkSampleCountFlags) -> Self {
        self.multisample = Some(VkPipelineMultisampleStateCreateInfo::new(
            0,
            sample_count,
            false,
            0.0,
            &[],
            false,
            false,
        ));

        self
    }

    pub fn custom_multisample(
        mut self,
        sample_count: VkSampleCountFlags,
        sample_shading_enable: bool,
        min_sample_shading: f32,
        sample_masks: &[VkSampleMask],
        alpha_to_coverage_enable: bool,
        alpha_to_one_enable: bool,
    ) -> Self {
        self.multisample = Some(VkPipelineMultisampleStateCreateInfo::new(
            0,
            sample_count,
            sample_shading_enable,
            min_sample_shading,
            sample_masks,
            alpha_to_coverage_enable,
            alpha_to_one_enable,
        ));

        self
    }

    pub fn add_dynamic_state(mut self, dynamic_state: VkDynamicState) -> Self {
        self.dynamic_states.push(dynamic_state);

        self
    }

    pub fn default_depth_stencil(mut self, depth_test: bool, stencil_test: bool) -> Self {
        let stencil_op_state = VkStencilOpState {
            failOp: VK_STENCIL_OP_KEEP,
            passOp: VK_STENCIL_OP_KEEP,
            depthFailOp: VK_STENCIL_OP_KEEP,
            compareOp: VK_COMPARE_OP_ALWAYS,
            compareMask: 0,
            writeMask: 0,
            reference: 0,
        };

        self.depth_stencil = Some(VkPipelineDepthStencilStateCreateInfo::new(
            VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_NULL_BIT,
            depth_test,
            depth_test,
            VK_COMPARE_OP_LESS,
            false,
            stencil_test,
            stencil_op_state.clone(),
            stencil_op_state,
            0.0,
            0.0,
        ));

        self
    }

    pub fn custom_depth_stencil(
        mut self,
        depth_test_enable: bool,
        depth_write_enable: bool,
        depth_compare_op: VkCompareOp,
        depth_bounds_test_enable: bool,
        stencil_test_enable: bool,
        front: VkStencilOpState,
        back: VkStencilOpState,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    ) -> Self {
        self.depth_stencil = Some(VkPipelineDepthStencilStateCreateInfo::new(
            0,
            depth_test_enable,
            depth_write_enable,
            depth_compare_op,
            depth_bounds_test_enable,
            stencil_test_enable,
            front,
            back,
            min_depth_bounds,
            max_depth_bounds,
        ));

        self
    }

    pub fn default_color_blend(
        mut self,
        attachments: Vec<VkPipelineColorBlendAttachmentState>,
    ) -> Self {
        self.blend_attachments = attachments;

        self.color_blend = Some(VkPipelineColorBlendStateCreateInfo::new(
            0,
            false,
            VK_LOGIC_OP_NO_OP,
            &self.blend_attachments,
            [1.0, 1.0, 1.0, 1.0],
        ));

        self
    }

    pub fn custom_color_blend(
        mut self,
        logic_op_enable: bool,
        logic_op: VkLogicOp,
        attachments: Vec<VkPipelineColorBlendAttachmentState>,
        blend_constants: [f32; 4],
    ) -> Self {
        self.blend_attachments = attachments;

        self.color_blend = Some(VkPipelineColorBlendStateCreateInfo::new(
            0,
            logic_op_enable,
            logic_op,
            &self.blend_attachments,
            blend_constants,
        ));

        self
    }

    pub fn add_viewport(mut self, viewport: VkViewport) -> Self {
        self.viewports.push(viewport);

        self
    }

    pub fn add_scissor(mut self, scissor: VkRect2D) -> Self {
        self.scissors.push(scissor);

        self
    }

    pub fn build(
        mut self,
        device: Arc<Device>,
        pipeline_layout: &Arc<PipelineLayout>,
        render_pass: &Arc<RenderPass>,
        subpass: u32,
    ) -> VerboseResult<Arc<Pipeline>> {
        let mut rasterization = self.rasterization.expect("rasterization state is required");

        if let Some(amd_rasterization_order) = &self.amd_rasterization_order {
            if device.enabled_extensions().amd_rasterization_order {
                rasterization.chain(amd_rasterization_order);
            }
        }

        let vertex_input = VkPipelineVertexInputStateCreateInfo::new(
            0,
            &self.vertex_binding_description,
            &self.vertex_attribute_description,
        );

        let mut stages = Vec::new();

        if let Some(shader) = &self.vertex_shader {
            stages.push(shader.pipeline_stage_info());
        }

        if let Some(shader) = &self.geometry_shader {
            stages.push(shader.pipeline_stage_info());
        }

        if let Some((tesselation_control, tesselation_evaluation)) = &self.tesselation_shader {
            stages.push(tesselation_control.pipeline_stage_info());
            stages.push(tesselation_evaluation.pipeline_stage_info());
        }

        if let Some(shader) = &self.fragment_shader {
            stages.push(shader.pipeline_stage_info());
        }

        if self.viewports.is_empty() {
            self.dynamic_states.push(VK_DYNAMIC_STATE_VIEWPORT);
            self.viewports.push(VkViewport::default());
        }

        if self.scissors.is_empty() {
            self.dynamic_states.push(VK_DYNAMIC_STATE_SCISSOR);
            self.scissors.push(VkRect2D::default());
        }

        let viewport_state =
            VkPipelineViewportStateCreateInfo::new(0, &self.viewports, &self.scissors);

        let tesselation = if self.patch_control_points != 0 {
            Some(VkPipelineTessellationStateCreateInfo::new(
                0,
                self.patch_control_points,
            ))
        } else {
            None
        };

        let dynamic_state = VkPipelineDynamicStateCreateInfo::new(0, &self.dynamic_states);

        let pipeline_ci = VkGraphicsPipelineCreateInfo::new(
            self.flags,
            &stages,
            Some(&vertex_input),
            self.input_assembly.as_ref(),
            tesselation.as_ref(),
            Some(&viewport_state),
            &rasterization,
            self.multisample.as_ref(),
            self.depth_stencil.as_ref(),
            self.color_blend.as_ref(),
            Some(&dynamic_state),
            pipeline_layout.vk_handle(),
            render_pass.vk_handle(),
            subpass,
        );

        let pipeline = device.create_graphics_pipelines(
            match self.pipeline_cache {
                Some(cache) => Some(cache.vk_handle()),
                None => None,
            },
            &[pipeline_ci],
        )?[0];

        Ok(Arc::new(Pipeline::new(
            device.clone(),
            pipeline_layout.clone(),
            PipelineType::Graphics,
            pipeline,
        )))
    }
}

impl Default for GraphicsPipelineBuilder {
    fn default() -> Self {
        GraphicsPipelineBuilder {
            flags: 0.into(),

            pipeline_cache: None,

            amd_rasterization_order: None,

            vertex_shader: None,
            vertex_binding_description: Vec::new(),
            vertex_attribute_description: Vec::new(),

            input_assembly: None,

            tesselation_shader: None,
            patch_control_points: 0,

            geometry_shader: None,

            fragment_shader: None,

            viewports: Vec::new(),
            scissors: Vec::new(),

            rasterization: None,
            multisample: None,
            depth_stencil: None,

            blend_attachments: Vec::new(),
            color_blend: None,

            dynamic_states: Vec::new(),
        }
    }
}
