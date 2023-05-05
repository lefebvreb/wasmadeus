//mod computed;
//mod mutable;
mod raw;
mod traits;

// pub use computed::*;
// pub use mutable::*;
pub use traits::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct SignalError;

pub type Result<T> = core::result::Result<T, SignalError>;
