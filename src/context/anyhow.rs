/*
* This file is part of the cloneable_errors library, licensed under the MIT license:
* https://github.com/mini-bomba/cloneable_errors
*
* Copyright (C) 2024-2025 mini_bomba
*/

use std::sync::Arc;

use crate::{ErrorContext, SharedString};

impl From<anyhow::Error> for ErrorContext {
    /// <div class="warning">
    ///
    /// NOTE: Converting [`anyhow::Error`] into [`ErrorContext`] causes the anyhow error stack to
    ///       be flattened into a stack of string errors! Extracting the error type-specific data
    ///       that is not exposed in the [`std::fmt::Display`] impl will not be possible!
    ///
    /// </div>
    fn from(value: anyhow::Error) -> Self {
        let flattened = crate::SerializableError::from_anyhow(&value);
        ErrorContext {
            context: flattened.context,
            cause: flattened
                .cause
                .map(|arc| arc as Arc<(dyn std::error::Error + Send + Sync + 'static)>),
            #[cfg(feature = "extensions")]
            extensions: None,
        }
    }
}

/// A helper trait for converting an anyhow error stack into an [`ErrorContext`] stack
pub trait AnyhowErrContext {
    /// Convert this anyhow error into a new [`ErrorContext`] error, annotated with the specified context message
    ///
    /// Note: This function will flatten the entire error stack into a [`crate::SerializableError`], any data not
    ///       exposed in the Display implementations of errors will be lost!
    fn context<M>(self, msg: M) -> ErrorContext
    where
        M: Into<SharedString>;
}

impl AnyhowErrContext for anyhow::Error {
    fn context<M>(self, msg: M) -> ErrorContext
    where
        M: Into<SharedString>,
    {
        ErrorContext {
            context: msg.into(),
            cause: Some(Arc::new(ErrorContext::from(self))),
            #[cfg(feature = "extensions")]
            extensions: None,
        }
    }
}

#[allow(clippy::missing_errors_doc)]
/// A helper trait for converting anyhow results into [`ErrorContext`] results
pub trait AnyhowResContext<T, E> {
    /// Map the anyhow error into a new [`ErrorContext`] error, annotated with the specified context message
    ///
    /// Note: This function will flatten the entire error stack into a [`crate::SerializableError`], any data not
    ///       exposed in the Display implementations of errors will be lost!
    ///       (this only applies if the result error is an anyhow error)
    fn context<M>(self, msg: M) -> Result<T, ErrorContext>
    where
        M: Into<SharedString>;

    /// Map the anyhow error into a new [`ErrorContext`] error, annotated with a dynamically
    /// computed context message
    ///
    /// Note: This function will flatten the entire error stack into a [`crate::SerializableError`], any data not
    ///       exposed in the Display implementations of errors will be lost!
    ///       (this only applies if the result error is an anyhow error)
    fn with_context<M, F>(self, f: F) -> Result<T, ErrorContext>
    where
        M: Into<SharedString>,
        F: FnOnce() -> M;
}

impl<T, E> AnyhowResContext<T, E> for Result<T, E>
where
    E: AnyhowErrContext,
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
