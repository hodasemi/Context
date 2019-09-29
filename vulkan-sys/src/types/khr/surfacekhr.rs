use crate::prelude::*;
use crate::SetupU64Conv;

use std::mem;
use std::ptr;
use std::sync::Once;

/*
pub type PFN_vkCreateXlibSurfaceKHR = extern "system" fn(
    Instance: VkInstance,
    pCreateInfo: *const VkXlibSurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;

pub type PFN_vkCreateXcbSurfaceKHR = extern "system" fn(
    Instance: VkInstance,
    pCreateInfo: *const VkXcbSurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;

pub type PFN_vkCreateWaylandSurfaceKHR = extern "system" fn(
    Instance: VkInstance,
    pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;

pub type PFN_vkCreateMirSurfaceKHR = extern "system" fn(
    Instance: VkInstance,
    pCreateInfo: *const VkMirSurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;

pub type PFN_vkCreateAndroidSurfaceKHR = extern "system" fn(
    Instance: VkInstance,
    pCreateInfo: *const VkAndroidSurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;

pub type PFN_vkCreateWin32SurfaceKHR = extern "system" fn(
    Instance: VkInstance,
    pCreateInfo: *const VkWin32SurfaceCreateInfoKHR,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;

pub type PFN_vkCreateMacOSSurfaceMVK = extern "system" fn(
    Instance: VkInstance,
    pCreateInfo: *const VkMacOSSurfaceCreateInfoMVK,
    pAllocator: *const VkAllocationCallbacks,
    pSurface: *mut VkSurfaceKHR,
) -> VkResult;

pub type PFN_vkDestroySurfaceKHR = extern "system" fn(
    Instance: VkInstance,
    surface: VkSurfaceKHR,
    pAllocator: *const VkAllocationCallbacks,
) -> ();

// functions
static mut vkCreateXlibSurfaceKHR: Option<PFN_vkCreateXlibSurfaceKHR> = None;
static mut vkCreateXcbSurfaceKHR: Option<PFN_vkCreateXcbSurfaceKHR> = None;
static mut vkCreateWaylandSurfaceKHR: Option<PFN_vkCreateWaylandSurfaceKHR> = None;
static mut vkCreateMirSurfaceKHR: Option<PFN_vkCreateMirSurfaceKHR> = None;
static mut vkCreateAndroidSurfaceKHR: Option<PFN_vkCreateAndroidSurfaceKHR> = None;
static mut vkCreateWin32SurfaceKHR: Option<PFN_vkCreateWin32SurfaceKHR> = None;
static mut vkCreateMacOSSurfaceMVK: Option<PFN_vkCreateMacOSSurfaceMVK> = None;
static mut vkDestroySurfaceKHR: Option<PFN_vkDestroySurfaceKHR> = None;

static INIT: Once = Once::new();
*/

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VkSurfaceKHR(u64);
SetupU64Conv!(VkSurfaceKHR);

