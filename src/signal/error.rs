use core::fmt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SignalUpdatingError;

impl fmt::Display for SignalUpdatingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "signal is already updating")
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SignalGetError {
    Uninit,
    Updating,
}

impl fmt::Display for SignalGetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uninit => write!(f, "signal is uninitialized"),
            Self::Updating => write!(f, "signal is currently updating"),
        }
    }
}
