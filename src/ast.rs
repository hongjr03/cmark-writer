//! Abstract Syntax Tree for CommonMark document structure.
//!
//! This module defines various node types for representing CommonMark documents,
//! including headings, paragraphs, lists, code blocks, etc.

/// Represents a node type in a CommonMark document
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    /// Root document node, containing child nodes
    Document(Vec<Node>),

    /// Heading, containing level (1-6) and content
    Heading {
        /// Heading level, 1-6
        level: u8,
        /// Heading content, containing inline elements
        content: Vec<Node>,
    },

    /// Paragraph node, containing inline elements
    Paragraph(Vec<Node>),

    /// Block quote, containing any block-level elements
    BlockQuote(Vec<Node>),

    /// Code block, containing optional language identifier and content
    CodeBlock {
        /// Optional language identifier
        language: Option<String>,
        /// Code content
        content: String,
    },

    /// Unordered list, containing list items
    UnorderedList(Vec<ListItem>),

    /// Ordered list, containing starting number and list items
    OrderedList {
        /// List starting number
        start: u32,
        /// List items
        items: Vec<ListItem>,
    },

    /// Thematic break (horizontal rule)
    ThematicBreak,

    /// Table
    Table {
        /// Header cells
        headers: Vec<Node>,
        /// Table rows, each containing multiple cells
        rows: Vec<Vec<Node>>,
        /// Column alignments
        alignments: Vec<Alignment>,
    },

    /// Link
    Link {
        /// Link URL
        url: String,
        /// Optional link title
        title: Option<String>,
        /// Link text content
        content: Vec<Node>,
    },

    /// Image
    Image {
        /// Image URL
        url: String,
        /// Optional image title
        title: Option<String>,
        /// Image alt text
        alt: String,
    },

    /// Emphasis (italic)
    Emphasis(Vec<Node>),

    /// Strong emphasis (bold)
    Strong(Vec<Node>),

    /// Strikethrough text
    Strike(Vec<Node>),

    /// Inline code
    InlineCode(String),

    /// Plain text
    Text(String),

    /// Inline container, content is written inline without any formatting or line breaks
    Inline(Vec<Node>),

    /// HTML block
    Html(String),

    /// Custom HTML element with attributes and children
    HtmlElement(HtmlElement),

    /// Soft line break (single newline)
    SoftBreak,

    /// Hard line break (two spaces followed by newline or backslash followed by newline)
    HardBreak,
}

/// Represents a list item
#[derive(Debug, Clone, PartialEq)]
pub struct ListItem {
    /// List item content, containing one or more block-level elements
    pub content: Vec<Node>,
    /// Whether this is a task list item
    pub is_task: bool,
    /// Whether the task is completed
    pub task_completed: bool,
}

/// Table column alignment
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alignment {
    /// No specified alignment
    None,
    /// Left alignment
    Left,
    /// Center alignment
    Center,
    /// Right alignment
    Right,
}

/// Represents an HTML attribute with name and value
#[derive(Debug, Clone, PartialEq)]
pub struct HtmlAttribute {
    /// Attribute name
    pub name: String,
    /// Attribute value
    pub value: String,
}

/// Represents an HTML element with tag name, attributes and child nodes
#[derive(Debug, Clone, PartialEq)]
pub struct HtmlElement {
    /// Element tag name
    pub tag: String,
    /// Element attributes
    pub attributes: Vec<HtmlAttribute>,
    /// Element children
    pub children: Vec<Node>,
    /// Whether this is a self-closing tag (e.g., <img />)
    pub self_closing: bool,
}
