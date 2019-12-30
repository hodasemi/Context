pub mod constants;
pub mod core;
pub mod ext;
pub mod khr;
pub mod nv;
pub mod types;
pub mod voidfunction;

pub mod prelude;

#[macro_export]
macro_rules! SetupU64Conv {
    ($name: ident) => {
        impl $name {
            pub const NULL_HANDLE: $name = $name(0);

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
            pub const NULL_HANDLE: $name = $name(0);

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
