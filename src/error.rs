//! Error handling for CommonMark writer.
//!
//! This module provides error types and implementations for handling errors
//! that can occur during CommonMark writing.

use std::error::Error;
use std::fmt::{self, Display};

/// Errors that can occur during CommonMark writing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WriteError {
    /// An invalid heading level was encountered (must be 1-6).
    InvalidHeadingLevel(u8),
    /// A newline character was found in an inline element where it's not allowed (e.g., in strict mode or specific contexts like table cells, link text, image alt text).
    NewlineInInlineElement(String),
    /// A newline character was found in image alt text.
    NewlineInImageAltText,
    /// An underlying formatting error occurred.
    FmtError(String),
}

impl Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WriteError::InvalidHeadingLevel(level) => write!(
                f,
                "Invalid heading level: {}. Level must be between 1 and 6.",
                level
            ),
            WriteError::NewlineInInlineElement(context) => write!(
                f,
                "Newline character found within an inline element ({}) which is not allowed in strict mode or this context.",
                context
            ),
            WriteError::NewlineInImageAltText => {
                write!(f, "Newline character found in image alt text, which is not allowed.")
            }
            WriteError::FmtError(msg) => write!(f, "Formatting error: {}", msg),
        }
    }
}

impl Error for WriteError {}

// Allow converting fmt::Error into WriteError for convenience when using `?`
impl From<fmt::Error> for WriteError {
    fn from(err: fmt::Error) -> Self {
        WriteError::FmtError(err.to_string())
    }
}

/// Result type alias for writer operations.
pub type WriteResult<T> = Result<T, WriteError>;
