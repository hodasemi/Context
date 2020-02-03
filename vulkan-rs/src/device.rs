use utilities::prelude::*;

use crate::impl_vk_handle;
use crate::loader::*;
use crate::mappedmemory::VkMappedMemory;
use crate::prelude::*;
use crate::Extensions;

use std::cmp::min;
use std::fmt;
use std::mem::{size_of, MaybeUninit};
use std::ptr;
use std::slice;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use core::ffi::c_void;

Extensions!(DeviceExtensions, {
    (nv_ray_tracing, "VK_NV_ray_tracing"),
    (amd_rasterization_order, "VK_AMD_rasterization_order"),
    (maintenance3, "VK_KHR_maintenance3"),
    (descriptor_indexing, "VK_EXT_descriptor_indexing"),
    (memory_requirements2, "VK_KHR_get_memory_requirements2"),
    (swapchain, "VK_KHR_swapchain"),
    (memory_budget, "VK_EXT_memory_budget"),
    (memory_priority, "VK_EXT_memory_priority"),
    (debug_marker, "VK_EXT_debug_marker"),
});

pub struct MemoryHeap {
    pub usage: VkDeviceSize,
    pub budget: VkDeviceSize,
}

pub struct Device {
    device_functions: DeviceFunctions,
    device_wsi_functions: DeviceWSIFunctions,
    maintenance3_functions: Maintenance3Functions,

    nv_ray_tracing_functions: NVRayTracingFunctions,

    enabled_extensions: DeviceExtensions,

    physical_device: Arc<PhysicalDevice>,
    device: VkDevice,
}

impl Device {
    pub fn new(
        physical_device: Arc<PhysicalDevice>,
        extensions: DeviceExtensions,
        queue_infos: &[VkDeviceQueueCreateInfo],
    ) -> VerboseResult<Arc<Device>> {
        let device_extensions = physical_device.extensions();

        let mut checked_extensions = Vec::new();

        let extension_list = extensions.as_list();

        for extension in extension_list {
            for ext_prop in device_extensions {
                if *ext_prop == extension {
                    checked_extensions.push(extension);
                    break;
                }
            }
        }

        let names = VkNames::new(checked_extensions.as_slice());

        println!("\nenabled device extensions ({}):", names.len());

        for extension_name in names.iter() {
            println!("\t- {:?}", extension_name);
        }

        println!();

        let local_device_features = physical_device.features();

        let mut device_ci = VkDeviceCreateInfo::new(
            VK_DEVICE_CREATE_NULL_BIT,
            queue_infos,
            &names,
            slice::from_ref(&local_device_features),
        );

        let descriptor_indexing_features = physical_device.descriptor_indexing_features();

        device_ci.chain(&descriptor_indexing_features);

        let instance = physical_device.instance();

        let device = instance.create_device(physical_device.vk_handle(), &device_ci)?;

        let device_functions = load_device(
            |device, name| instance.get_device_proc_addr_raw(device, name),
            device,
        );
        let device_wsi_functions = load_device_wsi(
            |device, name| instance.get_device_proc_addr_raw(device, name),
            device,
        );
        let nv_ray_tracing_functions = load_nv_ray_tracing(
            |device, name| instance.get_device_proc_addr_raw(device, name),
            device,
        );
        let maintenance3_functions = load_maintenance3(
            |device, name| instance.get_device_proc_addr_raw(device, name),
            device,
        );

        let enabled_extensions = DeviceExtensions::from_list(&checked_extensions);

        if let Err(missing_extensions) = extensions.check_availability(&enabled_extensions) {
            for m in missing_extensions {
                println!("{}", m);
            }
        }

        Ok(Arc::new(Device {
            device_functions,
            device_wsi_functions,
            maintenance3_functions,

            nv_ray_tracing_functions,

            enabled_extensions,

            physical_device,
            device,
        }))
    }

    pub fn get_queue(
        device: &Arc<Device>,
        queue_family_index: u32,
        queue_index: u32,
    ) -> Arc<Mutex<Queue>> {
        Queue::new(
            device.clone(),
            device.get_device_queue(queue_family_index, queue_index),
            queue_family_index,
            queue_index,
        )
    }

    pub fn memory_type_from_properties(
        &self,
        device_reqs: impl Into<VkMemoryPropertyFlagBits>,
        host_reqs: impl Into<VkMemoryPropertyFlagBits>,
    ) -> VerboseResult<u32> {
        let memory_types = self.physical_device().memory_properties().memory_types();
        let device_requirements = device_reqs.into();
        let host_requirements = host_reqs.into();

        for (i, memory_type) in memory_types.iter().enumerate() {
            if (device_requirements & (1u32 << i)) != 0
                && host_requirements == (memory_type.propertyFlagBits & host_requirements)
            {
                return Ok(i as u32);
            }
        }

        create_error!("failed finding device requirements")
    }

    pub fn physical_device(&self) -> &Arc<PhysicalDevice> {
        &self.physical_device
    }

    pub fn wait_for_fences(
        &self,
        fences: &[&Arc<Fence>],
        wait_all: bool,
        timeout: Duration,
    ) -> VerboseResult<()> {
        let vkfences: Vec<VkFence> = fences.iter().map(|fence| fence.vk_handle()).collect();

        self.device_wait_for_fences(vkfences.as_slice(), wait_all, timeout.as_nanos() as u64)?;

        Ok(())
    }

    pub fn enabled_extensions(&self) -> &DeviceExtensions {
        &self.enabled_extensions
    }

    pub fn memory_budgets(&self) -> Vec<MemoryHeap> {
        let phys_dev = self.physical_device();

        let (budget, count) = phys_dev
            .instance()
            .physical_device_memory_budget(phys_dev.vk_handle());

        let mut heaps = Vec::with_capacity(count as usize);
        let usages = budget.heap_usages(count);
        let budgets = budget.heap_budgets(count);

        for i in 0..count {
            heaps.push(MemoryHeap {
                usage: usages[i as usize],
                budget: budgets[i as usize],
            })
        }

        heaps
    }

    pub fn max_supported_sample_count(
        &self,
        requested_sample_count: VkSampleCountFlags,
    ) -> VkSampleCountFlags {
        let dev_props = self.physical_device.properties();

        let phys_counts = min(
            dev_props.limits.framebufferColorSampleCounts,
            dev_props.limits.framebufferDepthSampleCounts,
        );

        let counts = min(phys_counts, requested_sample_count.into());

        if (counts & VK_SAMPLE_COUNT_64_BIT) != 0 {
            VK_SAMPLE_COUNT_64_BIT
        } else if (counts & VK_SAMPLE_COUNT_32_BIT) != 0 {
            VK_SAMPLE_COUNT_32_BIT
        } else if (counts & VK_SAMPLE_COUNT_16_BIT) != 0 {
            VK_SAMPLE_COUNT_16_BIT
        } else if (counts & VK_SAMPLE_COUNT_8_BIT) != 0 {
            VK_SAMPLE_COUNT_8_BIT
        } else if (counts & VK_SAMPLE_COUNT_4_BIT) != 0 {
            VK_SAMPLE_COUNT_4_BIT
        } else if (counts & VK_SAMPLE_COUNT_2_BIT) != 0 {
            VK_SAMPLE_COUNT_2_BIT
        } else {
            VK_SAMPLE_COUNT_1_BIT
        }
    }
}

