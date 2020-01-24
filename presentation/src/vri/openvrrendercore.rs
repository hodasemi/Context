use openvr::{
    compositor::texture::vulkan::Texture as OpenVRVulkanTexture, compositor::texture::ColorSpace,
    compositor::texture::Handle, compositor::texture::Texture, compositor::WaitPoses, Compositor,
    Eye as OpenVREye, System, TrackedDeviceClass, TrackedDevicePose,
};

use cgmath::{vec4, Matrix4, SquareMatrix};

use utilities::prelude::*;
use vulkan_rs::prelude::*;

use super::openvrintegration::OpenVRIntegration;

use crate::{p_try, prelude::*, renderbackend::RenderBackend, RenderCoreCreateInfo};

use std::mem::transmute;
use std::sync::{Arc, Mutex, RwLock};

pub struct OpenVRRenderCore {
    compositor: Arc<Compositor>,
    system: Arc<System>,

    render_backend: RenderBackend,

    render_fence: Arc<Fence>,

    format: VkFormat,

    current_image_indices: TargetMode<usize>,
    images: TargetMode<Vec<Arc<Image>>>,
    transformations: RwLock<(VRTransformations, VRTransformations)>,

    width: u32,
    height: u32,
}

impl OpenVRRenderCore {
    pub fn new(
        vri: &OpenVRIntegration,
        device: &Arc<Device>,
        queue: &Arc<Mutex<Queue>>,
        create_info: RenderCoreCreateInfo,
    ) -> VerboseResult<(Self, TargetMode<()>)> {
        let sample_count = VK_SAMPLE_COUNT_1_BIT;
        let (width, height) = vri.image_size();

        let (left_image, right_image) = Self::create_target_images(
            width,
            height,
            sample_count,
            create_info.usage,
            create_info.format,
            device,
            queue,
        )?;

        let images = TargetMode::Stereo(vec![left_image], vec![right_image]);
        let render_backend = RenderBackend::new(device, queue, images.clone())?;

        let openvr_render_core = OpenVRRenderCore {
            compositor: vri.compositor().clone(),
            system: vri.system().clone(),

            render_backend,

            render_fence: Fence::builder().build(device.clone())?,

            format: create_info.format,

            current_image_indices: TargetMode::Stereo(0, 0),
            images,
            transformations: RwLock::new((
                VRTransformations::default(),
                VRTransformations::default(),
            )),

            width,
            height,
        };

        let post_process = PostRenderLayoutBarrier::new(&openvr_render_core)?;
        openvr_render_core
            .render_backend
            .add_post_processing_routine(post_process)?;

        Ok((openvr_render_core, TargetMode::Stereo((), ())))
    }

    #[inline]
    fn create_target_images(
        width: u32,
        height: u32,
        sample_count: VkSampleCountFlags,
        usage: VkImageUsageFlagBits,
        format: VkFormat,
        device: &Arc<Device>,
        queue: &Arc<Mutex<Queue>>,
    ) -> VerboseResult<(Arc<Image>, Arc<Image>)> {
        // OpenVR requires the image to be transfer_src and sampled
        let image_usage = usage | VK_IMAGE_USAGE_SAMPLED_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT;

        if !Image::check_configuration(device, VK_IMAGE_TILING_OPTIMAL, format, usage) {
            create_error!(format!(
                "wrong config: {:?}, {:?}, {:?}",
                VK_IMAGE_TILING_OPTIMAL, format, usage
            ));
        }

        let left_image = Image::empty(width, height, image_usage, sample_count)
            .nearest_sampler()
            .format(format)
            .build(device, queue)?;

        left_image.convert_layout(VK_IMAGE_LAYOUT_PRESENT_SRC_KHR)?;

        let right_image = Image::empty(width, height, image_usage, sample_count)
            .nearest_sampler()
            .format(format)
            .build(device, queue)?;

        right_image.convert_layout(VK_IMAGE_LAYOUT_PRESENT_SRC_KHR)?;

        Ok((left_image, right_image))
    }

