//! Node definitions for the CommonMark AST.

use super::custom::CustomNode;
use super::html::HtmlElement;
use std::boxed::Box;

/// Code block type according to CommonMark specification
#[derive(Debug, Clone, PartialEq, Default)]
pub enum CodeBlockType {
    /// Indented code block - composed of one or more indented chunks, each preceded by four or more spaces
    Indented,
    /// Fenced code block - surrounded by backtick or tilde fences
    #[default]
    Fenced,
}

/// Heading type according to CommonMark specification
#[derive(Debug, Clone, PartialEq, Default)]
pub enum HeadingType {
    /// ATX Type - Beginning with #
    #[default]
    Atx,
    /// Setext Type - Underlined or overlined text
    Setext,
}

/// Main node type, representing an element in a CommonMark document
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    /// Root document node, contains child nodes
    Document(Vec<Node>),

    // Leaf blocks
    // Thematic breaks
    /// Thematic break (horizontal rule)
    ThematicBreak,

    // ATX headings & Setext headings
    /// Heading, contains level (1-6) and inline content
    Heading {
        /// Heading level, 1-6
        level: u8,
        /// Heading content, containing inline elements
        content: Vec<Node>,
        /// Heading type (ATX or Setext)
        heading_type: HeadingType,
    },

    // Indented code blocks & Fenced code blocks
    /// Code block, containing optional language identifier and content
    CodeBlock {
        /// Optional language identifier (None for indented code blocks, Some for fenced code blocks)
        language: Option<String>,
        /// Code content
        content: String,
        /// The type of code block (Indented or Fenced)
        block_type: CodeBlockType,
    },

    // HTML blocks
    /// HTML block
    HtmlBlock(String),

    // Link reference definitions
    /// Link reference definition
    LinkReferenceDefinition {
        /// Link label (used for reference)
        label: String,
        /// Link destination URL
        destination: String,
        /// Optional link title
        title: Option<String>,
    },

    // Paragraphs
    /// Paragraph node, containing inline elements
    Paragraph(Vec<Node>),

    // Blank lines - typically handled during parsing, not represented in AST

    // Container blocks
    // Block quotes
    /// Block quote, containing any block-level elements
    BlockQuote(Vec<Node>),

    // & List items and Lists
    /// Ordered list, containing starting number and list items
    OrderedList {
        /// List starting number
        start: u32,
        /// List items
        items: Vec<ListItem>,
    },

    /// Unordered list, containing list items
    UnorderedList(Vec<ListItem>),

    /// Table (extension to CommonMark)
    Table {
        /// Header cells
        headers: Vec<Node>,
        /// Table rows, each row containing multiple cells
        rows: Vec<Vec<Node>>,
    },

    // Inlines
    // Code spans
    /// Inline code
    InlineCode(String),

    // Emphasis and strong emphasis
    /// Emphasis (italic)
    Emphasis(Vec<Node>),

    /// Strong emphasis (bold)
    Strong(Vec<Node>),

    // Links
    /// Link
    Link {
        /// Link URL
        url: String,
        /// Optional link title
        title: Option<String>,
        /// Link text
        content: Vec<Node>,
    },

    /// Reference link
    ReferenceLink {
        /// Link reference label
        label: String,
        /// Link text content (optional, if empty it's a shortcut reference)
        content: Vec<Node>,
    },

    // Images
    /// Image
    Image {
        /// Image URL
        url: String,
        /// Optional image title
        title: Option<String>,
        /// Alternative text, containing inline elements
        alt: Vec<Node>,
    },

    // Autolinks
    /// Autolink (URI or email wrapped in < and >)
    Autolink {
        /// Link URL
        url: String,
        /// Whether this is an email autolink
        is_email: bool,
    },

    // Raw HTML
    /// HTML inline element
    HtmlElement(HtmlElement),

    // Hard line breaks
    /// Hard break (two spaces followed by a line break, or backslash followed by a line break)
    HardBreak,

    // Soft line breaks
    /// Soft break (single line break)
    SoftBreak,

    // Textual content
    /// Plain text
    Text(String),

    /// Custom node that allows users to implement their own writing behavior
    Custom(Box<dyn CustomNode>),
}

impl Default for Node {
    fn default() -> Self {
        Node::Document(vec![])
    }
}

/// List item type
#[derive(Debug, Clone, PartialEq)]
pub enum ListItem {
    /// Unordered list item
    Unordered {
        /// List item content, containing one or more block-level elements
        content: Vec<Node>,
    },
    /// Ordered list item
    Ordered {
        /// Optional item number for ordered lists, allowing manual numbering
        number: Option<u32>,
        /// List item content, containing one or more block-level elements
        content: Vec<Node>,
    },
}

impl Node {
    /// Check if a node is a block-level node
    pub fn is_block(&self) -> bool {
        matches!(
            self,
            Node::Document(_)
                // Leaf blocks
                | Node::ThematicBreak
                | Node::Heading { .. }
                | Node::CodeBlock { .. }
                | Node::HtmlBlock(_)
                | Node::LinkReferenceDefinition { .. }
                | Node::Paragraph(_)
                // Container blocks
                | Node::BlockQuote(_)
                | Node::OrderedList { .. }
                | Node::UnorderedList(_)
                | Node::Table { .. }

                | Node::Custom(_)
        )
    }

    /// Check if a node is an inline node
    pub fn is_inline(&self) -> bool {
        matches!(
            self,
            // Inlines
            // Code spans
            Node::InlineCode(_)
                // Emphasis and strong emphasis
                | Node::Emphasis(_)
                | Node::Strong(_)
                // Links
                | Node::Link { .. }
                | Node::ReferenceLink { .. }
                // Images
                | Node::Image { .. }
                // Autolinks
                | Node::Autolink { .. }
                // Raw HTML
                | Node::HtmlElement(_)
                // Hard line breaks
                | Node::HardBreak
                // Soft line breaks
                | Node::SoftBreak
                // Textual content
                | Node::Text(_)

                | Node::Custom(_)
        )
    }
    /// Create a heading node
    ///
    /// # Arguments
    /// * `level` - Heading level (1-6)
    /// * `content` - Heading content
    ///
    /// # Returns
    /// A new heading node, default ATX type
    pub fn heading(level: u8, content: Vec<Node>) -> Self {
        Node::Heading {
            level,
            content,
            heading_type: HeadingType::default(),
        }
    }

    /// Create a code block node
    ///
    /// # Arguments
    /// * `language` - Optional language identifier
    /// * `content` - Code content
    ///
    /// # Returns
    /// A new code block node, default Fenced type
    pub fn code_block(language: Option<String>, content: String) -> Self {
        Node::CodeBlock {
            language,
            content,
            block_type: CodeBlockType::default(),
        }
    }
}
