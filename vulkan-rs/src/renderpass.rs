use utilities::prelude::*;

use crate::impl_vk_handle;
use crate::prelude::*;

use std::sync::Arc;

#[derive(Debug)]
pub struct RenderPass {
    device: Arc<Device>,
    render_pass: VkRenderPass,
}

impl RenderPass {
    pub fn new(
        device: Arc<Device>,
        sub_passes: &[VkSubpassDescription],
        attachments: &[VkAttachmentDescription],
        dependencies: &[VkSubpassDependency],
    ) -> VerboseResult<Arc<RenderPass>> {
        let render_pass_ci = VkRenderPassCreateInfo::new(
            VK_RENDERPASS_CREATE_NULL_BIT,
            attachments,
            sub_passes,
            dependencies,
        );

        let render_pass = device.create_render_pass(&render_pass_ci)?;

        Ok(Arc::new(RenderPass {
            device,
            render_pass,
        }))
    }
}

impl VulkanDevice for RenderPass {
    fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

impl_vk_handle!(RenderPass, VkRenderPass, render_pass);

impl Drop for RenderPass {
    fn drop(&mut self) {
        self.device.destroy_render_pass(self.render_pass);
    }
}