    #[inline]
    fn submit_left(&self, image: &Arc<Image>) -> VerboseResult<()> {
        self.submit(image, OpenVREye::Left)
    }

    #[inline]
    fn submit_right(&self, image: &Arc<Image>) -> VerboseResult<()> {
        self.submit(image, OpenVREye::Right)
    }

    #[inline]
    fn submit(&self, image: &Arc<Image>, eye: OpenVREye) -> VerboseResult<()> {
        let queue_lock = image.queue().lock()?;

        let vulkan_texture = OpenVRVulkanTexture {
            image: unsafe { transmute::<VkImage, u64>(image.vk_handle()) },
            device: unsafe { transmute(image.device().vk_handle()) },
            physical_device: unsafe { transmute(image.device().physical_device().vk_handle()) },
            instance: unsafe { transmute(image.device().physical_device().instance().vk_handle()) },
            queue: unsafe { transmute(queue_lock.vk_handle()) },
            queue_family_index: queue_lock.family_index(),

            width: image.width(),
            height: image.height(),
            format: image.vk_format() as u32,
            sample_count: image.sample_count().into(),
        };

        let texture = Texture {
            handle: Handle::Vulkan(vulkan_texture),
            color_space: ColorSpace::Auto,
        };

        if let Err(err) = unsafe { self.compositor.submit(eye, &texture, None, None) } {
            create_error!(format!(
                "image ({:#?}) failed submission for {:?} eye with error {:?}",
                image, eye, err
            ));
        }

        Ok(())
    }

    #[inline]
    fn find_tracked_hmd(
        system: &System,
        poses: [TrackedDevicePose; 64],
    ) -> Option<TrackedDevicePose> {
        for (i, pose) in poses.iter().enumerate() {
            if system.tracked_device_class(i as u32) == TrackedDeviceClass::HMD
                && pose.pose_is_valid()
                && pose.device_is_connected()
            {
                return Some(*pose);
            }
        }

        None
    }

    #[inline]
    fn setup_transformations(
        system: &System,
        wait_poses: WaitPoses,
    ) -> (VRTransformations, VRTransformations) {
        let pose = Self::find_tracked_hmd(system, wait_poses.render);

        let left = Self::vr_transform(system, OpenVREye::Left, pose);
        let right = Self::vr_transform(system, OpenVREye::Right, pose);

        (left, right)
    }

    #[inline]
    fn vr_transform(
        system: &System,
        eye: OpenVREye,
        pose: Option<TrackedDevicePose>,
    ) -> VRTransformations {
        let proj = system.projection_matrix(eye, 0.1, 1000.0);
        let eye = system.eye_to_head_transform(eye);

        let view = match pose {
            Some(pose) => Self::openvr43_to_matrix4(*pose.device_to_absolute_tracking())
                .invert()
                .expect("failed to invert OpenVR View Matrix"),
            None => Matrix4::identity(),
        };

        VRTransformations {
            proj: Self::openvr44_to_matrix4(proj),
            view: Self::openvr43_to_matrix4(eye) * view,
        }
    }

    #[inline]
    fn openvr44_to_matrix4(m: [[f32; 4]; 4]) -> Matrix4<f32> {
        let col_0 = vec4(m[0][0], m[1][0], m[2][0], m[3][0]);
        let col_1 = vec4(m[0][1], -m[1][1], m[2][1], m[3][1]);
        let col_2 = vec4(m[0][2], m[1][2], m[2][2], m[3][2]);
        let col_3 = vec4(m[0][3], m[1][3], m[2][3], m[3][3]);

        Matrix4::from_cols(col_0, col_1, col_2, col_3)
    }

    #[inline]
    fn openvr43_to_matrix4(m: [[f32; 4]; 3]) -> Matrix4<f32> {
        let col_0 = vec4(m[0][0], m[1][0], m[2][0], 0.0);
        let col_1 = vec4(m[0][1], m[1][1], m[2][1], 0.0);
        let col_2 = vec4(m[0][2], m[1][2], m[2][2], 0.0);
        let col_3 = vec4(m[0][3], m[1][3], m[2][3], 1.0);

        Matrix4::from_cols(col_0, col_1, col_2, col_3)
    }
}

