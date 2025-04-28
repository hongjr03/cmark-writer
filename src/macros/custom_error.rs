//! Custom error macro definitions
//!
//! This module provides a macro to simplify the creation of custom error types for CommonMark writers.

/// Define custom error factories for CommonMark writing
///
/// This macro simplifies the creation of custom error factories by automatically
/// generating structs that implement the CustomErrorFactory trait.
///
/// # Usage
///
/// ```rust
/// use cmark_writer::define_custom_errors;
/// use cmark_writer::CustomErrorFactory;
/// use cmark_writer::WriteResult;
/// use cmark_writer::WriteError;
///
/// // Define custom error types
/// define_custom_errors! {
///     // Structure error - format string with arguments
///     struct TableStructureError(message: &str) with format = "Table structure error: {}";
///     
///     // Coded error - with message and code
///     coded TableAlignmentError(message: &str, code: &str);
///     
///     struct AnotherStructureError(message: &str) with format = "Another error: {}";
/// }
///
/// // Later in your code:
/// fn validate_table() -> WriteResult<()> {
///     // Using a structure error
///     return Err(TableStructureError::new("Rows don't match").into_error());
///     
///     // Using a coded error
///     return Err(TableAlignmentError::new("Invalid alignment", "INVALID_ALIGNMENT").into_error());
/// }
/// ```
#[macro_export]
macro_rules! define_custom_errors {
    (
        struct $structure_name:ident ( $($param_name:ident : $param_type:ty),* ) with format = $format:expr;

        $( $rest:tt )*
    ) => {
        /// Custom structure error factory
        #[derive(Debug, Clone)]
        pub struct $structure_name {
            format: String,
            args: Vec<String>,
        }

        impl $structure_name {
            /// Create a new structure error
            pub fn new($($param_name: $param_type),*) -> Self {
                let format = $format.to_string();
                let mut args = Vec::new();
                $(
                    args.push($param_name.to_string());
                )*
                Self { format, args }
            }

            /// Convert to a WriteError
            pub fn into_error(self) -> $crate::WriteError {
                let mut error_factory = $crate::StructureError::new(self.format);
                for arg in self.args {
                    error_factory = error_factory.arg(arg);
                }
                error_factory.create_error()
            }
        }

        impl From<$structure_name> for $crate::WriteError {
            fn from(factory: $structure_name) -> Self {
                factory.into_error()
            }
        }

        impl $crate::CustomErrorFactory for $structure_name {
            fn create_error(&self) -> $crate::WriteError {
                let mut error_factory = $crate::StructureError::new(self.format.clone());
                for arg in &self.args {
                    error_factory = error_factory.arg(arg.clone());
                }
                error_factory.create_error()
            }
        }

        $crate::define_custom_errors! { $( $rest )* }
    };

    (
        coded $coded_name:ident ( $message_param:ident : $message_type:ty, $code_param:ident : $code_type:ty );

        $( $rest:tt )*
    ) => {
        /// Custom coded error factory
        #[derive(Debug, Clone)]
        pub struct $coded_name {
            message: String,
            code: String,
        }

        impl $coded_name {
            /// Create a new coded error
            pub fn new($message_param: $message_type, $code_param: $code_type) -> Self {
                Self {
                    message: $message_param.to_string(),
                    code: $code_param.to_string(),
                }
            }

            /// Convert to a WriteError
            pub fn into_error(self) -> $crate::WriteError {
                $crate::CodedError::new(self.message, self.code).create_error()
            }
        }

        impl From<$coded_name> for $crate::WriteError {
            fn from(factory: $coded_name) -> Self {
                factory.into_error()
            }
        }

        impl $crate::CustomErrorFactory for $coded_name {
            fn create_error(&self) -> $crate::WriteError {
                $crate::CodedError::new(self.message.clone(), self.code.clone()).create_error()
            }
        }

        $crate::define_custom_errors! { $( $rest )* }
    };

    (;) => {};

    () => {};
}
