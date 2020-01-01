use sdl2;
use sdl2::controller::Button;
use sdl2::event::{Event, EventType as SdlEventType};
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseButton as SdlMouseButton, MouseUtil};
use sdl2::EventPump;
use sdl2::EventSubsystem;
use sdl2::GameControllerSubsystem;
use sdl2::Sdl;

use utilities::prelude::*;

use std::ops::Deref;
use std::sync::{Arc, Mutex, RwLock, RwLockReadGuard};

use super::controller::Controller;
use super::mousebutton::MouseButton;

#[derive(Debug)]
pub enum PresentationEventType {
    // mouse events
    MouseMotion(u32, u32),
    MouseButtonDown(MouseButton),
    MouseButtonUp(MouseButton),
    MouseWheel(),

    // keyboard events
    KeyDown(Keycode),
    KeyUp(Keycode),

    // controller events
    ControllerAxis(Arc<RwLock<Controller>>),
    ControllerButtonDown(Button),
    ControllerButtonUp(Button),
    ControllerAdded(Arc<RwLock<Controller>>),
    ControllerRemoved(Arc<RwLock<Controller>>),
}

pub struct EventSystem {
    event_pump: RwLock<EventPump>,
    mouse: Mutex<MouseUtil>,
    controller_subsystem: Mutex<GameControllerSubsystem>,
    event_subsystem: Mutex<EventSubsystem>,

    controller_axis_deadzone: RwLock<f32>,

    selected_controller: RwLock<Option<Arc<RwLock<Controller>>>>,
    connected_controllers: RwLock<Vec<Arc<RwLock<Controller>>>>,

    event_callback: RwLock<Box<dyn Fn(PresentationEventType) -> VerboseResult<()> + Send + Sync>>,
}

impl EventSystem {
    pub fn new(sdl2_context: &Sdl) -> VerboseResult<EventSystem> {
        let event_system = EventSystem {
            event_pump: RwLock::new(sdl2_context.event_pump()?),
            mouse: Mutex::new(sdl2_context.mouse()),
            controller_subsystem: Mutex::new(sdl2_context.game_controller()?),
            event_subsystem: Mutex::new(sdl2_context.event()?),

            controller_axis_deadzone: RwLock::new(0.25),

            selected_controller: RwLock::new(None),
            connected_controllers: RwLock::new(Vec::new()),

            event_callback: RwLock::new(Box::new(move |_| Ok(()))),
        };

        event_system.disable_mouse()?;
        event_system.disable_keyboard()?;
        event_system.disable_controller()?;

        Ok(event_system)
    }

    pub fn set_callback<F>(&self, f: F) -> VerboseResult<()>
    where
        F: Fn(PresentationEventType) -> VerboseResult<()> + 'static + Send + Sync,
    {
        *self.event_callback.write()? = Box::new(f);

        Ok(())
    }

    pub fn enable_mouse(&self) -> VerboseResult<()> {
        let mut event_pump = self.event_pump.write()?;

        event_pump.enable_event(SdlEventType::MouseMotion);
        event_pump.enable_event(SdlEventType::MouseButtonDown);
        event_pump.enable_event(SdlEventType::MouseButtonUp);

        self.mouse.lock()?.show_cursor(true);

        Ok(())
    }

    pub fn disable_mouse(&self) -> VerboseResult<()> {
        let mut event_pump = self.event_pump.write()?;

        event_pump.disable_event(SdlEventType::MouseMotion);
        event_pump.disable_event(SdlEventType::MouseButtonDown);
        event_pump.disable_event(SdlEventType::MouseButtonUp);

        self.mouse.lock()?.show_cursor(false);

        Ok(())
    }

    pub fn enable_keyboard(&self) -> VerboseResult<()> {
        let mut event_pump = self.event_pump.write()?;

        event_pump.enable_event(SdlEventType::KeyUp);
        event_pump.enable_event(SdlEventType::KeyDown);

        Ok(())
    }

    pub fn disable_keyboard(&self) -> VerboseResult<()> {
        let mut event_pump = self.event_pump.write()?;

        event_pump.disable_event(SdlEventType::KeyUp);

        event_pump.disable_event(SdlEventType::KeyDown);

        Ok(())
    }

