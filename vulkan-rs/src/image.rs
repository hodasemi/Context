use utilities::prelude::*;

use crate::impl_vk_handle;
use crate::prelude::*;

use std::cmp;
use std::sync::{Arc, Mutex};

enum ImageSourceType {
    Empty,
    Raw(Vec<u8>),
    Array(Vec<Arc<Image>>),
}

struct ImageCreateInfo {
    vk_image_create_info: VkImageCreateInfo,

    source_type: ImageSourceType,
    memory_properties: VkMemoryPropertyFlagBits,
}

impl ImageCreateInfo {
    fn default(source_type: ImageSourceType) -> Self {
        ImageCreateInfo {
            vk_image_create_info: VkImageCreateInfo::new(
                0,
                VK_IMAGE_TYPE_2D,
                VK_FORMAT_UNDEFINED,
                VkExtent3D {
                    width: 0,
                    height: 0,
                    depth: 0,
                },
                1,
                1,
                VK_SAMPLE_COUNT_1_BIT,
                VK_IMAGE_TILING_OPTIMAL,
                VK_IMAGE_USAGE_TRANSFER_DST_BIT,
                VK_SHARING_MODE_EXCLUSIVE,
                &[],
                VK_IMAGE_LAYOUT_UNDEFINED,
            ),

            source_type,
            memory_properties: VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT.into(),
        }
    }
}

struct PreinitializedImage {
    image: VkImage,
    format: VkFormat,

    width: u32,
    height: u32,

    layers: u32,
    sample_count: VkSampleCountFlagBits,
    layout: VkImageLayout,
}

enum ImageBuilderInternalType {
    PreinitializedImage(PreinitializedImage),
    NewImage(ImageCreateInfo),
}

/// Implements the builder pattern for Image
pub struct ImageBuilder {
    builder_type: ImageBuilderInternalType,
    sampler_info: Option<VkSamplerCreateInfo>,
    components: VkComponentMapping,
    view_type: VkImageViewType,
    subresource_range: VkImageSubresourceRange,
}

impl ImageBuilder {
    /// Sets up the ImageBuilder for further use
    fn new(internal_type: ImageBuilderInternalType) -> Self {
        ImageBuilder {
            builder_type: internal_type,
            sampler_info: None,
            components: VkComponentMapping::default(),
            subresource_range: VkImageSubresourceRange {
                aspectMask: VK_IMAGE_ASPECT_COLOR_BIT.into(),
                baseMipLevel: 0,
                levelCount: 1,
                baseArrayLayer: 0,
                layerCount: 1,
            },
            view_type: VK_IMAGE_VIEW_TYPE_2D,
        }
    }

    pub fn build(
        self,
        device: &Arc<Device>,
        queue: &Arc<Mutex<Queue>>,
    ) -> VerboseResult<Arc<Image>> {
        let mut image_view_ci = self.vk_image_view_create_info();

        match self.builder_type {
            ImageBuilderInternalType::PreinitializedImage(preinitialized_image) => {
                image_view_ci.image = preinitialized_image.image;

                let image_view = device.create_image_view(&image_view_ci)?;

                let sampler = match self.sampler_info {
                    Some(sampler_info) => device.create_sampler(&sampler_info)?,
                    None => VkSampler::NULL_HANDLE,
                };

                let image = Arc::new(Image {
                    device: device.clone(),
                    queue: queue.clone(),

                    image: preinitialized_image.image,
                    image_view,
                    memory: None,
                    attached: true,
                    sampler,

                    format: preinitialized_image.format,
                    image_layout: Mutex::new(VK_IMAGE_LAYOUT_UNDEFINED),

                    aspect_mask: self.subresource_range.aspectMask,

                    width: preinitialized_image.width,
                    height: preinitialized_image.height,
                    layers: preinitialized_image.layers,
                    levels: 1,
                    sample_count: preinitialized_image.sample_count,
                });

                if preinitialized_image.layout != VK_IMAGE_LAYOUT_UNDEFINED {
                    image.convert_layout(preinitialized_image.layout)?;
                }

                Ok(image)
            }
            ImageBuilderInternalType::NewImage(ref info) => match info.source_type {
                ImageSourceType::Array(ref array) => {
                    let arc_image = Self::create_from_source(
                        device,
                        queue,
                        &info,
                        &self.sampler_info,
                        image_view_ci,
                    )?;

                    copy_images_to_imagearray(&device, &queue, &arc_image, &array)?;

                    Ok(arc_image)
                }
                ImageSourceType::Raw(ref raw) => {
                    let arc_image = Self::create_from_source(
                        device,
                        queue,
                        &info,
                        &self.sampler_info,
                        image_view_ci,
                    )?;

                    Self::optimize_fill(&device, &queue, &raw, &arc_image)?;

                    Ok(arc_image)
                }
                ImageSourceType::Empty => {
                    let arc_image = Self::create_from_source(
                        device,
                        queue,
                        &info,
                        &self.sampler_info,
                        image_view_ci,
                    )?;

                    Ok(arc_image)
                }
            },
        }
    }

