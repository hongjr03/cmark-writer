//! CommonMark writer implementation.
//!
//! This module provides functionality to convert AST nodes to CommonMark format text.
//! The main component is the CommonMarkWriter class, which serializes AST nodes to CommonMark-compliant text.

use crate::ast::{Alignment, BlockNode, HtmlElement, InlineNode, ListItem, Node};
use std::{
    cmp::max,
    fmt::{self},
};

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

/// CommonMark writer
///
/// This struct is responsible for serializing AST nodes to CommonMark-compliant text.
#[derive(Debug)]
pub struct CommonMarkWriter {
    options: WriterOptions,
    buffer: String,
    /// Current indentation level
    indent_level: usize,
}

impl CommonMarkWriter {
    /// Create a new CommonMark writer with default options
    ///
    /// # Example
    ///
    /// ```
    /// use cmark_writer::writer::CommonMarkWriter;
    /// use cmark_writer::ast::{Node, InlineNode};
    ///
    /// let mut writer = CommonMarkWriter::new();
    /// writer.write(&Node::Inline(InlineNode::Text("Hello".to_string()))).unwrap();
    /// assert_eq!(writer.into_string(), "Hello");
    /// ```
    pub fn new() -> Self {
        Self::with_options(WriterOptions::default())
    }

    /// Create a new CommonMark writer with specified options
    ///
    /// # Parameters
    ///
    /// * `options` - Custom CommonMark formatting options
    ///
    /// # Example
    ///
    /// ```
    /// use cmark_writer::writer::{CommonMarkWriter, WriterOptions};
    ///
    /// let options = WriterOptions {
    ///     strict: true,
    ///     hard_break_spaces: false,  // Use backslash for line breaks
    ///     indent_spaces: 2,          // Use 2 spaces for indentation
    /// };
    /// let writer = CommonMarkWriter::with_options(options);
    /// ```
    pub fn with_options(options: WriterOptions) -> Self {
        Self {
            options,
            buffer: String::new(),
            indent_level: 0,
        }
    }

    /// Write an AST node as CommonMark format
    ///
    /// # Parameters
    ///
    /// * `node` - The AST node to write
    ///
    /// # Returns
    ///
    /// If writing succeeds, returns `Ok(())`, otherwise returns `Err(fmt::Error)`
    ///
    /// # Example
    ///
    /// ```
    /// use cmark_writer::writer::CommonMarkWriter;
    /// use cmark_writer::ast::{Node, InlineNode};
    ///
    /// let mut writer = CommonMarkWriter::new();
    /// writer.write(&Node::Inline(InlineNode::Text("Hello".to_string()))).unwrap();
    /// ```
    pub fn write(&mut self, node: &Node) -> fmt::Result {
        match node {
            Node::Block(block_node) => self.write_block(block_node),
            Node::Inline(inline_node) => self.write_inline(inline_node),
        }
    }

    /// Write a block node as CommonMark format
    fn write_block(&mut self, node: &BlockNode) -> fmt::Result {
        match node {
            BlockNode::Document(children) => self.write_document(children),
            BlockNode::Heading { level, content } => self.write_heading(*level, content),
            BlockNode::Paragraph(content) => self.write_paragraph(content),
            BlockNode::BlockQuote(content) => self.write_blockquote(content),
            BlockNode::CodeBlock { language, content } => self.write_code_block(language, content),
            BlockNode::UnorderedList(items) => self.write_unordered_list(items),
            BlockNode::OrderedList { start, items } => self.write_ordered_list(*start, items),
            BlockNode::ThematicBreak => self.write_thematic_break(),
            BlockNode::Table {
                headers,
                rows,
                alignments,
            } => self.write_table(headers, rows, alignments),
            BlockNode::HtmlBlock(content) => self.write_html_block(content),
        }
    }

    /// Write an inline node as CommonMark format
    fn write_inline(&mut self, node: &InlineNode) -> fmt::Result {
        match node {
            InlineNode::Text(content) => self.write_text(content),
            InlineNode::Emphasis(content) => self.write_emphasis(content),
            InlineNode::Strong(content) => self.write_strong(content),
            InlineNode::Strike(content) => self.write_strike(content),
            InlineNode::InlineCode(content) => self.write_inline_code(content),
            InlineNode::Link {
                url,
                title,
                content,
            } => self.write_link(url, title, content),
            InlineNode::Image { url, title, alt } => self.write_image(url, title, alt),
            InlineNode::HtmlElement(element) => self.write_html_element(element),
            InlineNode::InlineContainer(content) => self.write_inline_container(content),
            InlineNode::SoftBreak => self.write_soft_break(),
            InlineNode::HardBreak => self.write_hard_break(),
        }
    }

