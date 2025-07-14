/*
* This file is part of the cloneable_errors library, licensed under the MIT license:
* https://github.com/mini-bomba/cloneable_errors
*
* Copyright (C) 2025 mini_bomba
*/

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
    sync::{Arc, LazyLock, Mutex},
};

/// This trait should be implemented by any structs
/// that are intended to be used as error extensions.
pub trait Extension: 'static + Send + Sync + Any {}

pub(crate) type ExtensionMap = Arc<HashMap<TypeId, Arc<dyn Extension>>>;

static MASK_CACHE: LazyLock<Mutex<HashMap<TypeId, Arc<dyn Extension>>>> =
    LazyLock::new(Mutex::default);

/// A special type that masks any extesions of a given
/// type that may exist deeper along the cause chain
/// from being seen in higher-level errors.
#[derive(Clone, Copy, Debug)]
pub(crate) struct MaskExtension<T: Extension + ?Sized> {
    _phantom: PhantomData<T>,
}
impl<T: Extension + ?Sized> Extension for MaskExtension<T> {}

impl<T: Extension + ?Sized> MaskExtension<T> {
    pub(crate) fn get() -> Arc<dyn Extension> {
        MASK_CACHE
            .lock()
            .expect("Internal lock got poisoned")
            .entry(TypeId::of::<T>())
            .or_insert_with(|| {
                Arc::new(MaskExtension {
                    _phantom: PhantomData::<T>,
                })
            })
            .clone()
    }
}