impl RenderCore for OpenVRRenderCore {
    fn format(&self) -> VkFormat {
        self.format
    }

    fn next_frame(&self) -> VerboseResult<bool> {
        let wait_poses = p_try!(self.compositor.wait_get_poses());

        *self.transformations.write()? = Self::setup_transformations(&self.system, wait_poses);

        let command_buffer = self
            .render_backend
            .render(self.current_image_indices.clone())?;

        let submits = &[SubmitInfo::default().add_command_buffer(&command_buffer)];

        {
            let queue_lock = self.render_backend.queue().lock()?;

            queue_lock.submit(Some(&self.render_fence), submits)?;
        }

        // make sure command_buffer is ready
        self.render_backend
            .device()
            .wait_for_fences(&[&self.render_fence], true, 2_000_000_000)?;
        self.render_fence.reset();

        let (left_images, right_images) = self.images.stereo()?;
        let (left_index, right_index) = self.current_image_indices.stereo()?;

        self.submit_left(&left_images[*left_index])?;
        self.submit_right(&right_images[*right_index])?;

        Ok(true)
    }

    fn set_clear_color(&self, color: [f32; 4]) -> VerboseResult<()> {
        self.render_backend.set_clear_color(color)
    }

    // scene handling
    fn add_scene(&self, scene: Arc<dyn TScene + Send + Sync>) -> VerboseResult<()> {
        self.render_backend.add_scene(scene)
    }

    fn remove_scene(&self, scene: &Arc<dyn TScene + Send + Sync>) -> VerboseResult<()> {
        self.render_backend.remove_scene(scene)
    }

    fn clear_scenes(&self) -> VerboseResult<()> {
        self.render_backend.clear_scenes()
    }

    // post process handling
    fn add_post_processing_routine(
        &self,
        post_process: Arc<dyn PostProcess + Send + Sync>,
    ) -> VerboseResult<()> {
        self.render_backend
            .add_post_processing_routine(post_process)
    }

    fn remove_post_processing_routine(
        &self,
        post_process: &Arc<dyn PostProcess + Send + Sync>,
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

    fn images(&self) -> VerboseResult<TargetMode<Vec<Arc<Image>>>> {
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

    fn transformations(&self) -> VerboseResult<Option<(VRTransformations, VRTransformations)>> {
        Ok(Some(self.transformations.read()?.clone()))
    }
}

impl std::fmt::Debug for OpenVRRenderCore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OpenVRRenderCore {{ }}")
    }
}

struct PostRenderLayoutBarrier {
    left_images: Vec<Arc<Image>>,
    right_images: Vec<Arc<Image>>,
}

impl PostRenderLayoutBarrier {
    fn new(render_core: &OpenVRRenderCore) -> VerboseResult<Arc<Self>> {
        let (left_images, right_images) = render_core.images.stereo()?;

        Ok(Arc::new(PostRenderLayoutBarrier {
            left_images: left_images.clone(),
            right_images: right_images.clone(),
        }))
    }
}

impl PostProcess for PostRenderLayoutBarrier {
    fn priority(&self) -> u32 {
        // priority == 0 means that is executed lastly
        0
    }

    fn process(
        &self,
        command_buffer: &Arc<CommandBuffer>,
        indices: &TargetMode<usize>,
    ) -> VerboseResult<()> {
        let (left_index, right_index) = indices.stereo()?;

        command_buffer.set_full_image_layout(
            &self.left_images[*left_index],
            VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
        )?;

        command_buffer.set_full_image_layout(
            &self.right_images[*right_index],
            VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
        )?;

        Ok(())
    }

    fn resize(&self, _: u32, _: u32) -> VerboseResult<()> {
        Ok(())
    }
}
