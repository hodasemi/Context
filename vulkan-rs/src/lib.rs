//! `vulkan` module is a collection of abstractions for vulkan functions
#![deny(rust_2018_idioms)]

pub mod prelude;

pub mod accelerationstructure;
pub mod buffer;
pub mod commandbuffer;
pub mod commandpool;
pub mod descriptorpool;
pub mod descriptorset;
pub mod descriptorsetlayout;
pub mod device;
pub mod fence;
pub mod framebuffer;
pub mod googledisplaytiming;
pub mod image;
pub mod instance;
pub mod mappedmemory;
pub mod memory;
pub mod physicaldevice;
pub mod pipeline;
pub mod pipelinecache;
pub mod pipelinelayout;
pub mod pipelines;
pub mod querypool;
pub mod queue;
pub mod renderpass;
pub mod semaphore;
pub mod shadermodule;
pub mod surface;
pub mod swapchain;

pub mod ffi;

mod allocator;
mod loader;

pub enum OutOfDate<T> {
    Ok(T),
    OutOfDate,
}

pub trait VkHandle<T> {
    fn vk_handle(&self) -> T;
}

pub trait VulkanDevice {
    fn device(&self) -> &std::sync::Arc<device::Device>;
}

#[macro_export]
macro_rules! impl_vk_handle {
    ($struct_name:ident, $target_name:ident, $value:ident) => {
        impl VkHandle<$target_name> for $struct_name {
            fn vk_handle(&self) -> $target_name {
                self.$value
            }
        }

        impl<'a> VkHandle<$target_name> for &'a $struct_name {
            fn vk_handle(&self) -> $target_name {
                self.$value
            }
        }

        impl VkHandle<$target_name> for Arc<$struct_name> {
            fn vk_handle(&self) -> $target_name {
                self.$value
            }
        }

        impl<'a> VkHandle<$target_name> for &'a Arc<$struct_name> {
            fn vk_handle(&self) -> $target_name {
                self.$value
            }
        }
    };
}

#[macro_export]
macro_rules! impl_vk_handle_t {
    ($struct_name:ident, $target_name:ident, $value:ident) => {
        impl<T> VkHandle<$target_name> for $struct_name<T> {
            fn vk_handle(&self) -> $target_name {
                self.$value
            }
        }

        impl<'a, T> VkHandle<$target_name> for &'a $struct_name<T> {
            fn vk_handle(&self) -> $target_name {
                self.$value
            }
        }

        impl<T> VkHandle<$target_name> for Arc<$struct_name<T>> {
            fn vk_handle(&self) -> $target_name {
                self.$value
            }
        }

        impl<'a, T> VkHandle<$target_name> for &'a Arc<$struct_name<T>> {
            fn vk_handle(&self) -> $target_name {
                self.$value
            }
        }
    };
}

#[macro_export]
macro_rules! Extensions {
    ($struct_name:ident, { $(($var:ident, $name:expr),)+ }) => {
        pub struct $struct_name {
            $(
                pub $var: bool,
            )+

            raw_names: Vec<String>,
        }

        impl $struct_name {
            pub fn into_list(self) -> Vec<VkString> {
                let mut list = Vec::new();

                $(
                    if self.$var {
                        list.push(VkString::new($name));
                    }
                )+

                list
            }

            pub fn as_list(&self) -> Vec<VkString> {
                let mut list = Vec::new();

                $(
                    if self.$var {
                        list.push(VkString::new($name));
                    }
                )+

                let mut raw_vk_names = self.raw_names.iter().map(|raw_name| VkString::new(raw_name)).collect();
                list.append(&mut raw_vk_names);

                list
            }

            pub fn from_list(list: &[VkString]) -> Self {
                let mut extensions = Self::default();

                $(
                    if list.contains(&VkString::new($name)) {
                        extensions.$var = true;
                    }
                )+

                extensions
            }

            pub fn check_availability(&self, other: &$struct_name) -> Result<(), Vec<String>> {
                let mut missings = Vec::new();

                // requested extensions is not available in other
                $(
                    if self.$var && !other.$var {
                        missings.push(format!("{} is not available", $name));
                    }
                )+

                if missings.is_empty() {
                    Ok(())
                } else {
                    Err(missings)
                }
            }

            pub fn activate(&mut self, extension_name: &str) -> Result<(), String> {
                if self.check(extension_name) {
                    return Ok(());
                }

                Err(format!("Extension ({}) currently not supported!", extension_name))
            }

            pub unsafe fn add_raw_name(&mut self, extension_name: &str) {
                if self.check(extension_name) {
                    return;
                }

                println!("Add raw extension name: {}", extension_name);
                self.raw_names.push(extension_name.to_string());
            }

            fn check(&mut self, extension_name: &str) -> bool {
                $(
                    if extension_name == $name {
                        self.$var = true;
                        return true;
                    }
                )+

                false
            }
        }

        impl Default for $struct_name {
            fn default() -> Self {
                $struct_name {
                    $(
                        $var: false,
                    )+

                    raw_names: Vec::new(),
                }
            }
        }
    };
}
