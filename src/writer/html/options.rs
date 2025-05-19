/// Options for configuring the HTML rendering process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlRenderOptions {
    /// A prefix for the class name applied to fenced code blocks.
    /// For example, if set to "lang-", a Rust code block might get class "lang-rust".
    /// If None, no language class is added.
    pub code_block_language_class_prefix: Option<String>,

    /// Enables GFM-specific HTML rendering behaviors.
    #[cfg(feature = "gfm")]
    pub enable_gfm: bool,
    /// A list of HTML tags that should be rendered as text when GFM is enabled.
    #[cfg(feature = "gfm")]
    pub gfm_disallowed_html_tags: Vec<String>,
    
    /// Determines if HTML parsing/rendering errors should be strict (panic/Err) or lenient (warn and attempt to recover/textualize).
    pub strict: bool,
}

impl Default for HtmlRenderOptions {
    fn default() -> Self {
        Self {
            code_block_language_class_prefix: Some("language-".to_string()),
            #[cfg(feature = "gfm")]
            enable_gfm: false, // Default to false, cmark.rs options should override
            #[cfg(feature = "gfm")]
            gfm_disallowed_html_tags: Vec::new(), // Default to empty
            strict: true, // Default to strict for HTML, can be overridden by cmark.rs options
        }
    }
}
