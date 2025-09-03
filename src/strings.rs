/* 
* This file is part of the cloneable_errors library, licensed under the MIT license: 
* https://github.com/mini-bomba/cloneable_errors
*
* Copyright (C) 2024-2025 mini_bomba
*/

use std::{fmt::Display, ptr, sync::Arc};


/// A helper enum for easily cloneable strings
///
/// NOTE: `SharedString`s are compared using pointer equality
#[derive(Debug, Clone)]
pub enum SharedString {
    Arc(Arc<str>),
    Static(&'static str),
}

/// `SharedStrings` are compared using pointer equality.
impl PartialEq for SharedString {
    fn eq(&self, other: &Self) -> bool {
        // Compare by pointer
        match (self, other) {
            (Self::Arc(this), Self::Arc(other)) => Arc::ptr_eq(this, other),
            (Self::Static(this), Self::Static(other)) => ptr::eq(this, other),
            // different types
            _ => false
        }
    }
}
impl Eq for SharedString {}

impl Display for SharedString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&'static str> for SharedString {
    fn from(value: &'static str) -> Self {
        SharedString::Static(value)
    }
}

impl From<Arc<str>> for SharedString {
    fn from(value: Arc<str>) -> Self {
        SharedString::Arc(value)
    }
}

impl From<String> for SharedString
{
    fn from(value: String) -> Self {
        SharedString::Arc(Arc::from(value))
    }
}

impl SharedString {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Arc(s) => s,
            Self::Static(s) => s,
        }
    }
}

// serde

#[cfg(feature = "serde")]
mod serde_impl {
    use serde::{Serialize, Deserialize};
    use super::{SharedString, Arc};

    impl Serialize for SharedString {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
        {
            serializer.serialize_str(self.as_str())
        }
    }

    impl<'de> Deserialize<'de> for SharedString {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de>
        {
            Ok(Self::Arc(Arc::<str>::deserialize(deserializer)?))
        }
    }
}

// bincode

#[cfg(feature = "bincode")]
mod bincode_impl {
    use bincode::{Encode, Decode, impl_borrow_decode};
    use super::SharedString;

    impl Encode for SharedString {
        fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
            self.as_str().encode(encoder)
        }
    }

    impl<Context> Decode<Context> for SharedString {
        fn decode<D: bincode::de::Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
            Ok(SharedString::Arc(Decode::decode(decoder)?))
        }
    }

    impl_borrow_decode!(SharedString);
}
