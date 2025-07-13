/* 
* This file is part of the cloneable_errors library, licensed under the MIT license: 
* https://github.com/mini-bomba/cloneable_errors
*
* Copyright (C) 2025 mini_bomba
*/

use std::{any::{Any, TypeId}, collections::HashMap, sync::Arc};

/// This trait should be implemented by any structs
/// that are intended to be used as error extensions.
pub trait Extension: 'static + Send + Sync + Any {}

pub(crate) type ExtensionMap = Arc<HashMap<TypeId, Arc<dyn Extension>>>;

