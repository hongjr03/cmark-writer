//! Node definitions for the CommonMark AST.

use super::html::HtmlElement;
use crate::traits::CustomNode;
use ecow::EcoString;
use std::boxed::Box;

/// Code block type according to CommonMark specification
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum CodeBlockType {
    /// Indented code block - composed of one or more indented chunks, each preceded by four or more spaces
    Indented,
    /// Fenced code block - surrounded by backtick or tilde fences
    #[default]
    Fenced,
}

/// Heading type according to CommonMark specification
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum HeadingType {
    /// ATX Type - Beginning with #
    #[default]
    Atx,
    /// Setext Type - Underlined or overlined text
    Setext,
}

/// Table column alignment options for GFM tables
#[cfg(feature = "gfm")]
#[derive(Debug, Clone, PartialEq, Default)]
pub enum TableAlignment {
    /// Left alignment (default)
    #[default]
    Left,
    /// Center alignment
    Center,
    /// Right alignment
    Right,
    /// No specific alignment specified
    None,
}

/// Task list item status for GFM task lists
#[cfg(feature = "gfm")]
#[derive(Debug, Clone, PartialEq)]
pub enum TaskListStatus {
    /// Checked/completed task
    Checked,
    /// Unchecked/incomplete task
    Unchecked,
}

/// Main node type, representing an element in a CommonMark document
#[derive(Debug)]
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
        language: Option<EcoString>,
        /// Code content
        content: EcoString,
        /// The type of code block (Indented or Fenced)
        block_type: CodeBlockType,
    },

    // HTML blocks
    /// HTML block
    HtmlBlock(EcoString),

    // Link reference definitions
    /// Link reference definition
    LinkReferenceDefinition {
        /// Link label (used for reference)
        label: EcoString,
        /// Link destination URL
        destination: EcoString,
        /// Optional link title
        title: Option<EcoString>,
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
        /// Column alignments for the table
        #[cfg(feature = "gfm")]
        alignments: Vec<TableAlignment>,
        /// Table rows, each row containing multiple cells
        rows: Vec<Vec<Node>>,
    },

    // Inlines
    // Code spans
    /// Inline code
    InlineCode(EcoString),

    // Emphasis and strong emphasis
    /// Emphasis (italic)
    Emphasis(Vec<Node>),

    /// Strong emphasis (bold)
    Strong(Vec<Node>),

    /// Strikethrough (GFM extension)
    Strikethrough(Vec<Node>),

    // Links
    /// Link
    Link {
        /// Link URL
        url: EcoString,
        /// Optional link title
        title: Option<EcoString>,
        /// Link text
        content: Vec<Node>,
    },

    /// Reference link
    ReferenceLink {
        /// Link reference label
        label: EcoString,
        /// Link text content (optional, if empty it's a shortcut reference)
        content: Vec<Node>,
    },

    // Images
    /// Image
    Image {
        /// Image URL
        url: EcoString,
        /// Optional image title
        title: Option<EcoString>,
        /// Alternative text, containing inline elements
        alt: Vec<Node>,
    },

    // Autolinks
    /// Autolink (URI or email wrapped in < and >)
    Autolink {
        /// Link URL
        url: EcoString,
        /// Whether this is an email autolink
        is_email: bool,
    },

    /// GFM Extended Autolink (without angle brackets, automatically detected)
    ExtendedAutolink(EcoString),

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
    Text(EcoString),

    /// Custom node that allows users to implement their own writing behavior
    Custom(Box<dyn CustomNode>),
}

