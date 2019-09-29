use sdl2;
use sdl2::controller::Button;
use sdl2::event::{Event, EventType};
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

pub struct EventSystem {
    event_pump: RefCell<EventPump>,
    mouse: MouseUtil,
    controller_subsystem: GameControllerSubsystem,

    controller_axis_deadzone: Cell<f32>,

    selected_controller: RefCell<Option<Rc<RefCell<Controller>>>>,
    connected_controllers: RefCell<Vec<Rc<RefCell<Controller>>>>,

    mouse_motion_event: RefCell<Option<Box<dyn Fn(u32, u32) -> VerboseResult<()>>>>,
    mouse_down_event: RefCell<Option<Box<dyn Fn(MouseButton) -> VerboseResult<()>>>>,
    mouse_up_event: RefCell<Option<Box<dyn Fn(MouseButton) -> VerboseResult<()>>>>,

    key_down_event: RefCell<Option<Box<dyn Fn(Keycode) -> VerboseResult<()>>>>,
    key_up_event: RefCell<Option<Box<dyn Fn(Keycode) -> VerboseResult<()>>>>,

    button_down_event: RefCell<Option<Box<dyn Fn(Button) -> VerboseResult<()>>>>,
    button_up_event: RefCell<Option<Box<dyn Fn(Button) -> VerboseResult<()>>>>,
    axis_changed_event: RefCell<Option<Box<dyn Fn(&Controller) -> VerboseResult<()>>>>,
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

            mouse_motion_event: RefCell::new(None),
            mouse_down_event: RefCell::new(None),
            mouse_up_event: RefCell::new(None),

            key_down_event: RefCell::new(None),
            key_up_event: RefCell::new(None),

