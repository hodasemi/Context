use utilities::prelude::*;

use crate::pipeline::PipelineType;
use crate::prelude::*;

use std::iter::IntoIterator;
use std::sync::Arc;

pub struct RayTracingPipelineBuilder {
    shader_modules: Vec<(Arc<ShaderModule>, Option<SpecializationConstants>)>,
    shader_groups: Vec<VkRayTracingShaderGroupCreateInfoNV>,
    max_recursion_depth: u32,
    shader_binding_table_builder: ShaderBindingTableBuilder,
}

impl RayTracingPipelineBuilder {
    pub fn max_recursion_depth(mut self, max_recursion_depth: u32) -> Self {
        self.max_recursion_depth = max_recursion_depth;

        self
    }

    // TODO: add support for specialization constants
    pub fn add_shader(
        mut self,
        shader_module: Arc<ShaderModule>,
        data: Option<Vec<u8>>,
        specialization_constants: Option<SpecializationConstants>,
    ) -> Self {
        self.shader_binding_table_builder = match shader_module.shader_type() {
            ShaderType::RayGeneration => self
                .shader_binding_table_builder
                .add_ray_gen_program(self.shader_groups.len() as u32, data),
            ShaderType::Miss => self
                .shader_binding_table_builder
                .add_miss_program(self.shader_groups.len() as u32, data),
            _ => panic!(
                "unsupported shader type: {:?}, expected RayGen or Miss Shader",
                shader_module.shader_type()
            ),
        };

        let shader_index = self.shader_modules.len();
        self.shader_modules
            .push((shader_module, specialization_constants));

        self.shader_groups
            .push(VkRayTracingShaderGroupCreateInfoNV::new(
                VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV,
                shader_index as u32,
                VK_SHADER_UNUSED_NV,
                VK_SHADER_UNUSED_NV,
                VK_SHADER_UNUSED_NV,
            ));

        self
    }

    // TODO: add support for specialization constants
    pub fn add_hit_shaders<'a>(
        mut self,
        shader_modules: impl IntoIterator<Item = &'a Arc<ShaderModule>>,
        data: Option<Vec<u8>>,
        specialization_constants: impl IntoIterator<Item = Option<SpecializationConstants>>,
    ) -> Self {
        let mut group = VkRayTracingShaderGroupCreateInfoNV::new(
            VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV,
            VK_SHADER_UNUSED_NV,
            VK_SHADER_UNUSED_NV,
            VK_SHADER_UNUSED_NV,
            VK_SHADER_UNUSED_NV,
        );

        for (shader_module, specialization_constant) in
            shader_modules.into_iter().zip(specialization_constants)
        {
            let shader_index = self.shader_modules.len() as u32;

            match shader_module.shader_type() {
                ShaderType::AnyHit => {
                    // sanity check
                    if cfg!(debug_assertions) && group.anyHitShader != VK_SHADER_UNUSED_NV {
                        panic!("any hit shader already used in current hit group");
                    }

                    group.anyHitShader = shader_index;
                }
                ShaderType::ClosestHit => {
                    // sanity check
                    if cfg!(debug_assertions) && group.closestHitShader != VK_SHADER_UNUSED_NV {
                        panic!("closest hit shader already used in current hit group");
                    }

                    group.closestHitShader = shader_index;
                }
                ShaderType::Intersection => {
                    // sanity check
                    if cfg!(debug_assertions) && group.intersectionShader != VK_SHADER_UNUSED_NV {
                        panic!("intersection shader already used in current hit group");
                    }

                    group.intersectionShader = shader_index;
                    group.r#type = VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV;
                }
                _ => panic!("unsupported shader type: {:?}, expected AnyHit, ClosestHit or Intersection Shader", shader_module.shader_type()),
            }

            self.shader_modules
                .push((shader_module.clone(), specialization_constant));
        }
        self.shader_binding_table_builder = self
            .shader_binding_table_builder
            .add_hit_group_program(self.shader_groups.len() as u32, data);
        self.shader_groups.push(group);

        self
    }

    pub fn build(
        mut self,
        device: &Arc<Device>,
        descriptor_set_layouts: &[&dyn VkHandle<VkDescriptorSetLayout>],
    ) -> VerboseResult<(Arc<Pipeline>, ShaderBindingTable)> {
        let pipeline_layout = PipelineLayout::new(device.clone(), descriptor_set_layouts, &[])?;

        let shader_stages: Vec<VkPipelineShaderStageCreateInfo> = self
            .shader_modules
            .iter()
            .map(|(shader, specialization_constant)| {
                let mut stage_info = shader.pipeline_stage_info();
                if let Some(specialization_constant) = specialization_constant {
                    stage_info.set_specialization_info(specialization_constant.vk_info());
                }

                stage_info
            })
            .collect();

        // check that we dont exceed the gpu's capabilities
        let max_recursion = self.max_recursion_depth.min(
            device
                .physical_device()
                .ray_tracing_properties()
                .maxRecursionDepth,
        );

        let pipeline = Self::new_ray_tracing(
            device.clone(),
            None,
            0,
            &shader_stages,
            &self.shader_groups,
            max_recursion,
            &pipeline_layout,
        )?;

        let sbt = self.shader_binding_table_builder.build(device, &pipeline)?;

        Ok((pipeline, sbt))
    }

    fn new_ray_tracing(
        device: Arc<Device>,
        pipeline_cache: Option<&Arc<PipelineCache>>,
        flags: impl Into<VkPipelineCreateFlagBits>,
        stages: &[VkPipelineShaderStageCreateInfo],
        groups: &[VkRayTracingShaderGroupCreateInfoNV],
        max_recursion_depth: u32,
        pipeline_layout: &Arc<PipelineLayout>,
    ) -> VerboseResult<Arc<Pipeline>> {
        let pipeline_ci = VkRayTracingPipelineCreateInfoNV::new(
            flags,
            stages,
            groups,
            max_recursion_depth,
            pipeline_layout.vk_handle(),
        );

        let pipeline = device.create_ray_tracing_pipelines(
            match pipeline_cache {
                Some(cache) => Some(cache.vk_handle()),
                None => None,
            },
            &[pipeline_ci],
        )?[0];

        Ok(Arc::new(Pipeline::new(
            device.clone(),
            pipeline_layout.clone(),
            PipelineType::RayTracing,
            pipeline,
        )))
    }
}

