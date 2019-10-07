use std::convert::TryFrom;

use cgmath::{vec3, Vector3};
use utilities::prelude::*;

/// `TextColor` describes the color of the text
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Color {
    White,
    Black,
    Red,
    Blue,
    Green,
    Orange,
    Yellow,
    Custom(u8, u8, u8),
}

impl Into<Vector3<f32>> for Color {
    fn into(self) -> Vector3<f32> {
        match self {
            Color::White => vec3(1.0, 1.0, 1.0),
            Color::Black => vec3(0.0, 0.0, 0.0),
            Color::Red => vec3(1.0, 0.0, 0.0),
            Color::Blue => vec3(0.0, 0.0, 1.0),
            Color::Green => vec3(0.0, 1.0, 0.0),
            Color::Orange => vec3(1.0, 0.65, 0.0),
            Color::Yellow => vec3(1.0, 1.0, 0.0),
            Color::Custom(r, g, b) => vec3(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0),
        }
    }
}

impl TryFrom<&str> for Color {
    type Error = UtilError;

    fn try_from(text: &str) -> VerboseResult<Color> {
        // check if color is a hex value
        if text.starts_with("#") {
            let without_prefix = text.trim_start_matches("#");

            if without_prefix.len() != 6 {
                create_error!("wrong hex color format");
            }

            let r = match u8::from_str_radix(&without_prefix[0..1], 16) {
                Ok(r) => r,
                Err(err) => create_error!(format!("failed parsing red part of {} ({})", text, err)),
            };

            let g = match u8::from_str_radix(&without_prefix[2..3], 16) {
                Ok(g) => g,
                Err(err) => {
                    create_error!(format!("failed parsing green part of {} ({})", text, err))
                }
            };

            let b = match u8::from_str_radix(&without_prefix[4..5], 16) {
                Ok(b) => b,
                Err(err) => {
                    create_error!(format!("failed parsing blue part of {} ({})", text, err))
                }
            };

            return Ok(Color::Custom(r, g, b));
        }

        match text {
            "white" => Ok(Color::White),
            "black" => Ok(Color::Black),
            "red" => Ok(Color::Red),
            "blue" => Ok(Color::Blue),
            "green" => Ok(Color::Green),
            "orange" => Ok(Color::Orange),
            "yellow" => Ok(Color::Yellow),
            _ => create_error!(format!("value is not a valid text color {}", text)),
        }
    }
}