impl_vk_handle!(Device, VkDevice, device);

impl fmt::Debug for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Device {{ device: {:#?}, physical_device: {:#?} }}",
            self.device, self.physical_device
        )
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        self.destroy_device();
    }
}

impl Device {
    pub fn device_proc_addr(&self, name: VkString) -> PFN_vkVoidFunction {
        self.physical_device
            .instance()
            .get_device_proc_addr(self.device, name)
    }

    fn destroy_device(&self) {
        unsafe {
            self.device_functions
                .vkDestroyDevice(self.device, ptr::null());
        }
    }

    fn get_device_queue(&self, queue_family_index: u32, queue_index: u32) -> VkQueue {
        unsafe {
            let mut queue = MaybeUninit::uninit();

            self.device_functions.vkGetDeviceQueue(
                self.device,
                queue_family_index,
                queue_index,
                queue.as_mut_ptr(),
            );

            queue.assume_init()
        }
    }

    fn device_wait_for_fences(
        &self,
        fences: &[VkFence],
        wait_all: impl Into<VkBool32>,
        timeout: u64,
    ) -> VerboseResult<()> {
        unsafe {
            let result = self.device_functions.vkWaitForFences(
                self.device,
                fences.len() as u32,
                fences.as_ptr(),
                wait_all.into(),
                timeout,
            );

            if result == VK_SUCCESS {
                Ok(())
            } else {
                create_error!(format!("failed waiting for fences {:?}", result))
            }
        }
    }

    pub fn query_pool_results<T>(
        &self,
        query_pool: VkQueryPool,
        first_query: u32,
        query_count: u32,
        data: &mut T,
        stride: VkDeviceSize,
        flags: impl Into<VkQueryResultFlagBits>,
    ) -> VerboseResult<()> {
        unsafe {
            let result = self.device_functions.vkGetQueryPoolResults(
                self.device,
                query_pool,
                first_query,
                query_count,
                size_of::<T>(),
                data as *mut T as *mut c_void,
                stride,
                flags.into(),
            );

            if result == VK_SUCCESS {
                Ok(())
            } else {
                create_error!(format!("failed getting query pool results {:?}", result))
            }
        }
    }

    pub fn queue_submit(
        &self,
        queue: VkQueue,
        submits: &[VkSubmitInfo],
        fence: VkFence,
    ) -> VerboseResult<()> {
        unsafe {
            let result = self.device_functions.vkQueueSubmit(
                queue,
                submits.len() as u32,
                submits.as_ptr(),
                fence,
            );

            if result == VK_SUCCESS {
                Ok(())
            } else {
                create_error!(format!("failed submitting to queue {:?}", result))
            }
        }
    }

    pub fn queue_wait_idle(&self, queue: VkQueue) -> VerboseResult<()> {
        unsafe {
            let result = self.device_functions.vkQueueWaitIdle(queue);

            if result == VK_SUCCESS {
                Ok(())
            } else {
                create_error!(format!("failed waiting for queue idling {:?}", result))
            }
        }
    }

    pub fn create_buffer(&self, create_info: &VkBufferCreateInfo) -> VerboseResult<VkBuffer> {
        unsafe {
            let mut buffer = MaybeUninit::uninit();

            let result = self.device_functions.vkCreateBuffer(
                self.device,
                create_info,
                ptr::null(),
                buffer.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(buffer.assume_init())
            } else {
                create_error!(format!("failed creating buffer {:?}", result))
            }
        }
    }

    pub fn destroy_buffer(&self, buffer: VkBuffer) {
        unsafe {
            self.device_functions
                .vkDestroyBuffer(self.device, buffer, ptr::null())
        };
    }

    pub fn buffer_memory_requirements(&self, buffer: VkBuffer) -> VkMemoryRequirements {
        unsafe {
            let mut memory_requirements = MaybeUninit::uninit();

            self.device_functions.vkGetBufferMemoryRequirements(
                self.device,
                buffer,
                memory_requirements.as_mut_ptr(),
            );

            memory_requirements.assume_init()
        }
    }

    pub fn allocate_memory(
        &self,
        allocate_info: &VkMemoryAllocateInfo,
    ) -> VerboseResult<VkDeviceMemory> {
        unsafe {
            let mut memory = MaybeUninit::uninit();

            let result = self.device_functions.vkAllocateMemory(
                self.device,
                allocate_info,
                ptr::null(),
                memory.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(memory.assume_init())
            } else {
                create_error!(format!("failed allocating memory {:?}", result))
            }
        }
    }

    pub fn free_memory(&self, memory: VkDeviceMemory) {
        unsafe {
            self.device_functions
                .vkFreeMemory(self.device, memory, ptr::null())
        };
    }

