use openxr::{
    CompositionLayerProjection, CompositionLayerProjectionView, Duration, EnvironmentBlendMode,
    EventDataBuffer, Extent2Di, FrameStream, FrameWaiter, Instance as OpenXRInstance, Offset2Di,
    Posef, Quaternionf, Rect2Di, ReferenceSpaceType, Session, SessionState, Space, Swapchain,
    SwapchainCreateFlags, SwapchainCreateInfo, SwapchainSubImage, SwapchainUsageFlags, Vector3f,
    View, ViewConfigurationType, ViewConfigurationView, Vulkan,
};

use cgmath::{vec3, Matrix4, Quaternion, SquareMatrix};

use crate::{p_try, prelude::*, renderbackend::RenderBackend};

use super::openxrintegration::OpenXRIntegration;

use utilities::prelude::*;
use vulkan_rs::prelude::*;

use std::cell::RefCell;
use std::mem;
use std::sync::{Arc, Mutex};

pub struct OpenXRRenderCore {
    instance: Arc<OpenXRInstance>,
    session: Session<Vulkan>,
    frame_waiter: RefCell<FrameWaiter>,
    frame_stream: RefCell<FrameStream<Vulkan>>,

    swapchains: RefCell<TargetMode<Swapchain<Vulkan>>>,

    render_backend: RenderBackend,
    render_fence: Arc<Fence>,

    format: VkFormat,
    width: u32,
    height: u32,
    current_image_indices: RefCell<TargetMode<usize>>,

    space: Space,
    view_config_type: ViewConfigurationType,
    blend_mode: EnvironmentBlendMode,
}

impl OpenXRRenderCore {
    pub fn new(
        xri: &OpenXRIntegration,
        device: &Arc<Device>,
        queue: &Arc<Mutex<Queue>>,
    ) -> VerboseResult<(OpenXRRenderCore, TargetMode<()>)> {
        // create OpenXR session handle
        let (session, frame_waiter, frame_stream) = {
            let queue_lock = queue.lock()?;

            xri.create_session(device, queue_lock.family_index(), queue_lock.queue_index())?
        };

        // request view parameter
        let view_config_type = Self::find_view_config_type(xri)?;
        let view_type_properties = p_try!(xri.view_config_properties(view_config_type));
        println!("OpenXR View Properties: {:#?}", view_type_properties);
        let system_properties = p_try!(xri.system_properties());
        OpenXRIntegration::print_system_properties(&system_properties);
        let (view_config_view, view_count) = Self::find_view_config_view(xri, view_config_type)?;

        // make sure that there are 2 views, since we implemented stereo 'TargetMode'
        if view_count != 2 {
            create_error!("exactly 2 views are required");
        }

        // define image extents
        let width = view_config_view.recommended_image_rect_width;
        let height = view_config_view.recommended_image_rect_height;

        // query format
        let formats = p_try!(session.enumerate_swapchain_formats());

        let format = if formats
            .iter()
            .find(|f| **f == VK_FORMAT_R8G8B8A8_UNORM as u32)
            .is_some()
        {
            VK_FORMAT_R8G8B8A8_UNORM as u32
        } else {
            println!(
                "OpenXR: VK_FORMAT_R8G8B8A8_UNORM not present, take {:?} instead",
                VkFormat::from(formats[0])
            );

            formats[0]
        };

        // create swapchains
        let swapchain_ci = SwapchainCreateInfo {
            create_flags: SwapchainCreateFlags::EMPTY,
            usage_flags: SwapchainUsageFlags::COLOR_ATTACHMENT | SwapchainUsageFlags::TRANSFER_DST,
            format,
            sample_count: view_config_view.recommended_swapchain_sample_count,
            width,
            height,
            face_count: 1,
            array_size: 1,
            mip_count: 1,
        };

        // left swapchain
        let left_swapchain = p_try!(session.create_swapchain(&swapchain_ci));
        let left_swapchain_images = Self::create_swapchain_images(
            &left_swapchain,
            width,
            height,
            VkFormat::from(format),
            device,
            queue,
        )?;

        // right swapchain
        let right_swapchain = p_try!(session.create_swapchain(&swapchain_ci));
        let right_swapchain_images = Self::create_swapchain_images(
            &right_swapchain,
            width,
            height,
            VkFormat::from(format),
            device,
            queue,
        )?;

        let swapchains = TargetMode::Stereo(left_swapchain, right_swapchain);

        // create render backend for the heavy lifting
        let render_backend = RenderBackend::new(
            device,
            queue,
            TargetMode::Stereo(left_swapchain_images, right_swapchain_images),
        )?;

        // query blend mode
        let blend_modes = xri.enumerate_environment_blend_modes(view_config_type)?;

        let blend_mode = if blend_modes.contains(&EnvironmentBlendMode::OPAQUE) {
            EnvironmentBlendMode::OPAQUE
        } else {
            blend_modes[0]
        };

        // create VR space
        let space = Self::create_space(&session)?;

        let openxr_render_core = OpenXRRenderCore {
            instance: xri.instance().clone(),
            session,
            frame_waiter: RefCell::new(frame_waiter),
            frame_stream: RefCell::new(frame_stream),

            swapchains: RefCell::new(swapchains),

            render_backend,
            render_fence: Fence::new().build(device.clone())?,

            format: VkFormat::from(format),
            width,
            height,
            current_image_indices: RefCell::new(TargetMode::Stereo(0, 0)),

            space,
            view_config_type,
            blend_mode,
        };

        Ok((openxr_render_core, TargetMode::Stereo((), ())))
    }

