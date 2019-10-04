use crate::prelude::*;
use crate::SetupUSizeConv;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkQueue(usize);
SetupUSizeConv!(VkQueue);
