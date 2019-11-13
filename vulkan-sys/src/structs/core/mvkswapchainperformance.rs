use std::os::raw::c_double;

#[repr(C)]
#[derive(Debug)]
pub struct VkMVKSwapchainPerformance {
    pub lastFrameInterval: c_double,
    pub averageFrameInterval: c_double,
    pub averageFramesPerSecond: c_double,
}
