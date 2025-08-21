//! Core trait definitions for cmark-writer
//!
//! This module provides a well-organized trait hierarchy following SOLID principles
//! with clear separation of concerns.

// Re-export all public traits
pub use self::core::*;
pub use self::formatting::*;
pub use self::processing::*;
pub use self::utils::*;

/// Core node and content traits
pub mod core;

/// Format and rendering traits
pub mod formatting;

/// Node processing traits
pub mod processing;

/// Utility traits for error handling and configuration
pub mod utils;