impl Default for RayTracingPipelineBuilder {
    fn default() -> Self {
        RayTracingPipelineBuilder {
            shader_modules: Vec::new(),
            shader_groups: Vec::new(),
            max_recursion_depth: 2,
            shader_binding_table_builder: ShaderBindingTableBuilder::new(),
        }
    }
}

struct ShaderBindingTableEntry {
    group_index: u32,
    inline_data: Vec<u8>,
}
struct ShaderBindingTableBuilder {
    ray_gen_entries: Vec<ShaderBindingTableEntry>,
    miss_entries: Vec<ShaderBindingTableEntry>,
    hit_group_entries: Vec<ShaderBindingTableEntry>,
}

pub struct ShaderBindingTable {
    sbt_buffer: Arc<Buffer<u8>>,

    miss_offset: VkDeviceSize,
    miss_stride: VkDeviceSize,
    hit_group_offset: VkDeviceSize,
    hit_group_stride: VkDeviceSize,
}

impl ShaderBindingTable {
    pub fn sbt_buffer(&self) -> &Arc<Buffer<u8>> {
        &self.sbt_buffer
    }

    pub fn ray_gen_offset(&self) -> VkDeviceSize {
        0
    }

    pub fn miss_offset(&self) -> VkDeviceSize {
        self.miss_offset
    }

    pub fn miss_stride(&self) -> VkDeviceSize {
        self.miss_stride
    }

    pub fn hit_group_offset(&self) -> VkDeviceSize {
        self.hit_group_offset
    }

    pub fn hit_group_stride(&self) -> VkDeviceSize {
        self.hit_group_stride
    }
}

impl ShaderBindingTableBuilder {
    pub(crate) fn new() -> ShaderBindingTableBuilder {
        ShaderBindingTableBuilder {
            ray_gen_entries: Vec::new(),
            miss_entries: Vec::new(),
            hit_group_entries: Vec::new(),
        }
    }

    fn add_ray_gen_program(mut self, group_index: u32, data: Option<Vec<u8>>) -> Self {
        self.ray_gen_entries.push(ShaderBindingTableEntry {
            group_index,
            inline_data: match data {
                Some(data) => data,
                None => Vec::new(),
            },
        });

        self
    }

    fn add_miss_program(mut self, group_index: u32, data: Option<Vec<u8>>) -> Self {
        self.miss_entries.push(ShaderBindingTableEntry {
            group_index,
            inline_data: match data {
                Some(data) => data,
                None => Vec::new(),
            },
        });

        self
    }

