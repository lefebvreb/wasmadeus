#![no_std]

extern crate alloc;

pub mod errors;
pub mod stores;

pub mod prelude {
    //! use wasmide::prelude::*; to import common stores, components, and styles.

    pub use super::stores::{Store, Subscribable, Value};
}