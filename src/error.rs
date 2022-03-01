//! Errors that may occur during the execution of the frontend.
//! 
//! It is very important that the frontend does not ever panic.
//! The type [`FrontendError`] provided in this module is used to communicate errors
//! to the user of the framework. Results shall not be `unwrap`ped.

use alloc::string::String;

/// An error that may occur during execution of the frontend.
#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum FrontendError {
    /// A store was updated while already updating.
    StoreUpdating,
    /// Custom error.
    Custom(String),
}

impl FrontendError {
    /// Creates a new [`FrontendError::Custom`] with the given message.
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use wasmide::error::FrontendError;
    /// let error = FrontendError::custom("Custom error");
    /// ```
    #[inline]
    pub fn custom(msg: impl Into<String>) -> Self {
        Self::Custom(msg.into())
    }
}

/// A type alias for a rust standard [`Result`](core::result::Result) 
/// that has a [`FrontendError`] as the error type.
pub type Result<T> = core::result::Result<T, FrontendError>;