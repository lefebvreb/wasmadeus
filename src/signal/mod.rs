mod computed;
mod mutable;
mod traits;

pub use computed::*;
pub use mutable::*;
pub use traits::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct SignalMutatingError;

pub type Result<T> = core::result::Result<T, SignalMutatingError>;
