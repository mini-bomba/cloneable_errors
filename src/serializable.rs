/* 
* This file is part of the cloneable_errors library, licensed under the MIT license: 
* https://github.com/mini-bomba/cloneable_errors
*
* Copyright (C) 2024 mini_bomba
*/

use std::{error::Error, fmt::{Display, Debug}, sync::Arc};

#[cfg(feature="serde")]
use serde::{Deserialize, Serialize};

use crate::{IntoErrorIterator, SharedString};


/// An error stack with all messages flattened into strings, trivial to (de)serialize
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SerializableError {
    pub context: SharedString,
    pub cause: Option<Arc<SerializableError>>,
}

impl Display for SerializableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.context)
    }
}

impl Debug for SerializableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")?;

        let mut iter = self.error_chain().skip(1).enumerate();
        if let Some((i, item)) = iter.next() {
            write!(f, "\n\nCaused by:\n    {i}: {item}")?;

            for (i, item) in iter {
                write!(f, "\n    {i}: {item}")?;
            }
        }

        Ok(())
    }
}

impl Error for SerializableError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Some(cause) = self.cause.as_deref() {
            Some(cause)
        } else {
            None
        }
    }
}

#[cfg(feature = "anyhow")]
impl SerializableError {
    /// Convert an [`anyhow::Error`] into a [`SerializableError`]
    #[must_use]
    #[allow(clippy::missing_panics_doc)] // should never panic
    pub fn from_anyhow(err: &anyhow::Error) -> Self {
        crate::ErrorIterator::from(&**err as &(dyn Error + 'static)).serializable_copy()
    }
}
