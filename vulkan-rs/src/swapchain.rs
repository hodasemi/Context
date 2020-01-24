use utilities::prelude::*;

use crate::prelude::*;

use std::cmp;
use std::sync::{
    atomic::{AtomicU32, Ordering::SeqCst},
    Arc, Mutex,
};

#[derive(Debug)]
pub struct Swapchain {
    width: AtomicU32,
    height: AtomicU32,

    device: Arc<Device>,
    surface: Arc<Surface>,

    create_info: Mutex<VkSwapchainCreateInfoKHR>,
    swapchain: Mutex<VkSwapchainKHR>,
}

impl Swapchain {
    pub fn new(
        device: Arc<Device>,
        surface: &Arc<Surface>,
        vsync: bool,
        image_count: u32,
        image_usage: impl Into<VkImageUsageFlagBits>,
        prefered_format: VkFormat,
        array_layers: u32,
    ) -> VerboseResult<Arc<Swapchain>> {
        let surface_caps = surface.capabilities(&device)?;

        let extent = if surface_caps.currentExtent.width == u32::max_value() {
            create_error!("surface has no extent!")
        } else {
            VkExtent2D {
                width: surface_caps.currentExtent.width,
                height: surface_caps.currentExtent.height,
            }
        };

        let mut present_mode = VK_PRESENT_MODE_FIFO_KHR;

        if !vsync {
            for present_mode_iter in surface.present_modes(&device)? {
                if present_mode_iter == VK_PRESENT_MODE_MAILBOX_KHR {
                    present_mode = VK_PRESENT_MODE_MAILBOX_KHR;
                    break;
                } else if present_mode_iter == VK_PRESENT_MODE_IMMEDIATE_KHR {
                    present_mode = VK_PRESENT_MODE_IMMEDIATE_KHR;
                }
            }
        }

        let swapchain_image_count = if surface_caps.maxImageCount < surface_caps.minImageCount {
            cmp::max(image_count, surface_caps.minImageCount)
        } else {
            cmp::max(
                cmp::min(image_count, surface_caps.maxImageCount),
                surface_caps.minImageCount,
            )
        };

        let pretransform =
            if (surface_caps.supportedTransforms & VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR) != 0 {
                VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR.into()
            } else {
                surface_caps.currentTransform
            };

        let (format, colorspace) = surface.format_colorspace(&device, prefered_format)?;

        let swapchain_ci = VkSwapchainCreateInfoKHR::new(
            0,
            surface.vk_handle(),
            swapchain_image_count,
            format,
            colorspace,
            extent,
            array_layers,
            image_usage,
            VK_SHARING_MODE_EXCLUSIVE,
            &[],
            pretransform,
            VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
            present_mode,
            device.physical_device().features().shaderClipDistance,
        );

        let swapchain = device.create_swapchain(&swapchain_ci)?;

        Ok(Arc::new(Swapchain {
            width: AtomicU32::new(extent.width),
            height: AtomicU32::new(extent.height),

            device,
            surface: surface.clone(),

            create_info: Mutex::new(swapchain_ci),

            swapchain: Mutex::new(swapchain),
        }))
    }

    pub fn recreate(&self) -> VerboseResult<()> {
        let surface_caps = self.surface.capabilities(&self.device)?;

        let extent = if surface_caps.currentExtent.width == u32::max_value() {
            create_error!("surface has no extent!")
        } else {
            VkExtent2D {
                width: surface_caps.currentExtent.width,
                height: surface_caps.currentExtent.height,
            }
        };

        let mut swapchain_ci = self.create_info.lock()?;
        swapchain_ci.imageExtent = extent;
        swapchain_ci.set_old_swapchain(self.swapchain.lock()?.clone());

        let swapchain = self.device.create_swapchain(&swapchain_ci)?;

        // destroy the old swapchain
        self.destroy();

        // replace swapchain
        *self.swapchain.lock()? = swapchain;

        // set new surface size
        self.width.store(extent.width, SeqCst);
        self.height.store(extent.height, SeqCst);

        Ok(())
    }

    pub fn acquire_next_image(
        &self,
        time_out: u64,
        present_complete_semaphore: Option<&Arc<Semaphore>>,
        fence: Option<&Arc<Fence>>,
    ) -> VerboseResult<OutOfDate<u32>> {
        self.device.acquire_next_image(
            self.swapchain.lock()?.clone(),
            time_out,
            match present_complete_semaphore {
                Some(sem) => Some(sem.vk_handle()),
                None => None,
            },
            match fence {
                Some(fence) => Some(fence.vk_handle()),
                None => None,
            },
        )
    }

    pub fn vk_images(&self) -> VerboseResult<Vec<VkImage>> {
        self.device.swapchain_images(self.swapchain.lock()?.clone())
    }

    pub fn width(&self) -> u32 {
        self.width.load(SeqCst)
    }

    pub fn height(&self) -> u32 {
        self.height.load(SeqCst)
    }

    pub fn format(&self) -> VerboseResult<VkFormat> {
        Ok(self.create_info.lock()?.imageFormat)
    }

    #[inline]
    fn destroy(&self) {
        self.device
            .destroy_swapchain(self.swapchain.lock().unwrap().clone())
    }
}

impl VulkanDevice for Swapchain {
    fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

impl VkHandle<VkSwapchainKHR> for Swapchain {
    fn vk_handle(&self) -> VkSwapchainKHR {
        self.swapchain.lock().unwrap().clone()
    }
}

impl<'a> VkHandle<VkSwapchainKHR> for &'a Swapchain {
    fn vk_handle(&self) -> VkSwapchainKHR {
        self.swapchain.lock().unwrap().clone()
    }
}

impl VkHandle<VkSwapchainKHR> for Arc<Swapchain> {
    fn vk_handle(&self) -> VkSwapchainKHR {
        self.swapchain.lock().unwrap().clone()
    }
}

impl<'a> VkHandle<VkSwapchainKHR> for &'a Arc<Swapchain> {
    fn vk_handle(&self) -> VkSwapchainKHR {
        self.swapchain.lock().unwrap().clone()
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        self.destroy();
    }
}