    pub fn check_configuration(&self, device: &Arc<Device>) -> bool {
        let physical_device = device.physical_device();

        match &self.builder_type {
            ImageBuilderInternalType::NewImage(create_info) => {
                let format = create_info.vk_image_create_info.format;

                match create_info.vk_image_create_info.tiling {
                    VK_IMAGE_TILING_OPTIMAL => physical_device.check_optimal_format_features(
                        format,
                        create_info.vk_image_create_info.usage,
                    ),
                    VK_IMAGE_TILING_LINEAR => physical_device.check_linear_format_features(
                        format,
                        create_info.vk_image_create_info.usage,
                    ),
                }
            }
            _ => true,
        }
    }

    pub fn view_type(mut self, view_type: VkImageViewType) -> Self {
        self.view_type = view_type;

        self
    }

    pub fn component_swizzle(
        mut self,
        r: VkComponentSwizzle,
        g: VkComponentSwizzle,
        b: VkComponentSwizzle,
        a: VkComponentSwizzle,
    ) -> Self {
        self.components.r = r;
        self.components.g = g;
        self.components.b = b;
        self.components.a = a;

        self
    }

    pub fn update_data(mut self, data: Vec<u8>) -> Self {
        match self.builder_type {
            ImageBuilderInternalType::NewImage(ref mut info) => match info.source_type {
                ImageSourceType::Raw(ref mut old_data) => *old_data = data,
                _ => panic!("wrong source type in ImageBuilder"),
            },
            _ => panic!("wrong builder type in ImageBuilder"),
        }

        self
    }

    pub fn format(mut self, format: VkFormat) -> Self {
        match &mut self.builder_type {
            ImageBuilderInternalType::NewImage(info) => {
                info.vk_image_create_info.format = format;
            }
            ImageBuilderInternalType::PreinitializedImage(preinitialized_image) => {
                preinitialized_image.format = format;
            }
        }

        self
    }

    pub fn sample_count(mut self, sample_count: impl Into<VkSampleCountFlagBits>) -> Self {
        match &mut self.builder_type {
            ImageBuilderInternalType::NewImage(info) => {
                info.vk_image_create_info.samples = sample_count.into();
            }
            ImageBuilderInternalType::PreinitializedImage(preinitialized_image) => {
                preinitialized_image.sample_count = sample_count.into();
            }
        }

        self
    }

    pub fn add_usage<T>(mut self, usage: T) -> Self
    where
        T: Into<VkImageUsageFlagBits>,
    {
        match self.builder_type {
            ImageBuilderInternalType::NewImage(ref mut info) => {
                info.vk_image_create_info.usage |= usage.into();
            }
            _ => panic!("wrong builder type in ImageBuilder"),
        }

        self
    }

    pub fn flags<T>(mut self, flags: T) -> Self
    where
        T: Into<VkImageCreateFlagBits>,
    {
        match self.builder_type {
            ImageBuilderInternalType::NewImage(ref mut info) => {
                info.vk_image_create_info.flags = flags.into();
            }
            _ => panic!("wrong builder type in ImageBuilder"),
        }

        self
    }

    pub fn array_layers(mut self, layers: u32) -> Self {
        match &mut self.builder_type {
            ImageBuilderInternalType::NewImage(info) => {
                info.vk_image_create_info.arrayLayers = layers;
                self.subresource_range.layerCount = layers;
            }
            ImageBuilderInternalType::PreinitializedImage(preinitialized_image) => {
                preinitialized_image.layers = layers;
            }
        }

        self
    }

    pub fn mip_map_levels(mut self, levels: u32) -> Self {
        match self.builder_type {
            ImageBuilderInternalType::NewImage(ref mut info) => {
                info.vk_image_create_info.mipLevels = levels;
                self.subresource_range.levelCount = levels;

                info.vk_image_create_info.usage |=
                    VK_IMAGE_USAGE_TRANSFER_DST_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT;

                if let Some(ref mut sampler) = self.sampler_info {
                    sampler.maxLod = levels as f32;
                }
            }
            _ => panic!("wrong builder type in ImageBuilder"),
        }

        self
    }

    pub fn max_mip_map_levels(mut self) -> Self {
        match self.builder_type {
            ImageBuilderInternalType::NewImage(ref mut info) => {
                let levels = Self::calc_mip_map_levels(
                    info.vk_image_create_info.extent.width,
                    info.vk_image_create_info.extent.height,
                );

                info.vk_image_create_info.mipLevels = levels;
                self.subresource_range.levelCount = levels;

                info.vk_image_create_info.usage |=
                    VK_IMAGE_USAGE_TRANSFER_DST_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT;

                if let Some(ref mut sampler) = self.sampler_info {
                    sampler.maxLod = levels as f32;
                }
            }
            _ => panic!("wrong builder type in ImageBuilder"),
        }

        self
    }

