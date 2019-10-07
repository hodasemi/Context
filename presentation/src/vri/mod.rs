#[cfg(feature = "OpenVR")]
pub mod openvrintegration;

#[cfg(not(feature = "OpenVR"))]
pub mod openvrintegration {
    use utilities::prelude::*;
    use vulkan_rs::prelude::*;

    use std::sync::Arc;

    pub struct OpenVRIntegration;

    impl OpenVRIntegration {
        pub fn new() -> VerboseResult<Self> {
            create_error!("OpenVR feature not enabled!")
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
            _: &Arc<PhysicalDevice>,
        ) -> VerboseResult<()> {
            unimplemented!()
        }

        pub fn physical_device(&self, _: &Arc<Instance>) -> VerboseResult<VkPhysicalDevice> {
            unimplemented!()
        }
    }

    impl std::fmt::Debug for OpenVRIntegration {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "OpenVRIntegration {{ }}")
        }
    }
}

#[cfg(feature = "OpenVR")]
pub mod openvrrendercore;

#[cfg(not(feature = "OpenVR"))]
pub mod openvrrendercore {
    use utilities::prelude::*;
    use vulkan_rs::prelude::*;

    use std::sync::{Arc, Mutex};

    use super::openvrintegration::OpenVRIntegration;

    use crate::prelude::*;

    pub struct OpenVRRenderCore {
        _dummy: u32,
    }

    impl OpenVRRenderCore {
        pub fn new(
            _: &OpenVRIntegration,
            _: &Arc<Device>,
            _: &Arc<Mutex<Queue>>,
        ) -> VerboseResult<(Self, TargetMode<()>)> {
            create_error!("OpenVR feature not enabled!")
        }
    }

    impl RenderCore for OpenVRRenderCore {
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
        // fn current_index(&self) -> TargetMode<usize> {
        //     unimplemented!()
        // }

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

    impl std::fmt::Debug for OpenVRRenderCore {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "OpenVRRenderCore {{ }}")
        }
    }
}
