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

pub trait RenderCore: std::fmt::Debug {
    fn next_frame(&self) -> VerboseResult<bool>;

    // scene handling
    fn add_scene(&self, scene: Arc<dyn TScene>) -> VerboseResult<()>;
    fn remove_scene(&self, scene: &Arc<dyn TScene>) -> VerboseResult<()>;
    fn clear_scenes(&self) -> VerboseResult<()>;

    // callbacks
    fn set_resize_callback(
        &self,
        resize_callback: Option<Box<dyn Fn(u32, u32) -> VerboseResult<()>>>,
    ) -> VerboseResult<()>;
    fn set_gui_callback(
        &self,
        render_gui: Option<
            Box<
                dyn Fn(
                    Option<Eye>,
                    usize,
                    &Arc<Framebuffer>,
                    &Arc<RenderPass>,
                ) -> VerboseResult<Arc<CommandBuffer>>,
            >,
        >,
    ) -> VerboseResult<()>;

    // getter
    // fn current_index(&self) -> TargetMode<usize>;
    fn image_count(&self) -> usize;
    fn images(&self) -> TargetMode<Vec<Arc<Image>>>;
    fn gui_render_pass(&self) -> &Arc<RenderPass>;
    fn allocate_primary_buffer(&self) -> VerboseResult<Arc<CommandBuffer>>;
    fn allocate_secondary_buffer(&self) -> VerboseResult<Arc<CommandBuffer>>;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}