    pub fn aspect_mask(mut self, mask: VkImageAspectFlags) -> Self {
        self.subresource_range.aspectMask = mask.into();

        self
    }

    pub fn memory_properties<T>(mut self, properties: T) -> Self
    where
        T: Into<VkMemoryPropertyFlagBits>,
    {
        match self.builder_type {
            ImageBuilderInternalType::NewImage(ref mut info) => {
                info.memory_properties = properties.into();
            }
            _ => panic!("wrong builder type in ImageBuilder"),
        }

        self
    }

    pub fn nearest_sampler(mut self) -> Self {
        self.new_sampler(VkSamplerCreateInfo::new(
            0,
            VK_FILTER_NEAREST,
            VK_FILTER_NEAREST,
            VK_SAMPLER_MIPMAP_MODE_NEAREST,
            VK_SAMPLER_ADDRESS_MODE_REPEAT,
            VK_SAMPLER_ADDRESS_MODE_REPEAT,
            VK_SAMPLER_ADDRESS_MODE_REPEAT,
            0.0,
            false,
            1.0,
            false,
            VK_COMPARE_OP_NEVER,
            0.0,
            0.0,
            VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE,
            false,
        ));

        self
    }

    pub fn pretty_sampler(mut self) -> Self {
        self.new_sampler(VkSamplerCreateInfo::new(
            0,
            VK_FILTER_LINEAR,
            VK_FILTER_LINEAR,
            VK_SAMPLER_MIPMAP_MODE_LINEAR,
            VK_SAMPLER_ADDRESS_MODE_REPEAT,
            VK_SAMPLER_ADDRESS_MODE_REPEAT,
            VK_SAMPLER_ADDRESS_MODE_REPEAT,
            0.0,
            true,
            8.0,
            false,
            VK_COMPARE_OP_NEVER,
            0.0,
            0.0,
            VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE,
            false,
        ));

        self
    }

    fn new_sampler(&mut self, mut sampler_info: VkSamplerCreateInfo) {
        if let ImageBuilderInternalType::NewImage(ref mut info) = self.builder_type {
            sampler_info.maxLod = info.vk_image_create_info.mipLevels as f32;
            info.vk_image_create_info.usage |= VK_IMAGE_USAGE_SAMPLED_BIT;
        }

        self.sampler_info = Some(sampler_info);
    }

    pub fn sampler_min_mag_filter(mut self, min_filter: VkFilter, mag_filter: VkFilter) -> Self {
        match self.sampler_info {
            Some(ref mut sampler_info) => {
                sampler_info.minFilter = min_filter;
                sampler_info.magFilter = mag_filter;
            }
            None => panic!("no sampler info attached in ImageBuilder"),
        }

        self
    }

    pub fn sampler_map_map_mode(mut self, mode: VkSamplerMipmapMode) -> Self {
        match self.sampler_info {
            Some(ref mut sampler_info) => {
                sampler_info.mipmapMode = mode;
            }
            None => panic!("no sampler info attached in ImageBuilder"),
        }

        self
    }

    pub fn sampler_address_mode(
        mut self,
        u: VkSamplerAddressMode,
        v: VkSamplerAddressMode,
        w: VkSamplerAddressMode,
    ) -> Self {
        match self.sampler_info {
            Some(ref mut sampler_info) => {
                sampler_info.addressModeU = u;
                sampler_info.addressModeV = v;
                sampler_info.addressModeW = w;
            }
            None => panic!("no sampler info attached in ImageBuilder"),
        }

        self
    }

    pub fn sampler_min_load_bias(mut self, bias: f32) -> Self {
        match self.sampler_info {
            Some(ref mut sampler_info) => {
                sampler_info.mipLodBias = bias;
            }
            None => panic!("no sampler info attached in ImageBuilder"),
        }

        self
    }

    pub fn sampler_anisotropy(mut self, anisotropy: f32) -> Self {
        match self.sampler_info {
            Some(ref mut sampler_info) => {
                sampler_info.anisotropyEnable = VK_TRUE;
                sampler_info.maxAnisotropy = anisotropy;
            }
            None => panic!("no sampler info attached in ImageBuilder"),
        }

        self
    }

    pub fn sampler_compare(mut self, compare_op: VkCompareOp) -> Self {
        match self.sampler_info {
            Some(ref mut sampler_info) => {
                sampler_info.compareEnable = VK_TRUE;
                sampler_info.compareOp = compare_op;
            }
            None => panic!("no sampler info attached in ImageBuilder"),
        }

        self
    }

    pub fn sampler_min_max_lod(mut self, min_lod: f32, max_lod: f32) -> Self {
        match self.sampler_info {
            Some(ref mut sampler_info) => {
                sampler_info.minLod = min_lod;
                sampler_info.maxLod = max_lod;
            }
            None => panic!("no sampler info attached in ImageBuilder"),
        }

        self
    }

