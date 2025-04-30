#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub use crate::ast::{
    CodeBlockType, CustomNode, CustomNodeWriter, HeadingType, HtmlAttribute, HtmlElement, ListItem,
    Node,
};
pub use crate::error::{
    CodedError, CustomErrorFactory, StructureError, WriteError, WriteResult, WriteResultExt,
};
pub use crate::options::WriterOptions;
pub use crate::writer::CommonMarkWriter;

// Export proc-macro attributes
pub use cmark_writer_macros::{coded_error, custom_node, structure_error};

pub mod ast;
pub mod error;
pub mod options;
pub mod writer;

/// GitHub Flavored Markdown (GFM) extensions
///
/// This module is only available when the `gfm` feature is enabled.
#[cfg(feature = "gfm")]
pub mod gfm;