    pub fn map_memory<U: Clone>(
        &self,
        memory: VkDeviceMemory,
        offset: VkDeviceSize,
        length: VkDeviceSize,
        flags: impl Into<VkMemoryMapFlags>,
    ) -> VerboseResult<VkMappedMemory<'_, U>> {
        unsafe {
            let mut data = MaybeUninit::uninit();

            let size = length * size_of::<U>() as VkDeviceSize;

            let result = self.device_functions.vkMapMemory(
                self.device,
                memory,
                offset,
                size,
                flags.into(),
                data.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                let slice =
                    slice::from_raw_parts_mut(data.assume_init() as *mut U, length as usize);
                Ok(VkMappedMemory::new(self, memory, slice))
            } else {
                create_error!(format!("failed mapping memory {:?}", result))
            }
        }
    }

    pub fn unmap_memory(&self, memory: VkDeviceMemory) {
        unsafe { self.device_functions.vkUnmapMemory(self.device, memory) };
    }

    pub fn bind_buffer_memory(
        &self,
        buffer: VkBuffer,
        memory: VkDeviceMemory,
        offset: VkDeviceSize,
    ) -> VerboseResult<()> {
        unsafe {
            let result =
                self.device_functions
                    .vkBindBufferMemory(self.device, buffer, memory, offset);

            if result == VK_SUCCESS {
                Ok(())
            } else {
                create_error!(format!("failed binding buffer to memory {:?}", result))
            }
        }
    }

    pub fn create_render_pass(
        &self,
        create_info: &VkRenderPassCreateInfo,
    ) -> VerboseResult<VkRenderPass> {
        unsafe {
            let mut render_pass = MaybeUninit::uninit();

            let result = self.device_functions.vkCreateRenderPass(
                self.device,
                create_info,
                ptr::null(),
                render_pass.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(render_pass.assume_init())
            } else {
                create_error!(format!("failed creating render pass {:?}", result))
            }
        }
    }

    pub fn destroy_render_pass(&self, render_pass: VkRenderPass) {
        unsafe {
            self.device_functions
                .vkDestroyRenderPass(self.device, render_pass, ptr::null())
        };
    }

    pub fn create_image(&self, create_info: &VkImageCreateInfo) -> VerboseResult<VkImage> {
        unsafe {
            let mut image = MaybeUninit::uninit();

            let result = self.device_functions.vkCreateImage(
                self.device,
                create_info,
                ptr::null(),
                image.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(image.assume_init())
            } else {
                create_error!(format!("failed creating image {:?}", result))
            }
        }
    }

    pub fn destroy_image(&self, image: VkImage) {
        unsafe {
            self.device_functions
                .vkDestroyImage(self.device, image, ptr::null())
        };
    }

    pub fn image_subresource_layout(
        &self,
        image: VkImage,
        subresource: &VkImageSubresource,
    ) -> VkSubresourceLayout {
        unsafe {
            let mut subresource_layout = MaybeUninit::uninit();

            self.device_functions.vkGetImageSubresourceLayout(
                self.device,
                image,
                subresource,
                subresource_layout.as_mut_ptr(),
            );

            subresource_layout.assume_init()
        }
    }

    pub fn create_image_view(
        &self,
        create_info: &VkImageViewCreateInfo,
    ) -> VerboseResult<VkImageView> {
        unsafe {
            let mut image_view = MaybeUninit::uninit();

            let result = self.device_functions.vkCreateImageView(
                self.device,
                create_info,
                ptr::null(),
                image_view.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(image_view.assume_init())
            } else {
                create_error!(format!("failed creating image view {:?}", result))
            }
        }
    }

    pub fn destroy_image_view(&self, image_view: VkImageView) {
        unsafe {
            self.device_functions
                .vkDestroyImageView(self.device, image_view, ptr::null())
        };
    }

    pub fn image_memory_requirements(&self, image: VkImage) -> VkMemoryRequirements {
        unsafe {
            let mut memory_requirements = MaybeUninit::uninit();

            self.device_functions.vkGetImageMemoryRequirements(
                self.device,
                image,
                memory_requirements.as_mut_ptr(),
            );

            memory_requirements.assume_init()
        }
    }

    pub fn image_sparse_memory_requirements(
        &self,
        image: VkImage,
    ) -> Vec<VkSparseImageMemoryRequirements> {
        let mut count: u32 = 0;

        unsafe {
            self.device_functions.vkGetImageSparseMemoryRequirements(
                self.device,
                image,
                &mut count,
                ptr::null_mut(),
            )
        };

        let mut sparse_memory_requirements = Vec::with_capacity(count as usize);
        unsafe { sparse_memory_requirements.set_len(count as usize) };

        unsafe {
            self.device_functions.vkGetImageSparseMemoryRequirements(
                self.device,
                image,
                &mut count,
                sparse_memory_requirements.as_mut_ptr(),
            )
        };

        sparse_memory_requirements
    }

    pub fn bind_image_memory(
        &self,
        image: VkImage,
        memory: VkDeviceMemory,
        offset: VkDeviceSize,
    ) -> VerboseResult<()> {
        unsafe {
            let result =
                self.device_functions
                    .vkBindImageMemory(self.device, image, memory, offset);

            if result == VK_SUCCESS {
                Ok(())
            } else {
                create_error!(format!("failed binding image to memory {:?}", result))
            }
        }
    }

    pub fn create_sampler(&self, create_info: &VkSamplerCreateInfo) -> VerboseResult<VkSampler> {
        unsafe {
            let mut sampler = MaybeUninit::uninit();

            let result = self.device_functions.vkCreateSampler(
                self.device,
                create_info,
                ptr::null(),
                sampler.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(sampler.assume_init())
            } else {
                create_error!(format!("failed creating sampler {:?}", result))
            }
        }
    }

    pub fn destroy_sampler(&self, sampler: VkSampler) {
        unsafe {
            self.device_functions
                .vkDestroySampler(self.device, sampler, ptr::null())
        };
    }

    pub fn create_buffer_view(
        &self,
        create_info: &VkBufferViewCreateInfo,
    ) -> VerboseResult<VkBufferView> {
        unsafe {
            let mut buffer_view = MaybeUninit::uninit();

            let result = self.device_functions.vkCreateBufferView(
                self.device,
                create_info,
                ptr::null(),
                buffer_view.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(buffer_view.assume_init())
            } else {
                create_error!(format!("failed creating buffer view {:?}", result))
            }
        }
    }

    pub fn destroy_buffer_view(&self, buffer_view: VkBufferView) {
        unsafe {
            self.device_functions
                .vkDestroyBufferView(self.device, buffer_view, ptr::null())
        };
    }

    pub fn create_fence(&self, create_info: &VkFenceCreateInfo) -> VerboseResult<VkFence> {
        unsafe {
            let mut fence = MaybeUninit::uninit();

            let result = self.device_functions.vkCreateFence(
                self.device,
                create_info,
                ptr::null(),
                fence.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(fence.assume_init())
            } else {
                create_error!(format!("failed creating fence {:?}", result))
            }
        }
    }

    pub fn destroy_fence(&self, fence: VkFence) {
        unsafe {
            self.device_functions
                .vkDestroyFence(self.device, fence, ptr::null())
        };
    }

    pub fn reset_fences(&self, fences: &[VkFence]) -> VerboseResult<()> {
        unsafe {
            let result = self.device_functions.vkResetFences(
                self.device,
                fences.len() as u32,
                fences.as_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(())
            } else {
                create_error!(format!("failed resetting fences {:?}", result))
            }
        }
    }

    pub fn create_semaphore(
        &self,
        create_info: &VkSemaphoreCreateInfo,
    ) -> VerboseResult<VkSemaphore> {
        unsafe {
            let mut semaphore = MaybeUninit::uninit();

            let result = self.device_functions.vkCreateSemaphore(
                self.device,
                create_info,
                ptr::null(),
                semaphore.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(semaphore.assume_init())
            } else {
                create_error!(format!("failed creating semaphore {:?}", result))
            }
        }
    }

    pub fn destroy_semaphore(&self, semaphore: VkSemaphore) {
        unsafe {
            self.device_functions
                .vkDestroySemaphore(self.device, semaphore, ptr::null())
        };
    }

    pub fn create_shader_module(
        &self,
        create_info: &VkShaderModuleCreateInfo,
    ) -> VerboseResult<VkShaderModule> {
        unsafe {
            let mut shader_module = MaybeUninit::uninit();

            let result = self.device_functions.vkCreateShaderModule(
                self.device,
                create_info,
                ptr::null(),
                shader_module.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(shader_module.assume_init())
            } else {
                create_error!(format!("failed creating shader module {:?}", result))
            }
        }
    }

    pub fn destroy_shader_module(&self, shader_module: VkShaderModule) {
        unsafe {
            self.device_functions
                .vkDestroyShaderModule(self.device, shader_module, ptr::null())
        };
    }

    pub fn create_descriptor_pool(
        &self,
        create_info: &VkDescriptorPoolCreateInfo,
    ) -> VerboseResult<VkDescriptorPool> {
        unsafe {
            let mut descriptor_pool = MaybeUninit::uninit();

            let result = self.device_functions.vkCreateDescriptorPool(
                self.device,
                create_info,
                ptr::null(),
                descriptor_pool.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(descriptor_pool.assume_init())
            } else {
                create_error!(format!("failed creating descriptor pool {:?}", result))
            }
        }
    }

    pub fn destroy_descriptor_pool(&self, descriptor_pool: VkDescriptorPool) {
        unsafe {
            self.device_functions
                .vkDestroyDescriptorPool(self.device, descriptor_pool, ptr::null())
        };
    }

    pub fn reset_descriptor_pool<T>(
        &self,
        descriptor_pool: VkDescriptorPool,
        flags: T,
    ) -> VerboseResult<()>
    where
        T: Into<VkDescriptorPoolResetFlags>,
    {
        unsafe {
            let result = self.device_functions.vkResetDescriptorPool(
                self.device,
                descriptor_pool,
                flags.into(),
            );

            if result == VK_SUCCESS {
                Ok(())
            } else {
                create_error!(format!("failed resetting descriptor pool {:?}", result))
            }
        }
    }

    pub fn create_descriptor_set_layout(
        &self,
        create_info: &VkDescriptorSetLayoutCreateInfo,
    ) -> VerboseResult<VkDescriptorSetLayout> {
        unsafe {
            let mut descriptor_set_layout = MaybeUninit::uninit();

            let result = self.device_functions.vkCreateDescriptorSetLayout(
                self.device,
                create_info,
                ptr::null(),
                descriptor_set_layout.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(descriptor_set_layout.assume_init())
            } else {
                create_error!(format!(
                    "failed creating descriptor set layout {:?}",
                    result
                ))
            }
        }
    }

    pub fn destroy_descriptor_set_layout(&self, descriptor_set_layout: VkDescriptorSetLayout) {
        unsafe {
            self.device_functions.vkDestroyDescriptorSetLayout(
                self.device,
                descriptor_set_layout,
                ptr::null(),
            )
        };
    }

    pub fn allocate_descriptor_sets<'a>(
        &self,
        allocate_info: &VkDescriptorSetAllocateInfo<'a>,
    ) -> VerboseResult<Vec<VkDescriptorSet>> {
        unsafe {
            let count = allocate_info.descriptorSetCount as usize;

            let mut descriptor_sets = Vec::with_capacity(count);
            descriptor_sets.set_len(count);

            let result = self.device_functions.vkAllocateDescriptorSets(
                self.device,
                allocate_info,
                descriptor_sets.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(descriptor_sets)
            } else {
                create_error!(format!("failed allocating descriptor sets {:?}", result))
            }
        }
    }

    pub fn free_descriptor_sets(
        &self,
        descriptor_pool: VkDescriptorPool,
        descriptor_sets: &[VkDescriptorSet],
    ) -> VerboseResult<()> {
        unsafe {
            let result = self.device_functions.vkFreeDescriptorSets(
                self.device,
                descriptor_pool,
                descriptor_sets.len() as u32,
                descriptor_sets.as_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(())
            } else {
                create_error!(format!("failed freeing descriptor sets {:?}", result))
            }
        }
    }

    pub fn update_descriptor_sets(
        &self,
        writes: &[VkWriteDescriptorSet],
        copies: &[VkCopyDescriptorSet],
    ) {
        unsafe {
            self.device_functions.vkUpdateDescriptorSets(
                self.device,
                writes.len() as u32,
                writes.as_ptr(),
                copies.len() as u32,
                copies.as_ptr(),
            );
        }
    }

    pub fn create_event(&self, create_info: &VkEventCreateInfo) -> VerboseResult<VkEvent> {
        unsafe {
            let mut event = MaybeUninit::uninit();

            let result = self.device_functions.vkCreateEvent(
                self.device,
                create_info,
                ptr::null(),
                event.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(event.assume_init())
            } else {
                create_error!(format!("failed creating event {:?}", result))
            }
        }
    }

    pub fn destroy_event(&self, event: VkEvent) {
        unsafe {
            self.device_functions
                .vkDestroyEvent(self.device, event, ptr::null())
        };
    }

    pub fn event_status(&self, event: VkEvent) -> VerboseResult<()> {
        unsafe {
            let result = self.device_functions.vkGetEventStatus(self.device, event);

            if result == VK_SUCCESS {
                Ok(())
            } else {
                create_error!(format!("failed getting event status {:?}", result))
            }
        }
    }

    pub fn set_event(&self, event: VkEvent) -> VerboseResult<()> {
        unsafe {
            let result = self.device_functions.vkSetEvent(self.device, event);

            if result == VK_SUCCESS {
                Ok(())
            } else {
                create_error!(format!("failed setting event {:?}", result))
            }
        }
    }

    pub fn reset_event(&self, event: VkEvent) -> VerboseResult<()> {
        unsafe {
            let result = self.device_functions.vkResetEvent(self.device, event);

            if result == VK_SUCCESS {
                Ok(())
            } else {
                create_error!(format!("failed resetting event {:?}", result))
            }
        }
    }

    pub fn create_command_pool(
        &self,
        create_info: &VkCommandPoolCreateInfo,
    ) -> VerboseResult<VkCommandPool> {
        unsafe {
            let mut command_pool = MaybeUninit::uninit();

            let result = self.device_functions.vkCreateCommandPool(
                self.device,
                create_info,
                ptr::null(),
                command_pool.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(command_pool.assume_init())
            } else {
                create_error!(format!("failed creating command pool {:?}", result))
            }
        }
    }

    pub fn destroy_command_pool(&self, command_pool: VkCommandPool) {
        unsafe {
            self.device_functions
                .vkDestroyCommandPool(self.device, command_pool, ptr::null())
        };
    }

    pub fn reset_command_pool<T>(&self, command_pool: VkCommandPool, flags: T) -> VerboseResult<()>
    where
        T: Into<VkCommandPoolResetFlags>,
    {
        unsafe {
            let result =
                self.device_functions
                    .vkResetCommandPool(self.device, command_pool, flags.into());

            if result == VK_SUCCESS {
                Ok(())
            } else {
                create_error!(format!("failed resetting command pool {:?}", result))
            }
        }
    }

    pub fn trim_command_pool<T>(&self, command_pool: VkCommandPool, flags: T)
    where
        T: Into<VkCommandPoolTrimFlags>,
    {
        unsafe {
            self.device_functions
                .vkTrimCommandPool(self.device, command_pool, flags.into());
        }
    }

    pub fn create_framebuffer(
        &self,
        create_info: &VkFramebufferCreateInfo,
    ) -> VerboseResult<VkFramebuffer> {
        unsafe {
            let mut framebuffer = MaybeUninit::uninit();

            let result = self.device_functions.vkCreateFramebuffer(
                self.device,
                create_info,
                ptr::null(),
                framebuffer.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(framebuffer.assume_init())
            } else {
                create_error!(format!("failed creating framebuffer {:?}", result))
            }
        }
    }

    pub fn destroy_framebuffer(&self, framebuffer: VkFramebuffer) {
        unsafe {
            self.device_functions
                .vkDestroyFramebuffer(self.device, framebuffer, ptr::null())
        };
    }

    pub fn allocate_command_buffers(
        &self,
        allocate_info: &VkCommandBufferAllocateInfo,
    ) -> VerboseResult<Vec<VkCommandBuffer>> {
        unsafe {
            let count = allocate_info.commandBufferCount as usize;

            let mut command_buffers = Vec::with_capacity(count);
            command_buffers.set_len(count);

            let result = self.device_functions.vkAllocateCommandBuffers(
                self.device,
                allocate_info,
                command_buffers.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(command_buffers)
            } else {
                create_error!(format!("failed allocating commandbuffers {:?}", result))
            }
        }
    }

    pub fn free_command_buffers(
        &self,
        command_pool: VkCommandPool,
        command_buffers: &[VkCommandBuffer],
    ) {
        unsafe {
            self.device_functions.vkFreeCommandBuffers(
                self.device,
                command_pool,
                command_buffers.len() as u32,
                command_buffers.as_ptr(),
            )
        }
    }

    pub fn create_query_pool(
        &self,
        create_info: &VkQueryPoolCreateInfo,
    ) -> VerboseResult<VkQueryPool> {
        unsafe {
            let mut query_pool = MaybeUninit::uninit();

            let result = self.device_functions.vkCreateQueryPool(
                self.device,
                create_info,
                ptr::null(),
                query_pool.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(query_pool.assume_init())
            } else {
                create_error!(format!("failed creating query pool {:?}", result))
            }
        }
    }

    pub fn destroy_query_pool(&self, query_pool: VkQueryPool) {
        unsafe {
            self.device_functions
                .vkDestroyQueryPool(self.device, query_pool, ptr::null())
        };
    }

    pub fn create_pipeline_cache(
        &self,
        create_info: &VkPipelineCacheCreateInfo,
    ) -> VerboseResult<VkPipelineCache> {
        unsafe {
            let mut pipeline_cache = MaybeUninit::uninit();

            let result = self.device_functions.vkCreatePipelineCache(
                self.device,
                create_info,
                ptr::null(),
                pipeline_cache.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(pipeline_cache.assume_init())
            } else {
                create_error!(format!("failed creating pipeline cache {:?}", result))
            }
        }
    }

    pub fn destroy_pipeline_cache(&self, pipeline_cache: VkPipelineCache) {
        unsafe {
            self.device_functions
                .vkDestroyPipelineCache(self.device, pipeline_cache, ptr::null())
        };
    }

    pub fn pipeline_cache_data<T>(&self, pipeline_cache: VkPipelineCache) -> VerboseResult<T> {
        let mut count = 0;

        let result = unsafe {
            self.device_functions.vkGetPipelineCacheData(
                self.device,
                pipeline_cache,
                &mut count,
                ptr::null_mut(),
            )
        };

        if result != VK_SUCCESS {
            create_error!(format!("failed getting pipeline cache data {:?}", result))
        }

        if count != size_of::<T>() {
            create_error!(format!("failed getting pipeline cache data {:?}", result))
        }

        unsafe {
            let mut data = MaybeUninit::<T>::uninit();

            let result = self.device_functions.vkGetPipelineCacheData(
                self.device,
                pipeline_cache,
                &mut count,
                data.as_mut_ptr() as *mut c_void,
            );

            if result == VK_SUCCESS {
                Ok(data.assume_init())
            } else {
                create_error!(format!("failed getting pipeline cache data {:?}", result))
            }
        }
    }

    pub fn merge_pipeline_cache(
        &self,
        sources: &[VkPipelineCache],
        destination: VkPipelineCache,
    ) -> VerboseResult<()> {
        unsafe {
            let result = self.device_functions.vkMergePipelineCaches(
                self.device,
                destination,
                sources.len() as u32,
                sources.as_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(())
            } else {
                create_error!(format!("failed merging pipeline caches {:?}", result))
            }
        }
    }

    pub fn create_pipeline_layout(
        &self,
        create_info: &VkPipelineLayoutCreateInfo,
    ) -> VerboseResult<VkPipelineLayout> {
        unsafe {
            let mut pipeline_layout = MaybeUninit::uninit();

            let result = self.device_functions.vkCreatePipelineLayout(
                self.device,
                create_info,
                ptr::null(),
                pipeline_layout.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(pipeline_layout.assume_init())
            } else {
                create_error!(format!("failed creating pipeline layout {:?}", result))
            }
        }
    }

    pub fn destroy_pipeline_layout(&self, pipeline_layout: VkPipelineLayout) {
        unsafe {
            self.device_functions
                .vkDestroyPipelineLayout(self.device, pipeline_layout, ptr::null())
        };
    }

    pub fn create_graphics_pipelines(
        &self,
        pipeline_cache: Option<VkPipelineCache>,
        create_infos: &[VkGraphicsPipelineCreateInfo],
    ) -> VerboseResult<Vec<VkPipeline>> {
        unsafe {
            let count = create_infos.len() as usize;

            let mut pipelines = Vec::with_capacity(count);
            pipelines.set_len(count);

            let result = self.device_functions.vkCreateGraphicsPipelines(
                self.device,
                match pipeline_cache {
                    Some(cache) => cache,
                    None => VkPipelineCache::NULL_HANDLE,
                },
                create_infos.len() as u32,
                create_infos.as_ptr(),
                ptr::null(),
                pipelines.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(pipelines)
            } else {
                create_error!(format!("failed creating graphic pipelines {:?}", result))
            }
        }
    }

    pub fn create_compute_pipelines(
        &self,
        pipeline_cache: Option<VkPipelineCache>,
        create_infos: &[VkComputePipelineCreateInfo],
    ) -> VerboseResult<Vec<VkPipeline>> {
        unsafe {
            let count = create_infos.len() as usize;

            let mut pipelines = Vec::with_capacity(count);
            pipelines.set_len(count);

            let result = self.device_functions.vkCreateComputePipelines(
                self.device,
                match pipeline_cache {
                    Some(cache) => cache,
                    None => VkPipelineCache::NULL_HANDLE,
                },
                create_infos.len() as u32,
                create_infos.as_ptr(),
                ptr::null(),
                pipelines.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(pipelines)
            } else {
                create_error!(format!("failed creating compute pipelines {:?}", result))
            }
        }
    }

    pub fn destroy_pipeline(&self, pipeline: VkPipeline) {
        unsafe {
            self.device_functions
                .vkDestroyPipeline(self.device, pipeline, ptr::null())
        };
    }

    pub fn queue_present(
        &self,
        queue: VkQueue,
        present_info: &VkPresentInfoKHR,
    ) -> VerboseResult<OutOfDate<()>> {
        unsafe {
            let result = self
                .device_wsi_functions
                .vkQueuePresentKHR(queue, present_info);

            if result == VK_SUCCESS {
                Ok(OutOfDate::Ok(()))
            } else if result == VK_ERROR_OUT_OF_DATE_KHR || result == VK_SUBOPTIMAL_KHR {
                Ok(OutOfDate::OutOfDate)
            } else {
                create_error!(format!("failed presenting queue {:?}", result))
            }
        }
    }

    pub fn create_swapchain(
        &self,
        create_info: &VkSwapchainCreateInfoKHR,
    ) -> VerboseResult<VkSwapchainKHR> {
        unsafe {
            let mut swapchain = MaybeUninit::uninit();

            let result = self.device_wsi_functions.vkCreateSwapchainKHR(
                self.device,
                create_info,
                ptr::null(),
                swapchain.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(swapchain.assume_init())
            } else {
                create_error!(format!("failed creating swapchain {:?}", result))
            }
        }
    }

    pub fn destroy_swapchain(&self, swapchain: VkSwapchainKHR) {
        unsafe {
            self.device_wsi_functions
                .vkDestroySwapchainKHR(self.device, swapchain, ptr::null())
        };
    }

    pub fn swapchain_images(&self, swapchain: VkSwapchainKHR) -> VerboseResult<Vec<VkImage>> {
        let mut count = 0;

        let result = unsafe {
            self.device_wsi_functions.vkGetSwapchainImagesKHR(
                self.device,
                swapchain,
                &mut count,
                ptr::null_mut(),
            )
        };

        if result != VK_SUCCESS {
            create_error!(format!("failed getting swapchain images {:?}", result))
        }

        let mut images = Vec::with_capacity(count as usize);
        unsafe { images.set_len(count as usize) };

        let result = unsafe {
            self.device_wsi_functions.vkGetSwapchainImagesKHR(
                self.device,
                swapchain,
                &mut count,
                images.as_mut_ptr(),
            )
        };

        if result == VK_SUCCESS {
            Ok(images)
        } else {
            create_error!(format!("failed getting swapchain images {:?}", result))
        }
    }

    pub fn acquire_next_image(
        &self,
        swapchain: VkSwapchainKHR,
        timeout: u64,
        semaphore: Option<VkSemaphore>,
        fence: Option<VkFence>,
    ) -> VerboseResult<OutOfDate<u32>> {
        unsafe {
            let mut image_index = 0;

            let result = self.device_wsi_functions.vkAcquireNextImageKHR(
                self.device,
                swapchain,
                timeout,
                match semaphore {
                    Some(sem) => sem,
                    None => VkSemaphore::NULL_HANDLE,
                },
                match fence {
                    Some(fence) => fence,
                    None => VkFence::NULL_HANDLE,
                },
                &mut image_index,
            );

            if result == VK_SUCCESS {
                Ok(OutOfDate::Ok(image_index))
            } else if result == VK_ERROR_OUT_OF_DATE_KHR || result == VK_SUBOPTIMAL_KHR {
                Ok(OutOfDate::OutOfDate)
            } else {
                create_error!(format!(
                    "failed acquiring next swapchain image {:?}",
                    result
                ))
            }
        }
    }
}

