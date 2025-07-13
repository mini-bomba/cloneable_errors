/*
* This file is part of the cloneable_errors library, licensed under the MIT license:
* https://github.com/mini-bomba/cloneable_errors
*
* Copyright (C) 2024-2025 mini_bomba
*/

use std::convert::Infallible;

use crate::{ErrContext, ErrorContext, SharedString};


#[allow(clippy::missing_errors_doc)]
/// A helper trait for annotating result errors and empty options
pub trait ResContext<T, E> {
    /// Map the error into a new cloneable [`ErrorContext`] error, annotated with a specified context message
    fn context<M>(self, msg: M) -> Result<T, ErrorContext>
    where
        M: Into<SharedString>;

    /// Map the error into a new cloneable [`ErrorContext`] error, annotated with a dynamically computed context
    /// message
    fn with_context<M, F>(self, f: F) -> Result<T, ErrorContext>
    where
        M: Into<SharedString>,
        F: FnOnce() -> M;
}

impl<T, E> ResContext<T, E> for Result<T, E>
where
    E: ErrContext,
{
    fn context<M>(self, msg: M) -> Result<T, ErrorContext>
    where
        M: Into<SharedString>,
    {
        self.map_err(|e| e.context(msg))
    }

    fn with_context<M, F>(self, f: F) -> Result<T, ErrorContext>
    where
        M: Into<SharedString>,
        F: FnOnce() -> M,
    {
        self.map_err(|e| e.context(f()))
    }
}

impl<T> ResContext<T, Infallible> for Option<T> {
    fn context<M>(self, msg: M) -> Result<T, ErrorContext>
    where
        M: Into<SharedString>,
    {
        self.ok_or_else(|| ErrorContext {
            context: msg.into(),
            cause: None,
            #[cfg(feature = "extensions")]
            extensions: None,
        })
    }

    fn with_context<M, F>(self, f: F) -> Result<T, ErrorContext>
    where
        M: Into<SharedString>,
        F: FnOnce() -> M,
    {
        self.ok_or_else(|| ErrorContext {
            context: f().into(),
            cause: None,
            #[cfg(feature = "extensions")]
            extensions: None,
        })
    }
}
