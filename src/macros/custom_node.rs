//! Custom node macro definitions
//!
//! This module provides a macro to simplify the implementation of custom CommonMark nodes.

/// Automatically implement the CustomNode trait for custom node types
///
/// This macro automatically implements all necessary CustomNode trait methods. Developers only need to implement
/// the `write_custom` and `is_block_custom` methods.
///
/// # Requirements
///
/// Types using this macro must implement the following traits:
/// - `Debug`
/// - `Clone`
/// - `PartialEq`
///
/// And must implement the following methods:
/// - `fn write_custom(&self, writer: &mut dyn CustomNodeWriter) -> cmark_writer::error::WriteResult<()>`
/// - `fn is_block_custom(&self) -> bool`
///
/// # Example
///
/// ```rust
/// use cmark_writer::ast::{CustomNodeWriter};
/// use cmark_writer::error::WriteResult;
/// use cmark_writer::derive_custom_node;
///
/// #[derive(Debug, Clone, PartialEq)]
/// struct HighlightNode {
///     content: String,
///     color: String,
/// }
///
/// // Use the macro to automatically implement the CustomNode trait
/// derive_custom_node!(HighlightNode);
///
/// // Implement the necessary methods
/// impl HighlightNode {
///     // Implement the custom node's writing logic
///     fn write_custom(&self, writer: &mut dyn CustomNodeWriter) -> WriteResult<()> {
///         writer.write_str("<span style=\"background-color: ")?;
///         writer.write_str(&self.color)?;
///         writer.write_str("\">")?;
///         writer.write_str(&self.content)?;
///         writer.write_str("</span>")?;
///         Ok(())
///     }
///     
///     // Specify whether the node is a block-level node or an inline node
///     fn is_block_custom(&self) -> bool {
///         false // This is an inline element
///     }
/// }
/// ```
///
/// # Generated code
///
/// For the above example, the macro will generate the following code:
///
/// ```rust,ignore
/// impl CustomNode for HighlightNode {
///     fn write(&self, writer: &mut dyn CustomNodeWriter) -> WriteResult<()> {
///         self.write_custom(writer)
///     }
///
///     fn clone_box(&self) -> Box<dyn CustomNode> {
///         Box::new(self.clone())
///     }
///
///     fn eq_box(&self, other: &dyn CustomNode) -> bool {
///         if let Some(other) = other.as_any().downcast_ref::<Self>() {
///             self == other
///         } else {
///             false
///         }
///     }
///
///     fn is_block(&self) -> bool {
///         self.is_block_custom()
///     }
///
///     fn as_any(&self) -> &dyn Any {
///         self
///     }
/// }
/// ```
#[macro_export]
macro_rules! derive_custom_node {
    ($type:ty) => {
        impl $crate::ast::CustomNode for $type {
            fn write(
                &self,
                writer: &mut dyn $crate::ast::CustomNodeWriter,
            ) -> $crate::error::WriteResult<()> {
                self.write_custom(writer)
            }

            fn clone_box(&self) -> Box<dyn $crate::ast::CustomNode> {
                Box::new(self.clone())
            }

            fn eq_box(&self, other: &dyn $crate::ast::CustomNode) -> bool {
                if let Some(other) = other.as_any().downcast_ref::<Self>() {
                    self == other
                } else {
                    false
                }
            }

            fn is_block(&self) -> bool {
                self.is_block_custom()
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
}
