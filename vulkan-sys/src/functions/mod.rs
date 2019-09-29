#[macro_export]
macro_rules! load_function_ptrs {
    ($struct_name:ident, { $($name:ident($($param_n:ident: $param_ty:ty),*) -> $ret:ty,)+ }) => (
        use std::mem;
        use std::ffi::CStr;

        pub struct $struct_name {
            $(
                pub $name: extern "system" fn($($param_ty),*) -> $ret,
            )+
        }

        impl $struct_name {
            pub fn load<F>(mut f: F) -> $struct_name
                where F: FnMut(&CStr) -> *const c_void
            {
                $struct_name {
                    $(
                        $name: unsafe {
                            extern "system" fn $name($(_: $param_ty),*) { panic!("function pointer `{}` not loaded", stringify!($name)) }
                            let name = CStr::from_bytes_with_nul_unchecked(concat!(stringify!($name), "\0").as_bytes());
                            let val = f(name);

                            if val.is_null() {
                                println!("failed loading {}", stringify!($name));
                                mem::transmute($name as *const ())
                            } else {
                                mem::transmute(val)
                            }
                        },
                    )+
                }
            }

            $(
                #[inline]
                pub unsafe fn $name(&self $(, $param_n: $param_ty)*) -> $ret {
                    let ptr = self.$name;
                    ptr($($param_n),*)
                }
            )+
        }
    )
}
pub mod core;
pub mod ext;
pub mod khr;
pub mod nv;

pub mod prelude;
