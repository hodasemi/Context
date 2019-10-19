// needed since RLS won't accept #[repr(C)]
#![allow(improper_ctypes)]

use sdl2;
use sdl2::mouse::Cursor;
use sdl2::surface::Surface as SDL_Surface;
use sdl2::sys::SDL_Window;
use sdl2::video::{FullscreenType, WindowPos};
use sdl2::IntegerOrSdlError;
use sdl2::Sdl;

use utilities::prelude::*;
use vulkan_rs::prelude::*;

use std::cell::{Cell, RefCell};
use std::error::Error;
use std::mem::MaybeUninit;
use std::path::Path;
use std::sync::Arc;

const SDL_SYSWM_WINDOWS: u32 = 0x1;
const SDL_SYSWM_X11: u32 = 0x2;
const SDL_SYSWM_COCOA: u32 = 0x4;
const SDL_SYSWM_WAYLAND: u32 = 0x6;
const SDL_SYSWM_ANDROID: u32 = 0x9;

#[repr(C)]
struct SdlSysWmInfo {
    version: sdl2::version::Version,
    subsystem: u32,
    info: [u64; 32],
}

extern "C" {
    fn SDL_GetWindowWMInfo(window: *const sdl2::sys::SDL_Window, info: *mut SdlSysWmInfo) -> bool;
}

#[derive(Default, Debug)]
pub struct WindowCreateInfo {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub requested_display: Option<String>,
}

pub struct Display {
    pub name: String,

    // bounds
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,

    pub dpi: [f32; 3],
}

#[derive(Default)]
struct CellRect {
    x: Cell<i32>,
    y: Cell<i32>,
    width: Cell<u32>,
    height: Cell<u32>,
}

impl CellRect {
    fn update_from_window(&self, window: &sdl2::video::Window) {
        let (w, h) = window.size();
        let (x, y) = window.position();

        self.x.set(x);
        self.y.set(y);
        self.width.set(w);
        self.height.set(h);
    }

    fn update_to_window(&self, window: &mut sdl2::video::Window) -> VerboseResult<()> {
        set_window_size(window, self.width.get(), self.height.get())?;
        window.set_position(
            WindowPos::Positioned(self.x.get()),
            WindowPos::Positioned(self.y.get()),
        );

        Ok(())
    }
}

pub struct WindowSystemIntegration {
    // sdl
    _video_subsystem: sdl2::VideoSubsystem,
    window: RefCell<sdl2::video::Window>,

    cursor: RefCell<Option<Cursor>>,

    displays: Vec<Display>,
    enabled_display_index: usize,

    pre_fullscreen_rect: CellRect,

    surface: RefCell<Option<Arc<Surface>>>,
}

impl WindowSystemIntegration {
    pub(crate) fn new(
        create_info: &WindowCreateInfo,
        context: &Sdl,
    ) -> VerboseResult<WindowSystemIntegration> {
        // create video subsystem
        let video_subsystem = context.video()?;

        // query display count
        let display_count = match video_subsystem.num_video_displays() {
            Ok(num_displays) => num_displays,
            Err(_) => 0,
        };

        if display_count == 0 {
            create_error!("failed detecting displays");
        }

        // query display information
        let mut displays = Vec::with_capacity(display_count as usize);

        for i in 0..display_count {
            let rect = video_subsystem.display_bounds(i)?;
            let name = video_subsystem.display_name(i)?;

            let (dpi0, dpi1, dpi2) = match video_subsystem.display_dpi(i) {
                Ok(dpis) => dpis,
                Err(msg) => {
                    println!("failed getting dpi for display {} ({}): {}", i, name, msg);
                    (0.0, 0.0, 0.0)
                }
            };

            let display = Display {
                name,

                x: rect.x(),
                y: rect.y(),
                w: rect.width(),
                h: rect.height(),

                dpi: [dpi0, dpi1, dpi2],
            };

            displays.push(display);
        }

        // check if there is an preferred display
        let mut display_index = 0;

        match &create_info.requested_display {
            Some(requested_display) => match displays
                .iter()
                .position(|display| display.name == *requested_display)
            {
                Some(index) => display_index = index,
                None => {
                    println!("could not find display: {}", requested_display);
                    println!("defaulting to display 0 ({})", displays[0].name);
                }
            },
            None => println!(
                "no display requested, defaulting to display 0 ({})",
                displays[0].name
            ),
        }

        // create window
        let mut window = if create_info.fullscreen {
            let display = &displays[display_index];

            let window = match video_subsystem
                .window(&create_info.title, display.w, display.h)
                .fullscreen()
                .resizable()
                .vulkan()
                .build()
            {
                Ok(window) => window,
                Err(build_error) => create_error!(build_error.description().to_string()),
            };

            window
        } else {
            let window = match video_subsystem
                .window(&create_info.title, create_info.width, create_info.height)
                .position_centered()
                .resizable()
                .vulkan()
                .build()
            {
                Ok(window) => window,
                Err(build_error) => create_error!(build_error.description().to_string()),
            };

            window
        };

        // force window borders
        window.set_bordered(true);

        let rect = CellRect::default();
        rect.update_from_window(&window);

        Ok(WindowSystemIntegration {
            _video_subsystem: video_subsystem,
            window: RefCell::new(window),

            cursor: RefCell::new(None),

            displays,

            enabled_display_index: display_index,

            pre_fullscreen_rect: rect,

            surface: RefCell::new(None),
        })
    }

