//! Utility traits for error handling and configuration

use crate::error::WriteError;
use ecow::EcoString;

/// Error context trait
pub trait ErrorContext<T> {
    /// Add context information to error
    fn with_context<S: Into<EcoString>>(self, context: S) -> Result<T, WriteError>;

    /// Add context information using closure
    fn with_context_fn<F, S>(self, f: F) -> Result<T, WriteError>
    where
        F: FnOnce() -> S,
        S: Into<EcoString>;
}

/// Error factory trait
pub trait ErrorFactory<E> {
    /// Create error
    fn create_error(&self) -> E;

    /// Create error with context
    fn create_error_with_context<S: Into<EcoString>>(&self, _context: S) -> E {
        self.create_error()
    }
}

/// Configurable processor trait
pub trait ConfigurableProcessor {
    /// Configuration type
    type Config;

    /// Apply configuration
    fn configure(&mut self, config: Self::Config);

    /// Get current configuration
    fn config(&self) -> &Self::Config;
}

// Implement ErrorContext for Result
impl<T> ErrorContext<T> for Result<T, WriteError> {
    fn with_context<S: Into<EcoString>>(self, context: S) -> Result<T, WriteError> {
        self.map_err(|e| {
            let context_str = context.into();
            WriteError::custom(format!("{}: {}", context_str, e))
        })
    }

    fn with_context_fn<F, S>(self, f: F) -> Result<T, WriteError>
    where
        F: FnOnce() -> S,
        S: Into<EcoString>,
    {
        self.map_err(|e| {
            let context_str = f().into();
            WriteError::custom(format!("{}: {}", context_str, e))
        })
    }
}
