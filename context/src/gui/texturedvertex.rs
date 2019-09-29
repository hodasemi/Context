use crate::prelude::*;
use cgmath::prelude::Zero;

use std::mem;

#[derive(Copy, Clone, Debug)]
pub struct TexturedVertex {
    pub position: cgmath::Vector4<f32>,
    pub texture_coordinates: cgmath::Vector2<f32>,
}

impl TexturedVertex {
    pub fn vertex_input_state() -> (
        VkPipelineVertexInputStateCreateInfo,
        Vec<VkVertexInputBindingDescription>,
        Vec<VkVertexInputAttributeDescription>,
    ) {
        let input_bindings = vec![VkVertexInputBindingDescription {
            binding: 0,
            stride: mem::size_of::<TexturedVertex>() as u32,
            inputRate: VK_VERTEX_INPUT_RATE_VERTEX,
        }];

        let input_attributes = vec![
            VkVertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: VK_FORMAT_R32G32B32A32_SFLOAT,
                offset: 0,
            },
            VkVertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: VK_FORMAT_R32G32_SFLOAT,
                offset: 16, // mem::size_of::<cgmath::Vector4<f32>>() as u32
            },
        ];

        let input_state =
            VkPipelineVertexInputStateCreateInfo::new(0, &input_bindings, &input_attributes);

        (input_state, input_bindings, input_attributes)
    }
}

impl Default for TexturedVertex {
    fn default() -> TexturedVertex {
        TexturedVertex {
            position: cgmath::Vector4::zero(),
            texture_coordinates: cgmath::Vector2::zero(),
        }
    }
}
