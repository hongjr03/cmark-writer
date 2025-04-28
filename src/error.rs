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
    /// An underlying formatting error occurred.
    FmtError(String),
    /// An unsupported node type was encountered.
    UnsupportedNodeType,
    /// Invalid structure in a node (e.g., mismatched table columns)
    InvalidStructure(String),
    /// A custom error with a message and optional error code.
    Custom {
        /// Custom error message
        message: String,
        /// Optional error code for programmatic identification
        code: Option<String>,
    },
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
            WriteError::FmtError(msg) => write!(f, "Formatting error: {}", msg),
            WriteError::UnsupportedNodeType => {
                write!(f, "Unsupported node type encountered during writing.")
            },
            WriteError::InvalidStructure(msg) => {
                write!(f, "Invalid structure: {}", msg)
            },
            WriteError::Custom { message, code } => {
                if let Some(code) = code {
                    write!(f, "Custom error [{}]: {}", code, message)
                } else {
                    write!(f, "Custom error: {}", message)
                }
            }
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

/// Convenience methods for creating custom errors
impl WriteError {
    /// Create a new custom error with a message
    pub fn custom<S: Into<String>>(message: S) -> Self {
        WriteError::Custom {
            message: message.into(),
            code: None,
        }
    }

    /// Create a new custom error with a message and error code
    pub fn custom_with_code<S1: Into<String>, S2: Into<String>>(message: S1, code: S2) -> Self {
        WriteError::Custom {
            message: message.into(),
            code: Some(code.into()),
        }
    }
}

/// Trait to define custom error factories for WriteError
///
/// This trait allows extending WriteError with custom error constructors
/// while allowing both library and user code to define their own error types.
pub trait CustomErrorFactory {
    /// Create an error from this factory
    fn create_error(&self) -> WriteError;
}

/// Struct to create structure errors with formatted messages
pub struct StructureError {
    /// Format string for the error message
    format: String,
    /// Arguments for formatting
    args: Vec<String>,
}

impl StructureError {
    /// Create a new structure error with a format string and arguments
    pub fn new<S: Into<String>>(format: S) -> Self {
        Self {
            format: format.into(),
            args: Vec::new(),
        }
    }

    /// Add an argument to the format string
    pub fn arg<S: Into<String>>(mut self, arg: S) -> Self {
        self.args.push(arg.into());
        self
    }
}

impl CustomErrorFactory for StructureError {
    fn create_error(&self) -> WriteError {
        let message = match self.args.len() {
            0 => self.format.clone(),
            1 => self.format.replace("{}", &self.args[0]),
            _ => {
                let mut result = self.format.clone();
                for arg in &self.args {
                    if let Some(pos) = result.find("{}") {
                        result.replace_range(pos..pos + 2, arg);
                    }
                }
                result
            }
        };

        WriteError::InvalidStructure(message)
    }
}

/// Struct to create custom errors with codes
pub struct CodedError {
    /// The error message
    message: String,
    /// The error code
    code: String,
}

impl CodedError {
    /// Create a new custom error with message and code
    pub fn new<S1: Into<String>, S2: Into<String>>(message: S1, code: S2) -> Self {
        Self {
            message: message.into(),
            code: code.into(),
        }
    }
}

impl CustomErrorFactory for CodedError {
    fn create_error(&self) -> WriteError {
        WriteError::custom_with_code(&self.message, &self.code)
    }
}

/// Extensions for Result<T, WriteError> to work with custom error factories
pub trait WriteResultExt<T> {
    /// Convert a custom error factory into an Err result
    fn custom_error<F: CustomErrorFactory>(factory: F) -> Result<T, WriteError>;
}

impl<T> WriteResultExt<T> for Result<T, WriteError> {
    fn custom_error<F: CustomErrorFactory>(factory: F) -> Result<T, WriteError> {
        Err(factory.create_error())
    }
}