    pub fn sampler_border_color(mut self, border_color: VkBorderColor) -> Self {
        match self.sampler_info {
            Some(ref mut sampler_info) => {
                sampler_info.borderColor = border_color;
            }
            None => panic!("no sampler info attached in ImageBuilder"),
        }

        self
    }

    pub fn sampler_coordinates<T>(mut self, unnormalized_coordinates: T) -> Self
    where
        T: Into<VkBool32>,
    {
        match self.sampler_info {
            Some(ref mut sampler_info) => {
                sampler_info.unnormalizedCoordinates = unnormalized_coordinates.into()
            }
            None => panic!("no sampler info attached in ImageBuilder"),
        }

        self
    }

    fn calc_mip_map_levels(width: u32, height: u32) -> u32 {
        1 + (cmp::max(width, height) as f32).log2().floor() as u32
    }

    fn vk_image_view_create_info(&self) -> VkImageViewCreateInfo {
        VkImageViewCreateInfo::new(
            0,
            VkImage::NULL_HANDLE,
            self.view_type,
            match &self.builder_type {
                ImageBuilderInternalType::NewImage(info) => info.vk_image_create_info.format,
                ImageBuilderInternalType::PreinitializedImage(preinitialized_image) => {
                    preinitialized_image.format
                }
            },
            self.components.clone(),
            self.subresource_range.clone(),
        )
    }

    fn create_from_source(
        device: &Arc<Device>,
        queue: &Arc<Mutex<Queue>>,
        info: &ImageCreateInfo,
        sampler_info: &Option<VkSamplerCreateInfo>,
        mut view_ci: VkImageViewCreateInfo,
    ) -> VerboseResult<Arc<Image>> {
        let format = view_ci.format;

        let (image, memory) =
            Self::create_texture(device, &info.vk_image_create_info, info.memory_properties)?;

        view_ci.image = image;

        let image_view = device.create_image_view(&view_ci)?;

        let sampler = match sampler_info {
            Some(ref sampler_ci) => device.create_sampler(&sampler_ci)?,
            None => VkSampler::NULL_HANDLE,
        };

        Ok(Arc::new(Image {
            device: device.clone(),
            queue: queue.clone(),

            image,
            image_view,
            attached: false,
            memory: Some(memory),
            sampler,

            format,
            image_layout: Mutex::new(info.vk_image_create_info.initialLayout),

            aspect_mask: view_ci.subresourceRange.aspectMask,

            width: info.vk_image_create_info.extent.width,
            height: info.vk_image_create_info.extent.height,
            layers: info.vk_image_create_info.arrayLayers,
            levels: info.vk_image_create_info.mipLevels,
            sample_count: info.vk_image_create_info.samples,
        }))
    }

    fn create_texture(
        device: &Arc<Device>,
        image_ci: &VkImageCreateInfo,
        reqs: VkMemoryPropertyFlagBits,
    ) -> VerboseResult<(VkImage, Arc<Memory<u8>>)> {
        let image = Self::create_image(device, image_ci)?;
        let memory = Memory::image_memory(device, reqs, image)?;

        Ok((image, memory))
    }

    fn create_image(device: &Arc<Device>, image_ci: &VkImageCreateInfo) -> VerboseResult<VkImage> {
        device.create_image(&image_ci)
    }

    fn optimize_fill(
        device: &Arc<Device>,
        queue: &Arc<Mutex<Queue>>,
        data: &[u8],
        image: &Arc<Image>,
    ) -> VerboseResult<()> {
        let staging_buffer = Buffer::builder()
            .set_usage(VK_BUFFER_USAGE_TRANSFER_SRC_BIT)
            .set_memory_properties(
                VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT,
            )
            .set_data(data)
            .build(device.clone())?;

        copy_buffer_to_image(device, queue, &staging_buffer, image)?;

        Ok(())
    }
}

/// Wrapper type around VkImage
///
/// handles VkImage, VkSampler, VkDeviceSize and VkImageView internally
/// just as you set it up to
#[derive(Debug)]
pub struct Image {
    // device handle
    device: Arc<Device>,

    // queue handle
    queue: Arc<Mutex<Queue>>,

    // image handle
    attached: bool,
    image: VkImage,

    // image_view
    image_view: VkImageView,

    // optional handles
    memory: Option<Arc<Memory<u8>>>,
    sampler: VkSampler,

    // image information
    format: VkFormat,
    image_layout: Mutex<VkImageLayout>,

    aspect_mask: VkImageAspectFlagBits,
    width: u32,
    height: u32,
    layers: u32, // array layers
    levels: u32, // mip map levels
    sample_count: VkSampleCountFlagBits,
}

