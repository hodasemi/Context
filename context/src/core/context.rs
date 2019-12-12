#![allow(unused)]

use super::configs::WindowConfig;
use super::osspecific::osspecific::OsSpecific;
use super::vulkancore::VulkanCore;

#[cfg(feature = "audio")]
use super::soundhandler::SoundHandler;

use crate::prelude::*;

use presentation::{input::eventsystem::PresentationEventType, prelude::*};

use std::cell::{Cell, Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::env::set_var;
use std::path::Path;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub trait ContextObject {
    fn name(&self) -> &str;

    fn update(&self) -> VerboseResult<()>;

    fn event(&self, event: PresentationEventType) -> VerboseResult<()>;
}

pub struct Context {
    core: VulkanCore,
    pub(crate) presentation: PresentationCore,
    render_core: Box<dyn RenderCore>,

    #[cfg(feature = "audio")]
    sound_handler: RefCell<SoundHandler>,

    os_specific: OsSpecific,

    application_start_time: Instant,
    exit_call: Cell<bool>,

    // gui
    context_object: RefCell<Option<Arc<dyn ContextObject>>>,

    fallback: RefCell<Option<Box<dyn Fn(&str) -> VerboseResult<()>>>>,
}

impl Context {
    pub fn new<'a>() -> ContextBuilder {
        ContextBuilder::default()
    }

    pub fn set_context_object(
        &self,
        context_object: Option<Arc<dyn ContextObject>>,
    ) -> VerboseResult<()> {
        *self.context_object.try_borrow_mut()? = context_object;

        Ok(())
    }

    pub fn window_config<'a>(&'a self) -> VerboseResult<WindowConfig<'a>> {
        match self.presentation.backend() {
            PresentationBackend::Window(wsi) => Ok(WindowConfig::new(wsi)),
            PresentationBackend::OpenXR(_xri) => {
                create_error!("OpenXR backend has no window config")
            }
            PresentationBackend::OpenVR(_vri) => {
                create_error!("OpenVR backend has no window config")
            }
        }
    }

    #[cfg(feature = "audio")]
    pub fn sound(&self) -> VerboseResult<RefMut<'_, SoundHandler>> {
        Ok(self.sound_handler.try_borrow_mut()?)
    }

    pub fn run(&self) -> VerboseResult<()> {
        'running: loop {
            if self.exit_call.get() {
                break 'running;
            }

            match self.presentation.event_system().poll_events() {
                Ok(res) => {
                    if !res {
                        break 'running;
                    }
                }
                Err(err) => {
                    if let Some(fallback) = self.fallback.try_borrow()?.as_ref() {
                        (fallback)(&err.message())?;
                    }
                }
            }

            if let Err(err) = self.update() {
                if let Some(fallback) = &self.fallback.try_borrow()?.as_ref() {
                    (fallback)(&err.message())?;
                }
            }

            if !self.render_core.next_frame()? {
                break 'running;
            }
        }

        self.set_context_object(None)?;
        self.render_core.clear_scenes()?;
        self.render_core.clear_post_processing_routines()?;

        Ok(())
    }

    pub fn render_core(&self) -> &Box<dyn RenderCore> {
        &self.render_core
    }

    pub fn set_fallback<F>(&self, fallback: F) -> VerboseResult<()>
    where
        F: Fn(&str) -> VerboseResult<()> + 'static,
    {
        *self.fallback.try_borrow_mut()? = Some(Box::new(fallback));

        Ok(())
    }

    pub fn close(&self) {
        self.exit_call.set(true);
    }

    pub fn device(&self) -> &Arc<Device> {
        &self.core.device()
    }

    pub fn queue(&self) -> &Arc<Mutex<Queue>> {
        self.core.queue()
    }

    pub fn time(&self) -> f64 {
        self.application_start_time.elapsed().as_secs_f64()
    }

    pub fn controllers(&self) -> VerboseResult<Ref<'_, Vec<Rc<RefCell<Controller>>>>> {
        self.presentation.event_system().controllers()
    }

    pub fn active_controller(&self) -> VerboseResult<Option<Rc<RefCell<Controller>>>> {
        self.presentation.event_system().active_controller()
    }

    pub fn set_active_controller(&self, controller: &Rc<RefCell<Controller>>) -> VerboseResult<()> {
        self.presentation
            .event_system()
            .set_active_controller(controller)
    }
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Context {{ TODO }}")
    }
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    #[inline]
    fn update(&self) -> VerboseResult<()> {
        if let Some(context_object) = self.context_object.try_borrow()?.as_ref() {
            if let Err(err) = context_object.update() {
                return Err(err);
            }
        }

        Ok(())
    }
}

