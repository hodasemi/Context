use presentation::prelude::*;

use cgmath::vec2;

pub struct AxisEmulator {
    controller_axis: ControllerAxis,

    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

impl AxisEmulator {
    pub fn key_down(&mut self, direction: GuiDirection) {
        match direction {
            GuiDirection::Left => self.left = true,
            GuiDirection::Right => self.right = true,
            GuiDirection::Up => self.up = true,
            GuiDirection::Down => self.down = true,
            _ => (),
        }
    }

    pub fn key_up(&mut self, direction: GuiDirection) {
        match direction {
            GuiDirection::Left => self.left = false,
            GuiDirection::Right => self.right = false,
            GuiDirection::Up => self.up = false,
            GuiDirection::Down => self.down = false,
            _ => (),
        }
    }

    pub fn controller_axis(&mut self) -> ControllerAxis {
        let mut x = 0.0;
        let mut y = 0.0;

        if self.left {
            x += -1.0;
        }

        if self.right {
            x += 1.0;
        }

        if self.up {
            y += 1.0;
        }

        if self.down {
            y += -1.0;
        }

        self.controller_axis.left_stick = vec2(x, y);

        self.controller_axis
    }
}

impl Default for AxisEmulator {
    fn default() -> AxisEmulator {
        AxisEmulator {
            controller_axis: ControllerAxis::default(),

            left: false,
            right: false,
            up: false,
            down: false,
        }
    }
}
