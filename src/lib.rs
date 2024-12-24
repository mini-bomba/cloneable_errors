/* 
* This file is part of the cloneable_errors library, licensed under the MIT license: 
* https://github.com/mini-bomba/cloneable_errors
*
* Copyright (C) 2024 mini_bomba
*/
#![cfg_attr(docsrs, feature(doc_cfg))]

mod context;
mod iterator;
mod serializable;
mod strings;

pub use context::*;
pub use iterator::*;
pub use serializable::*;
pub use strings::*;

#[macro_export]
/// Create a new [`ErrorContext`] stack
macro_rules! anyhow {
    ($val:expr) => {
        $crate::ErrorContext::new($val)
    };
    ($($tok:tt)+) => {
        $crate::ErrorContext::new(format!($($tok)+))
    };
}

#[macro_export]
/// Create a new [`ErrorContext`] stack and immediately return it as [`Result::Err`]
macro_rules! bail {
    ($($tok:tt)+) => {
        return Err($crate::anyhow!($($tok)+));
    };
}
