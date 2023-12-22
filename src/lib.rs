#![doc(html_logo_url = "https://raw.githubusercontent.com/lefebvreb/wasmadeus/main/logo.svg")]
#![cfg_attr(feature = "nightly", feature(auto_traits, negative_impls))]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![no_std]

extern crate alloc;

pub mod attribute;
pub mod component;
pub mod fetch;
pub mod html;
#[cfg(feature = "logger")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "logger")))]
pub mod logger;
pub mod signal;
pub mod utils;

pub mod prelude {
    pub use super::fetch::Fetch;
    pub use super::html;
    #[cfg(feature = "logger")]
    pub use super::logger::ConsoleLogger;
    pub use super::signal::SignalMut;
}