impl Image {
    /// Creates an `ImageBuilder` where you can define the image for your needs
    ///
    /// For example, this is used to wrap swapchain images
    ///
    /// # Arguments
    ///
    /// * `image` - valid VkImage handle
    /// * `format` -  format of this image
    pub fn from_preinitialized(
        image: VkImage,
        format: VkFormat,
        width: u32,
        height: u32,
        layout: VkImageLayout,
    ) -> ImageBuilder {
        ImageBuilder::new(ImageBuilderInternalType::PreinitializedImage(
            PreinitializedImage {
                image,
                format,
                width,
                height,
                layers: 1,
                sample_count: VK_SAMPLE_COUNT_1_BIT.into(),
                layout,
            },
        ))
    }

    /// Creates an `ImageBuilder` where you can define the image for your needs
    ///
    /// takes the image data in form of an Vec<u8> and sets up the `ImageBuilder`
    /// for further configuration
    ///
    /// # Arguments
    ///
    /// * `source` - The color information for the image
    /// * `width` - The target width of the image
    /// * `height` - The target height of the image
    pub fn from_raw(source: Vec<u8>, width: u32, height: u32) -> ImageBuilder {
        let mut create_info = ImageCreateInfo::default(ImageSourceType::Raw(source));
        create_info.vk_image_create_info.extent.width = width;
        create_info.vk_image_create_info.extent.height = height;
        create_info.vk_image_create_info.extent.depth = 1;
        create_info.vk_image_create_info.usage |= VK_IMAGE_USAGE_TRANSFER_SRC_BIT;

        ImageBuilder::new(ImageBuilderInternalType::NewImage(create_info))
    }

    /// Creates an `ImageBuilder` where you can define the image for your needs
    ///
    /// takes a path to the file and does the same as `raw_source`, but it
    /// extracts all needed bits from the file
    ///
    /// # Arguments
    ///
    /// * `file` - The path to the file
    pub fn from_file(file: &str) -> VerboseResult<ImageBuilder> {
        let texture = match image::open(file) {
            Ok(tex) => tex.to_rgba(),
            Err(err) => create_error!(format!("error loading image (\"{}\"): {}", file, err)),
        };

        let (width, height) = texture.dimensions();

        Ok(Self::from_raw(texture.into_raw(), width, height).format(VK_FORMAT_R8G8B8A8_UNORM))
    }

    /// Creates an `ImageBuilder` where you can define the image for your needs
    ///
    /// takes an array `Arc<Image>`'s and setups the `ImageBuilder` to create
    /// a single `Arc<Image>` with an 2d image array created from the provided images
    ///
    /// # Arguments
    ///
    /// * `array` - Source images
    pub fn from_array(array: Vec<Arc<Image>>) -> ImageBuilder {
        debug_assert!(array.is_empty(), "images array must not be empty");

        let width = array[0].width();
        let height = array[0].height();

        if cfg!(debug_assertions) {
            for image in &array {
                if width != image.width() || height != image.height() {
                    panic!("images are not equally sized");
                }
            }
        }

        let array_len = array.len() as u32;
        let mut create_info = ImageCreateInfo::default(ImageSourceType::Array(array));
        create_info.vk_image_create_info.arrayLayers = array_len;
        create_info.vk_image_create_info.imageType = VK_IMAGE_TYPE_2D;
        create_info.vk_image_create_info.extent.width = width;
        create_info.vk_image_create_info.extent.height = height;
        create_info.vk_image_create_info.extent.depth = 1;

        let mut image_builder = ImageBuilder::new(ImageBuilderInternalType::NewImage(create_info));
        image_builder.view_type = VK_IMAGE_VIEW_TYPE_2D_ARRAY;
        image_builder.subresource_range.layerCount = array_len;

        image_builder
    }

