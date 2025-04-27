//! Node definitions for the CommonMark AST.

use super::custom::CustomNode;
use super::html::HtmlElement;
use std::boxed::Box;

/// Main node type, representing an element in a CommonMark document
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    // Block-level nodes
    /// Root document node, contains child nodes
    Document(Vec<Node>),

    /// Heading, contains level (1-6) and inline content
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
        /// Table rows, each row containing multiple cells
        rows: Vec<Vec<Node>>,
        /// Column alignments
        alignments: Vec<Alignment>,
    },

    /// HTML block
    HtmlBlock(String),

    // Inline nodes
    /// Plain text
    Text(String),

    /// Emphasis (italic)
    Emphasis(Vec<Node>),

    /// Strong emphasis (bold)
    Strong(Vec<Node>),

    /// Strikethrough
    Strike(Vec<Node>),

    /// Inline code
    InlineCode(String),

    /// Link
    Link {
        /// Link URL
        url: String,
        /// Optional link title
        title: Option<String>,
        /// Link text
        content: Vec<Node>,
    },

    /// Image
    Image {
        /// Image URL
        url: String,
        /// Optional image title
        title: Option<String>,
        /// Alternative text, containing inline elements
        alt: Vec<Node>,
    },

    /// Inline element collection, without formatting and line breaks
    InlineContainer(Vec<Node>),

    /// HTML inline element
    HtmlElement(HtmlElement),

    /// Soft break (single line break)
    SoftBreak,

    /// Hard break (two spaces followed by a line break, or backslash followed by a line break)
    HardBreak,

    /// Custom node that allows users to implement their own writing behavior
    Custom(Box<dyn CustomNode>),
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

impl Node {
    /// Check if a node is a block-level node
    pub fn is_block(&self) -> bool {
        matches!(
            self,
            Node::Document(_)
                | Node::Heading { .. }
                | Node::Paragraph(_)
                | Node::BlockQuote(_)
                | Node::CodeBlock { .. }
                | Node::UnorderedList(_)
                | Node::OrderedList { .. }
                | Node::ThematicBreak
                | Node::Table { .. }
                | Node::HtmlBlock(_)
                | Node::Custom(_)
        )
    }

    /// Check if a node is an inline node
    pub fn is_inline(&self) -> bool {
        matches!(
            self,
            Node::Text(_)
                | Node::Emphasis(_)
                | Node::Strong(_)
                | Node::Strike(_)
                | Node::InlineCode(_)
                | Node::Link { .. }
                | Node::Image { .. }
                | Node::InlineContainer(_)
                | Node::HtmlElement(_)
                | Node::SoftBreak
                | Node::HardBreak
                | Node::Custom(_)
        )
    }
}
