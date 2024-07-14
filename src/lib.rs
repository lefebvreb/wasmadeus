#![doc(html_logo_url = "https://raw.githubusercontent.com/lefebvreb/wasmadeus/main/logo.svg")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

extern crate alloc;

pub mod attribute;
pub mod component;
#[cfg(feature = "fetch")]
#[cfg_attr(docsrs, doc(cfg(feature = "fetch")))]
pub mod fetch;
pub mod html;
#[cfg(feature = "logger")]
#[cfg_attr(docsrs, doc(cfg(feature = "logger")))]
pub mod logger;
pub mod signal;
pub mod utils;

pub mod prelude {
    #[cfg(feature = "fetch")]
    pub use super::fetch::Fetch;
    pub use super::html;
    #[cfg(feature = "logger")]
    pub use super::logger::ConsoleLogger;
    pub use super::signal::{Signal, SignalMut};
}

pub use web_sys;
pub use web_sys::js_sys;
pub use web_sys::wasm_bindgen;
