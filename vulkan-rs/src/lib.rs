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
pub mod querypool;
pub mod queue;
pub mod renderpass;
pub mod semaphore;
pub mod shadermodule;
pub mod surface;
pub mod swapchain;

mod loader;

pub enum OutOfDate<T> {
    Ok(T),
    OutOfDate,
}

pub trait VkHandle<T> {
    fn vk_handle(&self) -> T;
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
        }

        impl $struct_name {
            pub fn to_list(self) -> Vec<VkString> {
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

                list
            }

            pub fn from_list(list: &Vec<VkString>) -> Self {
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

            pub fn activate(&mut self, extensions_name: &str) -> Result<(), String> {
                $(
                    if extensions_name == $name {
                        self.$var = true;
                        return Ok(());
                    }
                )+

                Err(format!("Extension ({}) currently not supported!", extensions_name))
            }

        }

        impl Default for $struct_name {
            fn default() -> Self {
                $struct_name {
                    $(
                        $var: false,
                    )+
                }
            }
        }
    };
}