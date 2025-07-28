/* 
* This file is part of the cloneable_errors library, licensed under the MIT license: 
* https://github.com/mini-bomba/cloneable_errors
*
* Copyright (C) 2025 mini_bomba
*/
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

mod context;
#[cfg(feature = "extensions")]
mod extensions;
mod iterator;
mod serializable;
mod strings;
mod util;

pub use context::*;
#[cfg(feature = "extensions")]
pub use extensions::*;
pub use iterator::*;
pub use serializable::*;
pub use strings::*;

#[macro_export]
/// Create a new [`ErrorContext`] stack
///
/// if only one expression is passed in, the value is passed directly to [`ErrorContext::new`]
/// ```
/// # use cloneable_errors::anyhow;
/// let error = anyhow!("this is a literal value");
/// assert_eq!(format!("{error}"), "this is a literal value");
/// ```
///
/// otherwise, the arguments to this macro are passed through to [`format!`], which generates the
/// error message
/// ```
/// # use cloneable_errors::anyhow;
/// let error = anyhow!("this is a format string, x = {}", 1234);
/// assert_eq!(format!("{error}"), "this is a format string, x = 1234");
/// ```
///
/// if you want to use this behaviour while inlining all variables into the format string,
/// append a trailing comma, like this:
/// ```
/// # use cloneable_errors::anyhow;
/// let x = 1234;
/// let bad  = anyhow!("this variable reference wont work cuz no comma: x = {x}");
/// assert_eq!(format!("{bad}"), "this variable reference wont work cuz no comma: x = {x}");
///
/// let good = anyhow!("format string with inlined variable references, x = {x}",);
/// assert_eq!(format!("{good}"), "format string with inlined variable references, x = 1234")
/// ```
///
/// if a single expression is passed in, followed by `, extend:`, the value is passed directly to
/// [`ErrorContext::new`] and all further expressions are inserted into the error as extensions:
/// ```
/// # #[cfg(feature = "extensions")]
/// # {
/// # use std::sync::Arc;
/// # use cloneable_errors::{IntoErrorIterator, anyhow, Extension};
/// #[derive(Clone, Copy)]
/// struct A;
/// impl Extension for A {}
///
/// let error = anyhow!("literal message", extend: Arc::new(A));
/// assert_eq!(format!("{error}"), "literal message");
/// assert!(error.find_extension::<A>().is_some());
/// # }
/// ```
///
/// to insert extensions while using [`format!`] for the error message, the format string and the
/// format parameters must be wrapped in parentheses:
/// ```
/// # #[cfg(feature = "extensions")]
/// # {
/// # use std::sync::Arc;
/// # use cloneable_errors::{IntoErrorIterator, anyhow, Extension};
/// #[derive(Clone, Copy)]
/// struct A;
/// impl Extension for A {}
///
/// let error = anyhow!(("format string, x = {}", 1234), extend: Arc::new(A));
/// assert_eq!(format!("{error}"), "format string, x = 1234");
/// assert!(error.find_extension::<A>().is_some());
/// # }
/// ```
/// this is also how you use format strings with all variables inlined, no trailing comma
/// needed this time:
/// ```
/// # #[cfg(feature = "extensions")]
/// # {
/// # use std::sync::Arc;
/// # use cloneable_errors::{IntoErrorIterator, anyhow, Extension};
/// #[derive(Clone, Copy)]
/// struct A;
/// impl Extension for A {}
///
/// let x = 1234;
/// let error = anyhow!(("format string, x = {x}"), extend: Arc::new(A));
/// assert_eq!(format!("{error}"), "format string, x = 1234");
/// assert!(error.find_extension::<A>().is_some());
/// # }
/// ```
macro_rules! anyhow {
    ($val:expr) => {
        $crate::ErrorContext::new($val)
    };
    (($($format:tt)+), extend: $($ext:expr),+) => {
        $crate::ErrorContext::new(format!($($format)+))$(.with_extension($ext))+
    };
    ($val:expr, extend: $($ext:expr),+) => {
        $crate::ErrorContext::new($val)$(.with_extension($ext))+
    };
    ($($tok:tt)+) => {
        $crate::ErrorContext::new(format!($($tok)+))
    };
}

#[macro_export]
/// Create a new [`ErrorContext`] stack using [`anyhow!`] and immediately return it as [`Result::Err`]
macro_rules! bail {
    ($($tok:tt)+) => {
        return Err($crate::anyhow!($($tok)+));
    };
}
