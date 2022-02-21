#![no_std]

#![allow(unused)]

extern crate alloc;

pub mod app;
pub mod components;
pub mod errors;
pub mod stores;
pub mod styles;

use crate::styles::Style;
pub mod prelude {
    //! use wasmide::prelude::*; to import common stores, components, and styles.

    pub use crate::app::app;
    pub use super::components::Component;
    pub use super::stores::{Store, Subscribable, Value};
    pub use super::styles::Style;
}