pub struct ContextBuilder {
    #[cfg(feature = "audio")]
    volume_info: Option<VolumeInfo>,

    #[cfg(any(feature = "openvr", feature = "openxr"))]
    vr_mode: Option<VRMode>,

    #[cfg(feature = "openxr")]
    openxr_runtime_json: Option<String>,

    // app info
    app_info: ApplicationInfo,

    // window information
    window_create_info: WindowCreateInfo,

    // os specifics
    os_specific_config: OsSpecificConfig,

    // vulkan core settings
    sample_count: VkSampleCountFlags,
    enable_raytracing: bool,
    vsync: bool,

    // vulkan debug extension selection
    vulkan_debug_info: VulkanDebugInfo,

    // input settings
    enable_mouse: bool,
    enable_keyboard: bool,
    enable_controller: bool,
    controller_deadzone: f32,
}

impl<'a> Default for ContextBuilder {
    fn default() -> Self {
        ContextBuilder {
            #[cfg(feature = "audio")]
            volume_info: None,

            #[cfg(any(feature = "openvr", feature = "openxr"))]
            vr_mode: None,

            #[cfg(feature = "openxr")]
            openxr_runtime_json: None,

            // app info
            app_info: ApplicationInfo {
                application_name: "not set".to_string(),
                application_version: 0,
                engine_name: "not set".to_string(),
                engine_version: 0,
            },

            // window information
            window_create_info: WindowCreateInfo {
                title: "Vulkan Application".to_string(),
                width: 800,
                height: 600,
                fullscreen: false,
                requested_display: None,
            },

            // os specifics
            os_specific_config: OsSpecificConfig::default(),

            // vulkan core settings
            sample_count: VK_SAMPLE_COUNT_1_BIT,
            enable_raytracing: false,
            vsync: false,

            // vulkan debug extension selection
            vulkan_debug_info: VulkanDebugInfo::default(),

            // input settings
            enable_mouse: false,
            enable_keyboard: false,
            enable_controller: false,
            controller_deadzone: 0.2,
        }
    }
}

impl ContextBuilder {
    #[cfg(feature = "audio")]
    pub fn set_volume_info(mut self, volume_info: VolumeInfo) -> Self {
        self.volume_info = Some(volume_info);

        self
    }

    #[cfg(any(feature = "openvr", feature = "openxr"))]
    pub fn set_vr_mode(mut self, vr_mode: VRMode) -> Self {
        self.vr_mode = Some(vr_mode);

        self
    }

    #[cfg(feature = "openxr")]
    pub fn set_openxr_json(mut self, openxr_json_path: &str) -> Self {
        self.openxr_runtime_json = Some(openxr_json_path.to_string());

        self
    }

    pub fn set_app_info(mut self, app_info: ApplicationInfo) -> Self {
        self.app_info = app_info;

        self
    }

    pub fn set_window_info(mut self, window_info: WindowCreateInfo) -> Self {
        self.window_create_info = window_info;

        self
    }

    pub fn set_os_specific_info(mut self, os_specific: OsSpecificConfig) -> Self {
        self.os_specific_config = os_specific;

        self
    }

    pub fn set_sample_count(mut self, sample_count: VkSampleCountFlags) -> Self {
        self.sample_count = sample_count;

        self
    }

    pub fn enable_ray_tracing(mut self) -> Self {
        self.enable_raytracing = true;

        self
    }

    pub fn enable_vsync(mut self) -> Self {
        self.vsync = true;

        self
    }

    pub fn set_vulkan_debug_info(mut self, vulkan_debug_info: VulkanDebugInfo) -> Self {
        self.vulkan_debug_info = vulkan_debug_info;

        self
    }

