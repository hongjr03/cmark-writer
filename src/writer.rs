//! CommonMark writer implementation.
//!
//! This module provides functionality to convert AST nodes to CommonMark format text.
//! The main component is the CommonMarkWriter class, which serializes AST nodes to CommonMark-compliant text.

use crate::ast::{Alignment, CustomNode, CustomNodeWriter, HtmlElement, ListItem, Node};
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

/// Private trait for node processing strategy
trait NodeProcessor {
    /// Process a node and write its content
    fn process(&self, writer: &mut CommonMarkWriter, node: &Node) -> WriteResult<()>;
}

/// Strategy for processing block nodes
struct BlockNodeProcessor;

/// Strategy for processing inline nodes
struct InlineNodeProcessor;

impl NodeProcessor for BlockNodeProcessor {
    fn process(&self, writer: &mut CommonMarkWriter, node: &Node) -> WriteResult<()> {
        match node {
            Node::Document(children) => writer.write_document(children),
            Node::Heading { level, content } => writer.write_heading(*level, content),
            Node::Paragraph(content) => writer.write_paragraph(content),
            Node::BlockQuote(content) => writer.write_blockquote(content),
            Node::CodeBlock { language, content } => writer.write_code_block(language, content),
            Node::UnorderedList(items) => writer.write_unordered_list(items),
            Node::OrderedList { start, items } => writer.write_ordered_list(*start, items),
            Node::ThematicBreak => writer.write_thematic_break(),
            Node::Table {
                headers,
                rows,
                alignments,
            } => writer.write_table(headers, rows, alignments),
            Node::HtmlBlock(content) => writer.write_html_block(content),
            Node::Custom(custom_node) if custom_node.is_block() => writer.write_custom_node(custom_node),
            _ => Err(WriteError::UnsupportedNodeType),
        }
    }
}

impl NodeProcessor for InlineNodeProcessor {
    fn process(&self, writer: &mut CommonMarkWriter, node: &Node) -> WriteResult<()> {
        // Check for newlines in inline nodes in strict mode
        if writer.options.strict && !matches!(node, Node::SoftBreak | Node::HardBreak) {
            let context = writer.get_context_for_node(node);
            writer.check_no_newline(node, &context)?;
        }

        match node {
            Node::Text(content) => writer.write_text_content(content),
            Node::Emphasis(content) => writer.write_delimited(content, "*"),
            Node::Strong(content) => writer.write_delimited(content, "**"),
            Node::Strike(content) => writer.write_delimited(content, "~~"),
            Node::InlineCode(content) => writer.write_code_content(content),
            Node::Link {
                url,
                title,
                content,
            } => writer.write_link(url, title, content),
            Node::Image { url, title, alt } => writer.write_image(url, title, alt),
            Node::HtmlElement(element) => writer.write_html_element(element),
            Node::InlineContainer(content) => writer.write_inline_container(content),
            Node::SoftBreak => writer.write_soft_break(),
            Node::HardBreak => writer.write_hard_break(),
            Node::Custom(custom_node) if !custom_node.is_block() => writer.write_custom_node(custom_node),
            _ => Err(WriteError::UnsupportedNodeType),
        }
    }
}

impl CommonMarkWriter {
    /// Create a new CommonMark writer with default options
    ///
    /// # Example
    ///
    /// ```
    /// use cmark_writer::writer::CommonMarkWriter;
    /// use cmark_writer::ast::Node;
    ///
    /// let mut writer = CommonMarkWriter::new();
    /// writer.write(&Node::Text("Hello".to_string())).unwrap();
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
    /// use cmark_writer::ast::Node;
    ///
    /// let mut writer = CommonMarkWriter::new();
    /// writer.write(&Node::Text("Hello".to_string())).unwrap();
    /// ```
    pub fn write(&mut self, node: &Node) -> WriteResult<()> {
        if let Node::Custom(custom_node) = node {
            return self.write_custom_node(custom_node);
        }
        
        if node.is_block() {
            BlockNodeProcessor.process(self, node)
        } else if node.is_inline() {
            InlineNodeProcessor.process(self, node)
        } else {
            // Keep this branch for future implementation needs
            Err(WriteError::UnsupportedNodeType)
        }
    }