impl Default for Node {
    fn default() -> Self {
        Node::Document(vec![])
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        match self {
            Node::Document(nodes) => Node::Document(nodes.clone()),
            Node::ThematicBreak => Node::ThematicBreak,
            Node::Heading {
                level,
                content,
                heading_type,
            } => Node::Heading {
                level: *level,
                content: content.clone(),
                heading_type: *heading_type,
            },
            Node::CodeBlock {
                language,
                content,
                block_type,
            } => Node::CodeBlock {
                language: language.clone(),
                content: content.clone(),
                block_type: *block_type,
            },
            Node::HtmlBlock(html) => Node::HtmlBlock(html.clone()),
            Node::LinkReferenceDefinition {
                label,
                destination,
                title,
            } => Node::LinkReferenceDefinition {
                label: label.clone(),
                destination: destination.clone(),
                title: title.clone(),
            },
            Node::Paragraph(content) => Node::Paragraph(content.clone()),
            Node::BlockQuote(content) => Node::BlockQuote(content.clone()),
            Node::OrderedList { start, items } => Node::OrderedList {
                start: *start,
                items: items.clone(),
            },
            Node::UnorderedList(items) => Node::UnorderedList(items.clone()),
            #[cfg(feature = "gfm")]
            Node::Table {
                headers,
                alignments,
                rows,
            } => Node::Table {
                headers: headers.clone(),
                alignments: alignments.clone(),
                rows: rows.clone(),
            },
            #[cfg(not(feature = "gfm"))]
            Node::Table { headers, rows } => Node::Table {
                headers: headers.clone(),
                rows: rows.clone(),
            },
            Node::InlineCode(code) => Node::InlineCode(code.clone()),
            Node::Emphasis(content) => Node::Emphasis(content.clone()),
            Node::Strong(content) => Node::Strong(content.clone()),
            Node::Strikethrough(content) => Node::Strikethrough(content.clone()),
            Node::Link {
                url,
                title,
                content,
            } => Node::Link {
                url: url.clone(),
                title: title.clone(),
                content: content.clone(),
            },
            Node::ReferenceLink { label, content } => Node::ReferenceLink {
                label: label.clone(),
                content: content.clone(),
            },
            Node::Image { url, title, alt } => Node::Image {
                url: url.clone(),
                title: title.clone(),
                alt: alt.clone(),
            },
            Node::Autolink { url, is_email } => Node::Autolink {
                url: url.clone(),
                is_email: *is_email,
            },
            Node::ExtendedAutolink(url) => Node::ExtendedAutolink(url.clone()),
            Node::HtmlElement(element) => Node::HtmlElement(element.clone()),
            Node::HardBreak => Node::HardBreak,
            Node::SoftBreak => Node::SoftBreak,
            Node::Text(text) => Node::Text(text.clone()),
            Node::Custom(_custom) => {
                // 暂时不支持自定义节点的克隆，因为我们简化了设计
                // 用户应该使用 Format trait 而不是直接使用 Custom 节点
                panic!("Custom node cloning not supported in simplified design")
            }
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Node::Document(a), Node::Document(b)) => a == b,
            (Node::ThematicBreak, Node::ThematicBreak) => true,
            (
                Node::Heading {
                    level: l1,
                    content: c1,
                    heading_type: h1,
                },
                Node::Heading {
                    level: l2,
                    content: c2,
                    heading_type: h2,
                },
            ) => l1 == l2 && c1 == c2 && h1 == h2,
            (
                Node::CodeBlock {
                    language: l1,
                    content: c1,
                    block_type: b1,
                },
                Node::CodeBlock {
                    language: l2,
                    content: c2,
                    block_type: b2,
                },
            ) => l1 == l2 && c1 == c2 && b1 == b2,
            (Node::HtmlBlock(a), Node::HtmlBlock(b)) => a == b,
            (
                Node::LinkReferenceDefinition {
                    label: l1,
                    destination: d1,
                    title: t1,
                },
                Node::LinkReferenceDefinition {
                    label: l2,
                    destination: d2,
                    title: t2,
                },
            ) => l1 == l2 && d1 == d2 && t1 == t2,
            (Node::Paragraph(a), Node::Paragraph(b)) => a == b,
            (Node::BlockQuote(a), Node::BlockQuote(b)) => a == b,
            (
                Node::OrderedList {
                    start: s1,
                    items: i1,
                },
                Node::OrderedList {
                    start: s2,
                    items: i2,
                },
            ) => s1 == s2 && i1 == i2,
            (Node::UnorderedList(a), Node::UnorderedList(b)) => a == b,
            #[cfg(feature = "gfm")]
            (
                Node::Table {
                    headers: h1,
                    alignments: a1,
                    rows: r1,
                },
                Node::Table {
                    headers: h2,
                    alignments: a2,
                    rows: r2,
                },
            ) => h1 == h2 && a1 == a2 && r1 == r2,
            #[cfg(not(feature = "gfm"))]
            (
                Node::Table {
                    headers: h1,
                    rows: r1,
                },
                Node::Table {
                    headers: h2,
                    rows: r2,
                },
            ) => h1 == h2 && r1 == r2,
            (Node::InlineCode(a), Node::InlineCode(b)) => a == b,
            (Node::Emphasis(a), Node::Emphasis(b)) => a == b,
            (Node::Strong(a), Node::Strong(b)) => a == b,
            #[cfg(feature = "gfm")]
            (Node::Strikethrough(a), Node::Strikethrough(b)) => a == b,
            (
                Node::Link {
                    url: u1,
                    title: t1,
                    content: c1,
                },
                Node::Link {
                    url: u2,
                    title: t2,
                    content: c2,
                },
            ) => u1 == u2 && t1 == t2 && c1 == c2,
            (
                Node::ReferenceLink {
                    label: l1,
                    content: c1,
                },
                Node::ReferenceLink {
                    label: l2,
                    content: c2,
                },
            ) => l1 == l2 && c1 == c2,
            (
                Node::Image {
                    url: u1,
                    title: t1,
                    alt: a1,
                },
                Node::Image {
                    url: u2,
                    title: t2,
                    alt: a2,
                },
            ) => u1 == u2 && t1 == t2 && a1 == a2,
            (
                Node::Autolink {
                    url: u1,
                    is_email: e1,
                },
                Node::Autolink {
                    url: u2,
                    is_email: e2,
                },
            ) => u1 == u2 && e1 == e2,
            #[cfg(feature = "gfm")]
            (Node::ExtendedAutolink(a), Node::ExtendedAutolink(b)) => a == b,
            (Node::HtmlElement(a), Node::HtmlElement(b)) => a == b,
            (Node::HardBreak, Node::HardBreak) => true,
            (Node::SoftBreak, Node::SoftBreak) => true,
            (Node::Text(a), Node::Text(b)) => a == b,
            (Node::Custom(a), Node::Custom(b)) => a.eq_box(&**b),
            _ => false,
        }
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
    /// Task list item (GFM extension)
    #[cfg(feature = "gfm")]
    Task {
        /// Task completion status
        status: TaskListStatus,
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
                | Node::Strikethrough(_)
                // Links
                | Node::Link { .. }
                | Node::ReferenceLink { .. }
                // Images
                | Node::Image { .. }
                // Autolinks
                | Node::Autolink { .. }
                | Node::ExtendedAutolink(_)
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
    pub fn code_block(language: Option<EcoString>, content: EcoString) -> Self {
        Node::CodeBlock {
            language,
            content,
            block_type: CodeBlockType::default(),
        }
    }

    /// Create a strikethrough node
    ///
    /// # Arguments
    /// * `content` - Content to be struck through
    ///
    /// # Returns
    /// A new strikethrough node
    pub fn strikethrough(content: Vec<Node>) -> Self {
        Node::Strikethrough(content)
    }

    /// Create a task list item
    ///
    /// # Arguments
    /// * `status` - Task completion status
    /// * `content` - Task content
    ///
    /// # Returns
    /// A new task list item
    #[cfg(feature = "gfm")]
    pub fn task_list_item(status: TaskListStatus, content: Vec<Node>) -> Self {
        Node::UnorderedList(vec![ListItem::Task { status, content }])
    }

    /// Create a table with alignment
    ///
    /// # Arguments
    /// * `headers` - Table header cells
    /// * `alignments` - Column alignments
    /// * `rows` - Table rows
    ///
    /// # Returns
    /// A new table node with alignment information
    #[cfg(feature = "gfm")]
    pub fn table_with_alignment(
        headers: Vec<Node>,
        alignments: Vec<TableAlignment>,
        rows: Vec<Vec<Node>>,
    ) -> Self {
        Node::Table {
            headers,
            alignments,
            rows,
        }
    }
    /// Check if a custom node is of a specific type, and return a reference to that type
    pub fn as_custom_type<T: CustomNode + 'static>(&self) -> Option<&T> {
        if let Node::Custom(node) = self {
            node.as_any().downcast_ref::<T>()
        } else {
            None
        }
    }

    /// Check if a node is a custom node of a specific type
    pub fn is_custom_type<T: CustomNode + 'static>(&self) -> bool {
        self.as_custom_type::<T>().is_some()
    }
}

// Implement Format traits for Node
impl crate::format_traits::Format<crate::writer::CommonMarkWriter> for Node {
    fn format(
        &self,
        writer: &mut crate::writer::CommonMarkWriter,
    ) -> crate::error::WriteResult<()> {
        writer.write_node_internal(self)
    }
}

impl crate::format_traits::Format<crate::writer::HtmlWriter> for Node {
    fn format(&self, writer: &mut crate::writer::HtmlWriter) -> crate::error::WriteResult<()> {
        writer.write_node_internal(self).map_err(Into::into)
    }
}
