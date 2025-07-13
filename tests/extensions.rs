/* 
* This file is part of the cloneable_errors library, licensed under the MIT license: 
* https://github.com/mini-bomba/cloneable_errors
*
* Copyright (C) 2025 mini_bomba
*/
#![cfg(feature = "extensions")]

use std::sync::Arc;

use cloneable_errors::{ErrorContext, Extension};

#[derive(PartialEq, Eq, Debug)]
struct A;
#[derive(PartialEq, Eq, Debug)]
struct B(u32);

impl Extension for A {}
impl Extension for B {}

#[test]
fn test_extensions() {
    let mut error = ErrorContext::new("helo");

    // no extensions inserted yet
    assert_eq!(error.get_extension::<A>(), None);
    assert_eq!(error.get_extension::<B>(), None);

    // insert A
    let a = Arc::new(A);
    error.add_extension(a.clone());

    assert_eq!(error.get_extension::<A>().as_ref(), Some(&a));
    assert!(Arc::ptr_eq(&error.get_extension::<A>().unwrap(), &a));
    assert_eq!(error.get_extension::<B>(), None);

    // insert B
    let b1 = Arc::new(B(3));
    error.add_extension(b1.clone());

    assert_eq!(error.get_extension::<A>().as_ref(), Some(&a));
    assert!(Arc::ptr_eq(&error.get_extension::<A>().unwrap(), &a));
    assert_eq!(error.get_extension::<B>().as_ref(), Some(&b1));
    assert!(Arc::ptr_eq(&error.get_extension::<B>().unwrap(), &b1));

    // overwrite B
    let b2 = Arc::new(B(7));
    error.add_extension(b2.clone());

    assert_eq!(error.get_extension::<A>().as_ref(), Some(&a));
    assert!(Arc::ptr_eq(&error.get_extension::<A>().unwrap(), &a));
    assert_eq!(error.get_extension::<B>().as_ref(), Some(&b2));
    assert!(Arc::ptr_eq(&error.get_extension::<B>().unwrap(), &b2));
    assert_eq!(Arc::strong_count(&b1), 1);
    drop(b1);

    // remove A
    let removed_a = error.remove_extension::<A>();

    assert_eq!(removed_a.as_ref(), Some(&a));
    assert!(Arc::ptr_eq(&removed_a.unwrap(), &a));
    // removed_a gets dropped here
    assert_eq!(Arc::strong_count(&a), 1);
    assert_eq!(error.get_extension::<A>(), None);
    assert_eq!(error.get_extension::<B>().as_ref(), Some(&b2));
    assert!(Arc::ptr_eq(&error.get_extension::<B>().unwrap(), &b2));
}
