/*
* This file is part of the cloneable_errors library, licensed under the MIT license:
* https://github.com/mini-bomba/cloneable_errors
*
* Copyright (C) 2024-2025 mini_bomba
*/

use std::{error::Error, sync::Arc};

use crate::{ErrorContext, SharedString};

/// A helper trait for annotating any Error with an [`ErrorContext`]
pub trait ErrContext {
    /// Wrap this error into a new [`ErrorContext`] error, annotated with the specified context
    fn context<M>(self, msg: M) -> ErrorContext
    where
        M: Into<SharedString>;
}

impl<T> ErrContext for T
where
    T: Error + Send + Sync + 'static,
{
    fn context<M>(self, msg: M) -> ErrorContext
    where
        M: Into<SharedString>,
    {
        ErrorContext {
            context: msg.into(),
            cause: Some(Arc::new(self)),
            #[cfg(feature = "extensions")]
            extensions: None,
        }
    }
}
