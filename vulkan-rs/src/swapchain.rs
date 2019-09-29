use utilities::prelude::*;

use crate::prelude::*;

use std::cell::{Cell, RefCell};
use std::cmp;
use std::sync::Arc;

#[derive(Debug)]
pub struct Swapchain {
    width: Cell<u32>,
    height: Cell<u32>,

    device: Arc<Device>,
    surface: Arc<Surface>,

    create_info: RefCell<VkSwapchainCreateInfoKHR>,
    swapchain: Cell<VkSwapchainKHR>,
}

impl Swapchain {
    pub fn new(
        device: Arc<Device>,
        surface: &Arc<Surface>,
        vsync: bool,
        image_count: u32,
        image_usage: impl Into<VkImageUsageFlagBits>,
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

        let (format, colorspace) = surface.format_colorspace(&device)?;

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
            width: Cell::new(extent.width),
            height: Cell::new(extent.height),

            device,
            surface: surface.clone(),

            create_info: RefCell::new(swapchain_ci),

            swapchain: Cell::new(swapchain),
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

        let mut swapchain_ci = self.create_info.try_borrow_mut()?;
        swapchain_ci.imageExtent = extent;
        swapchain_ci.set_old_swapchain(self.swapchain.get());

        let swapchain = self.device.create_swapchain(&swapchain_ci)?;

        // destroy the old swapchain
        self.destroy();

        // replace swapchain
        self.swapchain.set(swapchain);

        // set new surface size
        self.width.set(extent.width);
        self.height.set(extent.height);

        Ok(())
    }

    pub fn acquire_next_image(
        &self,
        time_out: u64,
        present_complete_semaphore: Option<&Arc<Semaphore>>,
        fence: Option<&Arc<Fence>>,
    ) -> VerboseResult<OutOfDate<u32>> {
        self.device.acquire_next_image(
            self.swapchain.get(),
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
        self.device.swapchain_images(self.swapchain.get())
    }

    pub fn width(&self) -> u32 {
        self.width.get()
    }

    pub fn height(&self) -> u32 {
        self.height.get()
    }

    #[inline]
    fn destroy(&self) {
        self.device.destroy_swapchain(self.swapchain.get())
    }
}

impl VkHandle<VkSwapchainKHR> for Swapchain {
    fn vk_handle(&self) -> VkSwapchainKHR {
        self.swapchain.get()
    }
}

impl<'a> VkHandle<VkSwapchainKHR> for &'a Swapchain {
    fn vk_handle(&self) -> VkSwapchainKHR {
        self.swapchain.get()
    }
}

impl VkHandle<VkSwapchainKHR> for Arc<Swapchain> {
    fn vk_handle(&self) -> VkSwapchainKHR {
        self.swapchain.get()
    }
}

impl<'a> VkHandle<VkSwapchainKHR> for &'a Arc<Swapchain> {
    fn vk_handle(&self) -> VkSwapchainKHR {
        self.swapchain.get()
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        self.destroy();
    }
}
