/* 
* This file is part of the cloneable_errors library, licensed under the MIT license: 
* https://github.com/mini-bomba/cloneable_errors
*
* Copyright (C) 2025 mini_bomba
*/

use std::sync::Arc;

/// Compares two optional Arc<>s by address.
///
/// Returns true if either both are None, or if both are Some and the inner Arc<>s point to the
/// same memory
#[allow(clippy::ref_option)]
pub fn option_ptr_eq<T: ?Sized>(a: &Option<Arc<T>>, b: &Option<Arc<T>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => Arc::ptr_eq(a, b),
        // one is None, other is Some
        _ => false
    }
}
