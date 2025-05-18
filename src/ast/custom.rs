//! Custom node definitions for the CommonMark AST.

use crate::error::WriteResult;
use crate::writer::{HtmlRenderOptions, HtmlWriteResult};
use std::any::Any;

/// Trait for implementing custom node behavior
pub trait CustomNode: std::fmt::Debug + Send + Sync {
    /// Write the custom node content to the provided writer (for CommonMark output)
    fn write(&self, writer: &mut dyn CustomNodeWriter) -> WriteResult<()>;

    /// Returns the HTML representation of the custom node as a string, using specified options.
    ///
    /// By default, this returns an HTML comment indicating that HTML rendering is not implemented
    /// for this custom node type.
    fn to_html_string(&self, options: &HtmlRenderOptions) -> HtmlWriteResult<String> {
        let _ = options;
        Ok(format!(
            "<!-- HTML rendering not implemented for Custom Node: {} -->",
            self.type_name()
        ))
    }

    /// Clone the custom node
    fn clone_box(&self) -> Box<dyn CustomNode>;

    /// Check if two custom nodes are equal
    fn eq_box(&self, other: &dyn CustomNode) -> bool;

    /// Whether the custom node is a block element
    fn is_block(&self) -> bool;

    /// Convert to Any for type casting
    fn as_any(&self) -> &dyn Any;

    /// Convert to mutable Any for type casting
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Get the type name of the custom node for pattern matching
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

/// Trait for custom node writer implementation
pub trait CustomNodeWriter {
    /// Write a string to the output
    fn write_str(&mut self, s: &str) -> std::fmt::Result;

    /// Write a character to the output
    fn write_char(&mut self, c: char) -> std::fmt::Result;
}

// Implement Clone for Box<dyn CustomNode>
impl Clone for Box<dyn CustomNode> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

// Implement PartialEq for Box<dyn CustomNode>
impl PartialEq for Box<dyn CustomNode> {
    fn eq(&self, other: &Self) -> bool {
        self.eq_box(&**other)
    }
}
