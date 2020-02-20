use crate::input::guidirection::GuiDirection;

use super::controlleraxis::ControllerAxis;

use sdl2;
use utilities::prelude::*;

pub struct Controller {
    _sdl2_controller: sdl2::controller::GameController,
    deadzone: f32,
    name: String,
    id: u32,

    controller_axis: ControllerAxis,

    last_direction: GuiDirection,
}

impl Controller {
    pub fn new(
        controller_subsystem: &sdl2::GameControllerSubsystem,
        id: u32,
        deadzone: f32,
    ) -> VerboseResult<Controller> {
        if controller_subsystem.is_game_controller(id) {
            let test = controller_subsystem.open(id);

            let sdl2_controller = match test {
                Ok(controller) => controller,
                Err(error) => create_error!(error.to_string()),
            };

            let controller_axis = ControllerAxis::default();

            Ok(Controller {
                name: sdl2_controller.name(),
                id,
                deadzone,

                _sdl2_controller: sdl2_controller,

                controller_axis,

                last_direction: GuiDirection::None,
            })
        } else {
            create_error!("controller is not a game pad")
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    fn check_direction(x: f32, y: f32, deadzone: f32) -> GuiDirection {
        let thresh_x = if x.abs() < deadzone { 0.0 } else { x };
        let thresh_y = if y.abs() < deadzone { 0.0 } else { y };

        if thresh_x == 0.0 && thresh_y == 0.0 {
            return GuiDirection::None;
        }

        if thresh_x < 0.0 {
            if thresh_y < 0.0 {
                if thresh_x < thresh_y {
                    GuiDirection::Down
                } else {
                    GuiDirection::Left
                }
            } else {
                if thresh_x.abs() < thresh_y {
                    GuiDirection::Up
                } else {
                    GuiDirection::Left
                }
            }
        } else {
            if thresh_y < 0.0 {
                if thresh_x < thresh_y.abs() {
                    GuiDirection::Down
                } else {
                    GuiDirection::Right
                }
            } else {
                if thresh_x < thresh_y {
                    GuiDirection::Up
                } else {
                    GuiDirection::Right
                }
            }
        }
    }

    pub fn set_left_x(&mut self, x: f32) {
        self.controller_axis.left_stick.x = x;

        let direction = Controller::check_direction(
            self.controller_axis.left_stick.x,
            self.controller_axis.left_stick.y,
            self.deadzone,
        );

        if direction != self.last_direction {
            self.last_direction = direction;
        }
    }

    pub fn set_left_y(&mut self, y: f32) {
        self.controller_axis.left_stick.y = y;

        let direction = Controller::check_direction(
            self.controller_axis.left_stick.x,
            self.controller_axis.left_stick.y,
            self.deadzone,
        );

        if direction != self.last_direction {
            self.last_direction = direction;
        }
    }

    pub fn set_right_x(&mut self, x: f32) {
        self.controller_axis.right_stick.x = x;
    }

    pub fn set_right_y(&mut self, y: f32) {
        self.controller_axis.right_stick.y = y;
    }

    pub fn set_left_trigger(&mut self, trigger: f32) {
        self.controller_axis.left_trigger = trigger;
    }

    pub fn set_right_trigger(&mut self, trigger: f32) {
        self.controller_axis.right_trigger = trigger;
    }

    pub fn controller_axis(&self) -> ControllerAxis {
        self.controller_axis
    }

    pub fn direction(&self) -> GuiDirection {
        self.last_direction
    }
}

impl std::fmt::Debug for Controller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Point {{ name: {}, deadzone: {}, id: {} }}",
            self.name, self.deadzone, self.id
        )
    }
}

unsafe impl Send for Controller {}
unsafe impl Sync for Controller {}
