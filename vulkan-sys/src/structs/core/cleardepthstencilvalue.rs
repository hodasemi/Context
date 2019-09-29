#[repr(C)]
#[derive(Debug)]
pub struct VkClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32,
}
