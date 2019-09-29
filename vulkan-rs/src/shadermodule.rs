use utilities::prelude::*;

use crate::impl_vk_handle;
use crate::prelude::*;

use std::fs::File;
use std::io::Read;
use std::sync::Arc;

#[allow(clippy::cast_ptr_alignment)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShaderType {
    None,
    Vertex,
    Fragment,
    Geometry,
    TesselationControl,
    TesselationEvaluation,
    Compute,
    RayGeneration,
    ClosestHit,
    Miss,
    AnyHit,
    Intersection,
}

impl Default for ShaderType {
    fn default() -> Self {
        ShaderType::None
    }
}

#[derive(Debug)]
pub struct ShaderModule {
    device: Arc<Device>,
    shader_module: VkShaderModule,
    shader_type: ShaderType,
}

impl ShaderModule {
    pub fn new(
        device: Arc<Device>,
        path: &str,
        shader_type: ShaderType,
    ) -> VerboseResult<Arc<ShaderModule>> {
        let code = Self::shader_code(path)?;

        Self::from_slice(device, code.as_slice(), shader_type)
    }

    pub fn from_slice(
        device: Arc<Device>,
        code: &[u8],
        shader_type: ShaderType,
    ) -> VerboseResult<Arc<ShaderModule>> {
        let shader_module_ci =
            VkShaderModuleCreateInfo::new(VK_SHADER_MODULE_CREATE_NULL_BIT, code);

        let shader_module = device.create_shader_module(&shader_module_ci)?;

        Ok(Arc::new(ShaderModule {
            device,
            shader_module,
            shader_type,
        }))
    }

    fn shader_code(path: &str) -> VerboseResult<Vec<u8>> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(msg) => create_error!(format!("error opening shader file({}): {}", path, msg)),
        };

        let mut code: Vec<u8> = Vec::new();

        if let Err(msg) = file.read_to_end(&mut code).as_ref() {
            create_error!(format!("error reading shader file({}): {}", path, msg));
        }

        Ok(code)
    }

    pub fn shader_type(&self) -> ShaderType {
        self.shader_type
    }

    pub fn pipeline_stage_info(&self) -> VkPipelineShaderStageCreateInfo {
        match self.shader_type {
            ShaderType::None => unimplemented!(),
            ShaderType::Vertex => VkPipelineShaderStageCreateInfo::vertex(self.shader_module),
            ShaderType::Geometry => VkPipelineShaderStageCreateInfo::geometry(self.shader_module),
            ShaderType::TesselationControl => {
                VkPipelineShaderStageCreateInfo::tesselation_control(self.shader_module)
            }
            ShaderType::TesselationEvaluation => {
                VkPipelineShaderStageCreateInfo::tesselation_evaluation(self.shader_module)
            }
            ShaderType::Fragment => VkPipelineShaderStageCreateInfo::fragment(self.shader_module),
            ShaderType::Compute => VkPipelineShaderStageCreateInfo::compute(self.shader_module),
            ShaderType::AnyHit => VkPipelineShaderStageCreateInfo::any_hit(self.shader_module),
            ShaderType::Intersection => {
                VkPipelineShaderStageCreateInfo::intersection(self.shader_module)
            }
            ShaderType::ClosestHit => {
                VkPipelineShaderStageCreateInfo::closest_hit(self.shader_module)
            }
            ShaderType::RayGeneration => {
                VkPipelineShaderStageCreateInfo::ray_generation(self.shader_module)
            }
            ShaderType::Miss => VkPipelineShaderStageCreateInfo::miss(self.shader_module),
        }
    }
}

impl_vk_handle!(ShaderModule, VkShaderModule, shader_module);

impl Drop for ShaderModule {
    fn drop(&mut self) {
        self.device.destroy_shader_module(self.shader_module);
    }
}
