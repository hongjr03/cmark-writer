#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

// AST related exports
pub use crate::ast::{CodeBlockType, HeadingType, HtmlAttribute, HtmlElement, ListItem, Node};

// Error types
pub use crate::error::{CodedError, StructureError, WriteError, WriteResult};

// Unified trait architecture
pub use crate::traits::{
    // Core traits
    CustomNode, NodeClone, NodeContent, Writer,
    // Formatting traits
    CommonMarkRenderable, Format, HtmlRenderable, MultiFormat, ToCommonMark, ToHtml,
    // Processing traits
    BlockNodeProcessor, ConfigurableProcessor, InlineNodeProcessor, NodeProcessor,
    // Utility traits
    ErrorContext, ErrorFactory,
};

// New processors
pub use crate::writer::processors::{
    BlockProcessorConfig, CustomNodeProcessor, EnhancedBlockProcessor, EnhancedInlineProcessor,
    InlineProcessorConfig,
};

// Options
pub use crate::options::{WriterOptions, WriterOptionsBuilder};

// CommonMark writer
pub use crate::writer::CommonMarkWriter;

// HTML writer related exports
pub use crate::writer::{HtmlWriteError, HtmlWriteResult, HtmlWriter, HtmlWriterOptions};

// Export proc-macro attributes and derive macros
pub use cmark_writer_macros::{coded_error, structure_error, CommonMarkOnly};

pub mod ast;
pub mod error;
pub mod options;
pub mod traits;
pub mod writer;

/// GitHub Flavored Markdown (GFM) extensions
///
/// This module is only available when the `gfm` feature is enabled.
#[cfg(feature = "gfm")]
pub mod gfm;