// command buffer functions
impl Device {
    pub fn begin_command_buffer(
        &self,
        command_buffer: VkCommandBuffer,
        begin_info: &VkCommandBufferBeginInfo,
    ) -> VerboseResult<()> {
        unsafe {
            let result = self
                .device_functions
                .vkBeginCommandBuffer(command_buffer, begin_info);

            if result == VK_SUCCESS {
                Ok(())
            } else {
                create_error!(format!("failed beginning command buffer {:?}", result))
            }
        }
    }

    pub fn end_command_buffer(&self, command_buffer: VkCommandBuffer) -> VerboseResult<()> {
        unsafe {
            let result = self.device_functions.vkEndCommandBuffer(command_buffer);

            if result == VK_SUCCESS {
                Ok(())
            } else {
                create_error!(format!("failed ending command buffer {:?}", result))
            }
        }
    }

    pub fn reset_command_buffer(
        &self,
        command_buffer: VkCommandBuffer,
        flags: impl Into<VkCommandBufferResetFlagBits>,
    ) -> VerboseResult<()> {
        unsafe {
            let result = self
                .device_functions
                .vkResetCommandBuffer(command_buffer, flags.into());

            if result == VK_SUCCESS {
                Ok(())
            } else {
                create_error!(format!("failed resetting command buffer {:?}", result))
            }
        }
    }

