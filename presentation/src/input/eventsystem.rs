use sdl2;
use sdl2::controller::Button;
use sdl2::event::{Event, EventType as SdlEventType};
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseButton as SdlMouseButton, MouseUtil};
use sdl2::EventPump;
use sdl2::GameControllerSubsystem;
use sdl2::Sdl;

use utilities::prelude::*;

use std::cell::{Cell, Ref, RefCell};
use std::rc::Rc;

use super::controller::Controller;
use super::mousebutton::MouseButton;

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
    ControllerAxis(Rc<RefCell<Controller>>),
    ControllerButtonDown(Button),
    ControllerButtonUp(Button),
    ControllerAdded(Rc<RefCell<Controller>>),
    ControllerRemoved(Rc<RefCell<Controller>>),
}

pub struct EventSystem {
    event_pump: RefCell<EventPump>,
    mouse: MouseUtil,
    controller_subsystem: GameControllerSubsystem,

    controller_axis_deadzone: Cell<f32>,

    selected_controller: RefCell<Option<Rc<RefCell<Controller>>>>,
    connected_controllers: RefCell<Vec<Rc<RefCell<Controller>>>>,

    event_callback: RefCell<Box<dyn Fn(PresentationEventType) -> VerboseResult<()>>>,
}

impl EventSystem {
    pub fn new(sdl2_context: &Sdl) -> VerboseResult<EventSystem> {
        let event_system = EventSystem {
            event_pump: RefCell::new(sdl2_context.event_pump()?),
            mouse: sdl2_context.mouse(),
            controller_subsystem: sdl2_context.game_controller()?,

            controller_axis_deadzone: Cell::new(0.25),

            selected_controller: RefCell::new(None),
            connected_controllers: RefCell::new(Vec::new()),

            event_callback: RefCell::new(Box::new(move |_| Ok(()))),
        };

        event_system.disable_mouse()?;
        event_system.disable_keyboard()?;
        event_system.disable_controller()?;

        Ok(event_system)
    }

    pub fn set_callback<F>(&self, f: F) -> VerboseResult<()>
    where
        F: Fn(PresentationEventType) -> VerboseResult<()> + 'static,
    {
        *self.event_callback.try_borrow_mut()? = Box::new(f);

        Ok(())
    }

    pub fn enable_mouse(&self) -> VerboseResult<()> {
        let mut event_pump = self.event_pump.try_borrow_mut()?;

        event_pump.enable_event(SdlEventType::MouseMotion);
        event_pump.enable_event(SdlEventType::MouseButtonDown);
        event_pump.enable_event(SdlEventType::MouseButtonUp);

        self.mouse.show_cursor(true);

        Ok(())
    }

    pub fn disable_mouse(&self) -> VerboseResult<()> {
        let mut event_pump = self.event_pump.try_borrow_mut()?;

        event_pump.disable_event(SdlEventType::MouseMotion);
        event_pump.disable_event(SdlEventType::MouseButtonDown);
        event_pump.disable_event(SdlEventType::MouseButtonUp);

        self.mouse.show_cursor(false);

        Ok(())
    }

    pub fn enable_keyboard(&self) -> VerboseResult<()> {
        let mut event_pump = self.event_pump.try_borrow_mut()?;

        event_pump.enable_event(SdlEventType::KeyUp);
        event_pump.enable_event(SdlEventType::KeyDown);

        Ok(())
    }

    pub fn disable_keyboard(&self) -> VerboseResult<()> {
        let mut event_pump = self.event_pump.try_borrow_mut()?;

        event_pump.disable_event(SdlEventType::KeyUp);

        event_pump.disable_event(SdlEventType::KeyDown);

        Ok(())
    }

    pub fn enable_controller(&self) -> VerboseResult<()> {
        let mut event_pump = self.event_pump.try_borrow_mut()?;

        event_pump.enable_event(SdlEventType::ControllerAxisMotion);
        event_pump.enable_event(SdlEventType::ControllerButtonDown);
        event_pump.enable_event(SdlEventType::ControllerButtonUp);

        Ok(())
    }

    pub fn disable_controller(&self) -> VerboseResult<()> {
        let mut event_pump = self.event_pump.try_borrow_mut()?;

        event_pump.disable_event(SdlEventType::ControllerAxisMotion);
        event_pump.disable_event(SdlEventType::ControllerButtonDown);
        event_pump.disable_event(SdlEventType::ControllerButtonUp);

        Ok(())
    }

    pub fn set_controller_axis_deadzone(&self, deadzone: f32) {
        self.controller_axis_deadzone.set(deadzone);
    }