/*
pub trait VkSurfaceKHRCreation<T>: Sized {
    fn create(instance: VkInstance, create_info: &T) -> Result<Self, VkResult>;
    fn create_with_allocation_callbacks(
        instance: VkInstance,
        create_info: &T,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<Self, VkResult>;
}

impl VkSurfaceKHR {
    pub fn destroy(&self, instance: VkInstance) -> Result<(), VkResult> {
        INIT.call_once(|| Self::get_pfn(instance));

        match unsafe { vkDestroySurfaceKHR } {
            Some(destroy) => {
                (destroy)(instance, *self, ptr::null());
                Ok(())
            }
            None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
        }
    }

    pub fn destroy_with_allocation_callbacks(
        &self,
        instance: VkInstance,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<(), VkResult> {
        INIT.call_once(|| Self::get_pfn(instance));

        match unsafe { vkDestroySurfaceKHR } {
            Some(destroy) => {
                (destroy)(instance, *self, allocation_callbacks);
                Ok(())
            }
            None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
        }
    }

    fn get_pfn(instance: VkInstance) {
        unsafe {
            vkCreateXlibSurfaceKHR =
                mem::transmute(instance.get_proc_addr(&VkString::new("vkCreateXlibSurfaceKHR")));

            vkCreateXcbSurfaceKHR =
                mem::transmute(instance.get_proc_addr(&VkString::new("vkCreateXcbSurfaceKHR")));

            vkCreateWaylandSurfaceKHR =
                mem::transmute(instance.get_proc_addr(&VkString::new("vkCreateWaylandSurfaceKHR")));

            vkCreateMirSurfaceKHR =
                mem::transmute(instance.get_proc_addr(&VkString::new("vkCreateMirSurfaceKHR")));

            vkCreateAndroidSurfaceKHR =
                mem::transmute(instance.get_proc_addr(&VkString::new("vkCreateAndroidSurfaceKHR")));

            vkCreateWin32SurfaceKHR =
                mem::transmute(instance.get_proc_addr(&VkString::new("vkCreateWin32SurfaceKHR")));

            vkCreateMacOSSurfaceMVK =
                mem::transmute(instance.get_proc_addr(&VkString::new("vkCreateMacOSSurfaceMVK")));

            vkDestroySurfaceKHR =
                mem::transmute(instance.get_proc_addr(&VkString::new("vkDestroySurfaceKHR")));
        }
    }
}

impl VkSurfaceKHRCreation<VkWin32SurfaceCreateInfoKHR> for VkSurfaceKHR {
    fn create(
        instance: VkInstance,
        create_info: &VkWin32SurfaceCreateInfoKHR,
    ) -> Result<Self, VkResult> {
        INIT.call_once(|| Self::get_pfn(instance));

        match unsafe { vkCreateWin32SurfaceKHR } {
            Some(create) => unsafe {
                let mut surface_khr = mem::uninitialized();

                let result = (create)(instance, create_info, ptr::null(), &mut surface_khr);

                if result == VK_SUCCESS {
                    Ok(surface_khr)
                } else {
                    Err(result)
                }
            },
            None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
        }
    }

    fn create_with_allocation_callbacks(
        instance: VkInstance,
        create_info: &VkWin32SurfaceCreateInfoKHR,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<Self, VkResult> {
        INIT.call_once(|| Self::get_pfn(instance));

        match unsafe { vkCreateWin32SurfaceKHR } {
            Some(create) => unsafe {
                let mut surface_khr = mem::uninitialized();

                let result = (create)(
                    instance,
                    create_info,
                    allocation_callbacks,
                    &mut surface_khr,
                );

                if result == VK_SUCCESS {
                    Ok(surface_khr)
                } else {
                    Err(result)
                }
            },
            None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
        }
    }
}

impl VkSurfaceKHRCreation<VkXlibSurfaceCreateInfoKHR> for VkSurfaceKHR {
    fn create(
        instance: VkInstance,
        create_info: &VkXlibSurfaceCreateInfoKHR,
    ) -> Result<Self, VkResult> {
        INIT.call_once(|| Self::get_pfn(instance));

        match unsafe { vkCreateXlibSurfaceKHR } {
            Some(create) => unsafe {
                let mut surface_khr = mem::uninitialized();

                let result = (create)(instance, create_info, ptr::null(), &mut surface_khr);

                if result == VK_SUCCESS {
                    Ok(surface_khr)
                } else {
                    Err(result)
                }
            },
            None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
        }
    }

    fn create_with_allocation_callbacks(
        instance: VkInstance,
        create_info: &VkXlibSurfaceCreateInfoKHR,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<Self, VkResult> {
        INIT.call_once(|| Self::get_pfn(instance));

        match unsafe { vkCreateXlibSurfaceKHR } {
            Some(create) => unsafe {
                let mut surface_khr = mem::uninitialized();

                let result = (create)(
                    instance,
                    create_info,
                    allocation_callbacks,
                    &mut surface_khr,
                );

                if result == VK_SUCCESS {
                    Ok(surface_khr)
                } else {
                    Err(result)
                }
            },
            None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
        }
    }
}

impl VkSurfaceKHRCreation<VkWaylandSurfaceCreateInfoKHR> for VkSurfaceKHR {
    fn create(
        instance: VkInstance,
        create_info: &VkWaylandSurfaceCreateInfoKHR,
    ) -> Result<Self, VkResult> {
        INIT.call_once(|| Self::get_pfn(instance));

        match unsafe { vkCreateWaylandSurfaceKHR } {
            Some(create) => unsafe {
                let mut surface_khr = mem::uninitialized();

                let result = (create)(instance, create_info, ptr::null(), &mut surface_khr);

                if result == VK_SUCCESS {
                    Ok(surface_khr)
                } else {
                    Err(result)
                }
            },
            None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
        }
    }

    fn create_with_allocation_callbacks(
        instance: VkInstance,
        create_info: &VkWaylandSurfaceCreateInfoKHR,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<Self, VkResult> {
        INIT.call_once(|| Self::get_pfn(instance));

        match unsafe { vkCreateWaylandSurfaceKHR } {
            Some(create) => unsafe {
                let mut surface_khr = mem::uninitialized();

                let result = (create)(
                    instance,
                    create_info,
                    allocation_callbacks,
                    &mut surface_khr,
                );

                if result == VK_SUCCESS {
                    Ok(surface_khr)
                } else {
                    Err(result)
                }
            },
            None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
        }
    }
}

impl VkSurfaceKHRCreation<VkAndroidSurfaceCreateInfoKHR> for VkSurfaceKHR {
    fn create(
        instance: VkInstance,
        create_info: &VkAndroidSurfaceCreateInfoKHR,
    ) -> Result<Self, VkResult> {
        INIT.call_once(|| Self::get_pfn(instance));

        match unsafe { vkCreateAndroidSurfaceKHR } {
            Some(create) => unsafe {
                let mut surface_khr = mem::uninitialized();

                let result = (create)(instance, create_info, ptr::null(), &mut surface_khr);

                if result == VK_SUCCESS {
                    Ok(surface_khr)
                } else {
                    Err(result)
                }
            },
            None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
        }
    }

    fn create_with_allocation_callbacks(
        instance: VkInstance,
        create_info: &VkAndroidSurfaceCreateInfoKHR,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<Self, VkResult> {
        INIT.call_once(|| Self::get_pfn(instance));

        match unsafe { vkCreateAndroidSurfaceKHR } {
            Some(create) => unsafe {
                let mut surface_khr = mem::uninitialized();

                let result = (create)(
                    instance,
                    create_info,
                    allocation_callbacks,
                    &mut surface_khr,
                );

                if result == VK_SUCCESS {
                    Ok(surface_khr)
                } else {
                    Err(result)
                }
            },
            None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
        }
    }
}

impl VkSurfaceKHRCreation<VkMacOSSurfaceCreateInfoMVK> for VkSurfaceKHR {
    fn create(
        instance: VkInstance,
        create_info: &VkMacOSSurfaceCreateInfoMVK,
    ) -> Result<Self, VkResult> {
        INIT.call_once(|| Self::get_pfn(instance));

        match unsafe { vkCreateMacOSSurfaceMVK } {
            Some(create) => unsafe {
                let mut surface_khr = mem::uninitialized();

                let result = (create)(instance, create_info, ptr::null(), &mut surface_khr);

                if result == VK_SUCCESS {
                    Ok(surface_khr)
                } else {
                    Err(result)
                }
            },
            None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
        }
    }

    fn create_with_allocation_callbacks(
        instance: VkInstance,
        create_info: &VkMacOSSurfaceCreateInfoMVK,
        allocation_callbacks: &VkAllocationCallbacks,
    ) -> Result<Self, VkResult> {
        INIT.call_once(|| Self::get_pfn(instance));

        match unsafe { vkCreateMacOSSurfaceMVK } {
            Some(create) => unsafe {
                let mut surface_khr = mem::uninitialized();

                let result = (create)(
                    instance,
                    create_info,
                    allocation_callbacks,
                    &mut surface_khr,
                );

                if result == VK_SUCCESS {
                    Ok(surface_khr)
                } else {
                    Err(result)
                }
            },
            None => Err(VK_ERROR_EXTENSION_NOT_PRESENT),
        }
    }
}
*/
