//! CommonMark writer implementation.
//!
//! This module provides functionality to convert AST nodes to CommonMark format text.
//! The main component is the CommonMarkWriter class, which serializes AST nodes to CommonMark-compliant text.

use crate::ast::{Alignment, ListItem, Node};
use std::fmt::{self};

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
            hard_break_spaces: true,
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
    /// use cmark_rs::writer::CommonMarkWriter;
    /// use cmark_rs::ast::Node;
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
    /// use cmark_rs::writer::{CommonMarkWriter, WriterOptions};
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
    /// use cmark_rs::writer::CommonMarkWriter;
    /// use cmark_rs::ast::Node;
    ///
    /// let mut writer = CommonMarkWriter::new();
    /// writer.write(&Node::Text("Hello".to_string())).unwrap();
    /// ```
    pub fn write(&mut self, node: &Node) -> fmt::Result {
        match node {
            Node::Document(children) => self.write_document(children),
            Node::Heading { level, content } => self.write_heading(*level, content),
            Node::Paragraph(content) => self.write_paragraph(content),
            Node::BlockQuote(content) => self.write_blockquote(content),
            Node::CodeBlock { language, content } => self.write_code_block(language, content),
            Node::UnorderedList(items) => self.write_unordered_list(items),
            Node::OrderedList { start, items } => self.write_ordered_list(*start, items),
            Node::ThematicBreak => self.write_thematic_break(),
            Node::Table {
                headers,
                rows,
                alignments,
            } => self.write_table(headers, rows, alignments),
            Node::Link {
                url,
                title,
                content,
            } => self.write_link(url, title, content),
            Node::Image { url, title, alt } => self.write_image(url, title, alt),
            Node::Emphasis(content) => self.write_emphasis(content),
            Node::Strong(content) => self.write_strong(content),
            Node::InlineCode(content) => self.write_inline_code(content),
            Node::Text(content) => self.write_text(content),
            Node::Html(content) => self.write_html(content),
            Node::SoftBreak => self.write_soft_break(),
            Node::HardBreak => self.write_hard_break(),
        }
    }

    /// Write a document node
    fn write_document(&mut self, children: &[Node]) -> fmt::Result {
        for (i, child) in children.iter().enumerate() {
            self.write(child)?;
            if i < children.len() - 1 {
                self.write_str("\n\n")?;
            }
        }
        Ok(())
    }

    /// Write a heading node
    fn write_heading(&mut self, level: u8, content: &[Node]) -> fmt::Result {
        if level < 1 || level > 6 {
            return Err(fmt::Error);
        }

        for _ in 0..level {
            self.write_char('#')?;
        }
        self.write_char(' ')?;

        for (i, node) in content.iter().enumerate() {
            self.write(node)?;
            if i < content.len() - 1 && !matches!(node, Node::SoftBreak | Node::HardBreak) {
                self.write_char(' ')?;
            }
        }

        Ok(())
    }

    /// Write a paragraph node
    fn write_paragraph(&mut self, content: &[Node]) -> fmt::Result {
        for node in content.iter() {
            self.write(node)?;
        }
        Ok(())
    }

    /// Write a blockquote node
    fn write_blockquote(&mut self, content: &[Node]) -> fmt::Result {
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
    fn write_code_block(&mut self, language: &Option<String>, content: &str) -> fmt::Result {
        self.write_str("```")?;
        if let Some(lang) = language {
            self.write_str(lang)?;
        }
        self.write_char('\n')?;
        self.write_str(content)?;

        // Ensure content ends with a newline
        if !content.ends_with('\n') {
            self.write_char('\n')?;
        }

        self.write_str("```")?;
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
        self.write_str(prefix)?;

        if item.is_task {
            if item.task_completed {
                self.write_str("[x] ")?;
            } else {
                self.write_str("[ ] ")?;
            }
        }

        self.indent_level += 1;

        for (i, node) in item.content.iter().enumerate() {
            self.write(node)?;
            if i < item.content.len() - 1 {
                self.write_str("\n")?;
                // Add indentation for child items
                for _ in 0..self.options.indent_spaces {
                    self.write_char(' ')?;
                }
            }
        }

        self.indent_level -= 1;
        Ok(())
    }

    /// Write a thematic break (horizontal rule)
    fn write_thematic_break(&mut self) -> fmt::Result {
        self.write_str("---")
    }

    /// Write a table
    fn write_table(
        &mut self,
        headers: &[Node],
        rows: &[Vec<Node>],
        alignments: &[Alignment],
    ) -> fmt::Result {
        // Write header
        self.write_char('|')?;
        for header in headers {
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
                self.write_char(' ')?;
                self.write(cell)?;
                self.write_str(" |")?;
            }
            self.write_char('\n')?;
        }

        Ok(())
    }

    /// Write a link
    fn write_link(&mut self, url: &str, title: &Option<String>, content: &[Node]) -> fmt::Result {
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

        self.write_char(')')
    }

    /// Write an image
    fn write_image(&mut self, url: &str, title: &Option<String>, alt: &str) -> fmt::Result {
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
    fn write_emphasis(&mut self, content: &[Node]) -> fmt::Result {
        self.write_char('*')?;

        for node in content {
            self.write(node)?;
        }

        self.write_char('*')
    }

    /// Write strong emphasis (bold)
    fn write_strong(&mut self, content: &[Node]) -> fmt::Result {
        self.write_str("**")?;

        for node in content {
            self.write(node)?;
        }

        self.write_str("**")
    }

    /// Write inline code
    fn write_inline_code(&mut self, content: &str) -> fmt::Result {
        self.write_char('`')?;
        self.write_str(content)?;
        self.write_char('`')
    }

    /// Write plain text
    fn write_text(&mut self, content: &str) -> fmt::Result {
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

    /// Write HTML
    fn write_html(&mut self, content: &str) -> fmt::Result {
        self.write_str(content)
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
    /// use cmark_rs::writer::CommonMarkWriter;
    /// use cmark_rs::ast::Node;
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
}

// Implement Display trait
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut writer = CommonMarkWriter::new();
        writer.write(self)?;
        write!(f, "{}", writer.into_string())
    }
}