    /// Creates an `ImageBuilder` where you can define the image for your needs
    ///
    /// takes raw information to setup `ImageBuilder`, that creates an `Arc<Image>`
    /// with no color information
    ///
    /// # Arguments
    ///
    /// * `width` - The target width of the image
    /// * `height` - The target height of the image
    /// * `usage` - `VkImageUsageFlagBits` mask to define the image usage
    /// * `sample_count` - `VkSampleCountFlags` to define the image's sample count
    pub fn empty(
        width: u32,
        height: u32,
        usage: impl Into<VkImageUsageFlagBits>,
        sample_count: VkSampleCountFlags,
    ) -> ImageBuilder {
        let mut create_info = ImageCreateInfo::default(ImageSourceType::Empty);
        create_info.vk_image_create_info.samples = sample_count.into();
        create_info.vk_image_create_info.extent.width = width;
        create_info.vk_image_create_info.extent.height = height;
        create_info.vk_image_create_info.extent.depth = 1;
        create_info.vk_image_create_info.usage = usage.into();

        ImageBuilder::new(ImageBuilderInternalType::NewImage(create_info))
    }

    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }

    pub fn queue(&self) -> &Arc<Mutex<Queue>> {
        &self.queue
    }

    pub fn convert_layout(&self, target_layout: VkImageLayout) -> VerboseResult<()> {
        into_layout(self, target_layout)
    }

    pub fn vk_format(&self) -> VkFormat {
        self.format
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn layers(&self) -> u32 {
        self.layers
    }

    pub fn levels(&self) -> u32 {
        self.levels
    }

    pub fn sample_count(&self) -> VkSampleCountFlagBits {
        self.sample_count
    }

    pub fn image_layout(&self) -> VerboseResult<VkImageLayout> {
        Ok(self.image_layout.lock()?.clone())
    }

    pub fn set_image_layout(&self, layout: VkImageLayout) -> VerboseResult<()> {
        let mut image_layout = self.image_layout.lock()?;

        *image_layout = layout;

        Ok(())
    }

    pub fn full_resource_range(&self) -> VkImageSubresourceRange {
        VkImageSubresourceRange {
            aspectMask: self.aspect_mask,
            baseMipLevel: 0,
            levelCount: self.levels,
            baseArrayLayer: 0,
            layerCount: self.layers,
        }
    }

    pub fn full_resource_layers(&self) -> VkImageSubresourceLayers {
        VkImageSubresourceLayers {
            aspectMask: self.aspect_mask,
            mipLevel: 0,
            baseArrayLayer: 0,
            layerCount: self.layers,
        }
    }

    pub fn src_layout_to_access(image_layout: VkImageLayout) -> VkAccessFlagBits {
        match image_layout {
            VK_IMAGE_LAYOUT_UNDEFINED => 0u32.into(),
            VK_IMAGE_LAYOUT_PREINITIALIZED => VK_ACCESS_HOST_WRITE_BIT.into(),
            VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL => VK_ACCESS_TRANSFER_WRITE_BIT.into(),
            VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL => VK_ACCESS_TRANSFER_READ_BIT.into(),
            VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL => VK_ACCESS_SHADER_READ_BIT.into(),
            VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL => VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT.into(),
            VK_IMAGE_LAYOUT_PRESENT_SRC_KHR => VK_ACCESS_MEMORY_READ_BIT.into(),
            VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL => {
                VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT
                    | VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT
            }
            VK_IMAGE_LAYOUT_GENERAL => VK_ACCESS_MEMORY_READ_BIT | VK_ACCESS_MEMORY_WRITE_BIT,
            _ => unimplemented!("source image layout ({:?})", image_layout),
        }
    }

    pub fn dst_layout_to_access(image_layout: VkImageLayout) -> VkAccessFlagBits {
        match image_layout {
            VK_IMAGE_LAYOUT_UNDEFINED => {
                panic!("target image layout must not be VK_IMAGE_LAYOUT_UNDEFINED")
            }
            VK_IMAGE_LAYOUT_PREINITIALIZED => {
                panic!("target image layout must not be VK_IMAGE_LAYOUT_PREINITIALIZED")
            }
            VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL => VK_ACCESS_TRANSFER_WRITE_BIT.into(),
            VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL => VK_ACCESS_TRANSFER_READ_BIT.into(),
            VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL => VK_ACCESS_SHADER_READ_BIT.into(),
            VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL => VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT.into(),
            VK_IMAGE_LAYOUT_PRESENT_SRC_KHR => VK_ACCESS_MEMORY_READ_BIT.into(),
            VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL => {
                VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT
                    | VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT
            }
            VK_IMAGE_LAYOUT_GENERAL => VK_ACCESS_MEMORY_READ_BIT | VK_ACCESS_MEMORY_WRITE_BIT,
            VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL => VK_ACCESS_SHADER_READ_BIT.into(),
        }
    }
}

