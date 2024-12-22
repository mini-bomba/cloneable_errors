/* 
* This file is part of the cloneable_errors library, licensed under the MIT license: 
* https://github.com/mini-bomba/cloneable_errors
*
* Copyright (C) 2024 mini_bomba
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
        match self {
            SharedString::Arc(s) => write!(f, "{s}"),
            SharedString::Static(s) => write!(f, "{s}"),
        }
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