    /// Write a custom node using its implementation
    fn write_custom_node(&mut self, node: &Box<dyn CustomNode>) -> WriteResult<()> {
        node.write(self)
    }

    /// Get context description for a node, used for error reporting
    fn get_context_for_node(&self, node: &Node) -> String {
        match node {
            Node::Text(_) => "Text".to_string(),
            Node::Emphasis(_) => "Emphasis".to_string(),
            Node::Strong(_) => "Strong".to_string(),
            Node::Strike(_) => "Strike".to_string(),
            Node::InlineCode(_) => "InlineCode".to_string(),
            Node::Link { .. } => "Link content".to_string(),
            Node::Image { .. } => "Image alt text".to_string(),
            Node::HtmlElement(_) => "HtmlElement content".to_string(),
            Node::InlineContainer(_) => "InlineContainer".to_string(),
            Node::Custom(_) => "Custom node".to_string(),
            _ => "Unknown inline element".to_string(),
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
        self.write_char('`')?;
        self.write_str(content)?;
        self.write_char('`')?;
        Ok(())
    }

    /// Helper function for writing content with delimiters
    fn write_delimited(&mut self, content: &[Node], delimiter: &str) -> WriteResult<()> {
        self.write_str(delimiter)?;

        for node in content {
            self.write(node)?;
        }

        self.write_str(delimiter)?;
        Ok(())
    }

    /// Write a document node
    fn write_document(&mut self, children: &[Node]) -> WriteResult<()> {
        for (i, child) in children.iter().enumerate() {
            self.write(child)?;
            if i < children.len() - 1 {
                self.write_str("\n\n")?;
            }
        }
        Ok(())
    }

    /// Write a heading node
    fn write_heading(&mut self, level: u8, content: &[Node]) -> WriteResult<()> {
        if !(1..=6).contains(&level) {
            return Err(WriteError::InvalidHeadingLevel(level));
        }

        for _ in 0..level {
            self.write_char('#')?;
        }
        self.write_char(' ')?;

        for node in content.iter() {
            self.write(node)?;
        }

        Ok(())
    }

    /// Write a paragraph node
    fn write_paragraph(&mut self, content: &[Node]) -> WriteResult<()> {
        for node in content.iter() {
            self.write(node)?;
        }
        Ok(())
    }

    /// Write a blockquote node
    fn write_blockquote(&mut self, content: &[Node]) -> WriteResult<()> {
        self.indent_level += 1;

        for (i, node) in content.iter().enumerate() {
            self.write_str("> ")?;
            self.write(node)?;
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
        // Track the current item number
        let mut current_number = start;

        for (i, item) in items.iter().enumerate() {
            match item {
                // For ordered list items, check if there's a custom number
                ListItem::Ordered { number, content: _ } => {
                    if let Some(custom_num) = number {
                        // Use custom numbering
                        let prefix = format!("{}. ", custom_num);
                        self.write_list_item(item, &prefix)?;
                        // Next expected number
                        current_number = custom_num + 1;
                    } else {
                        // No custom number, use the current calculated number
                        let prefix = format!("{}. ", current_number);
                        self.write_list_item(item, &prefix)?;
                        current_number += 1;
                    }
                }
                // For other types of list items, still use the current number
                _ => {
                    let prefix = format!("{}. ", current_number);
                    self.write_list_item(item, &prefix)?;
                    current_number += 1;
                }
            }

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

        // Process different types of list items
        match item {
            ListItem::Unordered { content } => {
                self.write_str(prefix)?;
                self.write_list_item_content(content, prefix, false)?;
            }
            ListItem::Ordered { number, content } => {
                // If a custom number is provided, use it; otherwise use the prefix
                if let Some(num) = number {
                    // Override the given prefix with a custom number
                    let custom_prefix = format!("{}. ", num);
                    self.write_str(&custom_prefix)?;
                    self.write_list_item_content(content, &custom_prefix, false)?;
                } else {
                    // Use the given prefix
                    self.write_str(prefix)?;
                    self.write_list_item_content(content, prefix, false)?;
                }
            }
        }

        Ok(())
    }

    /// Write list item content
    fn write_list_item_content(
        &mut self,
        content: &[Node],
        prefix: &str,
        is_task: bool,
    ) -> WriteResult<()> {
        self.indent_level += 1;

        for (i, node) in content.iter().enumerate() {
            let is_list = matches!(node, Node::OrderedList { .. } | Node::UnorderedList(..));

            // Nested lists need special line break handling
            if is_list {
                if i > 0 {
                    self.write_char('\n')?;
                }
                self.write(node)?;
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

            self.write(node)?;
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
    fn check_no_newline(&self, node: &Node, context: &str) -> WriteResult<()> {
        if Self::node_contains_newline(node) {
            return Err(WriteError::NewlineInInlineElement(context.to_string()));
        }
        Ok(())
    }

    /// Check if the inline node contains a newline character recursively
    fn node_contains_newline(node: &Node) -> bool {
        match node {
            Node::Text(s) | Node::InlineCode(s) => s.contains('\n'),
            Node::Emphasis(children)
            | Node::Strong(children)
            | Node::Strike(children)
            | Node::InlineContainer(children) => children.iter().any(Self::node_contains_newline),
            Node::HtmlElement(element) => element.children.iter().any(Self::node_contains_newline),
            Node::Link { content, .. } => content.iter().any(Self::node_contains_newline),
            Node::Image { alt, .. } => alt.iter().any(Self::node_contains_newline),
            Node::SoftBreak | Node::HardBreak => true,
            // Custom nodes are handled separately
            Node::Custom(_) => false,
            _ => false,
        }
    }

    /// Write a table
    fn write_table(
        &mut self,
        headers: &[Node],
        rows: &[Vec<Node>],
        alignments: &[Alignment],
    ) -> WriteResult<()> {
        // Write header
        self.write_char('|')?;
        for header in headers {
            self.check_no_newline(header, "Table Header")?;
            self.write_char(' ')?;
            self.write(header)?;
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
                self.write(cell)?;
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
        content: &[Node],
    ) -> WriteResult<()> {
        for node in content {
            self.check_no_newline(node, "Link Text")?;
        }
        self.write_char('[')?;

        for node in content {
            self.write(node)?;
        }

        self.write_str("](")?;
        self.write_str(url)?;

        if let Some(title_text) = title {
            self.write_str(" \"")?;
            self.write_str(title_text)?;
            self.write_char('"')?;
        }

        self.write_char(')')?;
        Ok(())
    }

    /// Write an image
    fn write_image(&mut self, url: &str, title: &Option<String>, alt: &[Node]) -> WriteResult<()> {
        // Check for newlines in alt text content
        for node in alt {
            self.check_no_newline(node, "Image alt text")?;
        }

        self.write_str("![")?;

        // Write alt text content
        for node in alt {
            self.write(node)?;
        }

        self.write_str("](")?;
        self.write_str(url)?;

        if let Some(title_text) = title {
            self.write_str(" \"")?;
            self.write_str(title_text)?;
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
    /// use cmark_writer::ast::Node;
    ///
    /// let mut writer = CommonMarkWriter::new();
    /// writer.write(&Node::Text("Hello".to_string())).unwrap();
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
            self.write(child)?;
        }

        self.write_str("</")?;
        self.write_str(&element.tag)?;
        self.write_char('>')?;
        Ok(())
    }

    /// Write inline container content
    fn write_inline_container(&mut self, content: &[Node]) -> WriteResult<()> {
        for node in content {
            self.write(node)?;
        }
        Ok(())
    }
}

impl Default for CommonMarkWriter {
    fn default() -> Self {
        Self::new()
    }
}

// Implement Display trait for Node structure
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut writer = CommonMarkWriter::new();
        match writer.write(self) {
            Ok(_) => write!(f, "{}", writer.into_string()),
            Err(e) => write!(f, "Error writing Node: {}", e),
        }
    }
}

// Implement CustomNodeWriter trait for CommonMarkWriter
impl CustomNodeWriter for CommonMarkWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.buffer.push_str(s);
        Ok(())
    }
    
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.buffer.push(c);
        Ok(())
    }
}