impl VulkanDevice for Image {
    fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

impl_vk_handle!(Image, VkImage, image);
impl_vk_handle!(Image, VkSampler, sampler);
impl_vk_handle!(Image, VkImageView, image_view);

impl Drop for Image {
    fn drop(&mut self) {
        if self.sampler != VkSampler::NULL_HANDLE {
            self.device.destroy_sampler(self.sampler);
        }

        self.device.destroy_image_view(self.image_view);

        if !self.attached {
            self.device.destroy_image(self.image);
        }
    }
}

fn into_layout(image: &Image, layout: VkImageLayout) -> VerboseResult<()> {
    let queue_lock = image.queue.lock()?;

    // create a new command pool
    let command_pool = CommandPool::builder()
        .set_queue_family_index(queue_lock.family_index())
        .build(image.device.clone())?;

    // create a new command buffer
    let command_buffer = CommandPool::allocate_primary_buffer(&command_pool)?;

    // begin recording into this command buffer
    command_buffer.begin(VkCommandBufferBeginInfo::new(
        VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
    ))?;

    // subresource information
    let subresource_range = VkImageSubresourceRange {
        aspectMask: image.aspect_mask,
        baseMipLevel: 0,
        levelCount: image.levels(),
        baseArrayLayer: 0,
        layerCount: image.layers(),
    };

    // change image layout
    command_buffer.set_image_layout(image, layout, subresource_range)?;

    // end command buffer recording
    command_buffer.end()?;

    // submit current queue
    let submit = SubmitInfo::default().add_command_buffer(&command_buffer);
    let fence = Fence::builder().build(image.device.clone())?;

    queue_lock.submit(Some(&fence), &[submit])?;

    image
        .device
        .wait_for_fences(&[&fence], true, 1_000_000_000)?;

    Ok(())
}

fn copy_buffer_to_image<T>(
    device: &Arc<Device>,
    queue: &Arc<Mutex<Queue>>,
    buffer: &Arc<Buffer<T>>,
    image: &Arc<Image>,
) -> VerboseResult<()>
where
    T: Copy,
{
    let queue_lock = queue.lock()?;

    // create a new command pool
    let command_pool = CommandPool::builder()
        .set_queue_family_index(queue_lock.family_index())
        .build(device.clone())?;

    // create a new command buffer
    let command_buffer = CommandPool::allocate_primary_buffer(&command_pool)?;

    // begin recording into this command buffer
    command_buffer.begin(VkCommandBufferBeginInfo::new(
        VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
    ))?;

    // copy info for copying the content of the buffer into the image
    let buffer_image_copy = VkBufferImageCopy {
        bufferOffset: 0,
        bufferRowLength: 0,
        bufferImageHeight: 0,
        imageSubresource: VkImageSubresourceLayers {
            aspectMask: VK_IMAGE_ASPECT_COLOR_BIT.into(),
            mipLevel: 0,
            baseArrayLayer: 0,
            layerCount: 1,
        },
        imageOffset: VkOffset3D { x: 0, y: 0, z: 0 },
        imageExtent: VkExtent3D {
            width: image.width(),
            height: image.height(),
            depth: 1,
        },
    };

    // subresource information
    let mut subresource_range = VkImageSubresourceRange {
        aspectMask: VK_IMAGE_ASPECT_COLOR_BIT.into(),
        baseMipLevel: 0,
        levelCount: image.levels(),
        baseArrayLayer: 0,
        layerCount: 1,
    };

    // set image layout to receive content
    command_buffer.set_image_layout(
        image,
        VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
        subresource_range.clone(),
    )?;

    // the actual copy command
    command_buffer.copy_buffer_to_image(
        buffer,
        image,
        VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
        &[buffer_image_copy],
    );

    // just transition one mip level at a time
    subresource_range.levelCount = 1;

    // mip map creation
    if image.levels() > 1 {
        blit_mip_maps(
            &command_buffer,
            image,
            VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
        )?;
    } else {
        // set image to be usable inside a shader
        command_buffer.set_image_layout(
            image,
            VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
            subresource_range,
        )?;
    }

    // end command buffer recording
    command_buffer.end()?;

    // submit current queue
    let submit = SubmitInfo::default().add_command_buffer(&command_buffer);
    let fence = Fence::builder().build(device.clone())?;

    queue_lock.submit(Some(&fence), &[submit])?;

    device.wait_for_fences(&[&fence], true, 1_000_000_000)?;

    Ok(())
}

fn copy_images_to_imagearray(
    device: &Arc<Device>,
    queue: &Arc<Mutex<Queue>>,
    image_array: &Arc<Image>,
    images: &[Arc<Image>],
) -> VerboseResult<()> {
    let queue_lock = queue.lock()?;

    // create a new command pool
    let command_pool = CommandPool::builder()
        .set_queue_family_index(queue_lock.family_index())
        .build(device.clone())?;

    // create a new command buffer
    let command_buffer = CommandPool::allocate_primary_buffer(&command_pool)?;

    // set command buffer buffer in recording state
    command_buffer.begin(VkCommandBufferBeginInfo::new(
        VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
    ))?;

    // subresource range of the receiving image
    let array_subresource_range = VkImageSubresourceRange {
        aspectMask: VK_IMAGE_ASPECT_COLOR_BIT.into(),
        baseMipLevel: 0,
        levelCount: image_array.levels(),
        baseArrayLayer: 0,
        layerCount: image_array.layers(),
    };

    // set the target image into receiving state
    command_buffer.set_image_layout(
        image_array,
        VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
        array_subresource_range.clone(),
    )?;

    for (i, image) in images.iter().enumerate() {
        // if source and target image have the same count of
        // mip maps or the source has more mip maps,
        // we can just copy every mip level into the
        // correct target level and layer
        if image.levels() >= image_array.levels() {
            for k in 0..image_array.levels() {
                copy_image_to_image(&command_buffer, image, image_array, k, i as u32)?;
            }
        // if the source image has less mip maps than the target image,
        // we just gonna copy the first level and blit the rest
        } else {
            copy_image_to_image(&command_buffer, image, image_array, 0, i as u32)?;
            blit_mip_maps(
                &command_buffer,
                image_array,
                VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
            )?;
        }
    }

    // set the target image into a shader usable state
    command_buffer.set_image_layout(
        image_array,
        VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
        array_subresource_range,
    )?;

    // end command buffer recording
    command_buffer.end()?;

    // submit current queue
    let submit = SubmitInfo::default().add_command_buffer(&command_buffer);
    let fence = Fence::builder().build(device.clone())?;

    queue_lock.submit(Some(&fence), &[submit])?;

    device.wait_for_fences(&[&fence], true, 1_000_000_000)?;

    Ok(())
}

fn copy_image_to_image(
    command_buffer: &Arc<CommandBuffer>,
    src_image: &Arc<Image>,
    dst_image: &Arc<Image>,
    mip_level: u32,
    dst_layer: u32,
) -> VerboseResult<()> {
    // copy information to get every source into the right target slot
    let image_copy = VkImageCopy {
        srcSubresource: VkImageSubresourceLayers {
            aspectMask: VK_IMAGE_ASPECT_COLOR_BIT.into(),
            mipLevel: mip_level,
            baseArrayLayer: 0,
            layerCount: 1,
        },
        srcOffset: VkOffset3D { x: 0, y: 0, z: 0 },
        dstSubresource: VkImageSubresourceLayers {
            aspectMask: VK_IMAGE_ASPECT_COLOR_BIT.into(),
            mipLevel: mip_level,
            baseArrayLayer: dst_layer,
            layerCount: 1,
        },
        dstOffset: VkOffset3D { x: 0, y: 0, z: 0 },
        extent: VkExtent3D {
            width: src_image.width(),
            height: src_image.height(),
            depth: 1,
        },
    };

    let subresource_range = VkImageSubresourceRange {
        aspectMask: VK_IMAGE_ASPECT_COLOR_BIT.into(),
        baseMipLevel: mip_level,
        levelCount: 1,
        baseArrayLayer: 0,
        layerCount: 1,
    };

    // set the source image into sending state
    command_buffer.set_image_layout(
        src_image,
        VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
        subresource_range.clone(),
    )?;

    // copy the source data into the target slot
    command_buffer.copy_image(
        src_image,
        dst_image,
        VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
        VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
        &[image_copy],
    );

    // set the source image back to a usable state for shaders
    command_buffer.set_image_layout(
        src_image,
        VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
        subresource_range,
    )?;

    Ok(())
}

fn blit_mip_maps(
    command_buffer: &Arc<CommandBuffer>,
    image: &Arc<Image>,
    target_image_layout: VkImageLayout,
) -> VerboseResult<()> {
    let mut mip_width = image.width();
    let mut mip_height = image.height();

    // subresource information
    let mut subresource_range = VkImageSubresourceRange {
        aspectMask: VK_IMAGE_ASPECT_COLOR_BIT.into(),
        baseMipLevel: 0,
        levelCount: 1,
        baseArrayLayer: 0,
        layerCount: 1,
    };

    for i in 1..image.levels() {
        let source_mip_level = i - 1;
        let target_mip_level = i;

        // transition the previous mip level from destination to source
        subresource_range.baseMipLevel = source_mip_level;
        command_buffer.set_image_layout(
            image,
            VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
            subresource_range.clone(),
        )?;

        // create the blit information to blit the data from one mip level to another
        let image_blit = VkImageBlit {
            srcSubresource: VkImageSubresourceLayers {
                aspectMask: VK_IMAGE_ASPECT_COLOR_BIT.into(),
                mipLevel: source_mip_level,
                baseArrayLayer: 0,
                layerCount: 1,
            },
            srcOffsets: [
                VkOffset3D { x: 0, y: 0, z: 0 },
                VkOffset3D {
                    x: mip_width as i32,
                    y: mip_height as i32,
                    z: 1,
                },
            ],
            dstSubresource: VkImageSubresourceLayers {
                aspectMask: VK_IMAGE_ASPECT_COLOR_BIT.into(),
                mipLevel: target_mip_level,
                baseArrayLayer: 0,
                layerCount: 1,
            },
            dstOffsets: [
                VkOffset3D { x: 0, y: 0, z: 0 },
                VkOffset3D {
                    x: if mip_width > 1 {
                        mip_width as i32 / 2
                    } else {
                        1
                    },
                    y: if mip_height > 1 {
                        mip_height as i32 / 2
                    } else {
                        1
                    },
                    z: 1,
                },
            ],
        };

        // execute the actual blit
        command_buffer.blit_image(
            image,
            image,
            VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
            VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
            &[image_blit],
            VK_FILTER_LINEAR,
        );

        // set mip level i - 1 to target layout
        command_buffer.set_image_layout(image, target_image_layout, subresource_range.clone())?;

        mip_width = if mip_width > 1 { mip_width / 2 } else { 1 };
        mip_height = if mip_height > 1 { mip_height / 2 } else { 1 };
    }

    // set last level to be target layout
    subresource_range.baseMipLevel = image.levels() - 1;
    command_buffer.set_image_layout(image, target_image_layout, subresource_range)?;

    Ok(())
}
