//! Abstract Syntax Tree for CommonMark document structure.
//!
//! This module defines various node types for representing CommonMark documents,
//! including headings, paragraphs, lists, code blocks, etc.

/// Main node type, representing an element in a CommonMark document
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    /// Block-level node
    Block(BlockNode),
    /// Inline node
    Inline(InlineNode),
}

/// Block-level node type, representing content blocks that can exist independently
#[derive(Debug, Clone, PartialEq)]
pub enum BlockNode {
    /// Root document node, contains child block nodes
    Document(Vec<BlockNode>),

    /// Heading, contains level (1-6) and inline content
    Heading {
        /// Heading level, 1-6
        level: u8,
        /// Heading content, containing inline elements
        content: Vec<InlineNode>,
    },

    /// Paragraph node, containing inline elements
    Paragraph(Vec<InlineNode>),

    /// Block quote, containing any block-level elements
    BlockQuote(Vec<BlockNode>),

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
        headers: Vec<InlineNode>,
        /// Table rows, each row containing multiple cells
        rows: Vec<Vec<InlineNode>>,
        /// Column alignments
        alignments: Vec<Alignment>,
    },

    /// HTML block
    HtmlBlock(String),
}

/// Inline node type, representing inline elements used within block-level elements
#[derive(Debug, Clone, PartialEq)]
pub enum InlineNode {
    /// Plain text
    Text(String),

    /// Emphasis (italic)
    Emphasis(Vec<InlineNode>),

    /// Strong emphasis (bold)
    Strong(Vec<InlineNode>),

    /// Strikethrough
    Strike(Vec<InlineNode>),

    /// Inline code
    InlineCode(String),

    /// Link
    Link {
        /// Link URL
        url: String,
        /// Optional link title
        title: Option<String>,
        /// Link text
        content: Vec<InlineNode>,
    },

    /// Image
    Image {
        /// Image URL
        url: String,
        /// Optional image title
        title: Option<String>,
        /// Alternative text
        alt: String,
    },

    /// Inline element collection, without formatting and line breaks
    InlineContainer(Vec<InlineNode>),

    /// HTML inline element
    HtmlElement(HtmlElement),

    /// Soft break (single line break)
    SoftBreak,

    /// Hard break (two spaces followed by a line break, or backslash followed by a line break)
    HardBreak,
}

/// List item type
#[derive(Debug, Clone, PartialEq)]
pub enum ListItem {
    /// Regular list item
    Regular {
        /// List item content, containing one or more block-level elements
        content: Vec<BlockNode>,
    },
    /// Task list item
    Task {
        /// Whether the task is completed
        completed: bool,
        /// Task content
        content: Vec<BlockNode>,
    },
}

/// Table column alignment
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alignment {
    /// No specific alignment
    None,
    /// Left alignment
    Left,
    /// Center alignment
    Center,
    /// Right alignment
    Right,
}

/// Represents an HTML attribute, containing name and value
#[derive(Debug, Clone, PartialEq)]
pub struct HtmlAttribute {
    /// Attribute name
    pub name: String,
    /// Attribute value
    pub value: String,
}

/// Represents an HTML element, containing tag name, attributes, and child nodes
#[derive(Debug, Clone, PartialEq)]
pub struct HtmlElement {
    /// Element tag name
    pub tag: String,
    /// Element attributes
    pub attributes: Vec<HtmlAttribute>,
    /// Element child nodes (can only contain inline nodes)
    pub children: Vec<InlineNode>,
    /// Whether it's a self-closing tag (e.g. <img />)
    pub self_closing: bool,
}

// Provides backward compatibility conversion functions and trait implementations
impl BlockNode {
    /// Converts a BlockNode to Node
    pub fn into_node(self) -> Node {
        Node::Block(self)
    }
}

impl InlineNode {
    /// Converts an InlineNode to Node
    pub fn into_node(self) -> Node {
        Node::Inline(self)
    }
}
