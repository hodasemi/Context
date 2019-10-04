#![allow(unused)]

use super::axisemulator::AxisEmulator;
use super::osspecific::osspecific::OsSpecific;
use super::time::Time;
use super::vulkancore::VulkanCore;

#[cfg(feature = "user_interface")]
use super::guihandler::{GuiHandler, GuiHandlerCreateInfo};

#[cfg(feature = "audio")]
use super::soundhandler::SoundHandler;

use crate::prelude::*;

use presentation::prelude::*;

use std::cell::{Cell, RefCell, RefMut};
use std::collections::HashMap;
use std::env::set_var;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

pub struct ContextBuilder {
    #[cfg(feature = "audio")]
    volume_info: Option<VolumeInfo>,

    #[cfg(any(feature = "openvr", feature = "openxr"))]
    vr_mode: Option<VRMode>,

    #[cfg(feature = "openxr")]
    openxr_runtime_json: Option<String>,

    #[cfg(feature = "user_interface")]
    gui_info: Option<GuiHandlerCreateInfo>,

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
    mapped_keyboard_input: HashMap<Keycode, InputMap>,
    mapped_controller_menu_input: HashMap<ControllerButton, InputMap>,
    mapped_controller_game_input: HashMap<ControllerButton, InputMap>,
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

    #[cfg(feature = "user_interface")]
    pub fn set_gui_info(mut self, gui_info: GuiHandlerCreateInfo) -> Self {
        self.gui_info = Some(gui_info);

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

    pub fn set_keyboard_input(mut self, mapped_keyboard_input: HashMap<Keycode, InputMap>) -> Self {
        self.mapped_keyboard_input = mapped_keyboard_input;

        self
    }

    pub fn set_controller_menu_input(
        mut self,
        mapped_controller_menu_input: HashMap<ControllerButton, InputMap>,
    ) -> Self {
        self.mapped_controller_menu_input = mapped_controller_menu_input;

        self
    }

    pub fn set_controller_game_input(
        mut self,
        mapped_controller_game_input: HashMap<ControllerButton, InputMap>,
    ) -> Self {
        self.mapped_controller_game_input = mapped_controller_game_input;

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

        // default keyboard navigation
        let mut direction_mapping = HashMap::new();
        direction_mapping.insert(Keycode::A, GuiDirection::Left);
        direction_mapping.insert(Keycode::D, GuiDirection::Right);
        direction_mapping.insert(Keycode::W, GuiDirection::Up);
        direction_mapping.insert(Keycode::S, GuiDirection::Down);
        direction_mapping.insert(Keycode::Left, GuiDirection::Left);
        direction_mapping.insert(Keycode::Right, GuiDirection::Right);
        direction_mapping.insert(Keycode::Up, GuiDirection::Up);
        direction_mapping.insert(Keycode::Down, GuiDirection::Down);

        let os_specific = OsSpecific::new(&self.os_specific_config);

        let (render_core, _target_mode) =
            create_render_core(&presentation, core.device(), core.queue(), self.vsync)?;

        let context = Arc::new(Context {
            #[cfg(feature = "user_interface")]
            gui_handler: self.create_gui_handler(
                _target_mode,
                &render_core,
                core.device(),
                core.queue(),
            )?,

            core,
            presentation,
            render_core,

            #[cfg(feature = "audio")]
            sound_handler: RefCell::new(self.create_sound_handler()?),

            _os_specific: os_specific,

            timer: Time::new(),
            time: Cell::new(0.0),
            exit_call: Cell::new(false),

            controller_axis_emulator: RefCell::new(AxisEmulator::default()),
            keyboard_input: self.mapped_keyboard_input,
            direction_mapping,

            _controller_menu_input: self.mapped_controller_menu_input,
            controller_game_input: self.mapped_controller_game_input,

            game_object: RefCell::new(None),

            fallback: RefCell::new(None),
        });

        if self.enable_mouse {
            context.presentation.event_system().enable_mouse(
                {
                    let _context = context.clone();

                    Box::new(move |_x, _y| {
                        #[cfg(feature = "user_interface")]
                        _context.gui_handler.set_mouse_pos(_x, _y)?;

                        Ok(())
                    })
                },
                {
                    let _context = context.clone();

                    Box::new(move |_mouse_button| {
                        #[cfg(feature = "user_interface")]
                        {
                            if _context.gui_handler.mouse_down(_mouse_button)? {
                                // some event
                            }
                        }

                        Ok(())
                    })
                },
                {
                    let _context = context.clone();

                    Box::new(move |_mouse_button| {
                        #[cfg(feature = "user_interface")]
                        {
                            if _context.gui_handler.mouse_up(_mouse_button)? {
                                // some event
                            }
                        }

                        Ok(())
                    })
                },
            )?;
        }

        if self.enable_keyboard {
            context.presentation.event_system().enable_keyboard(
                {
                    let context = context.clone();

                    Box::new(move |key| {
                        context.key_down_event(key)?;

                        Ok(())
                    })
                },
                {
                    let context = context.clone();

                    Box::new(move |key| {
                        context.key_up_event(key)?;

                        Ok(())
                    })
                },
            )?;
        }

        if self.enable_controller {
            context.presentation.event_system().enable_controller(
                {
                    let context = context.clone();

                    Box::new(move |button| {
                        context.button_down_event(button)?;

                        Ok(())
                    })
                },
                {
                    let context = context.clone();

                    Box::new(move |button| {
                        context.button_up_event(button)?;

                        Ok(())
                    })
                },
                {
                    let context = context.clone();

                    Box::new(move |controller| {
                        #[cfg(feature = "user_interface")]
                        context
                            .gui_handler
                            .update_selection(controller.direction())?;

                        #[cfg(feature = "user_interface")]
                        {
                            if !context.gui_handler.check_navigatable()? {
                                context.axis_event(controller)?
                            }
                        }

                        #[cfg(not(feature = "user_interface"))]
                        context.axis_event(controller)?;

                        Ok(())
                    })
                },
            )?;
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

    #[cfg(feature = "user_interface")]
    fn create_gui_handler(
        &self,
        target_mode: TargetMode<()>,
        render_core: &Box<dyn RenderCore>,
        device: &Arc<Device>,
        queue: &Arc<Queue>,
    ) -> VerboseResult<Arc<GuiHandler>> {
        match &self.gui_info {
            Some(gui_info) => {
                let gui_handler = Arc::new(GuiHandler::new(
                    gui_info.clone(),
                    target_mode,
                    device,
                    queue,
                    render_core,
                )?);

                {
                    let gui_handler = gui_handler.clone();

                    render_core.set_gui_callback(Some(Box::new(
                        move |eye, index, framebuffer, render_pass| {
                            gui_handler.render(eye, index, framebuffer, render_pass)
                        },
                    )))?;
                }

                {
                    let gui_handler = gui_handler.clone();

                    render_core.set_resize_callback(Some(Box::new(move |width, height| {
                        gui_handler.enqueue_resize(width, height)
                    })))?;
                }

                Ok(gui_handler)
            }
            None => {
                create_error!("No gui info present, consider disabling 'user_interface' feature")
            }
        }
    }
}

impl Default for ContextBuilder {
    fn default() -> Self {
        ContextBuilder {
            #[cfg(feature = "audio")]
            volume_info: None,

            #[cfg(any(feature = "openvr", feature = "openxr"))]
            vr_mode: None,

            #[cfg(feature = "openxr")]
            openxr_runtime_json: None,

            #[cfg(feature = "user_interface")]
            gui_info: None,

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
            mapped_keyboard_input: HashMap::new(),
            mapped_controller_menu_input: HashMap::new(),
            mapped_controller_game_input: HashMap::new(),
        }
    }
}

pub trait GameObject {
    fn name(&self) -> &str;

    fn update(&self) -> VerboseResult<()>;

    fn on_key_down(&self, input: InputMap) -> VerboseResult<()>;
    fn on_key_up(&self, input: InputMap) -> VerboseResult<()>;
    fn on_axis(&self, axis: ControllerAxis) -> VerboseResult<()>;
}

pub struct Context {
    core: VulkanCore,
    presentation: PresentationCore,
    render_core: Box<dyn RenderCore>,

    #[cfg(feature = "audio")]
    sound_handler: RefCell<SoundHandler>,

    #[cfg(feature = "user_interface")]
    gui_handler: Arc<GuiHandler>,

    _os_specific: OsSpecific,

    timer: Time,
    time: Cell<f64>,
    exit_call: Cell<bool>,

    // gui
    controller_axis_emulator: RefCell<AxisEmulator>,
    keyboard_input: HashMap<Keycode, InputMap>,
    direction_mapping: HashMap<Keycode, GuiDirection>,

    _controller_menu_input: HashMap<ControllerButton, InputMap>,
    controller_game_input: HashMap<ControllerButton, InputMap>,

    game_object: RefCell<Option<Arc<dyn GameObject>>>,

    fallback: RefCell<Option<Box<dyn Fn(&str) -> VerboseResult<()>>>>,
}

impl Context {
    pub fn new() -> ContextBuilder {
        ContextBuilder::default()
    }

    pub fn set_game_object(&self, game_object: Option<Arc<dyn GameObject>>) -> VerboseResult<()> {
        *self.game_object.try_borrow_mut()? = game_object;

        Ok(())
    }

    pub fn set_cursor<T: AsRef<Path>>(&self, bmp: T) -> VerboseResult<()> {
        match self.presentation.backend() {
            PresentationBackend::Window(wsi) => wsi.set_cursor(bmp)?,
            PresentationBackend::OpenXR(_xri) => (),
            PresentationBackend::OpenVR(_vri) => (),
        };

        Ok(())
    }

    pub fn toggle_fullscreen(&self) -> VerboseResult<()> {
        match self.presentation.backend() {
            PresentationBackend::Window(wsi) => wsi.set_fullscreen(!wsi.is_fullscreen()?)?,
            PresentationBackend::OpenXR(_xri) => (),
            PresentationBackend::OpenVR(_vri) => (),
        };

        Ok(())
    }

    #[cfg(feature = "audio")]
    pub fn sound(&self) -> VerboseResult<RefMut<'_, SoundHandler>> {
        Ok(self.sound_handler.try_borrow_mut()?)
    }

    #[cfg(feature = "user_interface")]
    pub fn gui_handler(&self) -> &Arc<GuiHandler> {
        &self.gui_handler
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

            self.time.set(self.timer.time());

            if let Err(err) = self.update() {
                if let Some(fallback) = &self.fallback.try_borrow()?.as_ref() {
                    (fallback)(&err.message())?;
                }
            }

            if !self.render_core.next_frame()? {
                break 'running;
            }
        }

        self.set_game_object(None)?;
        self.render_core.clear_scenes()?;

        Ok(())
    }

    pub fn render_core(&self) -> &Box<dyn RenderCore> {
        &self.render_core
    }

    pub fn set_fallback(
        &self,
        fallback: Box<dyn Fn(&str) -> VerboseResult<()>>,
    ) -> VerboseResult<()> {
        *self.fallback.try_borrow_mut()? = Some(fallback);

        Ok(())
    }

    pub fn close(&self) {
        self.exit_call.set(true);
    }

    pub fn device(&self) -> &Arc<Device> {
        &self.core.device()
    }

    pub fn queue(&self) -> &Arc<Queue> {
        self.core.queue()
    }

    pub fn time(&self) -> f64 {
        self.time.get()
    }

    // pub(crate) fn presentation_core(&self) -> &PresentationCore {
    //     &self.presentation
    // }

    // pub fn controllers(&self) -> VerboseResult<Ref<'_, Vec<Rc<RefCell<Controller>>>>> {
    //     let controllers = self.connected_controllers.try_borrow()?;
    //     Ok(controllers)
    // }

    pub fn active_controller(&self) -> VerboseResult<Option<Rc<RefCell<Controller>>>> {
        self.presentation.event_system().active_controller()
    }

    // pub fn set_active_controller(&self, controller_id: u32) -> VerboseResult<()> {
    //     for controller in self.connected_controllers.try_borrow()?.deref() {
    //         if controller.try_borrow()?.id() == controller_id {
    //             *self.selected_controller.try_borrow_mut()? = Some(controller.clone());
    //         }
    //     }

    //     Ok(())
    // }
}

impl Context {
    #[inline]
    fn update(&self) -> VerboseResult<()> {
        if let Some(game_object) = &self.game_object.try_borrow()?.as_ref() {
            if let Err(err) = game_object.update() {
                return Err(err);
            }
        }

        Ok(())
    }

