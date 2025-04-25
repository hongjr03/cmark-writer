//! # cmark-writer
//!
//! `cmark-writer` is a Rust library for writing CommonMark format.
//!
//! This library provides functionality to serialize in-memory data structures to CommonMark compliant text.
//!
//! ## Example
//!
//! ```rust
//! use cmark_writer::ast::{Node, BlockNode, InlineNode, ListItem};
//! use cmark_writer::writer::CommonMarkWriter;
//!
//! // Create a simple document
//! let document = BlockNode::Document(vec![
//!     BlockNode::Heading {
//!         level: 1,
//!         content: vec![InlineNode::Text("Hello CommonMark".to_string())],
//!     },
//!     BlockNode::Paragraph(vec![
//!         InlineNode::Text("This is a simple ".to_string()),
//!         InlineNode::Strong(vec![InlineNode::Text("example".to_string())]),
//!         InlineNode::Text(".".to_string()),
//!     ]),
//! ]).into_node();
//!
//! // Write the document as CommonMark text
//! let mut writer = CommonMarkWriter::new();
//! writer.write(&document).expect("Failed to write document");
//! let markdown = writer.into_string();
//!
//! println!("{}", markdown);
//! ```

// Re-export main public API components so users can access them directly via `cmark_writer::Node`
pub use crate::ast::{
    Alignment, BlockNode, HtmlAttribute, HtmlElement, InlineNode, ListItem, Node,
};
pub use crate::error::{WriteError, WriteResult};
pub use crate::options::WriterOptions;
pub use crate::writer::CommonMarkWriter;

// Export public modules to allow users to access more advanced functionality
pub mod ast;
pub mod error;
pub mod options;
pub mod writer;