    /// Write a document node
    fn write_document(&mut self, children: &[BlockNode]) -> fmt::Result {
        for (i, child) in children.iter().enumerate() {
            self.write_block(child)?;
            if i < children.len() - 1 {
                self.write_str("\n\n")?;
            }
        }
        Ok(())
    }

    /// Write a heading node
    fn write_heading(&mut self, level: u8, content: &[InlineNode]) -> fmt::Result {
        if !(1..=6).contains(&level) {
            return Err(fmt::Error);
        }

        for _ in 0..level {
            self.write_char('#')?;
        }
        self.write_char(' ')?;

        for (i, node) in content.iter().enumerate() {
            self.write_inline(node)?;
            if i < content.len() - 1
                && !matches!(node, InlineNode::SoftBreak | InlineNode::HardBreak)
            {
                self.write_char(' ')?;
            }
        }

        Ok(())
    }

    /// Write a paragraph node
    fn write_paragraph(&mut self, content: &[InlineNode]) -> fmt::Result {
        let mut prev_is_inline = false;

        for (i, node) in content.iter().enumerate() {
            // Check if the current node is an inline element that should be kept inline
            let is_inline = !matches!(node, InlineNode::SoftBreak | InlineNode::HardBreak);

            // If both current and previous elements should be kept inline, and it's not the first element
            if prev_is_inline && is_inline && i > 0 {
                // Don't add extra whitespace to prevent incorrect line breaks
            } else if i > 0 {
                // Non-consecutive inline elements, add normal line break and indentation
                self.write_char('\n')?;
                // Add appropriate indentation (current indent level)
                for _ in 0..(self.indent_level * self.options.indent_spaces) {
                    self.write_char(' ')?;
                }
            }

            self.write_inline(node)?;
            prev_is_inline = is_inline;
        }
        Ok(())
    }

    /// Write a blockquote node
    fn write_blockquote(&mut self, content: &[BlockNode]) -> fmt::Result {
        self.indent_level += 1;

        for (i, node) in content.iter().enumerate() {
            self.write_str("> ")?;
            self.write_block(node)?;
            if i < content.len() - 1 {
                self.write_str("\n> \n")?;
            }
        }

        self.indent_level -= 1;
        Ok(())
    }

    /// Write a code block node
    fn write_code_block(&mut self, language: &Option<String>, content: &str) -> fmt::Result {
        let mut max_backticks = 0;
        let mut current = 0;
        for c in content.chars() {
            if c == '`' {
                current += 1;
                if current > max_backticks {
                    max_backticks = current;
                }
            } else {
                current = 0;
            }
        }
        let fence_len = max(max_backticks + 1, 3);
        let fence = "`".repeat(fence_len);

        self.write_str(&fence)?;
        if let Some(lang) = language {
            self.write_str(lang)?;
        }
        self.write_char('\n')?;
        self.write_str(content)?;

        // Ensure content ends with a newline
        if !content.ends_with('\n') {
            self.write_char('\n')?;
        }

        self.write_str(&fence)?;
        Ok(())
    }

    /// Write an unordered list node
    fn write_unordered_list(&mut self, items: &[ListItem]) -> fmt::Result {
        for (i, item) in items.iter().enumerate() {
            self.write_list_item(item, "- ")?;
            if i < items.len() - 1 {
                self.write_char('\n')?;
            }
        }
        Ok(())
    }

    /// Write an ordered list node
    fn write_ordered_list(&mut self, start: u32, items: &[ListItem]) -> fmt::Result {
        for (i, item) in items.iter().enumerate() {
            let num = start as usize + i;
            let prefix = format!("{}. ", num);
            self.write_list_item(item, &prefix)?;
            if i < items.len() - 1 {
                self.write_char('\n')?;
            }
        }
        Ok(())
    }

    /// Write a list item
    fn write_list_item(&mut self, item: &ListItem, prefix: &str) -> fmt::Result {
        // Apply indentation based on current level
        for _ in 0..(self.indent_level * self.options.indent_spaces) {
            self.write_char(' ')?;
        }
        self.write_str(prefix)?;

        match item {
            ListItem::Regular { content } => {
                self.write_list_item_content(content, prefix, false)?;
            }
            ListItem::Task { completed, content } => {
                // Write task checkbox
                if *completed {
                    self.write_str("[x] ")?;
                } else {
                    self.write_str("[ ] ")?;
                }
                self.write_list_item_content(content, prefix, true)?;
            }
        }

        Ok(())
    }

