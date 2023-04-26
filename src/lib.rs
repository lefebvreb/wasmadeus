#![doc(html_logo_url = "logo.svg")]
#![no_std]

extern crate alloc;

pub mod signal;
//mod utils;

pub mod prelude {
    pub use crate::signal::Signal;
}
