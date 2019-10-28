use core::fmt::{self, Display};
use failure::{Backtrace, Context, Fail};
use std::any::Any;

#[derive(Debug)]
pub struct UtilError {
    inner: Context<String>,
}

impl UtilError {
    pub fn message(&self) -> String {
        format!("{:?}", self.inner)
    }
}

impl Fail for UtilError {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for UtilError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl Into<String> for UtilError {
    fn into(self) -> String {
        format!("{}", self)
    }
}

impl From<&'static str> for UtilError {
    fn from(msg: &'static str) -> UtilError {
        UtilError {
            inner: Context::new(msg.into()),
        }
    }
}

impl From<String> for UtilError {
    fn from(msg: String) -> UtilError {
        UtilError {
            inner: Context::new(msg),
        }
    }
}

impl From<Context<String>> for UtilError {
    fn from(inner: Context<String>) -> UtilError {
        UtilError { inner }
    }
}

impl From<std::cell::BorrowError> for UtilError {
    fn from(error: std::cell::BorrowError) -> UtilError {
        UtilError {
            inner: Context::new(format!("{}", error)),
        }
    }
}

impl From<std::cell::BorrowMutError> for UtilError {
    fn from(error: std::cell::BorrowMutError) -> UtilError {
        UtilError {
            inner: Context::new(format!("{}", error)),
        }
    }
}

impl From<std::io::Error> for UtilError {
    fn from(error: std::io::Error) -> UtilError {
        UtilError {
            inner: Context::new(format!("{}", error)),
        }
    }
}

// error value of thread::join
impl From<Box<dyn Any + Send + 'static>> for UtilError {
    fn from(error: Box<dyn Any + Send + 'static>) -> UtilError {
        UtilError {
            inner: Context::new(format!("{:?}", error)),
        }
    }
}

impl<T: std::fmt::Debug> From<std::sync::PoisonError<T>> for UtilError {
    fn from(error: std::sync::PoisonError<T>) -> UtilError {
        UtilError {
            inner: Context::new(format!("{:?}", error.get_ref())),
        }
    }
}

impl<T> From<std::sync::TryLockError<T>> for UtilError {
    fn from(error: std::sync::TryLockError<T>) -> UtilError {
        UtilError {
            inner: Context::new(format!("{:?}", error)),
        }
    }
}

pub type VerboseResult<T> = Result<T, UtilError>;