    pub fn cmd_bind_pipeline(
        &self,
        command_buffer: VkCommandBuffer,
        pipeline_bind_point: VkPipelineBindPoint,
        pipeline: VkPipeline,
    ) {
        unsafe {
            self.device_functions
                .vkCmdBindPipeline(command_buffer, pipeline_bind_point, pipeline);
        }
    }

    pub fn cmd_set_viewport(
        &self,
        command_buffer: VkCommandBuffer,
        first: u32,
        viewports: &[VkViewport],
    ) {
        unsafe {
            self.device_functions.vkCmdSetViewport(
                command_buffer,
                first,
                viewports.len() as u32,
                viewports.as_ptr(),
            )
        }
    }

    pub fn cmd_set_scissor(
        &self,
        command_buffer: VkCommandBuffer,
        first: u32,
        scissors: &[VkRect2D],
    ) {
        unsafe {
            self.device_functions.vkCmdSetScissor(
                command_buffer,
                first,
                scissors.len() as u32,
                scissors.as_ptr(),
            )
        }
    }

    pub fn cmd_set_depth_bias(
        &self,
        command_buffer: VkCommandBuffer,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
    ) {
        unsafe {
            self.device_functions.vkCmdSetDepthBias(
                command_buffer,
                depth_bias_constant_factor,
                depth_bias_clamp,
                depth_bias_slope_factor,
            )
        }
    }