    pub fn enable_mouse(mut self) -> Self {
        self.enable_mouse = true;

        self
    }

    pub fn enable_keyboard(mut self) -> Self {
        self.enable_keyboard = true;

        self
    }

    pub fn enable_controller(mut self) -> Self {
        self.enable_controller = true;

        self
    }

    pub fn set_controller_deadzone(mut self, deadzone: f32) -> Self {
        self.controller_deadzone = deadzone;

        self
    }

    pub fn build(self) -> VerboseResult<Arc<Context>> {
        // use vulkan debug as indicator for debugging in generell
        if self.vulkan_debug_info.debugging {
            // set environment variable for Rust-debug-trace
            set_var("RUST_BACKTRACE", "1");
        }

        #[cfg(feature = "openxr")]
        self.use_openxr_json();

        let vr_mode = self.get_vr_mode();

        let presentation =
            PresentationCore::new(vr_mode, &self.window_create_info, self.app_info.clone())?;

        // vulkan core objects (VkInstance, VkDevice, ...)
        let core = VulkanCore::new(&presentation, &self.vulkan_debug_info, &self.app_info)?;

        let os_specific = OsSpecific::new(&self.os_specific_config);

        let (render_core, _target_mode) =
            create_render_core(&presentation, core.device(), core.queue(), self.vsync)?;

        let context = Arc::new(Context {
            core,
            presentation,
            render_core,

            #[cfg(feature = "audio")]
            sound_handler: RefCell::new(self.create_sound_handler()?),

            os_specific,

            application_start_time: Instant::now(),
            exit_call: Cell::new(false),

            context_object: RefCell::new(None),

            fallback: RefCell::new(None),
        });

        let weak_context = Arc::downgrade(&context);

        context
            .presentation
            .event_system()
            .set_callback(move |event| {
                if let Some(context) = weak_context.upgrade() {
                    // TODO: remove stupid workaround
                    let mut ctx_obj = None;

                    if let Some(context_object) = context.context_object.try_borrow()?.as_ref() {
                        ctx_obj = Some(context_object.clone());
                    }

                    if let Some(context_object) = ctx_obj {
                        context_object.event(event)?;
                    }
                }

                Ok(())
            });

        if self.enable_mouse {
            context.presentation.event_system().enable_mouse()?;
        }

        if self.enable_keyboard {
            context.presentation.event_system().enable_keyboard()?;
        }

        if self.enable_controller {
            context.presentation.event_system().enable_controller()?;
        }

        Ok(context)
    }

    #[cfg(feature = "openxr")]
    fn use_openxr_json(&self) {
        if let Some(openxr_json) = &self.openxr_runtime_json {
            // set environment variable for OpenXR
            set_var("XR_RUNTIME_JSON", openxr_json);
        }
    }

    fn get_vr_mode(&self) -> Option<VRMode> {
        #[cfg(any(feature = "openvr", feature = "openxr"))]
        // if we requested a VR mode, check if it is available
        match self.vr_mode {
            Some(vr_mode) => {
                let available_vr_modes = PresentationCore::enabled_vr_modes();

                // if requested VR mode is enabled, use it
                if available_vr_modes.contains(&vr_mode) {
                    return Some(vr_mode);
                }
                // fallback to the first available
                else if !available_vr_modes.is_empty() {
                    let mode = available_vr_modes[0];

                    println!(
                        "Requested VRMode ({:?}) is not available, using {:?} instead.",
                        vr_mode, mode
                    );

                    return Some(mode);
                }
                // use default desktop, as last resort
                else {
                    println!("No VRMode present, fallback to Window");

                    return None;
                }
            }
            None => {
                return None;
            }
        }

        #[cfg(not(any(feature = "openvr", feature = "openxr")))]
        None
    }

    #[cfg(feature = "audio")]
    fn create_sound_handler(&self) -> VerboseResult<SoundHandler> {
        match self.volume_info {
            Some(volume_info) => SoundHandler::new(volume_info),
            None => create_error!("No volume info present, consider disabling 'audio' feature"),
        }
    }
}
