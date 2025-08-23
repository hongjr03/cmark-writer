//! CommonMark writer implementation.
//!
//! This module provides functionality to convert AST nodes to various formats.

pub mod cmark;
pub mod context;
pub mod processors;

pub use self::cmark::CommonMarkWriter;
pub use self::context::{NewlineContext, NewlineStrategy, RenderingMode};

/// HTML specific modules are now grouped under writer::html
pub mod html;
pub use self::html::{HtmlWriteError, HtmlWriteResult, HtmlWriter, HtmlWriterOptions};