    pub fn poll_events(&self) -> VerboseResult<bool> {
        let mut controller_axis_changed = false;
        let mut event_pump = self.event_pump.try_borrow_mut()?;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Ok(false),
                // ----------------- Mouse Events ---------------------
                Event::MouseMotion { x, y, .. } => {
                    self.event_callback.try_borrow()?(PresentationEventType::MouseMotion(
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

                    self.event_callback.try_borrow()?(PresentationEventType::MouseButtonDown(
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

                    self.event_callback.try_borrow()?(PresentationEventType::MouseButtonUp(
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
                        self.event_callback.try_borrow()?(PresentationEventType::KeyDown(keycode))?;
                    }
                }
                Event::KeyUp {
                    keycode, repeat, ..
                } => {
                    if repeat {
                        continue;
                    }

                    if let Some(keycode) = keycode {
                        self.event_callback.try_borrow()?(PresentationEventType::KeyUp(keycode))?;
                    }
                }

                // --------------- Controller Events -------------------
                Event::ControllerDeviceAdded { which, .. } => {
                    if let Ok(controller) = Controller::new(
                        &self.controller_subsystem,
                        which as u32,
                        self.controller_axis_deadzone.get(),
                    ) {
                        let mut connected_controllers =
                            self.connected_controllers.try_borrow_mut()?;

                        let mut selected_controller = self.selected_controller.try_borrow_mut()?;

                        let rc_controller = Rc::new(RefCell::new(controller));

                        connected_controllers.push(rc_controller.clone());

                        if selected_controller.is_none() {
                            *selected_controller = Some(rc_controller.clone());
                        }

                        self.event_callback.try_borrow()?(PresentationEventType::ControllerAdded(
                            rc_controller,
                        ))?;
                    }
                }
                Event::ControllerDeviceRemoved { which, .. } => {
                    let mut selected_controller = self.selected_controller.try_borrow_mut()?;

                    if selected_controller.is_some() {
                        // unwrap is save since we just tested for `is_some()`
                        if selected_controller.as_ref().unwrap().try_borrow()?.id() as i32 == which
                        {
                            *selected_controller = None;
                        }
                    }

                    let mut connected_controllers = self.connected_controllers.try_borrow_mut()?;

                    let mut remove_index = 0;

                    for (i, controller_cell) in connected_controllers.iter().enumerate() {
                        let controller = controller_cell.try_borrow()?;
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

                    self.event_callback.try_borrow()?(PresentationEventType::ControllerRemoved(
                        removed_controller,
                    ))?;
                }
                // maybe make use of `which`, for support of multiple controllers
                Event::ControllerButtonDown { button, which, .. } => {
                    // only call back if the selected controller pressed a button
                    if let Some(selected_controller) =
                        self.selected_controller.try_borrow()?.as_ref()
                    {
                        if selected_controller.try_borrow()?.id() as i32 == which {
                            self.event_callback.try_borrow()?(
                                PresentationEventType::ControllerButtonDown(button),
                            )?;
                        }
                    }
                }
                // maybe make use of `which`, for support of multiple controllers
                Event::ControllerButtonUp { button, which, .. } => {
                    // only call back if the selected controller released a button
                    if let Some(selected_controller) =
                        self.selected_controller.try_borrow()?.as_ref()
                    {
                        if selected_controller.try_borrow()?.id() as i32 == which {
                            self.event_callback.try_borrow()?(
                                PresentationEventType::ControllerButtonUp(button),
                            )?;
                        }
                    }
                }
                Event::ControllerAxisMotion {
                    axis, value, which, ..
                } => {
                    let mut selected_controller = self.selected_controller.try_borrow_mut()?;

                    if let Some(controller) = selected_controller.as_mut() {
                        let mut controller = controller.try_borrow_mut()?;

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
            if let Some(controller) = self.selected_controller.try_borrow()?.as_ref() {
                self.event_callback.try_borrow()?(PresentationEventType::ControllerAxis(
                    controller.clone(),
                ))?;
            }
        }

        Ok(true)
    }

    pub fn controllers(&self) -> VerboseResult<Ref<'_, Vec<Rc<RefCell<Controller>>>>> {
        Ok(self.connected_controllers.try_borrow()?)
    }

    pub fn active_controller(&self) -> VerboseResult<Option<Rc<RefCell<Controller>>>> {
        Ok(self.selected_controller.try_borrow()?.clone())
    }

    pub fn set_active_controller(&self, controller: &Rc<RefCell<Controller>>) -> VerboseResult<()> {
        if let Some(res) = self
            .connected_controllers
            .try_borrow()?
            .iter()
            .find(|c| Rc::ptr_eq(c, controller))
        {
            *self.selected_controller.try_borrow_mut()? = Some(res.clone());
        }

        Ok(())
    }
}
