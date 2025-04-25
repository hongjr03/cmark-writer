//! CommonMark formatting options.
//!
//! This module provides configuration options for the CommonMark writer.

/// CommonMark formatting options
#[derive(Debug, Clone)]
pub struct WriterOptions {
    /// Whether to enable strict mode (strictly following CommonMark specification)
    pub strict: bool,
    /// Hard break mode (true uses two spaces followed by a newline, false uses backslash followed by a newline)
    pub hard_break_spaces: bool,
    /// Number of spaces to use for indentation levels
    pub indent_spaces: usize,
}

impl Default for WriterOptions {
    fn default() -> Self {
        Self {
            strict: true,
            hard_break_spaces: false,
            indent_spaces: 4,
        }
    }
}
