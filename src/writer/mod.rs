//! CommonMark writer implementation.
//!
//! This module provides functionality to convert AST nodes to CommonMark format text.

mod cmark;
mod processors;

pub use self::cmark::CommonMarkWriter;
