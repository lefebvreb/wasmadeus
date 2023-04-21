#![no_std]

extern crate alloc;

pub mod signal;

pub mod prelude {
    pub use crate::signal::Signal;
}