            button_down_event: RefCell::new(None),
            button_up_event: RefCell::new(None),
            axis_changed_event: RefCell::new(None),
        };

        event_system.disable_mouse()?;
        event_system.disable_keyboard()?;
        event_system.disable_controller()?;

        Ok(event_system)
    }

    pub fn enable_mouse(
        &self,
        mouse_motion_event: Box<dyn Fn(u32, u32) -> VerboseResult<()>>,
        mouse_down_event: Box<dyn Fn(MouseButton) -> VerboseResult<()>>,
        mouse_up_event: Box<dyn Fn(MouseButton) -> VerboseResult<()>>,
    ) -> VerboseResult<()> {
        let mut event_pump = self.event_pump.try_borrow_mut()?;

        *self.mouse_motion_event.try_borrow_mut()? = Some(mouse_motion_event);
        event_pump.enable_event(EventType::MouseMotion);

        *self.mouse_down_event.try_borrow_mut()? = Some(mouse_down_event);
        event_pump.enable_event(EventType::MouseButtonDown);

        *self.mouse_up_event.try_borrow_mut()? = Some(mouse_up_event);
        event_pump.enable_event(EventType::MouseButtonUp);

        self.mouse.show_cursor(true);

        Ok(())
    }

    pub fn disable_mouse(&self) -> VerboseResult<()> {
        let mut event_pump = self.event_pump.try_borrow_mut()?;

        *self.mouse_motion_event.try_borrow_mut()? = None;
        event_pump.disable_event(EventType::MouseMotion);

        *self.mouse_down_event.try_borrow_mut()? = None;
        event_pump.disable_event(EventType::MouseButtonDown);

        *self.mouse_up_event.try_borrow_mut()? = None;
        event_pump.disable_event(EventType::MouseButtonUp);

        self.mouse.show_cursor(false);

        Ok(())
    }

    pub fn enable_keyboard(
        &self,
        key_down_event: Box<dyn Fn(Keycode) -> VerboseResult<()>>,
        key_up_event: Box<dyn Fn(Keycode) -> VerboseResult<()>>,
    ) -> VerboseResult<()> {
        let mut event_pump = self.event_pump.try_borrow_mut()?;

        *self.key_up_event.try_borrow_mut()? = Some(key_up_event);
        event_pump.enable_event(EventType::KeyUp);

        *self.key_down_event.try_borrow_mut()? = Some(key_down_event);
        event_pump.enable_event(EventType::KeyDown);

        Ok(())
    }

    pub fn disable_keyboard(&self) -> VerboseResult<()> {
        let mut event_pump = self.event_pump.try_borrow_mut()?;

        *self.key_up_event.try_borrow_mut()? = None;
        event_pump.disable_event(EventType::KeyUp);

        *self.key_down_event.try_borrow_mut()? = None;
        event_pump.disable_event(EventType::KeyDown);

        Ok(())
    }

    pub fn enable_controller(
        &self,
        button_down_event: Box<dyn Fn(Button) -> VerboseResult<()>>,
        button_up_event: Box<dyn Fn(Button) -> VerboseResult<()>>,
        axis_changed_event: Box<dyn Fn(&Controller) -> VerboseResult<()>>,
    ) -> VerboseResult<()> {
        let mut event_pump = self.event_pump.try_borrow_mut()?;

        *self.axis_changed_event.try_borrow_mut()? = Some(axis_changed_event);
        event_pump.enable_event(EventType::ControllerAxisMotion);

        *self.button_down_event.try_borrow_mut()? = Some(button_down_event);
        event_pump.enable_event(EventType::ControllerButtonDown);

        *self.button_up_event.try_borrow_mut()? = Some(button_up_event);
        event_pump.enable_event(EventType::ControllerButtonUp);

        Ok(())
    }

    pub fn disable_controller(&self) -> VerboseResult<()> {
        let mut event_pump = self.event_pump.try_borrow_mut()?;

        *self.axis_changed_event.try_borrow_mut()? = None;
        event_pump.disable_event(EventType::ControllerAxisMotion);

        *self.button_down_event.try_borrow_mut()? = None;
        event_pump.disable_event(EventType::ControllerButtonDown);

        *self.button_up_event.try_borrow_mut()? = None;
        event_pump.disable_event(EventType::ControllerButtonUp);

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
                    if let Some(mouse_motion_event) = self.mouse_motion_event.try_borrow()?.as_ref()
                    {
                        mouse_motion_event(x as u32, y as u32)?;
                    }
                }
                Event::MouseButtonDown { mouse_btn, .. } => {
                    let mouse_button = match mouse_btn {
                        SdlMouseButton::Left => MouseButton::Left,
                        SdlMouseButton::Right => MouseButton::Right,
                        _ => continue,
                    };

                    if let Some(mouse_down_event) = self.mouse_down_event.try_borrow()?.as_ref() {
                        mouse_down_event(mouse_button)?;
                    }
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    let mouse_button = match mouse_btn {
                        SdlMouseButton::Left => MouseButton::Left,
                        SdlMouseButton::Right => MouseButton::Right,
                        _ => continue,
                    };

                    if let Some(mouse_up_event) = self.mouse_up_event.try_borrow()?.as_ref() {
                        mouse_up_event(mouse_button)?;
                    }
                }
                // ------------------- Key Events ---------------------
                Event::KeyDown {
                    keycode, repeat, ..
                } => {
                    if repeat {
                        continue;
                    }

                    if let Some(keycode) = keycode {
                        if let Some(key_down_event) = self.key_down_event.try_borrow()?.as_ref() {
                            key_down_event(keycode)?;
                        }
                    }
                }
                Event::KeyUp {
                    keycode, repeat, ..
                } => {
                    if repeat {
                        continue;
                    }

                    if let Some(keycode) = keycode {
                        if let Some(key_up_event) = self.key_up_event.try_borrow()?.as_ref() {
                            key_up_event(keycode)?;
                        }
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
                            *selected_controller = Some(rc_controller);
                        }
                    }
                }
                Event::ControllerDeviceRemoved { which, .. } => {
                    let mut selected_controller = self.selected_controller.try_borrow_mut()?;

                    if selected_controller.is_some() {
                        // unwrap is save since we just tested for `is_some()`
                        if selected_controller.as_ref().unwrap().try_borrow()?.id() == which as u32
                        {
                            *selected_controller = None;
                        }
                    }

                    let mut connected_controllers = self.connected_controllers.try_borrow_mut()?;

                    let mut remove_index = 0;

                    for (i, controller_cell) in connected_controllers.iter().enumerate() {
                        let controller = controller_cell.try_borrow()?;
                        if controller.id() == which as u32 {
                            remove_index = i;
                            break;
                        }
                    }

                    connected_controllers.swap_remove(remove_index);
                }
                // maybe make use of `which`, for support of multiple controllers
                Event::ControllerButtonDown { button, .. } => {
                    if let Some(button_down_event) = self.button_down_event.try_borrow()?.as_ref() {
                        button_down_event(button)?;
                    }
                }
                // maybe make use of `which`, for support of multiple controllers
                Event::ControllerButtonUp { button, .. } => {
                    if let Some(button_up_event) = self.button_up_event.try_borrow()?.as_ref() {
                        button_up_event(button)?;
                    }
                }
                Event::ControllerAxisMotion { axis, value, .. } => {
                    let mut selected_controller = self.selected_controller.try_borrow_mut()?;

                    if let Some(controller) = selected_controller.as_mut() {
                        let mut controller = controller.try_borrow_mut()?;

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
            if let Some(controller) = &self.selected_controller.try_borrow()?.as_ref() {
                let controller = controller.try_borrow()?;

                if let Some(axis_changed_event) = self.axis_changed_event.try_borrow()?.as_ref() {
                    axis_changed_event(&controller)?;
                }
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

    pub fn set_active_controller(
        &mut self,
        controller: &Rc<RefCell<Controller>>,
    ) -> VerboseResult<()> {
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
