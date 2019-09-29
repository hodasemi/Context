pub type PFN_vkVoidFunction = extern "system" fn() -> ();

// create dummy
pub extern "system" fn vkVoidFunction() -> () {}