    #[inline]
    fn key_up_event(&self, keycode: Keycode) -> VerboseResult<()> {
        if let Some(direction) = self.direction_mapping.get(&keycode) {
            if let Some(game_object) = &self.game_object.try_borrow()?.as_ref() {
                let mut controller_axis_emulator =
                    self.controller_axis_emulator.try_borrow_mut()?;

                controller_axis_emulator.key_up(*direction);

                if let Err(err) = game_object.on_axis(controller_axis_emulator.controller_axis()) {
                    return Err(err);
                }
            }

            return Ok(());
        }

        if !self.keyboard_input.contains_key(&keycode) {
            return Ok(());
        }

        if let Some(game_object) = &self.game_object.try_borrow()?.as_ref() {
            let mapped_input = &self.keyboard_input[&keycode];

            if let Err(err) = game_object.on_key_up(*mapped_input) {
                return Err(err);
            }
        }

        Ok(())
    }

    #[inline]
    fn key_down_event(&self, keycode: Keycode) -> VerboseResult<()> {
        if let Some(direction) = self.direction_mapping.get(&keycode) {
            #[cfg(feature = "user_interface")]
            {
                if self.gui_handler.update_selection(*direction)? {
                    return Ok(());
                }
            }

            if let Some(game_object) = &self.game_object.try_borrow()?.as_ref() {
                let mut controller_axis_emulator =
                    self.controller_axis_emulator.try_borrow_mut()?;

                controller_axis_emulator.key_down(*direction);

                if let Err(err) = game_object.on_axis(controller_axis_emulator.controller_axis()) {
                    return Err(err);
                }
            }

            return Ok(());
        }

        if let Some(mapped_input) = self.keyboard_input.get(&keycode) {
            #[cfg(feature = "user_interface")]
            {
                if keycode == Keycode::Backspace && self.gui_handler.remove_char()? {
                    return Ok(());
                }
            }

            if *mapped_input == InputMap::A {
                #[cfg(feature = "user_interface")]
                match self.gui_handler.accept_selection() {
                    Ok(success) => {
                        if success {
                            return Ok(());
                        }
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            } else if *mapped_input == InputMap::RightButton {
                #[cfg(feature = "user_interface")]
                match self.gui_handler.next_tab_topgui() {
                    Ok(success) => {
                        if success {
                            return Ok(());
                        }
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            } else if *mapped_input == InputMap::LeftButton {
                #[cfg(feature = "user_interface")]
                match self.gui_handler.previous_tab_topgui() {
                    Ok(success) => {
                        if success {
                            return Ok(());
                        }
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }

            if let Some(game_object) = &self.game_object.try_borrow()?.as_ref() {
                if let Err(err) = game_object.on_key_down(*mapped_input) {
                    return Err(err);
                }
            }
        }

        Ok(())
    }

    #[inline]
    fn button_up_event(&self, button: ControllerButton) -> VerboseResult<()> {
        #[cfg(feature = "user_interface")]
        {
            if self.gui_handler.check_navigatable()? {
                if let Some(mapped_input) = self._controller_menu_input.get(&button) {
                    if let Some(game_object) = &self.game_object.try_borrow()?.as_ref() {
                        if let Err(err) = game_object.on_key_up(*mapped_input) {
                            return Err(err);
                        }
                    }
                }
            } else if let Some(mapped_input) = self.controller_game_input.get(&button) {
                if let Some(game_object) = &self.game_object.try_borrow()?.as_ref() {
                    if let Err(err) = game_object.on_key_up(*mapped_input) {
                        return Err(err);
                    }
                }
            }
        }

        #[cfg(not(feature = "user_interface"))]
        {
            if let Some(mapped_input) = self.controller_game_input.get(&button) {
                if let Some(game_object) = &self.game_object.try_borrow()?.as_ref() {
                    if let Err(err) = game_object.on_key_up(*mapped_input) {
                        return Err(err);
                    }
                }
            }
        }

        Ok(())
    }

    #[inline]
    fn button_down_event(&self, button: ControllerButton) -> VerboseResult<()> {
        #[cfg(feature = "user_interface")]
        {
            if self.gui_handler.check_navigatable()? {
                if let Some(mapped_input) = self._controller_menu_input.get(&button) {
                    if *mapped_input == InputMap::A {
                        match self.gui_handler.accept_selection() {
                            Ok(success) => {
                                if success {
                                    return Ok(());
                                }
                            }
                            Err(err) => {
                                return Err(err);
                            }
                        }
                    } else if *mapped_input == InputMap::B {
                        match self.gui_handler.decline_topgui() {
                            Ok(success) => {
                                if success {
                                    return Ok(());
                                }
                            }
                            Err(err) => {
                                return Err(err);
                            }
                        }
                    } else if *mapped_input == InputMap::RightButton {
                        match self.gui_handler.next_tab_topgui() {
                            Ok(success) => {
                                if success {
                                    return Ok(());
                                }
                            }
                            Err(err) => {
                                return Err(err);
                            }
                        }
                    } else if *mapped_input == InputMap::LeftButton {
                        match self.gui_handler.previous_tab_topgui() {
                            Ok(success) => {
                                if success {
                                    return Ok(());
                                }
                            }
                            Err(err) => {
                                return Err(err);
                            }
                        }
                    }

                    if let Some(game_object) = &self.game_object.try_borrow()?.as_ref() {
                        if let Err(err) = game_object.on_key_down(*mapped_input) {
                            return Err(err);
                        }
                    }
                }
            } else if let Some(mapped_input) = self.controller_game_input.get(&button) {
                if let Some(game_object) = &self.game_object.try_borrow()?.as_ref() {
                    if let Err(err) = game_object.on_key_down(*mapped_input) {
                        return Err(err);
                    }
                }
            }
        }

        #[cfg(not(feature = "user_interface"))]
        {
            if let Some(mapped_input) = self.controller_game_input.get(&button) {
                if let Some(game_object) = &self.game_object.try_borrow()?.as_ref() {
                    if let Err(err) = game_object.on_key_down(*mapped_input) {
                        return Err(err);
                    }
                }
            }
        }

        Ok(())
    }

    #[inline]
    fn axis_event(&self, controller: &Controller) -> VerboseResult<()> {
        if let Some(game_object) = &self.game_object.try_borrow()?.as_ref() {
            if let Err(err) = game_object.on_axis(controller.controller_axis()) {
                return Err(err);
            }
        }

        Ok(())
    }
}

impl std::fmt::Debug for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Context {{ TODO }}")
    }
}
