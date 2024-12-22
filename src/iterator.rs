/* 
* This file is part of the cloneable_errors library, licensed under the MIT license: 
* https://github.com/mini-bomba/cloneable_errors
*
* Copyright (C) 2024 mini_bomba
*/

use std::{error::Error, sync::Arc};

use crate::SerializableError;


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

pub trait IntoErrorIterator {
    /// Creates an iterator over [`Error::source`]s
    fn error_chain(&self) -> ErrorIterator<'_>;

    /// Copies and flattens the error stack into a [`SerializableError`]
    fn serializable_copy(&self) -> SerializableError {
        let mut iter = self.error_chain();
        let mut result = SerializableError {
            context: format!("{}", iter.next().expect("first item should exist")).into(),
            cause: None,
        };
        let mut last = &mut result;

        for err in iter {
            last.cause = Some(Arc::new(SerializableError { context: format!("{err}").into(), cause: None }));
            last = Arc::get_mut(last.cause.as_mut().unwrap()).unwrap();
        }

        result
    }
}

impl<T> IntoErrorIterator for T
where T: Error + 'static
{
    fn error_chain(&self) -> ErrorIterator<'_> {
        ErrorIterator { next_item: Some(self) }
    }
}
