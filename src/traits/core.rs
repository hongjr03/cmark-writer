//! Core node content and structure traits

use crate::error::{WriteError, WriteResult};
use std::any::Any;

/// Core node content trait - focused on basic properties
pub trait NodeContent: std::fmt::Debug + Send + Sync {
    /// Check if the node is a block-level element
    fn is_block(&self) -> bool;

    /// Get type name for pattern matching
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    /// Cast to Any for type conversion
    fn as_any(&self) -> &dyn Any;

    /// Cast to mutable Any for type conversion
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Node cloning and equality trait
pub trait NodeClone: NodeContent {
    /// Clone node into a Box
    fn clone_box(&self) -> Box<dyn NodeContent>;

    /// Check equality with another node
    fn eq_box(&self, other: &dyn NodeContent) -> bool;
}

/// Custom node trait - now dyn compatible
pub trait CustomNode: NodeClone + super::formatting::CommonMarkRenderable {
    /// Default HTML rendering implementation
    fn html_render(&self, writer: &mut crate::writer::HtmlWriter) -> WriteResult<()> {
        // Use HtmlWriter's raw_html method
        writer
            .raw_html(&format!(
                "<!-- HTML rendering not implemented for {} -->",
                self.type_name()
            ))
            .map_err(WriteError::from)
    }

    /// Get custom attributes
    fn attributes(&self) -> Option<&std::collections::HashMap<String, String>> {
        None
    }

    /// Check if specific capability is supported
    fn supports_capability(&self, capability: &str) -> bool {
        match capability {
            "commonmark" => true,
            "html" => false,
            _ => false,
        }
    }
}

/// Output writer trait - simplified design for dyn compatibility
pub trait Writer {
    /// Write a string
    fn write_str(&mut self, s: &str) -> WriteResult<()>;

    /// Write a character
    fn write_char(&mut self, c: char) -> WriteResult<()>;

    /// Write formatted content
    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> WriteResult<()> {
        self.write_str(&args.to_string())
    }
}

// Implement Writer trait for CommonMarkWriter
impl Writer for crate::writer::CommonMarkWriter {
    fn write_str(&mut self, s: &str) -> WriteResult<()> {
        self.write_str(s)
    }

    fn write_char(&mut self, c: char) -> WriteResult<()> {
        self.write_char(c)
    }
}

// Implement Writer trait for HtmlWriter
impl Writer for crate::writer::HtmlWriter {
    fn write_str(&mut self, s: &str) -> WriteResult<()> {
        self.write_str(s).map_err(WriteError::from)
    }

    fn write_char(&mut self, c: char) -> WriteResult<()> {
        self.write_str(&c.to_string()).map_err(WriteError::from)
    }
}
