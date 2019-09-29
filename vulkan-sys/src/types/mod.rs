pub mod constants;
pub mod core;
pub mod ext;
pub mod khr;
pub mod nv;
pub mod types;
pub mod voidfunction;

pub mod prelude;

pub struct VK_NULL_HANDLE(u32);

#[macro_export]
macro_rules! SetupU64Conv {
    ($name: ident) => {
        impl $name {
            pub fn raw(&self) -> u64 {
                self.0
            }
        }

        impl From<u64> for $name {
            fn from(v: u64) -> Self {
                $name(v)
            }
        }
    };
}

#[macro_export]
macro_rules! SetupUSizeConv {
    ($name: ident) => {
        impl $name {
            pub fn raw(&self) -> usize {
                self.0
            }
        }

        impl From<usize> for $name {
            fn from(v: usize) -> Self {
                $name(v)
            }
        }
    };
}
