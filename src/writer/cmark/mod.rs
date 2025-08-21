//! CommonMark writer implementation modules.
//!
//! This module contains the CommonMark writer split into logical components:
//! - `writer`: Main writer struct and core functionality
//! - `block`: Block-level element writing
//! - `inline`: Inline element writing  
//! - `table`: Table-specific writing
//! - `utils`: Utility functions and escaping
//! - `html_fallback`: HTML fallback handling

mod block;
mod html_fallback;
mod inline;
mod table;
mod utils;
mod writer;

pub use utils::{escape_str, CommonMarkEscapes, Escapes};
pub use writer::CommonMarkWriter;
