
use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::os::raw::c_void;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkPipeline(u64);
SetupU64Conv!(VkPipeline);

/*
impl VkPipeline {
    pub fn create_graphics(
        device: VkDevice,
        pipeline_cache: Option<VkPipelineCache>,
        pipeline_create_infos: &[VkGraphicsPipelineCreateInfo],
    ) -> Result<Vec<VkPipeline>, VkResult> {
        unsafe {
            let count = pipeline_create_infos.len() as usize;

            let mut pipelines = Vec::with_capacity(count);
            pipelines.set_len(count);

            let result = vkCreateGraphicsPipelines(
                device,
                match pipeline_cache {
                    Some(cache) => cache,
                    None => VkPipelineCache::default(),
                },
                pipeline_create_infos.len() as u32,
                pipeline_create_infos.as_ptr(),
                ptr::null(),
                pipelines.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(pipelines)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_graphics_with_allocation_callbacks(
        device: VkDevice,
        pipeline_cache: Option<VkPipelineCache>,
        pipeline_create_infos: &[VkGraphicsPipelineCreateInfo],
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<Vec<VkPipeline>, VkResult> {
        unsafe {
            let count = pipeline_create_infos.len() as usize;

            let mut pipelines = Vec::with_capacity(count);
            pipelines.set_len(count);

            let result = vkCreateGraphicsPipelines(
                device,
                match pipeline_cache {
                    Some(cache) => cache,
                    None => VkPipelineCache::default(),
                },
                pipeline_create_infos.len() as u32,
                pipeline_create_infos.as_ptr(),
                allocation_callbacks,
                pipelines.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(pipelines)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_compute(
        device: VkDevice,
        pipeline_cache: Option<VkPipelineCache>,
        pipeline_create_infos: &[VkComputePipelineCreateInfo],
    ) -> Result<Vec<VkPipeline>, VkResult> {
        unsafe {
            let count = pipeline_create_infos.len() as usize;

            let mut pipelines = Vec::with_capacity(count);
            pipelines.set_len(count);

            let result = vkCreateComputePipelines(
                device,
                match pipeline_cache {
                    Some(cache) => cache,
                    None => VkPipelineCache::default(),
                },
                pipeline_create_infos.len() as u32,
                pipeline_create_infos.as_ptr(),
                ptr::null(),
                pipelines.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(pipelines)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_compute_with_allocation_callbacks(
        device: VkDevice,
        pipeline_cache: Option<VkPipelineCache>,
        pipeline_create_infos: &[VkComputePipelineCreateInfo],
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<Vec<VkPipeline>, VkResult> {
        unsafe {
            let count = pipeline_create_infos.len() as usize;

            let mut pipelines = Vec::with_capacity(count);
            pipelines.set_len(count);

            let result = vkCreateComputePipelines(
                device,
                match pipeline_cache {
                    Some(cache) => cache,
                    None => VkPipelineCache::default(),
                },
                pipeline_create_infos.len() as u32,
                pipeline_create_infos.as_ptr(),
                allocation_callbacks,
                pipelines.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(pipelines)
            } else {
                Err(result)
            }
        }
    }

    pub fn create_ray_tracing(
        device: VkDevice,
        pipeline_cache: Option<VkPipelineCache>,
        pipeline_create_infos: &[VkRayTracingPipelineCreateInfoNV],
    ) -> Result<Vec<VkPipeline>, VkResult> {
        unsafe {
            match vkCreateRayTracingPipelinesNV {
                Some(pfn) => {
                    let count = pipeline_create_infos.len() as usize;

                    let mut pipelines = Vec::with_capacity(count);
                    pipelines.set_len(count);

                    let result = pfn(
                        device,
                        match pipeline_cache {
                            Some(cache) => cache,
                            None => VkPipelineCache::default(),
                        },
                        pipeline_create_infos.len() as u32,
                        pipeline_create_infos.as_ptr(),
                        ptr::null(),
                        pipelines.as_mut_ptr(),
                    );

                    if result == VK_SUCCESS {
                        Ok(pipelines)
                    } else {
                        Err(result)
                    }
                }
                None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
            }
        }
    }

    pub fn create_ray_tracing_with_allocation_callbacks(
        device: VkDevice,
        pipeline_cache: Option<VkPipelineCache>,
        pipeline_create_infos: &[VkRayTracingPipelineCreateInfoNV],
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<Vec<VkPipeline>, VkResult> {
        unsafe {
            match vkCreateRayTracingPipelinesNV {
                Some(pfn) => {
                    let count = pipeline_create_infos.len() as usize;

                    let mut pipelines = Vec::with_capacity(count);
                    pipelines.set_len(count);

                    let result = pfn(
                        device,
                        match pipeline_cache {
                            Some(cache) => cache,
                            None => VkPipelineCache::default(),
                        },
                        pipeline_create_infos.len() as u32,
                        pipeline_create_infos.as_ptr(),
                        allocation_callbacks,
                        pipelines.as_mut_ptr(),
                    );

                    if result == VK_SUCCESS {
                        Ok(pipelines)
                    } else {
                        Err(result)
                    }
                }
                None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
            }
        }
    }

    pub fn get_ray_tracing_shader_group_handles(
        &self,
        device: VkDevice,
        first_group: u32,
        group_count: u32,
        shader_group_handle_size: u32,
    ) -> Result<Vec<u8>, VkResult> {
        unsafe {
            match vkGetRayTracingShaderGroupHandlesNV {
                Some(pfn) => {
                    let mut data = vec![0; (group_count * shader_group_handle_size) as usize];

                    let result = pfn(
                        device,
                        *self,
                        first_group,
                        group_count,
                        data.len(),
                        data.as_mut_ptr() as *mut c_void,
                    );

                    if result == VK_SUCCESS {
                        Ok(data)
                    } else {
                        Err(result)
                    }
                }
                None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
            }
        }
    }

    pub fn compile_deferred(&self, device: VkDevice, shader: u32) -> Result<(), VkResult> {
        unsafe {
            match vkCompileDeferredNV {
                Some(pfn) => {
                    let result = pfn(device, *self, shader);

                    if result == VK_SUCCESS {
                        Ok(())
                    } else {
                        Err(result)
                    }
                }
                None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
            }
        }
    }

    pub fn destroy(&self, device: VkDevice) {
        unsafe { vkDestroyPipeline(device, *self, ptr::null()) };
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        device: VkDevice,
        allocation_callbacks: &VkAllocationCallbacks,
    ) {
        unsafe { vkDestroyPipeline(device, *self, allocation_callbacks) };
    }
}
*/
