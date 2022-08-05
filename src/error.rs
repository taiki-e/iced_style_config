use std::io;

use thiserror::Error;

/// Alias for a `Result` with the error type `iced_style_config::Error`.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// An error that occurred during parsing the configuration file.
#[derive(Debug, Error)]
#[error(transparent)]
pub struct Error(#[from] ErrorKind);

// Hiding error variants from a library's public error type to prevent
// dependency updates from becoming breaking changes.
// We can add `is_*` methods that indicate the kind of error if needed, but
// don't expose dependencies' types directly in the public API.
#[derive(Debug, Error)]
pub(crate) enum ErrorKind {
    #[error(transparent)]
    Iced(#[from] iced::Error),

    /// An error that occurred during opening or reading the file.
    #[error(transparent)]
    Io(#[from] io::Error),

    /// An error that occurred during parsing TOML.
    #[error(transparent)]
    Toml(#[from] toml::de::Error),

    #[cfg(not(target_family = "wasm"))]
    #[cfg(feature = "hot-reloading")]
    #[error(transparent)]
    Notify(#[from] notify::Error),
}

impl Error {
    pub(crate) fn new(e: impl Into<ErrorKind>) -> Self {
        Self(e.into())
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::new(e)
    }
}

// iced is public dependency.
impl From<iced::Error> for Error {
    fn from(e: iced::Error) -> Self {
        Self::new(e)
    }
}

// Note: These implementations are intentionally not-exist to prevent dependency
// updates from becoming breaking changes.
// impl From<toml::de::Error> for Error
// impl From<notify::Error> for Error
