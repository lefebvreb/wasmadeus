use core::cell::BorrowMutError;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct SignalError;

impl From<BorrowMutError> for SignalError {
    #[inline]
    fn from(_: BorrowMutError) -> Self {
        Self
    }
}

pub type Result<T> = core::result::Result<T, SignalError>;
