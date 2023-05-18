#![doc(html_logo_url = "https://raw.githubusercontent.com/lefebvreb/wasmadeus/main/logo.svg")]
#![no_std]
#![cfg_attr(feature = "nightly", feature(auto_traits, negative_impls))]

extern crate alloc;

pub mod attributes;
pub mod signal;
mod utils;

pub mod prelude {
    // TODO
}
