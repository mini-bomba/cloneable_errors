/*
* This file is part of the cloneable_errors library, licensed under the MIT license:
* https://github.com/mini-bomba/cloneable_errors
*
* Copyright (C) 2025 mini_bomba
*/

#[cfg(feature = "anyhow")]
mod anyhow;
mod error;
mod result;
#[cfg(feature = "extensions")]
mod result_extensions;
mod r#struct;

#[cfg(feature = "anyhow")]
pub use anyhow::{AnyhowErrContext, AnyhowResContext};
pub use error::ErrContext;
pub use r#struct::ErrorContext;
pub use result::ResContext;
#[cfg(feature = "extensions")]
pub use result_extensions::ResExtensions;