    pub fn enable_controller(&self) -> VerboseResult<()> {
        let mut event_pump = self.event_pump.write()?;

        event_pump.enable_event(SdlEventType::ControllerAxisMotion);
        event_pump.enable_event(SdlEventType::ControllerButtonDown);
        event_pump.enable_event(SdlEventType::ControllerButtonUp);

        Ok(())
    }

    pub fn disable_controller(&self) -> VerboseResult<()> {
        let mut event_pump = self.event_pump.write()?;

        event_pump.disable_event(SdlEventType::ControllerAxisMotion);
        event_pump.disable_event(SdlEventType::ControllerButtonDown);
        event_pump.disable_event(SdlEventType::ControllerButtonUp);

        Ok(())
    }

    pub fn set_controller_axis_deadzone(&self, deadzone: f32) -> VerboseResult<()> {
        *self.controller_axis_deadzone.write()? = deadzone;

        Ok(())
    }

    pub fn quit(&self) -> VerboseResult<()> {
        Ok(self
            .event_subsystem
            .lock()?
            .push_event(Event::Quit { timestamp: 0 })?)
    }

    pub fn poll_events(&self) -> VerboseResult<bool> {
        let mut controller_axis_changed = false;
        let mut event_pump = self.event_pump.write()?;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Ok(false),
                // ----------------- Mouse Events ---------------------
                Event::MouseMotion { x, y, .. } => {
                    self.event_callback.read()?(PresentationEventType::MouseMotion(
                        x as u32, y as u32,
                    ))?;
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    let mouse_button = match mouse_btn {
                        SdlMouseButton::Left => MouseButton::Left,
                        SdlMouseButton::Right => MouseButton::Right,
                        SdlMouseButton::Middle => MouseButton::Middle,
                        SdlMouseButton::X1 => MouseButton::Forward,
                        SdlMouseButton::X2 => MouseButton::Backward,
                        SdlMouseButton::Unknown => continue,
                    };

                    self.event_callback.read()?(PresentationEventType::MouseButtonDown(
                        mouse_button,
                    ))?;
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    let mouse_button = match mouse_btn {
                        SdlMouseButton::Left => MouseButton::Left,
                        SdlMouseButton::Right => MouseButton::Right,
                        SdlMouseButton::Middle => MouseButton::Middle,
                        SdlMouseButton::X1 => MouseButton::Forward,
                        SdlMouseButton::X2 => MouseButton::Backward,
                        SdlMouseButton::Unknown => continue,
                    };

                    self.event_callback.read()?(PresentationEventType::MouseButtonUp(
                        mouse_button,
                    ))?;
                }
                Event::MouseWheel { .. } => {
                    println!("Mouse Wheel event is currently not implemented!");
                }
                // ------------------- Key Events ---------------------
                Event::KeyDown {
                    keycode, repeat, ..
                } => {
                    if repeat {
                        continue;
                    }

                    if let Some(keycode) = keycode {
                        self.event_callback.read()?(PresentationEventType::KeyDown(keycode))?;
                    }
                }
                Event::KeyUp {
                    keycode, repeat, ..
                } => {
                    if repeat {
                        continue;
                    }

                    if let Some(keycode) = keycode {
                        self.event_callback.read()?(PresentationEventType::KeyUp(keycode))?;
                    }
                }

                // --------------- Controller Events -------------------
                Event::ControllerDeviceAdded { which, .. } => {
                    if let Ok(controller) = Controller::new(
                        self.controller_subsystem.lock()?.deref(),
                        which as u32,
                        self.controller_axis_deadzone.read()?.clone(),
                    ) {
                        let controller = {
                            let mut connected_controllers = self.connected_controllers.write()?;

                            let mut selected_controller = self.selected_controller.write()?;

                            let arc_controller = Arc::new(RwLock::new(controller));

                            connected_controllers.push(arc_controller.clone());

                            if selected_controller.is_none() {
                                *selected_controller = Some(arc_controller.clone());
                            }

                            arc_controller
                        };

                        self.event_callback.read()?(PresentationEventType::ControllerAdded(
                            controller,
                        ))?;
                    }
                }
                Event::ControllerDeviceRemoved { which, .. } => {
                    let removed_controller = {
                        let mut selected_controller = self.selected_controller.write()?;

                        if selected_controller.is_some() {
                            // unwrap is save since we just tested for `is_some()`
                            if selected_controller.as_ref().unwrap().read()?.id() as i32 == which {
                                *selected_controller = None;
                            }
                        }

                        let mut connected_controllers = self.connected_controllers.write()?;

                        let mut remove_index = 0;

                        for (i, controller_cell) in connected_controllers.iter().enumerate() {
                            let controller = controller_cell.read()?;
                            if controller.id() as i32 == which {
                                remove_index = i;
                                break;
                            }
                        }

                        let removed_controller = connected_controllers.swap_remove(remove_index);

                        // if we removed the selected controller, take the controller at the first position if possible
                        if selected_controller.is_none() && !connected_controllers.is_empty() {
                            *selected_controller = Some(connected_controllers[0].clone());
                        }

                        removed_controller
                    };

                    self.event_callback.read()?(PresentationEventType::ControllerRemoved(
                        removed_controller,
                    ))?;
                }
                // maybe make use of `which`, for support of multiple controllers
                Event::ControllerButtonDown { button, which, .. } => {
                    // only call back if the selected controller pressed a button
                    match self.selected_controller.read()?.as_ref() {
                        Some(selected_controller) => {
                            if selected_controller.read()?.id() as i32 != which {
                                continue;
                            }
                        }
                        None => continue,
                    }

                    self.event_callback.read()?(PresentationEventType::ControllerButtonDown(
                        button,
                    ))?;
                }
                // maybe make use of `which`, for support of multiple controllers
                Event::ControllerButtonUp { button, which, .. } => {
                    // only call back if the selected controller released a button
                    match self.selected_controller.read()?.as_ref() {
                        Some(selected_controller) => {
                            if selected_controller.read()?.id() as i32 != which {
                                continue;
                            }
                        }
                        None => continue,
                    }

                    self.event_callback.read()?(PresentationEventType::ControllerButtonUp(button))?;
                }
                Event::ControllerAxisMotion {
                    axis, value, which, ..
                } => {
                    let mut selected_controller = self.selected_controller.write()?;

                    if let Some(controller) = selected_controller.as_mut() {
                        let mut controller = controller.write()?;

                        // only update axis, when selected controller made the change
                        if controller.id() as i32 != which {
                            continue;
                        }

                        // 1 / 32768 = 0,000030518
                        let normalized = value as f32 * 0.000_030_518;

                        match axis {
                            sdl2::controller::Axis::LeftX => {
                                controller.set_left_x(normalized);
                            }
                            sdl2::controller::Axis::RightX => {
                                controller.set_right_x(normalized);
                            }
                            sdl2::controller::Axis::LeftY => {
                                controller.set_left_y(-normalized);
                            }
                            sdl2::controller::Axis::RightY => {
                                controller.set_right_y(normalized);
                            }
                            sdl2::controller::Axis::TriggerLeft => {
                                controller.set_left_trigger(normalized);
                            }
                            sdl2::controller::Axis::TriggerRight => {
                                controller.set_right_trigger(normalized);
                            }
                        }

                        controller_axis_changed = true;
                    }
                }
                _ => (),
            }
        }

        if controller_axis_changed {
            if let Some(controller) = self.selected_controller.read()?.as_ref() {
                self.event_callback.read()?(PresentationEventType::ControllerAxis(
                    controller.clone(),
                ))?;
            }
        }

        Ok(true)
    }

    pub fn controllers(&self) -> VerboseResult<RwLockReadGuard<'_, Vec<Arc<RwLock<Controller>>>>> {
        Ok(self.connected_controllers.read()?)
    }

    pub fn active_controller(&self) -> VerboseResult<Option<Arc<RwLock<Controller>>>> {
        Ok(self.selected_controller.read()?.clone())
    }

    pub fn set_active_controller(&self, controller: &Arc<RwLock<Controller>>) -> VerboseResult<()> {
        if let Some(res) = self
            .connected_controllers
            .read()?
            .iter()
            .find(|c| Arc::ptr_eq(c, controller))
        {
            *self.selected_controller.write()? = Some(res.clone());
        }

        Ok(())
    }
}

unsafe impl Send for EventSystem {}
unsafe impl Sync for EventSystem {}
