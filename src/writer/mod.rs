//! CommonMark writer implementation.
//!
//! This module provides functionality to convert AST nodes to CommonMark format text.

mod common_mark;
mod processors;

pub use self::common_mark::CommonMarkWriter;
