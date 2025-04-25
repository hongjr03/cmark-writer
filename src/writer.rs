//! CommonMark writer implementation.
//!
//! This module provides functionality to convert AST nodes to CommonMark format text.
//! The main component is the CommonMarkWriter class, which serializes AST nodes to CommonMark-compliant text.

use crate::ast::{Alignment, BlockNode, HtmlElement, InlineNode, ListItem, Node};
use crate::error::{WriteError, WriteResult};
use crate::options::WriterOptions;
use std::{
    cmp::max,
    fmt::{self},
};

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
    /// use cmark_writer::writer::CommonMarkWriter;
    /// use cmark_writer::options::WriterOptions;
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
    /// If writing succeeds, returns `Ok(())`, otherwise returns `Err(WriteError)`
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
    pub fn write(&mut self, node: &Node) -> WriteResult<()> {
        match node {
            Node::Block(block_node) => self.write_block(block_node),
            Node::Inline(inline_node) => self.write_inline(inline_node),
        }
    }

    /// Write a block node as CommonMark format
    fn write_block(&mut self, node: &BlockNode) -> WriteResult<()> {
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
    fn write_inline(&mut self, node: &InlineNode) -> WriteResult<()> {
        if self.options.strict && !matches!(node, InlineNode::SoftBreak | InlineNode::HardBreak) {
            // Provide context for the error message
            let context = match node {
                InlineNode::Text(_) => "Text",
                InlineNode::Emphasis(_) => "Emphasis",
                InlineNode::Strong(_) => "Strong",
                InlineNode::Strike(_) => "Strike",
                InlineNode::InlineCode(_) => "InlineCode",
                InlineNode::Link { .. } => "Link content",
                InlineNode::Image { .. } => "Image alt text", // Although checked separately later
                InlineNode::HtmlElement(_) => "HtmlElement content",
                InlineNode::InlineContainer(_) => "InlineContainer",
                _ => "Unknown inline element",
            };
            self.check_no_newline(node, context)?;
        }

        match node {
            InlineNode::Text(content) => self.write_text_content(content),
            InlineNode::Emphasis(content) => self.write_delimited(content, "*"),
            InlineNode::Strong(content) => self.write_delimited(content, "**"),
            InlineNode::Strike(content) => self.write_delimited(content, "~~"),
            InlineNode::InlineCode(content) => self.write_code_content(content),
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

    /// Writes text content with character escaping
    fn write_text_content(&mut self, content: &str) -> WriteResult<()> {
        let escaped = content
            .replace('\\', "\\\\")
            .replace('*', "\\*")
            .replace('_', "\\_")
            .replace('[', "\\[")
            .replace(']', "\\]")
            .replace('<', "\\<")
            .replace('>', "\\>")
            .replace('`', "\\`");

        self.write_str(&escaped)?;
        Ok(())
    }

    /// Writes inline code content
    fn write_code_content(&mut self, content: &str) -> WriteResult<()> {
        // Check if the content contains a backtick
        self.write_char('`')?;
        self.write_str(content)?;
        self.write_char('`')?;
        Ok(())
    }

    /// Helper function for writing content with delimiters
    fn write_delimited(&mut self, content: &[InlineNode], delimiter: &str) -> WriteResult<()> {
        self.write_str(delimiter)?;

        for node in content {
            self.write_inline(node)?;
        }

        self.write_str(delimiter)?;
        Ok(())
    }

    /// Write a document node
    fn write_document(&mut self, children: &[BlockNode]) -> WriteResult<()> {
        for (i, child) in children.iter().enumerate() {
            self.write_block(child)?;
            if i < children.len() - 1 {
                self.write_str("\n\n")?;
            }
        }
        Ok(())
    }

    /// Write a heading node
    fn write_heading(&mut self, level: u8, content: &[InlineNode]) -> WriteResult<()> {
        if !(1..=6).contains(&level) {
            return Err(WriteError::InvalidHeadingLevel(level));
        }

        for _ in 0..level {
            self.write_char('#')?;
        }
        self.write_char(' ')?;

        for node in content.iter() {
            self.write_inline(node)?;
        }

        Ok(())
    }

    /// Write a paragraph node
    fn write_paragraph(&mut self, content: &[InlineNode]) -> WriteResult<()> {
        for node in content.iter() {
            self.write_inline(node)?;
        }
        Ok(())
    }

    /// Write a blockquote node
    fn write_blockquote(&mut self, content: &[BlockNode]) -> WriteResult<()> {
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
    fn write_code_block(&mut self, language: &Option<String>, content: &str) -> WriteResult<()> {
        let max_backticks = content
            .chars()
            .fold((0, 0), |(max, current), c| {
                if c == '`' {
                    (max.max(current + 1), current + 1)
                } else {
                    (max, 0)
                }
            })
            .0;

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
    fn write_unordered_list(&mut self, items: &[ListItem]) -> WriteResult<()> {
        for (i, item) in items.iter().enumerate() {
            self.write_list_item(item, "- ")?;
            if i < items.len() - 1 {
                self.write_char('\n')?;
            }
        }
        Ok(())
    }

    /// Write an ordered list node
    fn write_ordered_list(&mut self, start: u32, items: &[ListItem]) -> WriteResult<()> {
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
    fn write_list_item(&mut self, item: &ListItem, prefix: &str) -> WriteResult<()> {
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
    ) -> WriteResult<()> {
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
    fn write_thematic_break(&mut self) -> WriteResult<()> {
        self.write_str("---")?;
        Ok(())
    }

    /// Check if the inline node contains a newline character and return an error if it does
    fn check_no_newline(&self, node: &InlineNode, context: &str) -> WriteResult<()> {
        if Self::inline_node_contains_newline(node) {
            return Err(WriteError::NewlineInInlineElement(context.to_string()));
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
    ) -> WriteResult<()> {
        // Write header
        self.write_char('|')?;
        for header in headers {
            self.check_no_newline(header, "Table Header")?;
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
                self.check_no_newline(cell, "Table Cell")?;
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
    ) -> WriteResult<()> {
        for node in content {
            self.check_no_newline(node, "Link Text")?;
        }
        self.write_char('[')?;

        for node in content {
            self.write_inline(node)?;
        }

        self.write_str("](")?;
        self.write_str(url)?;

        if let Some(title_text) = title {
            self.write_str(" \"")?;
            self.write_str(title_text)?; // Title can contain escaped quotes, but not raw newlines
            self.write_char('"')?;
        }

        self.write_char(')')?;
        Ok(())
    }

    /// Write an image
    fn write_image(&mut self, url: &str, title: &Option<String>, alt: &str) -> WriteResult<()> {
        if alt.contains('\n') {
            // Specific check for alt text newline
            return Err(WriteError::NewlineInImageAltText);
        }

        self.write_str("![")?;
        self.write_str(alt)?; // Alt text cannot contain unescaped brackets or newlines
        self.write_str("](")?;
        self.write_str(url)?;

        if let Some(title_text) = title {
            self.write_str(" \"")?;
            self.write_str(title_text)?; // Title can contain escaped quotes, but not raw newlines
            self.write_char('"')?;
        }

        self.write_char(')')?;
        Ok(())
    }

    /// Write a soft line break
    fn write_soft_break(&mut self) -> WriteResult<()> {
        self.write_char('\n')?;
        Ok(())
    }

    /// Write a hard line break
    fn write_hard_break(&mut self) -> WriteResult<()> {
        if self.options.hard_break_spaces {
            self.write_str("  \n")?;
        } else {
            self.write_str("\\\n")?;
        }
        Ok(())
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
        // Keep fmt::Result here as it's the base operation
        self.buffer.push(c);
        Ok(())
    }

    /// Write a string to the buffer
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // Keep fmt::Result here as it's the base operation
        self.buffer.push_str(s);
        Ok(())
    }

    /// Write an HTML block
    fn write_html_block(&mut self, content: &str) -> WriteResult<()> {
        self.write_str(content)?;
        Ok(())
    }

    /// Write an HTML element
    fn write_html_element(&mut self, element: &HtmlElement) -> WriteResult<()> {
        self.write_char('<')?;
        self.write_str(&element.tag)?;

        for attr in &element.attributes {
            self.write_char(' ')?;
            self.write_str(&attr.name)?;
            self.write_str("=\"")?;
            self.write_str(&attr.value)?; // Assume attributes are pre-escaped if needed
            self.write_char('"')?;
        }

        if element.self_closing {
            self.write_str(" />")?;
            return Ok(());
        }

        self.write_char('>')?;

        for child in &element.children {
            // HTML element content can contain newlines, so no strict check here
            self.write_inline(child)?;
        }

        self.write_str("</")?;
        self.write_str(&element.tag)?;
        self.write_char('>')?;
        Ok(())
    }

    /// Write inline container content
    fn write_inline_container(&mut self, content: &[InlineNode]) -> WriteResult<()> {
        for node in content {
            self.write_inline(node)?;
        }
        Ok(())
    }
}

impl Default for CommonMarkWriter {
    fn default() -> Self {
        Self::new()
    }
}

// Implement Display trait for new Node structure using the new error type
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut writer = CommonMarkWriter::new();
        match writer.write(self) {
            Ok(_) => write!(f, "{}", writer.into_string()),
            Err(e) => write!(f, "Error writing Node: {}", e),
        }
    }
}

// Implement Display trait for BlockNode using the new error type
impl fmt::Display for BlockNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut writer = CommonMarkWriter::new();
        match writer.write_block(self) {
            Ok(_) => write!(f, "{}", writer.into_string()),
            Err(e) => write!(f, "Error writing BlockNode: {}", e),
        }
    }
}

// Implement Display trait for InlineNode using the new error type
impl fmt::Display for InlineNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut writer = CommonMarkWriter::new();
        match writer.write_inline(self) {
            Ok(_) => write!(f, "{}", writer.into_string()),
            Err(e) => write!(f, "Error writing InlineNode: {}", e),
        }
    }
}
