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

    /// Whether to enable GitHub Flavored Markdown (GFM) extensions
    #[cfg(feature = "gfm")]
    pub enable_gfm: bool,

    /// Whether to enable GFM strikethrough syntax
    #[cfg(feature = "gfm")]
    pub gfm_strikethrough: bool,

    /// Whether to enable GFM task lists
    #[cfg(feature = "gfm")]
    pub gfm_tasklists: bool,

    /// Whether to enable GFM tables with alignment
    #[cfg(feature = "gfm")]
    pub gfm_tables: bool,

    /// Whether to enable GFM autolinks without angle brackets
    #[cfg(feature = "gfm")]
    pub gfm_autolinks: bool,

    /// List of disallowed HTML tag names in GFM mode
    #[cfg(feature = "gfm")]
    pub gfm_disallowed_html_tags: Vec<String>,
}

impl Default for WriterOptions {
    fn default() -> Self {
        Self {
            strict: true,
            hard_break_spaces: false,
            indent_spaces: 4,

            #[cfg(feature = "gfm")]
            enable_gfm: false,

            #[cfg(feature = "gfm")]
            gfm_strikethrough: false,

            #[cfg(feature = "gfm")]
            gfm_tasklists: false,

            #[cfg(feature = "gfm")]
            gfm_tables: false,

            #[cfg(feature = "gfm")]
            gfm_autolinks: false,

            #[cfg(feature = "gfm")]
            gfm_disallowed_html_tags: vec![
                "title".to_string(),
                "textarea".to_string(),
                "style".to_string(),
                "xmp".to_string(),
                "iframe".to_string(),
                "noembed".to_string(),
                "noframes".to_string(),
                "script".to_string(),
                "plaintext".to_string(),
            ],
        }
    }
}

/// Builder for WriterOptions
pub struct WriterOptionsBuilder {
    options: WriterOptions,
}

impl WriterOptionsBuilder {
    /// Create a new WriterOptionsBuilder with default options
    pub fn new() -> Self {
        Self {
            options: WriterOptions::default(),
        }
    }

    /// Set strict mode (whether to strictly follow CommonMark specification)
    pub fn strict(mut self, strict: bool) -> Self {
        self.options.strict = strict;
        self
    }

    /// Set hard break mode (true uses two spaces followed by a newline, false uses backslash)
    pub fn hard_break_spaces(mut self, hard_break_spaces: bool) -> Self {
        self.options.hard_break_spaces = hard_break_spaces;
        self
    }

    /// Set number of spaces for indentation
    pub fn indent_spaces(mut self, indent_spaces: usize) -> Self {
        self.options.indent_spaces = indent_spaces;
        self
    }

    /// Enable all GitHub Flavored Markdown (GFM) extensions
    #[cfg(feature = "gfm")]
    pub fn enable_gfm(mut self) -> Self {
        self.options.enable_gfm = true;
        self.options.gfm_strikethrough = true;
        self.options.gfm_tasklists = true;
        self.options.gfm_tables = true;
        self.options.gfm_autolinks = true;
        self
    }

    /// Enable or disable GFM strikethrough syntax
    #[cfg(feature = "gfm")]
    pub fn gfm_strikethrough(mut self, enable: bool) -> Self {
        self.options.gfm_strikethrough = enable;
        if enable {
            self.options.enable_gfm = true;
        }
        self
    }

    /// Enable or disable GFM task lists
    #[cfg(feature = "gfm")]
    pub fn gfm_tasklists(mut self, enable: bool) -> Self {
        self.options.gfm_tasklists = enable;
        if enable {
            self.options.enable_gfm = true;
        }
        self
    }

    /// Enable or disable GFM tables with alignment
    #[cfg(feature = "gfm")]
    pub fn gfm_tables(mut self, enable: bool) -> Self {
        self.options.gfm_tables = enable;
        if enable {
            self.options.enable_gfm = true;
        }
        self
    }

    /// Enable or disable GFM autolinks without angle brackets
    #[cfg(feature = "gfm")]
    pub fn gfm_autolinks(mut self, enable: bool) -> Self {
        self.options.gfm_autolinks = enable;
        if enable {
            self.options.enable_gfm = true;
        }
        self
    }

    /// Set list of disallowed HTML tags in GFM mode
    #[cfg(feature = "gfm")]
    pub fn gfm_disallowed_html_tags(mut self, tags: Vec<String>) -> Self {
        self.options.gfm_disallowed_html_tags = tags;
        self
    }

    /// Build the WriterOptions
    pub fn build(self) -> WriterOptions {
        self.options
    }
}

impl Default for WriterOptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
