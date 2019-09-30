use openvr::{
    compositor::texture::vulkan::Texture as OpenVRVulkanTexture, compositor::texture::ColorSpace,
    compositor::texture::Handle, compositor::texture::Texture, compositor::WaitPoses, Compositor,
    Eye as OpenVREye, System, TrackedDevicePose,
};

use cgmath::Matrix4;

use utilities::prelude::*;
use vulkan_rs::prelude::*;

use super::openvrintegration::OpenVRIntegration;

use crate::{p_try, prelude::*, renderbackend::RenderBackend};

use std::mem::transmute;
use std::rc::Rc;
use std::sync::Arc;

pub struct OpenVRRenderCore {
    compositor: Arc<Compositor>,
    system: Arc<System>,

    render_backend: RenderBackend,

    render_fence: Arc<Fence>,

    openvr_textures: TargetMode<OpenVRVulkanTexture>,
    current_image_indices: TargetMode<usize>,

    width: u32,
    height: u32,
}

impl OpenVRRenderCore {
    pub fn new(
        vri: &OpenVRIntegration,
        device: &Arc<Device>,
        queue: &Arc<Queue>,
    ) -> VerboseResult<(Self, TargetMode<()>)> {
        let sample_count = VK_SAMPLE_COUNT_1_BIT;
        let (width, height) = vri.image_size();
        let format = VK_FORMAT_R8G8B8A8_UNORM;

        let left_image = Image::no_source(
            width,
            height,
            VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_DST_BIT,
            sample_count,
        )
        .nearest_sampler()
        .format(format)
        .build(device, queue)?;

        let right_iamge = Image::no_source(
            width,
            height,
            VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_DST_BIT,
            sample_count,
        )
        .nearest_sampler()
        .format(format)
        .build(device, queue)?;

        let left_openvr_texture = Self::create_openvr_texture(&left_image, sample_count);
        let right_openvr_texture = Self::create_openvr_texture(&right_iamge, sample_count);

        let images = TargetMode::Stereo(vec![left_image], vec![right_iamge]);
        let openvr_textures = TargetMode::Stereo(left_openvr_texture, right_openvr_texture);

        let render_backend = RenderBackend::new(device, queue, images, format)?;

        let openvr_render_core = OpenVRRenderCore {
            compositor: vri.compositor().clone(),
            system: vri.system().clone(),

            render_backend,

            render_fence: Fence::new().build(device.clone())?,

            openvr_textures,
            current_image_indices: TargetMode::Stereo(0, 0),

            width,
            height,
        };

        Ok((openvr_render_core, TargetMode::Stereo((), ())))
    }

    fn create_openvr_texture(
        image: &Arc<Image>,
        sample_count: VkSampleCountFlags,
    ) -> OpenVRVulkanTexture {
        OpenVRVulkanTexture {
            image: unsafe { transmute::<VkImage, u64>(image.vk_handle()) },
            device: unsafe { transmute(image.device().vk_handle()) },
            physical_device: unsafe { transmute(image.device().physical_device().vk_handle()) },
            instance: unsafe { transmute(image.device().physical_device().instance().vk_handle()) },
            queue: unsafe { transmute(image.queue().vk_handle()) },
            queue_family_index: image.queue().family_index(),
            width: image.width(),
            height: image.height(),
            format: image.vk_format() as u32,
            sample_count: sample_count.into(),
        }
    }

    #[inline]
    fn submit_left(&self, vulkan_texture: OpenVRVulkanTexture) -> VerboseResult<()> {
        self.submit(vulkan_texture, OpenVREye::Left)
    }

    #[inline]
    fn submit_right(&self, vulkan_texture: OpenVRVulkanTexture) -> VerboseResult<()> {
        self.submit(vulkan_texture, OpenVREye::Right)
    }

    #[inline]
    fn submit(&self, vulkan_texture: OpenVRVulkanTexture, eye: OpenVREye) -> VerboseResult<()> {
        let texture = Texture {
            handle: Handle::Vulkan(vulkan_texture),
            color_space: ColorSpace::Linear,
        };

        p_try!(unsafe { self.compositor.submit(eye, &texture, None, None) });

        Ok(())
    }

    #[inline]
    fn setup_transformations(
        system: &System,
        wait_poses: WaitPoses,
    ) -> TargetMode<VRTransformations> {
        let left = Self::vr_transform(system, OpenVREye::Left, &wait_poses.render[0]);
        let right = Self::vr_transform(system, OpenVREye::Right, &wait_poses.render[0]);

        TargetMode::Stereo(left, right)
    }

    #[inline]
    fn vr_transform(
        system: &System,
        eye: OpenVREye,
        pose: &TrackedDevicePose,
    ) -> VRTransformations {
        let proj = system.projection_matrix(eye, 0.1, 1000.0);
        let eye = system.eye_to_head_transform(eye);
        let view = *pose.device_to_absolute_tracking();

        VRTransformations {
            proj: Matrix4::from(proj),
            view: Self::openvr43_to_matrix4(eye) * Self::openvr43_to_matrix4(view),
        }
    }

    #[inline]
    fn openvr43_to_matrix4(m: [[f32; 4]; 3]) -> Matrix4<f32> {
        let nm = [m[0], m[1], m[2], [0.0, 0.0, 0.0, 1.0]];
        Matrix4::from(nm)
    }
}

impl RenderCore for OpenVRRenderCore {
    fn next_frame(&self) -> VerboseResult<bool> {
        let wait_poses = p_try!(self.compositor.wait_get_poses());

        let transforms = Self::setup_transformations(&self.system, wait_poses);

        let command_buffer = self
            .render_backend
            .render(self.current_image_indices.clone(), Some(transforms))?;

        let submits = &[SubmitInfo::new().add_command_buffer(&command_buffer)];

        self.render_backend
            .queue()
            .submit(Some(&self.render_fence), submits)?;

        // make sure command_buffer is ready
        self.render_backend
            .device()
            .wait_for_fences(&[&self.render_fence], true, 2_000_000_000)?;
        self.render_fence.reset();

        let (left_texture, right_texture) = self.openvr_textures.stereo()?;

        self.submit_left(*left_texture)?;
        self.submit_right(*right_texture)?;

        Ok(true)
    }

    // scene handling
    fn add_scene(&self, scene: Rc<dyn TScene>) -> VerboseResult<()> {
        self.render_backend.add_scene(scene)
    }

    fn remove_scene(&self, scene: &Rc<dyn TScene>) -> VerboseResult<()> {
        self.render_backend.remove_scene(scene)
    }

    fn clear_scenes(&self) -> VerboseResult<()> {
        self.render_backend.clear_scenes()
    }

    // callbacks
    fn set_resize_callback(
        &self,
        resize_callback: Option<Box<dyn Fn(u32, u32) -> VerboseResult<()>>>,
    ) -> VerboseResult<()> {
        self.render_backend.set_resize_callback(resize_callback)
    }

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
    ) -> VerboseResult<()> {
        self.render_backend.set_gui_callback(render_gui)
    }

    // getter
    fn image_count(&self) -> usize {
        self.render_backend.image_count()
    }

    fn images(&self) -> TargetMode<Vec<Arc<Image>>> {
        self.render_backend.images()
    }

    fn gui_render_pass(&self) -> &Arc<RenderPass> {
        &self.render_backend.gui_render_pass()
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