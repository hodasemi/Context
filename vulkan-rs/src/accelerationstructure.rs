use utilities::prelude::*;

use crate::impl_vk_handle;
use crate::prelude::*;

use cgmath::{Matrix, Matrix4, One};

use std::mem;
use std::sync::Arc;

pub struct AccelerationStructureBuilder {
    bottom_level: bool,
    instances: Vec<VkGeometryInstanceNV>,
    geometries: Vec<VkGeometryNV>,
    flags: VkBuildAccelerationStructureFlagBitsNV,
}

impl AccelerationStructureBuilder {
    pub fn add_instance(
        mut self,
        blas: &Arc<AccelerationStructure>,
        transform: Option<Matrix4<f32>>,
        flags: impl Into<VkGeometryInstanceFlagBitsNV>,
    ) -> Self {
        let transposed = match transform {
            Some(transform) => transform.transpose(),
            None => Matrix4::one(),
        };

        let typed: &[f32; 16] = transposed.as_ref();
        let cut: [f32; 12] = AccelerationStructure::clone_into_array(typed);

        self.instances.push(VkGeometryInstanceNV::new(
            cut,
            self.instances.len() as u32,
            0xFF,
            //self.instances.len() as u32,
            0,
            flags,
            blas.handle,
        ));

        self
    }

    pub fn add_vertices<T>(
        mut self,
        vertex_buffer: &Arc<Buffer<T>>,
        transform: Option<Arc<Buffer<Matrix4<f32>>>>,
    ) -> Self {
        self.geometries.push(VkGeometryNV::new(
            VK_GEOMETRY_TYPE_TRIANGLES_NV,
            VkGeometryDataNV {
                triangles: VkGeometryTrianglesNV::new(
                    vertex_buffer.vk_handle(),
                    0,
                    vertex_buffer.size() as u32,
                    mem::size_of::<T>() as VkDeviceSize,
                    VK_FORMAT_R32G32B32_SFLOAT,
                    VkBuffer::NULL_HANDLE,
                    0,
                    0,
                    VK_INDEX_TYPE_NONE_NV,
                    match transform {
                        Some(transform) => transform.vk_handle(),
                        None => VkBuffer::NULL_HANDLE,
                    },
                    0,
                ),
                aabbs: VkGeometryAABBNV::default(),
            },
            VK_GEOMETRY_OPAQUE_BIT_NV,
        ));

        self
    }

    pub fn add_indexed_vertices<T, U>(
        mut self,
        vertex_buffer: &Arc<Buffer<T>>,
        index_buffer: &Arc<Buffer<U>>,
        transform: Option<Arc<Buffer<Matrix4<f32>>>>,
    ) -> Self {
        self.geometries.push(VkGeometryNV::new(
            VK_GEOMETRY_TYPE_TRIANGLES_NV,
            VkGeometryDataNV {
                triangles: VkGeometryTrianglesNV::new(
                    vertex_buffer.vk_handle(),
                    0,
                    vertex_buffer.size() as u32,
                    mem::size_of::<T>() as VkDeviceSize,
                    VK_FORMAT_R32G32B32A32_SFLOAT,
                    index_buffer.vk_handle(),
                    0,
                    index_buffer.size() as u32,
                    VK_INDEX_TYPE_UINT32,
                    match transform {
                        Some(transform) => transform.vk_handle(),
                        None => VkBuffer::NULL_HANDLE,
                    },
                    0,
                ),
                aabbs: VkGeometryAABBNV::default(),
            },
            VK_GEOMETRY_OPAQUE_BIT_NV,
        ));

        self
    }

    pub fn set_flags(mut self, flags: impl Into<VkBuildAccelerationStructureFlagBitsNV>) -> Self {
        self.flags = flags.into();

        self
    }

