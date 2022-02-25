#![no_std]

#![allow(unused)]

extern crate alloc;

pub mod app;
pub mod components;
pub mod errors;
pub mod stores;
pub mod styles;

pub mod prelude {
    //! use wasmide::prelude::*; to import common stores, components, and styles.

    pub use crate::app::app;
    pub use crate::components::Component;
    pub use crate::stores::{Store, Subscribable, Value};
    pub use crate::styles::Style;
}