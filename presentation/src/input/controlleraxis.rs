use cgmath::{Vector2, Zero};
use utilities::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct ControllerAxis {
    pub left_stick: Vector2<f32>,
    pub right_stick: Vector2<f32>,
    pub left_trigger: f32,
    pub right_trigger: f32,
}

impl Default for ControllerAxis {
    fn default() -> Self {
        ControllerAxis {
            left_stick: Vector2::zero(),
            right_stick: Vector2::zero(),
            left_trigger: 0.0,
            right_trigger: 0.0,
        }
    }
}
