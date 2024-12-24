/* 
* This file is part of the cloneable_errors library, licensed under the MIT license: 
* https://github.com/mini-bomba/cloneable_errors
*
* Copyright (C) 2024 mini_bomba
*/

use std::{error::Error, sync::Arc};

use crate::{ErrorContext, SerializableError, SharedString};


/// `ErrorIterator` - iterates over the chain of [`Error::source`]
pub struct ErrorIterator<'a> {
    next_item: Option<&'a (dyn Error + 'static)>,
}

impl<'a> Iterator for ErrorIterator<'a> {
    type Item = &'a (dyn Error + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(err) = self.next_item {
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
}

pub trait IntoErrorIterator {
    /// Creates an iterator over [`Error::source`]s
    fn error_chain(&self) -> ErrorIterator<'_>;

    /// Copies and flattens the error stack into a [`SerializableError`]
    fn serializable_copy(&self) -> SerializableError {
        self.error_chain().serializable_copy()
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