    #[inline]
    fn create_swapchain_images(
        swapchain: &Swapchain<Vulkan>,
        width: u32,
        height: u32,
        format: VkFormat,
        device: &Arc<Device>,
        queue: &Arc<Mutex<Queue>>,
    ) -> VerboseResult<Vec<Arc<Image>>> {
        let images = p_try!(swapchain.enumerate_images());
        let mut swapchain_images = Vec::with_capacity(images.len());

        for image in images {
            swapchain_images.push(
                Image::from_preinitialized(
                    unsafe { mem::transmute(image) },
                    VkFormat::from(format),
                    width,
                    height,
                    VK_IMAGE_LAYOUT_PRESENT_SRC_KHR,
                )
                .nearest_sampler()
                .build(device, queue)?,
            );
        }

        Ok(swapchain_images)
    }

    fn find_view_config_type(xri: &OpenXRIntegration) -> VerboseResult<ViewConfigurationType> {
        let view_config_types = xri.view_configs()?;

        if view_config_types.contains(&ViewConfigurationType::PRIMARY_STEREO) {
            Ok(ViewConfigurationType::PRIMARY_STEREO)
        } else {
            println!(
                "OpenXR: PRIMARY_STEREO not present, take {:?} instead",
                view_config_types[0]
            );

            Ok(view_config_types[0])
        }
    }

    fn find_view_config_view(
        xri: &OpenXRIntegration,
        view_config_type: ViewConfigurationType,
    ) -> VerboseResult<(ViewConfigurationView, usize)> {
        let view_config_views = xri.view_config_views(view_config_type)?;

        // TODO: proper selection
        Ok((view_config_views[0], view_config_views.len()))
    }

