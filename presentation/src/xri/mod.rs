#[cfg(feature = "OpenXR")]
pub mod openxrintegration;

#[cfg(not(feature = "OpenXR"))]
pub mod openxrintegration {
    use utilities::prelude::*;
    use vulkan_rs::prelude::*;

    use std::sync::Arc;

    use crate::prelude::*;

    pub struct OpenXRIntegration;

    impl OpenXRIntegration {
        pub fn new(_: ApplicationInfo) -> VerboseResult<Self> {
            create_error!("OpenXR feature not enabled!")
        }

        pub fn activate_vulkan_instance_extensions(
            &self,
            _: &mut InstanceExtensions,
        ) -> VerboseResult<()> {
            unimplemented!()
        }

        pub fn activate_vulkan_device_extensions(
            &self,
            _: &mut DeviceExtensions,
        ) -> VerboseResult<()> {
            unimplemented!()
        }

        pub fn physical_device(&self, _: &Arc<Instance>) -> VerboseResult<VkPhysicalDevice> {
            unimplemented!()
        }
    }
}

#[cfg(feature = "OpenXR")]
pub mod openxrrendercore;

#[cfg(not(feature = "OpenXR"))]
pub mod openxrrendercore {
    use utilities::prelude::*;
    use vulkan_rs::prelude::*;

    use std::sync::Arc;

    use super::openxrintegration::OpenXRIntegration;

    use crate::prelude::*;

    pub struct OpenXRRenderCore {
        _dummy: u32,
    }

    impl OpenXRRenderCore {
        pub fn new(
            _: &OpenXRIntegration,
            _: &Arc<Device>,
            _: &Arc<Queue>,
        ) -> VerboseResult<(Self, TargetMode<()>)> {
            create_error!("OpenXR feature not enabled!")
        }
    }

    impl RenderCore for OpenXRRenderCore {
        fn next_frame(&self) -> VerboseResult<bool> {
            unimplemented!()
        }

        // scene handling
        fn add_scene(&self, _: Arc<dyn TScene>) -> VerboseResult<()> {
            unimplemented!()
        }

        fn remove_scene(&self, _: &Arc<dyn TScene>) -> VerboseResult<()> {
            unimplemented!()
        }

        fn clear_scenes(&self) -> VerboseResult<()> {
            unimplemented!()
        }

        // callbacks
        fn set_resize_callback(
            &self,
            _: Option<Box<dyn Fn(u32, u32) -> VerboseResult<()>>>,
        ) -> VerboseResult<()> {
            unimplemented!()
        }

        fn set_gui_callback(
            &self,
            _: Option<
                Box<
                    dyn Fn(
                        Option<Eye>,
                        usize,
                        &Arc<Framebuffer>,
                        &Arc<RenderPass>,
                    ) -> VerboseResult<Arc<CommandBuffer>>,
                >,
            >,
        ) -> VerboseResult<()> {
            unimplemented!()
        }

        // getter
        fn image_count(&self) -> usize {
            unimplemented!()
        }

        fn images(&self) -> TargetMode<Vec<Arc<Image>>> {
            unimplemented!()
        }

        fn gui_render_pass(&self) -> &Arc<RenderPass> {
            unimplemented!()
        }

        // fn current_gui_framebuffer(&self) -> Arc<Framebuffer> {
        //     unimplemented!()
        // }

        fn allocate_primary_buffer(&self) -> VerboseResult<Arc<CommandBuffer>> {
            unimplemented!()
        }

        fn allocate_secondary_buffer(&self) -> VerboseResult<Arc<CommandBuffer>> {
            unimplemented!()
        }

        fn width(&self) -> u32 {
            unimplemented!()
        }

        fn height(&self) -> u32 {
            unimplemented!()
        }
    }
}
