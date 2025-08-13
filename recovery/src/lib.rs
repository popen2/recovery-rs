#![doc = include_str!("../README.md")]
pub use recovery_derive::Recovery;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecoveryStrategy {
    /// This error is temporary and the operation should be attempted again.
    Auto,

    /// This error is due to an unexpected result and has to be investigated
    /// further to understand if it's a bug or an unrecoverable error.
    Manual,

    /// The operation failed in an unrecoverable way, for example due to
    /// invalid user input, and will not be attempted again.
    Never,
}

pub trait Recovery {
    fn recovery(&self) -> RecoveryStrategy;
}