    /// Write list item content
    fn write_list_item_content(
        &mut self,
        content: &[BlockNode],
        prefix: &str,
        is_task: bool,
    ) -> fmt::Result {
        self.indent_level += 1;

        for (i, node) in content.iter().enumerate() {
            let is_list = matches!(
                node,
                BlockNode::OrderedList { .. } | BlockNode::UnorderedList(..)
            );

            // Nested lists need special line break handling
            if is_list {
                if i > 0 {
                    self.write_char('\n')?;
                }
                self.write_block(node)?;
                continue;
            }

            if i > 0 {
                self.write_char('\n')?;
                // Add appropriate indentation (list item prefix length + current indent level)
                let prefix_length = prefix.len() + if is_task { 4 } else { 0 };
                for _ in 0..(self.indent_level * self.options.indent_spaces) + prefix_length {
                    self.write_char(' ')?;
                }
            }

            self.write_block(node)?;
        }

        self.indent_level -= 1;
        Ok(())
    }

    /// Write a thematic break (horizontal rule)
    fn write_thematic_break(&mut self) -> fmt::Result {
        self.write_str("---")
    }

    /// Check if the inline node contains a newline character and return an error if it does
    fn check_no_newline(&self, node: &InlineNode) -> fmt::Result {
        if Self::inline_node_contains_newline(node) {
            return Err(fmt::Error);
        }
        Ok(())
    }

    /// Check if the inline node contains a newline character recursively
    fn inline_node_contains_newline(node: &InlineNode) -> bool {
        match node {
            InlineNode::Text(s) | InlineNode::InlineCode(s) => s.contains('\n'),
            InlineNode::Emphasis(children)
            | InlineNode::Strong(children)
            | InlineNode::Strike(children)
            | InlineNode::InlineContainer(children) => {
                children.iter().any(Self::inline_node_contains_newline)
            }
            InlineNode::HtmlElement(element) => element
                .children
                .iter()
                .any(Self::inline_node_contains_newline),
            InlineNode::Link { content, .. } => {
                content.iter().any(Self::inline_node_contains_newline)
            }
            InlineNode::Image { alt, .. } => alt.contains('\n'),
            InlineNode::SoftBreak | InlineNode::HardBreak => true,
        }
    }

    /// Write a table
    fn write_table(
        &mut self,
        headers: &[InlineNode],
        rows: &[Vec<InlineNode>],
        alignments: &[Alignment],
    ) -> fmt::Result {
        // Write header
        self.write_char('|')?;
        for header in headers {
            self.check_no_newline(header)?;
            self.write_char(' ')?;
            self.write_inline(header)?;
            self.write_str(" |")?;
        }
        self.write_char('\n')?;

        // Write alignment row
        self.write_char('|')?;
        for alignment in alignments {
            match alignment {
                Alignment::None => self.write_str(" --- |")?,
                Alignment::Left => self.write_str(" :--- |")?,
                Alignment::Center => self.write_str(" :---: |")?,
                Alignment::Right => self.write_str(" ---: |")?,
            }
        }
        self.write_char('\n')?;

        // Write table content
        for row in rows {
            self.write_char('|')?;
            for cell in row {
                self.check_no_newline(cell)?;
                self.write_char(' ')?;
                self.write_inline(cell)?;
                self.write_str(" |")?;
            }
            self.write_char('\n')?;
        }

        Ok(())
    }

    /// Write a link
    fn write_link(
        &mut self,
        url: &str,
        title: &Option<String>,
        content: &[InlineNode],
    ) -> fmt::Result {
        for node in content {
            self.check_no_newline(node)?;
        }
        self.write_char('[')?;

        for node in content {
            self.write_inline(node)?;
        }

        self.write_str("](")?;
        self.write_str(url)?;

        if let Some(title_text) = title {
            self.write_str(" \"")?;
            self.write_str(title_text)?;
            self.write_char('"')?;
        }

        self.write_char(')')
    }

    /// Write an image
    fn write_image(&mut self, url: &str, title: &Option<String>, alt: &str) -> fmt::Result {
        if alt.contains('\n') {
            return Err(fmt::Error);
        }

        self.write_str("![")?;
        self.write_str(alt)?;
        self.write_str("](")?;
        self.write_str(url)?;

        if let Some(title_text) = title {
            self.write_str(" \"")?;
            self.write_str(title_text)?;
            self.write_char('"')?;
        }

        self.write_char(')')
    }

    /// Write emphasis (italic)
    fn write_emphasis(&mut self, content: &[InlineNode]) -> fmt::Result {
        for node in content {
            self.check_no_newline(node)?;
        }
        self.write_char('*')?;

        for node in content {
            self.write_inline(node)?;
        }

        self.write_char('*')
    }

