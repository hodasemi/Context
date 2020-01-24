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
    use crate::RenderCoreCreateInfo;

    pub struct OpenVRRenderCore {
        _dummy: u32,
    }

    impl OpenVRRenderCore {
        pub fn new(
            _: &OpenVRIntegration,
            _: &Arc<Device>,
            _: &Arc<Mutex<Queue>>,
            _: RenderCoreCreateInfo,
        ) -> VerboseResult<(Self, TargetMode<()>)> {
            create_error!("OpenVR feature not enabled!")
        }
    }

    impl RenderCore for OpenVRRenderCore {
        fn format(&self) -> VkFormat {
            unimplemented!()
        }

        fn next_frame(&self) -> VerboseResult<bool> {
            unimplemented!()
        }

        fn set_clear_color(&self, _: [f32; 4]) -> VerboseResult<()> {
            unimplemented!()
        }

        // scene handling
        fn add_scene(&self, _: Arc<dyn TScene + Sync + Send>) -> VerboseResult<()> {
            unimplemented!()
        }

        fn remove_scene(&self, _: &Arc<dyn TScene + Sync + Send>) -> VerboseResult<()> {
            unimplemented!()
        }

        fn clear_scenes(&self) -> VerboseResult<()> {
            unimplemented!()
        }

        // post process handling
        fn add_post_processing_routine(
            &self,
            _post_process: Arc<dyn PostProcess + Sync + Send>,
        ) -> VerboseResult<()> {
            unimplemented!()
        }

        fn remove_post_processing_routine(
            &self,
            _post_process: &Arc<dyn PostProcess + Sync + Send>,
        ) -> VerboseResult<()> {
            unimplemented!()
        }

        fn clear_post_processing_routines(&self) -> VerboseResult<()> {
            unimplemented!()
        }

        // getter
        fn image_count(&self) -> usize {
            unimplemented!()
        }

        fn images(&self) -> VerboseResult<TargetMode<Vec<Arc<Image>>>> {
            unimplemented!()
        }

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

        fn transformations(&self) -> VerboseResult<Option<(VRTransformations, VRTransformations)>> {
            unimplemented!()
        }
    }

    impl std::fmt::Debug for OpenVRRenderCore {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "OpenVRRenderCore {{ }}")
        }
    }
}