    pub fn build(self, device: Arc<Device>) -> VerboseResult<Arc<AccelerationStructure>> {
        // check for correct use
        if cfg!(debug_assertions) {
            if self.bottom_level {
                // in blas, instances must be zero
                if !self.instances.is_empty() {
                    create_error!("Instances are not allowed inside of a bottom level as!");
                }

                if self.geometries.is_empty() {
                    create_error!("No geometries added to blas!");
                }
            } else {
                // in tlas, geometries must be zero
                if !self.geometries.is_empty() {
                    create_error!("Geometries are not allowed inside of a top level as!");
                }

                if self.instances.is_empty() {
                    create_error!("No instances added to tlas!");
                }
            }

            let ray_tracing_properties = device.physical_device().ray_tracing_properties();

            if self.instances.len() as u64 > ray_tracing_properties.maxInstanceCount {
                create_error!("More than supported instances added");
            }

            if self.geometries.len() as u64 > ray_tracing_properties.maxGeometryCount {
                create_error!("More than supported geometries added");
            }

            let mut total_triangle_count = 0;

            for geometry in &self.geometries {
                if let VK_GEOMETRY_TYPE_TRIANGLES_NV = geometry.geometryType {
                    let triangles = &geometry.geometry.triangles;

                    // if there is no index buffer set, use vertex buffer size
                    if triangles.indexCount == 0 {
                        total_triangle_count += triangles.vertexCount;
                    } else {
                        total_triangle_count += triangles.indexCount;
                    }
                }
            }

            if total_triangle_count as u64 > ray_tracing_properties.maxTriangleCount {
                create_error!("More than supported triangles added");
            }

            // only one of these can be used at a time
            if (self.flags & VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV) != 0
                && (self.flags & VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV) != 0
            {
                create_error!("Either VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV or VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV can be enabled at a time!");
            }
        }

        let acceleration_structure_info = if self.bottom_level {
            VkAccelerationStructureInfoNV::bottom_level(
                VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV,
                self.instances.len() as u32,
                &self.geometries,
            )
        } else {
            VkAccelerationStructureInfoNV::top_level(
                VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV,
                self.instances.len() as u32,
                &self.geometries,
            )
        };

        let as_ci =
            VkAccelerationStructureCreateInfoNV::new(0, acceleration_structure_info.clone());

        let acceleration_structure = device.create_acceleration_structure(&as_ci)?;

        let scratch_size =
            AccelerationStructure::scratch_buffer_size(&device, acceleration_structure);
        let result_buffer =
            AccelerationStructure::create_result_buffer(&device, acceleration_structure)?;

        let handle = device.acceleration_structure_handle(acceleration_structure)?;

        Ok(Arc::new(AccelerationStructure {
            device: device.clone(),

            info: acceleration_structure_info,
            _geometries: self.geometries,

            acceleration_structure,
            handle,

            result_buffer,

            scratch_size,
            geometry_instances: if self.instances.is_empty() {
                None
            } else {
                Some(self.instances)
            },
        }))
    }
}

pub struct AccelerationStructure {
    device: Arc<Device>,
    info: VkAccelerationStructureInfoNV,
    _geometries: Vec<VkGeometryNV>,

    acceleration_structure: VkAccelerationStructureNV,
    handle: u64,

    result_buffer: Arc<Buffer<u8>>,

    scratch_size: VkDeviceSize,
    geometry_instances: Option<Vec<VkGeometryInstanceNV>>,
}

impl AccelerationStructure {
    pub fn bottom_level() -> AccelerationStructureBuilder {
        AccelerationStructureBuilder {
            bottom_level: true,
            instances: Vec::new(),
            geometries: Vec::new(),
            flags: VkBuildAccelerationStructureFlagBitsNV::default(),
        }
    }

    pub fn top_level() -> AccelerationStructureBuilder {
        AccelerationStructureBuilder {
            bottom_level: false,
            instances: Vec::new(),
            geometries: Vec::new(),
            flags: VkBuildAccelerationStructureFlagBitsNV::default(),
        }
    }

    pub fn result_buffer(&self) -> &Arc<Buffer<u8>> {
        &self.result_buffer
    }

    pub fn generate(&self, command_buffer: &Arc<CommandBuffer>) -> VerboseResult<()> {
        let instances_buffer = match &self.geometry_instances {
            Some(instances) => Some(Self::create_instances_buffer(&self.device, instances)?),
            None => None,
        };

        let scratch_buffer = Self::create_scratch_buffer(&self.device, self.scratch_size)?;

        command_buffer.build_acceleration_structure(
            &self.info,
            &instances_buffer,
            self,
            None,
            &scratch_buffer,
        );

        command_buffer.memory_barrier(
            VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_NV
                | VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_NV,
            VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_NV,
            VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_NV
                | VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_NV,
            VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_NV,
        );

        Ok(())
    }
}

