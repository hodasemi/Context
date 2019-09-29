// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// !!!!!!!!!!! very unstable, use at your own risk !!!!!!!!!!!!!!!!!
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use super::prelude::*;
use std::ops::Fn;

#[allow(non_upper_case_globals)]
pub const void: () = ();

pub struct Callback {
    function: FunctionType,
}

impl Callback {
    pub fn new<F, Args>(function: F) -> Callback
    where
        F: Fn(Args) -> (),
        F: 'static,
        FunctionType: FunctionTypeCreator<F, Args>,
    {
        Callback {
            function: FunctionType::new(function),
        }
    }

    pub fn execute<Args>(&self, arguments: Args)
    where
        FunctionType: FunctionTypeExecutorOneArg<Args>,
    {
        display_error!(self.function.execute(arguments));
    }
}

pub trait FunctionTypeCreator<F: Fn(Args) -> () + 'static, Args> {
    fn new(function: F) -> FunctionType;
}

pub trait FunctionTypeExecutorOneArg<Args> {
    fn execute(&self, execute: Args) -> VerboseResult<()>;
}

macro_rules! build_function_type {
    ($struct_name: ident, $($ty: ty, $name: ident), *) => {
        pub enum $struct_name {
            $($name(Box<dyn Fn($ty)>)),*
        }

        $(
            impl<F: Fn($ty) -> () + 'static> FunctionTypeCreator<F, $ty> for $struct_name {
                fn new(function: F) -> FunctionType {
                    $struct_name::$name(Box::new(function))
                }
            }

            impl FunctionTypeExecutorOneArg<$ty> for $struct_name {
                fn execute(&self, args: $ty) -> VerboseResult<()> {
                    match self {
                        FunctionType::$name(ref function) => {
                            (function)(args);
                            Ok(())
                        }
                        _ => {
                            create_error!("wrong argument type for this callback");
                        },
                    }
                }
            }
        ) *
    }
}

#[rustfmt::skip]
build_function_type!(
    FunctionType,
        String, String,
        i32, I32,
        u32, U32,
        f32, F32,
        (), Void
);
