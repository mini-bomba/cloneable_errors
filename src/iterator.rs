/* 
* This file is part of the cloneable_errors library, licensed under the MIT license: 
* https://github.com/mini-bomba/cloneable_errors
*
* Copyright (C) 2024 mini_bomba
*/

use std::{error::Error, sync::Arc};

#[cfg(feature = "extensions")]
use crate::Extension;
use crate::{ErrorContext, SerializableError, SharedString};


/// `ErrorIterator` - iterates over the chain of [`Error::source`]
///
/// The iterator will attempt to cast away any smart pointers to make downcasting to a concrete
/// type easier.
pub struct ErrorIterator<'a> {
    next_item: Option<&'a (dyn Error + 'static)>,
}

impl<'a> Iterator for ErrorIterator<'a> {
    type Item = &'a (dyn Error + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(mut err) = self.next_item {
            // attempt to cast away any smart pointers
            // this won't catch every single weird case of wrapping in smart pointers (as it's
            // impossible) but it catches the most common ones, probably
            loop {
                if let Some(downcasted) = err.downcast_ref::<Arc<dyn Error>>() {
                    err = &**downcasted;
                    continue;
                }
                if let Some(downcasted) = err.downcast_ref::<Arc<dyn Error + Send + Sync>>() {
                    err = &**downcasted;
                    continue;
                }
                break;
            }


            self.next_item = err.source();
            Some(err)
        } else {
            None
        }
    }
}

impl<'a> From<&'a (dyn Error + 'static)> for ErrorIterator<'a> {
    fn from(value: &'a (dyn Error + 'static)) -> Self {
        Self {next_item: Some(value)}
    }
}

impl ErrorIterator<'_> {
    /// Copies and flattens the error stack into a [`SerializableError`]
    /// 
    /// # Panics
    /// Will panic if the iterator is empty.
    /// Pro tip: don't use this on a used iterator.
    /// Any unused `ErrorIterator` is guaranteed to have at least one item (the error it was
    /// initialized with) and therefore will not panic.
    #[must_use]
    pub fn serializable_copy(mut self) -> SerializableError {
        let first_error = self.next().expect("empty iterator");
        if let Some(err) = first_error.downcast_ref::<SerializableError>() {
            return err.clone()
        }
        let mut result = SerializableError {
            context: extract_message(first_error),
            cause: None,
        };
        let mut last = &mut result;

        for err in self {
            if let Some(err) = err.downcast_ref::<SerializableError>() {
                last.cause = Some(err.clone().into());
                break;
            }
            last.cause = Some(Arc::new(SerializableError { context: extract_message(err), cause: None }));
            // should be safe: we've just set this to a new Some(Arc)
            last = Arc::get_mut(last.cause.as_mut().unwrap()).unwrap();
        }

        result
    }

    /// Retrieves the most recent instance of a given extension type from the error stack
    #[cfg(feature = "extensions")]
    #[must_use]
    #[allow(clippy::missing_panics_doc)] // the only panic path would be a bug
    pub fn find_extension<E: Extension>(self) -> Option<Arc<E>> {
        for err in self {
            use std::any::TypeId;
            use crate::MaskExtension;

            let Some(err) = err.downcast_ref::<ErrorContext>() else { continue };

            if let Some(ext) = err.extensions.as_ref()
                .and_then(|m| 
                    m.get(&TypeId::of::<E>()).cloned()
                )
            {
                return Some(Arc::downcast(ext).expect("BUG: Extension stored under the wrong TypeId!"))
            }
            if err.extensions.as_ref()
                .is_some_and(|m|
                    m.contains_key(&TypeId::of::<MaskExtension<E>>())
                )
            {
                // found a mask matching the requested extension
                return None
            }
        }
        None
    }
}

pub trait IntoErrorIterator {
    /// Creates an iterator over [`Error::source`]s
    #[must_use]
    fn error_chain(&self) -> ErrorIterator<'_>;

    /// Copies and flattens the error stack into a [`SerializableError`]
    #[must_use]
    fn serializable_copy(&self) -> SerializableError {
        self.error_chain().serializable_copy()
    }

    /// Retrieves the most recent instance of a given extension type from the error stack
    #[cfg(feature = "extensions")]
    #[must_use]
    fn find_extension<E: Extension>(&self) -> Option<Arc<E>> {
        self.error_chain().find_extension()
    }
}

impl<T> IntoErrorIterator for T
where T: Error + 'static
{
    fn error_chain(&self) -> ErrorIterator<'_> {
        ErrorIterator { next_item: Some(self) }
    }
}

/// Extracts the top-level error message into a [`SharedString`] with optimizations for types defined in this crate
fn extract_message(err: &(dyn Error + 'static)) -> SharedString {
    if let Some(err) = err.downcast_ref::<SerializableError>() {
        // clone the context
        err.context.clone()
    } else if let Some(err) = err.downcast_ref::<ErrorContext>() {
        // clone the context
        err.context.clone()
    } else {
        // not our type, format
        format!("{err}").into()
    }
}