    fn create_space(session: &Session<Vulkan>) -> VerboseResult<Space> {
        let space_types = p_try!(session.enumerate_reference_spaces());

        // proper selection
        let space_type = if space_types.contains(&ReferenceSpaceType::LOCAL) {
            ReferenceSpaceType::LOCAL
        } else {
            println!(
                "OpenXR: LOCAL space not present, take {:?} instead",
                space_types[0]
            );

            space_types[0]
        };

        let identity = Posef {
            orientation: Quaternionf {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            position: Vector3f {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        };

        Ok(p_try!(session.create_reference_space(space_type, identity)))
    }

    fn setup_transformations(views: &[View]) -> VerboseResult<TargetMode<VRTransformations>> {
        // only support stereo
        debug_assert!(views.len() == 2);

        let left = VRTransformations {
            proj: Self::proj_from_fov(&views[0], 0.1, 1000.0),
            view: Self::view_from_pose(&views[0])?,
        };

        let right = VRTransformations {
            proj: Self::proj_from_fov(&views[1], 0.1, 1000.0),
            view: Self::view_from_pose(&views[1])?,
        };

        Ok(TargetMode::Stereo(left, right))
    }

    fn proj_from_fov(view: &View, far_z: f32, near_z: f32) -> Matrix4<f32> {
        let fov = view.fov;

        let tan_left = fov.angle_left.tan();
        let tan_right = fov.angle_right.tan();

        let tan_down = fov.angle_down.tan();
        let tan_up = fov.angle_up.tan();

        let tan_width = tan_right - tan_left;
        let tan_height = tan_up - tan_down;

        let a11 = 2.0 / tan_width;
        let a22 = 2.0 / tan_height;

        let a31 = (tan_right + tan_left) / tan_width;
        let a32 = (tan_up + tan_down) / tan_height;
        let a33 = -far_z / (far_z - near_z);

        let a43 = -(far_z * near_z) / (far_z - near_z);

        Matrix4::new(
            a11, 0.0, 0.0, 0.0, 0.0, a22, 0.0, 0.0, a31, a32, a33, -1.0, 0.0, 0.0, a43, 0.0,
        )
    }

    fn view_from_pose(view: &View) -> VerboseResult<Matrix4<f32>> {
        let pose = view.pose;

        let quat = Quaternion::new(
            pose.orientation.w,
            pose.orientation.x,
            pose.orientation.y,
            pose.orientation.z,
        );

        let position = vec3(pose.position.x, pose.position.y, pose.position.z);

        let view = Matrix4::from_translation(position) * Matrix4::from(quat);
        let mut inv_view = view.invert().ok_or("failed to invert view matrix")?;

        Self::fix_handedness(&mut inv_view);

        Ok(inv_view)
    }

    #[inline]
    fn fix_handedness(m: &mut Matrix4<f32>) {
        m[0][1] = -m[0][1];
        m[1][0] = -m[1][0];
        m[1][2] = -m[1][2];
        m[2][1] = -m[2][1];
    }

    fn handle_events(&self) -> VerboseResult<bool> {
        let mut buffer = EventDataBuffer::new();

        while let Some(event) = p_try!(self.instance.poll_event(&mut buffer)) {
            use openxr::Event::*;

            match event {
                SessionStateChanged(session_change) => match session_change.state() {
                    SessionState::READY => {
                        p_try!(self.session.begin(self.view_config_type));
                    }
                    SessionState::STOPPING | SessionState::LOSS_PENDING => {
                        p_try!(self.session.end());
                    }
                    SessionState::EXITING => {
                        return Ok(false);
                    }
                    _ => println!(
                        "unhandled OpenXR session state change: {:?}",
                        session_change.state()
                    ),
                },
                _ => println!("unhandled OpenXR event"),
            }
        }

        Ok(true)
    }
}

impl RenderCore for OpenXRRenderCore {
    fn format(&self) -> VkFormat {
        self.format
    }

    fn next_frame(&self) -> VerboseResult<bool> {
        if !self.handle_events()? {
            return Ok(false);
        }

        let state = p_try!(self.frame_waiter.try_borrow_mut()?.wait());
        let predicted_display_time = state.predicted_display_time;

        let mut frame_stream = self.frame_stream.try_borrow_mut()?;
        p_try!(frame_stream.begin());

        let mut swapchains = self.swapchains.try_borrow_mut()?;
        let (left_eye_swapchain, right_eye_swapchain) = swapchains.stereo_mut()?;

        let (_, views) = p_try!(self.session.locate_views(
            self.view_config_type,
            predicted_display_time,
            &self.space
        ));

        let left_eye_image_index = p_try!(left_eye_swapchain.acquire_image()) as usize;
        let right_eye_image_index = p_try!(right_eye_swapchain.acquire_image()) as usize;

        // rendering
        p_try!(left_eye_swapchain.wait_image(Duration::INFINITE));
        p_try!(right_eye_swapchain.wait_image(Duration::INFINITE));

        *self.current_image_indices.try_borrow_mut()? =
            TargetMode::Stereo(left_eye_image_index, right_eye_image_index);

        if state.should_render {
            let command_buffer = self.render_backend.render(
                self.current_image_indices.try_borrow()?.clone(),
                Some(Self::setup_transformations(&views)?),
            )?;

            let submits = &[SubmitInfo::new()
                .add_command_buffer(command_buffer)
                .add_wait_stage(VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT)];

            {
                let queue_lock = self.render_backend.queue().lock()?;

                queue_lock.submit(Some(&self.render_fence), submits)?;
            }

            // make sure command_buffer is ready
            self.render_backend.device().wait_for_fences(
                &[&self.render_fence],
                true,
                2_000_000_000,
            )?;
            self.render_fence.reset();
        }

        p_try!(left_eye_swapchain.release_image());
        p_try!(right_eye_swapchain.release_image());

        let left_subimage = SwapchainSubImage::new()
            .swapchain(&left_eye_swapchain)
            .image_array_index(0)
            .image_rect(Rect2Di {
                offset: Offset2Di { x: 0, y: 0 },
                extent: Extent2Di {
                    width: self.width as i32,
                    height: self.height as i32,
                },
            });

        let right_subimage = SwapchainSubImage::new()
            .swapchain(&right_eye_swapchain)
            .image_array_index(0)
            .image_rect(Rect2Di {
                offset: Offset2Di { x: 0, y: 0 },
                extent: Extent2Di {
                    width: self.width as i32,
                    height: self.height as i32,
                },
            });

        let projection_view_left = CompositionLayerProjectionView::new()
            .sub_image(left_subimage)
            .fov(views[0].fov)
            .pose(views[0].pose);

        let projection_view_right = CompositionLayerProjectionView::new()
            .sub_image(right_subimage)
            .fov(views[1].fov)
            .pose(views[1].pose);

        let proj_views = [projection_view_left, projection_view_right];
        let projection = CompositionLayerProjection::new()
            .space(&self.space)
            .views(&proj_views);

        p_try!(frame_stream.end(predicted_display_time, self.blend_mode, &[&projection]));

        Ok(true)
    }

    // scene handling
    fn add_scene(&self, scene: Arc<dyn TScene>) -> VerboseResult<()> {
        self.render_backend.add_scene(scene)
    }

    fn remove_scene(&self, scene: &Arc<dyn TScene>) -> VerboseResult<()> {
        self.render_backend.remove_scene(scene)
    }

    fn clear_scenes(&self) -> VerboseResult<()> {
        self.render_backend.clear_scenes()
    }

    // post process handling
    fn add_post_processing_routine(&self, post_process: Arc<dyn PostProcess>) -> VerboseResult<()> {
        self.render_backend
            .add_post_processing_routine(post_process)
    }

    fn remove_post_processing_routine(
        &self,
        post_process: &Arc<dyn PostProcess>,
    ) -> VerboseResult<()> {
        self.render_backend
            .remove_post_processing_routine(post_process)
    }

    fn clear_post_processing_routines(&self) -> VerboseResult<()> {
        self.render_backend.clear_post_processing_routines()
    }

    // getter
    fn image_count(&self) -> usize {
        self.render_backend.image_count()
    }

    fn images(&self) -> TargetMode<Vec<Arc<Image>>> {
        self.render_backend.images()
    }

    fn allocate_primary_buffer(&self) -> VerboseResult<Arc<CommandBuffer>> {
        self.render_backend.allocate_primary_buffer()
    }

    fn allocate_secondary_buffer(&self) -> VerboseResult<Arc<CommandBuffer>> {
        self.render_backend.allocate_secondary_buffer()
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }
}

impl std::fmt::Debug for OpenXRRenderCore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OpenXRRenderCore {{ }}")
    }
}