    pub fn is_fullscreen(&self) -> VerboseResult<bool> {
        Ok(match self.window.try_borrow()?.fullscreen_state() {
            FullscreenType::Desktop => false,
            FullscreenType::True => true,
            FullscreenType::Off => false,
        })
    }

    pub fn set_fullscreen(&self, fullscreen: bool) -> VerboseResult<()> {
        let mut window = self.window.try_borrow_mut()?;

        if fullscreen {
            // store window information
            self.pre_fullscreen_rect.update_from_window(&window);

            // set fullscreen size to fit display
            let display = &self.displays[self.enabled_display_index];
            set_window_size(&mut window, display.w, display.h)?;

            // change fullscreen mode
            window.set_fullscreen(FullscreenType::True)?;
        } else {
            // change fullscreen mode
            window.set_fullscreen(FullscreenType::Off)?;

            // update window values
            self.pre_fullscreen_rect.update_to_window(&mut window)?;
        }

        Ok(())
    }

    pub fn set_opacity(&self, opacity: f32) -> VerboseResult<()> {
        self.window.try_borrow_mut()?.set_opacity(opacity)?;

        Ok(())
    }

    pub fn set_icon<T: AsRef<Path>>(&self, bmp: T) -> VerboseResult<()> {
        let surface = SDL_Surface::load_bmp(bmp)
            .map_err(|err| format!("failed to load icon image: {}", err))?;

        self.window.try_borrow_mut()?.set_icon(surface);

        Ok(())
    }

    pub fn set_cursor<T: AsRef<Path>>(&self, bmp: T) -> VerboseResult<()> {
        let surface = SDL_Surface::load_bmp(bmp)
            .map_err(|err| format!("failed to load cursor image: {}", err))?;

        let cursor = Cursor::from_surface(surface, 0, 0)
            .map_err(|err| format!("failed to load cursor: {}", err))?;

        cursor.set();

        *self.cursor.try_borrow_mut()? = Some(cursor);

        Ok(())
    }

    pub fn sdl2_window(&self) -> *mut SDL_Window {
        self.window.borrow().raw()
    }

    pub fn displays(&self) -> &[Display] {
        &self.displays
    }

    pub fn create_vulkan_surface(&self, instance: &Arc<Instance>) -> VerboseResult<()> {
        let vk_surface = self
            .window
            .try_borrow()?
            .vulkan_create_surface(instance.vk_instance().raw())?
            .into();

        *self.surface.try_borrow_mut()? = Some(Surface::from_vk_surface(vk_surface, instance));

        Ok(())
    }

    pub fn surface(&self) -> VerboseResult<Arc<Surface>> {
        Ok(self.surface.try_borrow()?.as_ref().unwrap().clone())
    }

    pub(crate) fn activate_vulkan_instance_extensions(
        &self,
        extensions: &mut InstanceExtensions,
    ) -> VerboseResult<()> {
        let sys_wm_info: SdlSysWmInfo = unsafe {
            let tmp = MaybeUninit::zeroed();
            let mut ret: SdlSysWmInfo = tmp.assume_init();
            ret.version = sdl2::version::version();

            SDL_GetWindowWMInfo(self.sdl2_window(), &mut ret);

            ret
        };

        match sys_wm_info.subsystem {
            SDL_SYSWM_ANDROID => extensions.android_surface = true,
            SDL_SYSWM_COCOA => extensions.macos_surface = true,
            SDL_SYSWM_WAYLAND => extensions.wayland_surface = true,
            SDL_SYSWM_WINDOWS => extensions.win32_surface = true,
            SDL_SYSWM_X11 => extensions.xlib_surface = true,
            _ => create_error!(format!(
                "unsupported subsystem flag: {}",
                sys_wm_info.subsystem
            )),
        }

        extensions.surface = true;

        Ok(())
    }

    pub(crate) fn activate_vulkan_device_extensions(
        &self,
        extensions: &mut DeviceExtensions,
    ) -> VerboseResult<()> {
        extensions.swapchain = true;

        Ok(())
    }
}

impl std::fmt::Debug for WindowSystemIntegration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WindowSystemIntegration {{ }}")
    }
}

/// helper function to wrap SDL2 error types
#[inline]
fn set_window_size(window: &mut sdl2::video::Window, width: u32, height: u32) -> VerboseResult<()> {
    if let Err(err) = window.set_size(width, height) {
        match err {
            IntegerOrSdlError::IntegerOverflows(msg, value) => {
                create_error!(format!("failed setting window size: {} ({})", msg, value))
            }
            IntegerOrSdlError::SdlError(msg) => {
                create_error!(format!("failed setting window size: {}", msg))
            }
        }
    }

    Ok(())
}
