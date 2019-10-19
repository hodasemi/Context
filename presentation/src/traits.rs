use utilities::prelude::*;
use vulkan_rs::prelude::*;

use std::sync::Arc;

use crate::prelude::*;

pub trait TScene {
    fn update(&self) -> VerboseResult<()>;
    fn process(
        &self,
        command_buffer: &Arc<CommandBuffer>,
        indices: &TargetMode<usize>,
        vr_data: &Option<TargetMode<VRTransformations>>,
    ) -> VerboseResult<()>;
    fn resize(&self) -> VerboseResult<()>;
}

pub trait PostProcess {
    /// higher priority means, it is executed earlier
    fn priority(&self) -> u32;

    fn process(
        &self,
        command_buffer: &Arc<CommandBuffer>,
        indices: &TargetMode<usize>,
    ) -> VerboseResult<()>;
    fn resize(&self, width: u32, height: u32) -> VerboseResult<()>;
}

pub trait RenderCore: std::fmt::Debug {
    fn next_frame(&self) -> VerboseResult<bool>;

    fn format(&self) -> VkFormat;
    fn image_layout(&self) -> VkImageLayout {
        VK_IMAGE_LAYOUT_PRESENT_SRC_KHR
    }

    // scene handling
    fn add_scene(&self, scene: Arc<dyn TScene>) -> VerboseResult<()>;
    fn remove_scene(&self, scene: &Arc<dyn TScene>) -> VerboseResult<()>;
    fn clear_scenes(&self) -> VerboseResult<()>;

    // post process handling
    fn add_post_processing_routine(&self, post_process: Arc<dyn PostProcess>) -> VerboseResult<()>;
    fn remove_post_processing_routine(
        &self,
        post_process: &Arc<dyn PostProcess>,
    ) -> VerboseResult<()>;

    // getter
    fn image_count(&self) -> usize;
    fn images(&self) -> TargetMode<Vec<Arc<Image>>>;
    fn allocate_primary_buffer(&self) -> VerboseResult<Arc<CommandBuffer>>;
    fn allocate_secondary_buffer(&self) -> VerboseResult<Arc<CommandBuffer>>;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}
