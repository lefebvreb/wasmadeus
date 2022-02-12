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
    /// # use wasmide::errors::FrontendError;
    /// let error = FrontendError::custom("Custom error");
    /// ```
    #[inline]
    pub fn custom(msg: impl Into<String>) -> Self {
        Self::Custom(msg.into())
    }
}