#[repr(C)]
#[derive(Debug, Clone)]
pub struct VkSpecializationMapEntry {
    pub constantID: u32,
    pub offset: u32,
    pub size: usize,
}
