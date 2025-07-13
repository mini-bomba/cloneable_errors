/*
* This file is part of the cloneable_errors library, licensed under the MIT license:
* https://github.com/mini-bomba/cloneable_errors
*
* Copyright (C) 2025 mini_bomba
*/

use std::sync::Arc;

use crate::{ErrorContext, Extension};

#[allow(clippy::missing_errors_doc)]
/// A helper trait for extending error variants of `Result<T, ErrorContext>`
pub trait ResExtensions<T> {
    /// Adds the given extension to the error
    fn extend(self, ext: Arc<dyn Extension>) -> Result<T, ErrorContext>;

    /// Adds a dynamically computed extension to the error
    ///
    /// The function is only called if the result is an error variant
    fn with_extension(self, ext: impl FnOnce() -> Arc<dyn Extension>) -> Result<T, ErrorContext>;

    /// Removes any extension of a given type from the error
    fn without_extension<E: Extension>(self) -> Result<T, ErrorContext>;
}

impl<T> ResExtensions<T> for Result<T, ErrorContext> {
    fn extend(self, ext: Arc<dyn Extension>) -> Result<T, ErrorContext> {
        self.map_err(|err| err.with_extension(ext))
    }

    fn with_extension(self, ext: impl FnOnce() -> Arc<dyn Extension>) -> Result<T, ErrorContext> {
        self.map_err(|err| err.with_extension(ext()))
    }

    fn without_extension<E: Extension>(self) -> Result<T, ErrorContext> {
        self.map_err(ErrorContext::without_extension::<E>)
    }
}
