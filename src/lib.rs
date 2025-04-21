//! # cmark-rs
//!
//! `cmark-rs` is a Rust library for parsing and writing CommonMark format.
//!
//! This library provides functionality to serialize in-memory data structures to CommonMark compliant text.
//!
//! ## Example
//!
//! ```rust
//! use cmark_rs::ast::{Node, ListItem};
//! use cmark_rs::writer::CommonMarkWriter;
//!
//! // Create a simple document
//! let document = Node::Document(vec![
//!     Node::Heading {
//!         level: 1,
//!         content: vec![Node::Text("Hello CommonMark".to_string())],
//!     },
//!     Node::Paragraph(vec![
//!         Node::Text("This is a simple ".to_string()),
//!         Node::Strong(vec![Node::Text("example".to_string())]),
//!         Node::Text(".".to_string()),
//!     ]),
//! ]);
//!
//! // Write the document as CommonMark text
//! let mut writer = CommonMarkWriter::new();
//! writer.write(&document).expect("Failed to write document");
//! let markdown = writer.into_string();
//!
//! println!("{}", markdown);
//! ```

// Re-export main public API components so users can access them directly via `cmark_rs::Node`
pub use crate::ast::{Alignment, ListItem, Node};
pub use crate::writer::{CommonMarkWriter, WriterOptions};

// Export public modules to allow users to access more advanced functionality
pub mod ast;
pub mod writer;