    fn add_hit_group_program(mut self, group_index: u32, data: Option<Vec<u8>>) -> Self {
        self.hit_group_entries.push(ShaderBindingTableEntry {
            group_index,
            inline_data: match data {
                Some(data) => data,
                None => Vec::new(),
            },
        });

        self
    }

    fn build(
        &mut self,
        device: &Arc<Device>,
        pipeline: &Arc<Pipeline>,
    ) -> VerboseResult<ShaderBindingTable> {
        let prog_id_size = device
            .physical_device()
            .ray_tracing_properties()
            .shaderGroupHandleSize;

        let ray_gen_entry_size = Self::entry_size(prog_id_size, &self.ray_gen_entries);
        let miss_entry_size = Self::entry_size(prog_id_size, &self.miss_entries);
        let hit_group_entry_size = Self::entry_size(prog_id_size, &self.hit_group_entries);

        let sbt_size = ray_gen_entry_size * self.ray_gen_entries.len() as VkDeviceSize
            + miss_entry_size * self.miss_entries.len() as VkDeviceSize
            + hit_group_entry_size * self.hit_group_entries.len() as VkDeviceSize;

        let group_count =
            self.ray_gen_entries.len() + self.miss_entries.len() + self.hit_group_entries.len();

        let shader_handle_storage =
            pipeline.ray_tracing_shader_group_handles(group_count as u32, prog_id_size)?;

        let mut sbt_data = vec![0; sbt_size as usize];
        let mut offset = 0;

        Self::copy_shader_data(
            &mut sbt_data,
            prog_id_size,
            &mut offset,
            &self.ray_gen_entries,
            ray_gen_entry_size,
            &shader_handle_storage,
        );

        Self::copy_shader_data(
            &mut sbt_data,
            prog_id_size,
            &mut offset,
            &self.miss_entries,
            miss_entry_size,
            &shader_handle_storage,
        );

        Self::copy_shader_data(
            &mut sbt_data,
            prog_id_size,
            &mut offset,
            &self.hit_group_entries,
            hit_group_entry_size,
            &shader_handle_storage,
        );

        let sbt_buffer = Buffer::builder()
            .set_usage(VK_BUFFER_USAGE_TRANSFER_SRC_BIT)
            .set_memory_properties(
                VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT,
            )
            .set_data(&sbt_data)
            .build(device.clone())?;

        let miss_offset = ray_gen_entry_size * self.ray_gen_entries.len() as VkDeviceSize;

        Ok(ShaderBindingTable {
            sbt_buffer,

            miss_offset,
            miss_stride: miss_entry_size,
            hit_group_offset: miss_offset
                + miss_entry_size * self.miss_entries.len() as VkDeviceSize,
            hit_group_stride: hit_group_entry_size,
        })
    }
}

impl ShaderBindingTableBuilder {
    #[inline]
    fn entry_size(prog_id_size: u32, entries: &[ShaderBindingTableEntry]) -> VkDeviceSize {
        let mut max_args = 0;

        for entry in entries {
            max_args = max_args.max(entry.inline_data.len());
        }

        let mut entry_size = prog_id_size as VkDeviceSize + max_args as VkDeviceSize;

        // The entries of the shader binding table must be 16-bytes-aligned
        entry_size = Self::round_up(entry_size, 16);

        entry_size
    }

    #[inline]
    fn round_up(source: u64, value: u64) -> u64 {
        ((source) + (value) - 1) & !((value) - 1)
    }

    #[inline]
    fn copy_shader_data(
        sbt_data: &mut Vec<u8>,
        prog_id_size: u32,
        offset: &mut VkDeviceSize,
        entries: &[ShaderBindingTableEntry],
        entry_size: VkDeviceSize,
        shader_handle_storage: &[u8],
    ) {
        for entry in entries {
            // copy the shader identifier
            {
                let sbt_start = *offset as usize;
                let sbt_end = sbt_start + prog_id_size as usize;

                let shs_start = (entry.group_index * prog_id_size) as usize;
                let shs_end = shs_start + prog_id_size as usize;

                sbt_data[sbt_start..sbt_end]
                    .copy_from_slice(&shader_handle_storage[shs_start..shs_end]);
            }

            // copy data if present
            if !entry.inline_data.is_empty() {
                let tmp_offset = *offset + prog_id_size as VkDeviceSize;

                let sbt_start = tmp_offset as usize;
                let sbt_end = sbt_start + entry.inline_data.len();

                sbt_data[sbt_start..sbt_end].copy_from_slice(&entry.inline_data);
            }

            // increase offset with correct alignment
            *offset += entry_size;
        }
    }
}