    pub fn cmd_bind_descriptor_sets(
        &self,
        command_buffer: VkCommandBuffer,
        pipeline_bind_point: VkPipelineBindPoint,
        pipeline_layout: VkPipelineLayout,
        first_set: u32,
        descriptor_sets: &[VkDescriptorSet],
        dynamic_offsets: &[u32],
    ) {
        unsafe {
            self.device_functions.vkCmdBindDescriptorSets(
                command_buffer,
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

    pub fn cmd_bind_index_buffer(
        &self,
        command_buffer: VkCommandBuffer,
        buffer: VkBuffer,
        offset: VkDeviceSize,
        index_type: VkIndexType,
    ) {
        unsafe {
            self.device_functions
                .vkCmdBindIndexBuffer(command_buffer, buffer, offset, index_type)
        }
    }

    pub fn cmd_bind_vertex_buffers(
        &self,
        command_buffer: VkCommandBuffer,
        first_binding: u32,
        buffers: &[VkBuffer],
        offsets: &[VkDeviceSize],
    ) {
        // sanity check
        debug_assert!(buffers.len() == offsets.len());

        unsafe {
            self.device_functions.vkCmdBindVertexBuffers(
                command_buffer,
                first_binding,
                buffers.len() as u32,
                buffers.as_ptr(),
                offsets.as_ptr(),
            )
        }
    }

    pub fn cmd_draw(
        &self,
        command_buffer: VkCommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        unsafe {
            self.device_functions.vkCmdDraw(
                command_buffer,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            )
        }
    }

    pub fn cmd_draw_indexed(
        &self,
        command_buffer: VkCommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        unsafe {
            self.device_functions.vkCmdDrawIndexed(
                command_buffer,
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            );
        }
    }

    pub fn cmd_dispatch(&self, command_buffer: VkCommandBuffer, x: u32, y: u32, z: u32) {
        unsafe { self.device_functions.vkCmdDispatch(command_buffer, x, y, z) }
    }

    pub fn cmd_begin_render_pass(
        &self,
        command_buffer: VkCommandBuffer,
        render_pass_begin: &VkRenderPassBeginInfo,
        contents: VkSubpassContents,
    ) {
        unsafe {
            self.device_functions
                .vkCmdBeginRenderPass(command_buffer, render_pass_begin, contents)
        }
    }

    pub fn cmd_next_subpass(&self, command_buffer: VkCommandBuffer, contents: VkSubpassContents) {
        unsafe {
            self.device_functions
                .vkCmdNextSubpass(command_buffer, contents)
        }
    }

    pub fn cmd_end_render_pass(&self, command_buffer: VkCommandBuffer) {
        unsafe { self.device_functions.vkCmdEndRenderPass(command_buffer) }
    }

    pub fn cmd_execute_commands(
        &self,
        command_buffer: VkCommandBuffer,
        command_buffers: &[VkCommandBuffer],
    ) {
        unsafe {
            self.device_functions.vkCmdExecuteCommands(
                command_buffer,
                command_buffers.len() as u32,
                command_buffers.as_ptr(),
            )
        }
    }

    pub fn cmd_pipeline_barrier(
        &self,
        command_buffer: VkCommandBuffer,
        src_stage_mask: impl Into<VkPipelineStageFlagBits>,
        dst_stage_mask: impl Into<VkPipelineStageFlagBits>,
        dependency_flags: impl Into<VkDependencyFlagBits>,
        memory_barriers: &[VkMemoryBarrier],
        buffer_memory_barriers: &[VkBufferMemoryBarrier],
        image_memory_barriers: &[VkImageMemoryBarrier],
    ) {
        unsafe {
            self.device_functions.vkCmdPipelineBarrier(
                command_buffer,
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

    pub fn cmd_copy_buffer(
        &self,
        command_buffer: VkCommandBuffer,
        src_buffer: VkBuffer,
        dst_buffer: VkBuffer,
        regions: &[VkBufferCopy],
    ) {
        unsafe {
            self.device_functions.vkCmdCopyBuffer(
                command_buffer,
                src_buffer,
                dst_buffer,
                regions.len() as u32,
                regions.as_ptr(),
            )
        }
    }

    pub fn cmd_copy_image(
        &self,
        command_buffer: VkCommandBuffer,
        src_image: VkImage,
        src_image_layout: VkImageLayout,
        dst_image: VkImage,
        dst_image_layout: VkImageLayout,
        regions: &[VkImageCopy],
    ) {
        unsafe {
            self.device_functions.vkCmdCopyImage(
                command_buffer,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                regions.len() as u32,
                regions.as_ptr(),
            )
        }
    }

    pub fn cmd_blit_image(
        &self,
        command_buffer: VkCommandBuffer,
        src_image: VkImage,
        src_image_layout: VkImageLayout,
        dst_image: VkImage,
        dst_image_layout: VkImageLayout,
        regions: &[VkImageBlit],
        filter: VkFilter,
    ) {
        unsafe {
            self.device_functions.vkCmdBlitImage(
                command_buffer,
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

    pub fn cmd_copy_buffer_to_image(
        &self,
        command_buffer: VkCommandBuffer,
        src_buffer: VkBuffer,
        dst_image: VkImage,
        dst_image_layout: VkImageLayout,
        regions: &[VkBufferImageCopy],
    ) {
        unsafe {
            self.device_functions.vkCmdCopyBufferToImage(
                command_buffer,
                src_buffer,
                dst_image,
                dst_image_layout,
                regions.len() as u32,
                regions.as_ptr(),
            )
        }
    }

    pub fn cmd_copy_image_to_buffer(
        &self,
        command_buffer: VkCommandBuffer,
        src_image: VkImage,
        src_image_layout: VkImageLayout,
        dst_buffer: VkBuffer,
        regions: &[VkBufferImageCopy],
    ) {
        unsafe {
            self.device_functions.vkCmdCopyImageToBuffer(
                command_buffer,
                src_image,
                src_image_layout,
                dst_buffer,
                regions.len() as u32,
                regions.as_ptr(),
            )
        }
    }

    pub fn cmd_push_constants<T>(
        &self,
        command_buffer: VkCommandBuffer,
        pipeline_layout: VkPipelineLayout,
        stage_flags: impl Into<VkShaderStageFlagBits>,
        offset: u32,
        data: &T,
    ) {
        unsafe {
            self.device_functions.vkCmdPushConstants(
                command_buffer,
                pipeline_layout,
                stage_flags.into(),
                offset,
                size_of::<T>() as u32,
                data as *const T as *const c_void,
            )
        }
    }

    pub fn cmd_begin_query(
        &self,
        command_buffer: VkCommandBuffer,
        query_pool: VkQueryPool,
        query: u32,
        flags: impl Into<VkQueryControlFlagBits>,
    ) {
        unsafe {
            self.device_functions
                .vkCmdBeginQuery(command_buffer, query_pool, query, flags.into())
        }
    }

    pub fn cmd_end_query(
        &self,
        command_buffer: VkCommandBuffer,
        query_pool: VkQueryPool,
        query: u32,
    ) {
        unsafe {
            self.device_functions
                .vkCmdEndQuery(command_buffer, query_pool, query)
        }
    }

    pub fn cmd_reset_query_pool(
        &self,
        command_buffer: VkCommandBuffer,
        query_pool: VkQueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        unsafe {
            self.device_functions.vkCmdResetQueryPool(
                command_buffer,
                query_pool,
                first_query,
                query_count,
            )
        }
    }

    pub fn cmd_write_timestamp(
        &self,
        command_buffer: VkCommandBuffer,
        pipeline_stage: impl Into<VkPipelineStageFlagBits>,
        query_pool: VkQueryPool,
        query: u32,
    ) {
        unsafe {
            self.device_functions.vkCmdWriteTimestamp(
                command_buffer,
                pipeline_stage.into(),
                query_pool,
                query,
            )
        }
    }

    pub fn cmd_clear_color_image(
        &self,
        command_buffer: VkCommandBuffer,
        image: VkImage,
        image_layout: VkImageLayout,
        clear_color: VkClearColorValue,
        ranges: &[VkImageSubresourceRange],
    ) {
        unsafe {
            self.device_functions.vkCmdClearColorImage(
                command_buffer,
                image,
                image_layout,
                &clear_color,
                ranges.len() as u32,
                ranges.as_ptr(),
            )
        }
    }

    pub fn descriptor_set_layout_support(
        &self,
        create_info: &VkDescriptorSetLayoutCreateInfo,
        support: &mut VkDescriptorSetLayoutSupport,
    ) {
        unsafe {
            self.maintenance3_functions.vkGetDescriptorSetLayoutSupport(
                self.device,
                create_info,
                support,
            );
        }
    }
}

// nv ray tracing
impl Device {
    pub fn cmd_build_acceleration_structure(
        &self,
        command_buffer: VkCommandBuffer,
        info: &VkAccelerationStructureInfoNV,
        instance_data: VkBuffer,
        instance_offset: VkDeviceSize,
        update: impl Into<VkBool32>,
        dst: VkAccelerationStructureNV,
        src: VkAccelerationStructureNV,
        scratch: VkBuffer,
        scratch_offset: VkDeviceSize,
    ) {
        unsafe {
            self.nv_ray_tracing_functions
                .vkCmdBuildAccelerationStructureNV(
                    command_buffer,
                    info,
                    instance_data,
                    instance_offset,
                    update.into(),
                    dst,
                    src,
                    scratch,
                    scratch_offset,
                );
        }
    }

    pub fn cmd_copy_acceleration_structure(
        &self,
        command_buffer: VkCommandBuffer,
        dst: VkAccelerationStructureNV,
        src: VkAccelerationStructureNV,
        mode: VkCopyAccelerationStructureModeNV,
    ) {
        unsafe {
            self.nv_ray_tracing_functions
                .vkCmdCopyAccelerationStructureNV(command_buffer, dst, src, mode);
        }
    }

    pub fn cmd_trace_rays(
        &self,
        command_buffer: VkCommandBuffer,
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
    ) {
        unsafe {
            self.nv_ray_tracing_functions.vkCmdTraceRaysNV(
                command_buffer,
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
        }
    }

    pub fn cmd_write_acceleration_structure_properties(
        &self,
        command_buffer: VkCommandBuffer,
        acceleration_structures: &[VkAccelerationStructureNV],
        query_type: VkQueryType,
        query_pool: VkQueryPool,
        first_query: u32,
    ) {
        unsafe {
            self.nv_ray_tracing_functions
                .vkCmdWriteAccelerationStructurePropertiesNV(
                    command_buffer,
                    acceleration_structures.len() as u32,
                    acceleration_structures.as_ptr(),
                    query_type,
                    query_pool,
                    first_query,
                );
        }
    }

    pub fn create_acceleration_structure(
        &self,
        create_info: &VkAccelerationStructureCreateInfoNV,
    ) -> VerboseResult<VkAccelerationStructureNV> {
        unsafe {
            let mut acceleration_structure = MaybeUninit::uninit();

            let result = self
                .nv_ray_tracing_functions
                .vkCreateAccelerationStructureNV(
                    self.device,
                    create_info,
                    ptr::null(),
                    acceleration_structure.as_mut_ptr(),
                );

            if result == VK_SUCCESS {
                Ok(acceleration_structure.assume_init())
            } else {
                create_error!(format!(
                    "failed creating acceleration structure {:?}",
                    result
                ))
            }
        }
    }

    pub fn destroy_acceleration_structure(
        &self,
        acceleration_structure: VkAccelerationStructureNV,
    ) {
        unsafe {
            self.nv_ray_tracing_functions
                .vkDestroyAccelerationStructureNV(self.device, acceleration_structure, ptr::null());
        }
    }

    /// needs to be called after binding the as to the memory
    pub fn acceleration_structure_handle(
        &self,
        acceleration_structure: VkAccelerationStructureNV,
    ) -> VerboseResult<u64> {
        unsafe {
            let mut handle = 0;
            let handle_ptr: *mut u64 = &mut handle;

            let result = self
                .nv_ray_tracing_functions
                .vkGetAccelerationStructureHandleNV(
                    self.device,
                    acceleration_structure,
                    size_of::<u64>(),
                    handle_ptr as *mut c_void,
                );

            if result == VK_SUCCESS {
                Ok(handle)
            } else {
                create_error!(format!(
                    "failed creating acceleration structure handle {:?}",
                    result
                ))
            }
        }
    }

    pub fn create_ray_tracing_pipelines(
        &self,
        pipeline_cache: Option<VkPipelineCache>,
        pipeline_create_infos: &[VkRayTracingPipelineCreateInfoNV],
    ) -> VerboseResult<Vec<VkPipeline>> {
        unsafe {
            let count = pipeline_create_infos.len() as usize;

            let mut pipelines = Vec::with_capacity(count);
            pipelines.set_len(count);

            let result = self.nv_ray_tracing_functions.vkCreateRayTracingPipelinesNV(
                self.device,
                match pipeline_cache {
                    Some(cache) => cache,
                    None => VkPipelineCache::NULL_HANDLE,
                },
                pipeline_create_infos.len() as u32,
                pipeline_create_infos.as_ptr(),
                ptr::null(),
                pipelines.as_mut_ptr(),
            );

            if result == VK_SUCCESS {
                Ok(pipelines)
            } else {
                create_error!(format!(
                    "failed creating ray tracing pipelines {:?}",
                    result
                ))
            }
        }
    }

    pub fn ray_tracing_shader_group_handles(
        &self,
        pipeline: VkPipeline,
        first_group: u32,
        group_count: u32,
        shader_group_handle_size: u32,
    ) -> VerboseResult<Vec<u8>> {
        unsafe {
            let mut data = vec![255; (group_count * shader_group_handle_size) as usize];

            let result = self
                .nv_ray_tracing_functions
                .vkGetRayTracingShaderGroupHandlesNV(
                    self.device,
                    pipeline,
                    first_group,
                    group_count,
                    data.len(),
                    data.as_mut_ptr() as *mut c_void,
                );

            if result == VK_SUCCESS {
                Ok(data)
            } else {
                create_error!(format!(
                    "failed gettting ray tracing shader group handles {:?}",
                    result
                ))
            }
        }
    }

    pub fn compile_deferred(&self, pipeline: VkPipeline, shader: u32) -> VerboseResult<()> {
        unsafe {
            let result =
                self.nv_ray_tracing_functions
                    .vkCompileDeferredNV(self.device, pipeline, shader);

            if result == VK_SUCCESS {
                Ok(())
            } else {
                create_error!(format!("failed compiling pipeline deferred {:?}", result))
            }
        }
    }

    pub fn acceleration_structure_memory_requirements(
        &self,
        info: &VkAccelerationStructureMemoryRequirementsInfoNV,
    ) -> VkMemoryRequirements2KHR {
        unsafe {
            let mut memory_requirements = MaybeUninit::uninit();

            self.nv_ray_tracing_functions
                .vkGetAccelerationStructureMemoryRequirementsNV(
                    self.device,
                    info,
                    memory_requirements.as_mut_ptr(),
                );

            memory_requirements.assume_init()
        }
    }

    pub fn bind_acceleration_structure_memory(
        &self,
        bind_infos: &[VkBindAccelerationStructureMemoryInfoNV],
    ) -> VerboseResult<()> {
        unsafe {
            let result = self
                .nv_ray_tracing_functions
                .vkBindAccelerationStructureMemoryNV(
                    self.device,
                    bind_infos.len() as u32,
                    bind_infos.as_ptr(),
                );

            if result == VK_SUCCESS {
                Ok(())
            } else {
                create_error!(format!(
                    "failed binding acceleration structure to memory {:?}",
                    result
                ))
            }
        }
    }
}