impl VulkanDevice for AccelerationStructure {
    fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

impl_vk_handle!(
    AccelerationStructure,
    VkAccelerationStructureNV,
    acceleration_structure
);

// impl VkHandle<VkAccelerationStructureNV> for Arc<AccelerationStructure> {
//     fn vk_handle(&self) -> VkAccelerationStructureNV {
//         self.acceleration_structure
//     }
// }

// impl<'a> VkHandle<VkAccelerationStructureNV> for &'a Arc<AccelerationStructure> {
//     fn vk_handle(&self) -> VkAccelerationStructureNV {
//         self.acceleration_structure
//     }
// }

// impl VkHandle<VkAccelerationStructureNV> for AccelerationStructure {
//     fn vk_handle(&self) -> VkAccelerationStructureNV {
//         self.acceleration_structure
//     }
// }

// impl<'a> VkHandle<VkAccelerationStructureNV> for &'a AccelerationStructure {
//     fn vk_handle(&self) -> VkAccelerationStructureNV {
//         self.acceleration_structure
//     }
// }

// private helper functions
impl AccelerationStructure {
    #[inline]
    fn create_instances_buffer(
        device: &Arc<Device>,
        geometry_instances: &[VkGeometryInstanceNV],
    ) -> VerboseResult<Arc<Buffer<VkGeometryInstanceNV>>> {
        Buffer::builder()
            .set_usage(VK_BUFFER_USAGE_RAY_TRACING_BIT_NV)
            .set_memory_properties(
                VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT,
            )
            .set_data(geometry_instances)
            .build(device.clone())
    }

    #[inline]
    fn scratch_buffer_size(
        device: &Arc<Device>,
        acceleration_structure: VkAccelerationStructureNV,
    ) -> VkDeviceSize {
        let build_memory_requirements = Self::memory_requirements(
            device,
            acceleration_structure,
            VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV,
        )
        .memoryRequirements;

        let update_memory_requirements = Self::memory_requirements(
            device,
            acceleration_structure,
            VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV,
        )
        .memoryRequirements;

        debug_assert_eq!(
            build_memory_requirements.alignment,
            update_memory_requirements.alignment
        );
        debug_assert_eq!(
            build_memory_requirements.memoryTypeBits,
            update_memory_requirements.memoryTypeBits
        );

        // make scratch size the maximum of both values
        build_memory_requirements
            .size
            .max(update_memory_requirements.size)
    }

    #[inline]
    fn create_scratch_buffer(
        device: &Arc<Device>,
        scratch_size_in_bytes: VkDeviceSize,
    ) -> VerboseResult<Arc<Buffer<u8>>> {
        // create scratch buffer
        Buffer::builder()
            .set_usage(VK_BUFFER_USAGE_RAY_TRACING_BIT_NV)
            .set_memory_properties(VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT)
            .set_size(scratch_size_in_bytes)
            .build(device.clone())
    }

    #[inline]
    fn create_result_buffer(
        device: &Arc<Device>,
        acceleration_structure: VkAccelerationStructureNV,
    ) -> VerboseResult<Arc<Buffer<u8>>> {
        // get buffer size information
        let memory_requirements = Self::memory_requirements(
            device,
            acceleration_structure,
            VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV,
        );

        let result_size_in_bytes = memory_requirements.memoryRequirements.size;

        // create result buffer
        let result_buffer = Buffer::builder()
            .set_usage(VK_BUFFER_USAGE_RAY_TRACING_BIT_NV)
            .set_memory_properties(VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT)
            .set_size(result_size_in_bytes)
            .build(device.clone())?;

        // bind the result buffer memory to the acceleration structure
        Self::bind_to_memory(device, acceleration_structure, &result_buffer)?;

        Ok(result_buffer)
    }

    #[inline]
    fn bind_to_memory(
        device: &Arc<Device>,
        acceleration_structure: VkAccelerationStructureNV,
        buffer: &Arc<Buffer<impl Copy>>,
    ) -> VerboseResult<()> {
        let bind_info = VkBindAccelerationStructureMemoryInfoNV::new(
            acceleration_structure,
            buffer.vk_handle(),
            buffer.offset(),
            &[],
        );

        device.bind_acceleration_structure_memory(&[bind_info])
    }

    #[inline]
    fn memory_requirements(
        device: &Arc<Device>,
        acceleration_structure: VkAccelerationStructureNV,
        r#type: VkAccelerationStructureMemoryRequirementsTypeNV,
    ) -> VkMemoryRequirements2KHR {
        let info =
            VkAccelerationStructureMemoryRequirementsInfoNV::new(r#type, acceleration_structure);

        device.acceleration_structure_memory_requirements(&info)
    }

    #[inline]
    fn clone_into_array(slice: &[f32]) -> [f32; 12] {
        let mut target = [0.0; 12];
        target.clone_from_slice(&slice[0..12]);
        target
    }
}

impl Drop for AccelerationStructure {
    fn drop(&mut self) {
        self.device
            .destroy_acceleration_structure(self.acceleration_structure);
    }
}
