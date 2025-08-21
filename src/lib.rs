#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

// AST related exports
pub use crate::ast::{CodeBlockType, HeadingType, HtmlAttribute, HtmlElement, ListItem, Node};

// Error types
pub use crate::error::{CodedError, StructureError, WriteError, WriteResult};

// New trait-based architecture
pub use crate::traits::{
    BlockNodeProcessor, CommonMarkRenderable, ConfigurableProcessor, CustomNode, ErrorContext,
    ErrorFactory, HtmlRenderable, InlineNodeProcessor, NodeClone, NodeContent, NodeProcessor,
    Writer,
};

// Format traits for better custom node design
pub use crate::format_traits::{
    default_html_render, CommonMarkFormat, Format, HtmlFormat, MultiFormat, ToCommonMark, ToHtml,
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

// Export proc-macro attributes (retain only error-related macros)
pub use cmark_writer_macros::{coded_error, structure_error};

pub mod ast;
pub mod error;
pub mod format_traits;
pub mod options;
pub mod traits;
pub mod writer;

/// GitHub Flavored Markdown (GFM) extensions
///
/// This module is only available when the `gfm` feature is enabled.
#[cfg(feature = "gfm")]
pub mod gfm;