    /// Write strong emphasis (bold)
    fn write_strong(&mut self, content: &[InlineNode]) -> fmt::Result {
        for node in content {
            self.check_no_newline(node)?;
        }
        self.write_str("**")?;

        for node in content {
            self.write_inline(node)?;
        }

        self.write_str("**")
    }

    /// Write a strikethrough text
    fn write_strike(&mut self, content: &[InlineNode]) -> fmt::Result {
        for node in content {
            self.check_no_newline(node)?;
        }
        self.write_str("~~")?;

        for node in content {
            self.write_inline(node)?;
        }

        self.write_str("~~")
    }

    /// Write inline code
    fn write_inline_code(&mut self, content: &str) -> fmt::Result {
        if content.contains('\n') {
            return Err(fmt::Error);
        }

        self.write_char('`')?;
        self.write_str(content)?;
        self.write_char('`')
    }

    /// Write plain text
    fn write_text(&mut self, content: &str) -> fmt::Result {
        if content.contains('\n') {
            return Err(fmt::Error);
        }

        // Escape special characters
        let escaped = content
            .replace('\\', "\\\\")
            .replace('*', "\\*")
            .replace('_', "\\_")
            .replace('[', "\\[")
            .replace(']', "\\]")
            .replace('<', "\\<")
            .replace('>', "\\>")
            .replace('`', "\\`");

        self.write_str(&escaped)
    }

    /// Write an HTML element with attributes and children
    fn write_html_element(&mut self, element: &HtmlElement) -> fmt::Result {
        self.write_char('<')?;
        self.write_str(&element.tag)?;

        // Write attributes
        for attr in &element.attributes {
            self.write_char(' ')?;
            self.write_str(&attr.name)?;
            self.write_str("=\"")?;
            // Escape quotes in attribute values
            let escaped_value = attr.value.replace('"', "&quot;");
            self.write_str(&escaped_value)?;
            self.write_char('"')?;
        }

        if element.self_closing {
            // Self-closing tag like <img />
            self.write_str(" />")?;
            return Ok(());
        }

        self.write_char('>')?;

        // Process children
        for child in &element.children {
            self.check_no_newline(child)?;
            self.write_inline(child)?;
        }

        // Close tag
        self.write_str("</")?;
        self.write_str(&element.tag)?;
        self.write_char('>')?;

        Ok(())
    }

    /// Write HTML block
    fn write_html_block(&mut self, content: &str) -> fmt::Result {
        self.write_str(content)
    }

    /// Write inline container
    fn write_inline_container(&mut self, children: &[InlineNode]) -> fmt::Result {
        for (i, child) in children.iter().enumerate() {
            self.check_no_newline(child)?;
            self.write_inline(child)?;
            if i < children.len() - 1 {
                self.write_str(" ")?;
            }
        }
        Ok(())
    }

    /// Write a soft line break
    fn write_soft_break(&mut self) -> fmt::Result {
        self.write_char('\n')
    }

    /// Write a hard line break
    fn write_hard_break(&mut self) -> fmt::Result {
        if self.options.hard_break_spaces {
            self.write_str("  \n")
        } else {
            self.write_str("\\\n")
        }
    }

    /// Get the generated CommonMark format text
    ///
    /// Consumes the writer and returns the generated string
    ///
    /// # Example
    ///
    /// ```
    /// use cmark_writer::writer::CommonMarkWriter;
    /// use cmark_writer::ast::{Node, InlineNode};
    ///
    /// let mut writer = CommonMarkWriter::new();
    /// writer.write(&Node::Inline(InlineNode::Text("Hello".to_string()))).unwrap();
    /// let result = writer.into_string();
    /// assert_eq!(result, "Hello");
    /// ```
    pub fn into_string(self) -> String {
        self.buffer
    }

    /// Write a character to the buffer
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.buffer.push(c);
        Ok(())
    }

    /// Write a string to the buffer
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.buffer.push_str(s);
        Ok(())
    }
}

impl Default for CommonMarkWriter {
    fn default() -> Self {
        Self::new()
    }
}

// Implement Display trait for new Node structure
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut writer = CommonMarkWriter::new();
        writer.write(self)?;
        write!(f, "{}", writer.into_string())
    }
}

// Implement Display trait for BlockNode
impl fmt::Display for BlockNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut writer = CommonMarkWriter::new();
        writer.write_block(self)?;
        write!(f, "{}", writer.into_string())
    }
}

// Implement Display trait for InlineNode
impl fmt::Display for InlineNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut writer = CommonMarkWriter::new();
        writer.write_inline(self)?;
        write!(f, "{}", writer.into_string())
    }
}
