//! Block-level element writing functionality with flexible newline control.

use super::CommonMarkWriter;
use crate::ast::{CodeBlockType, HeadingType, ListItem, Node};
use crate::error::{WriteError, WriteResult};
use crate::writer::context::NewlineContext;
use ecow::EcoString;
use log;

impl CommonMarkWriter {
    /// Write a heading node
    pub fn write_heading(
        &mut self,
        mut level: u8,
        content: &[Node],
        heading_type: &HeadingType,
    ) -> WriteResult<()> {
        // Validate heading level
        if level == 0 || level > 6 {
            if self.is_strict_mode() {
                return Err(WriteError::InvalidHeadingLevel(level));
            } else {
                let original_level = level;
                level = level.clamp(1, 6);
                log::warn!(
                    "Invalid heading level: {}. Corrected to {}. Strict mode is off.",
                    original_level,
                    level
                );
            }
        }

        match heading_type {
            HeadingType::Atx => {
                for _ in 0..level {
                    self.write_char('#')?;
                }
                self.write_char(' ')?;

                // Use inline context for heading content
                self.with_temporary_context(NewlineContext::pure_inline(), |writer| {
                    for node in content {
                        writer.write_node_content(node)?;
                    }
                    Ok(())
                })?;
            }
            HeadingType::Setext => {
                // Use inline context for heading content
                self.with_temporary_context(NewlineContext::pure_inline(), |writer| {
                    for node in content {
                        writer.write_node_content(node)?;
                    }
                    Ok(())
                })?;

                self.write_char('\n')?;
                let underline_char = if level == 1 { '=' } else { '-' };
                let min_len = 3;
                for _ in 0..min_len {
                    self.write_char(underline_char)?;
                }
            }
        }
        Ok(())
    }

    /// Write a paragraph node
    pub fn write_paragraph(&mut self, content: &[Node]) -> WriteResult<()> {
        // Use inline-with-blocks context to allow flexible content
        self.with_temporary_context(NewlineContext::inline_with_blocks(), |writer| {
            writer.write_paragraph_content(content)
        })
    }

    /// Write paragraph content without context switching
    fn write_paragraph_content(&mut self, content: &[Node]) -> WriteResult<()> {
        if self.options.trim_paragraph_trailing_hard_breaks {
            let mut last_non_hard_break_index = content.len();
            while last_non_hard_break_index > 0 {
                if !matches!(content[last_non_hard_break_index - 1], Node::HardBreak) {
                    break;
                }
                last_non_hard_break_index -= 1;
            }
            for node in content.iter().take(last_non_hard_break_index) {
                self.write_node_content(node)?;
            }
        } else {
            for node in content {
                self.write_node_content(node)?;
            }
        }
        Ok(())
    }

    /// Write a blockquote node
    pub fn write_blockquote(&mut self, content: &[Node]) -> WriteResult<()> {
        // Create a temporary writer buffer to write all blockquote content
        let mut temp_writer = CommonMarkWriter::with_context(
            self.options.clone(),
            NewlineContext::block(), // Use block context for blockquote content
        );

        // Write all content to temporary buffer
        for (i, node) in content.iter().enumerate() {
            if i > 0 {
                temp_writer.write_char('\n')?;
            }
            temp_writer.write_node(node)?;
        }

        // Get the content and apply blockquote prefix
        let blockquote_content = temp_writer.into_string();
        let formatted_content = self.apply_prefix(&blockquote_content, "> ", Some("> "));

        // Write formatted content
        self.buffer.push_str(&formatted_content);
        Ok(())
    }

    /// Write a code block node
    pub fn write_code_block(
        &mut self,
        language: &Option<EcoString>,
        content: &str,
        block_type: &CodeBlockType,
    ) -> WriteResult<()> {
        match block_type {
            CodeBlockType::Fenced => {
                // Write opening fence
                self.write_str("```")?;
                if let Some(lang) = language {
                    self.write_str(lang)?;
                }
                self.write_char('\n')?;

                // Write content (no processing needed for code blocks)
                self.write_str(content)?;

                // Ensure content ends with newline before closing fence
                if !content.ends_with('\n') {
                    self.write_char('\n')?;
                }

                // Write closing fence
                self.write_str("```")?;
            }
            CodeBlockType::Indented => {
                // Apply 4-space indentation to each line
                let indented_content = self.apply_prefix(content, "    ", Some("    "));
                self.buffer.push_str(&indented_content);

                // Remove trailing newline if present (context will handle it)
                if self.buffer.ends_with('\n') {
                    self.buffer.pop();
                }
            }
        }
        Ok(())
    }

    /// Write an unordered list
    pub fn write_unordered_list(&mut self, items: &[ListItem]) -> WriteResult<()> {
        self.with_temporary_context(NewlineContext::list_item(), |writer| {
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    writer.write_char('\n')?;
                }
                writer.write_list_item(item, None)?;
            }
            Ok(())
        })
    }

    /// Write an ordered list
    pub fn write_ordered_list(
        &mut self,
        items: &[ListItem],
        start: u32,
        tight: bool,
    ) -> WriteResult<()> {
        self.with_temporary_context(NewlineContext::list_item(), |writer| {
            let mut current_number = start;
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    if tight {
                        writer.write_char('\n')?;
                    } else {
                        writer.write_str("\n\n")?;
                    }
                }

                // Check if this item has a custom number
                let number = match item {
                    ListItem::Ordered {
                        number: Some(custom_num),
                        ..
                    } => {
                        current_number = *custom_num;
                        current_number
                    }
                    _ => {
                        // Use current sequential number
                        current_number
                    }
                };

                writer.write_list_item(item, Some(number))?;
                current_number += 1; // Increment for next item
            }
            Ok(())
        })
    }

    /// Write a list item
    fn write_list_item(&mut self, item: &ListItem, number: Option<u32>) -> WriteResult<()> {
        match item {
            ListItem::Unordered { content } => {
                if let Some(num) = number {
                    // In ordered list context, treat unordered items as ordered
                    self.write_str(&format!("{}. ", num))?;
                    let indent = format!("{}. ", num).len();
                    let indent_str = " ".repeat(indent);
                    self.write_list_item_content(content, &indent_str)?;
                } else {
                    // In unordered list context, use unordered marker
                    let marker = self.options.list_marker;
                    self.write_char(marker)?;
                    self.write_char(' ')?;
                    self.write_list_item_content(content, "  ")?;
                }
            }
            ListItem::Ordered {
                number: item_num,
                content,
            } => {
                let actual_number = number.or(*item_num).unwrap_or(1);
                self.write_str(&format!("{}. ", actual_number))?;
                let indent = format!("{}. ", actual_number).len();
                let indent_str = " ".repeat(indent);
                self.write_list_item_content(content, &indent_str)?;
            }
            #[cfg(feature = "gfm")]
            ListItem::Task { status, content } => {
                // Check if GFM task lists are enabled at runtime
                if self.options.gfm_tasklists {
                    let checkbox = match status {
                        crate::ast::TaskListStatus::Checked => "[x]",
                        crate::ast::TaskListStatus::Unchecked => "[ ]",
                    };
                    // Use appropriate prefix based on list type
                    if let Some(num) = number {
                        // Ordered list
                        self.write_str(&format!("{}. ", num))?;
                        self.write_str(checkbox)?;
                        self.write_char(' ')?;
                        let indent = format!("{}. ", num).len() + 4; // +4 for "[ ] "
                        let indent_str = " ".repeat(indent);
                        self.write_list_item_content(content, &indent_str)?;
                    } else {
                        // Unordered list
                        self.write_str("- ")?;
                        self.write_str(checkbox)?;
                        self.write_char(' ')?;
                        self.write_list_item_content(content, "    ")?;
                    }
                } else {
                    // When GFM task lists are disabled, render as regular list items
                    if let Some(num) = number {
                        // Ordered list
                        self.write_str(&format!("{}. ", num))?;
                        let indent = format!("{}. ", num).len();
                        let indent_str = " ".repeat(indent);
                        self.write_list_item_content(content, &indent_str)?;
                    } else {
                        // Unordered list
                        let marker = self.options.list_marker;
                        self.write_char(marker)?;
                        self.write_char(' ')?;
                        self.write_list_item_content(content, "  ")?;
                    }
                }
            }
        }
        Ok(())
    }

    /// Write list item content with proper indentation
    fn write_list_item_content(
        &mut self,
        content: &[Node],
        continuation_indent: &str,
    ) -> WriteResult<()> {
        // Create temporary writer for list item content
        let mut temp_writer = CommonMarkWriter::with_context(
            self.options.clone(),
            NewlineContext::list_item(), // Use list item context for proper spacing
        );

        // Write first node directly (inline with the marker)
        if let Some(first_node) = content.first() {
            temp_writer.write_node_content(first_node)?;

            // Handle remaining nodes with proper block spacing
            if content.len() > 1 {
                for node in &content[1..] {
                    if node.is_block() {
                        temp_writer.write_str("\n\n")?; // Add blank line before block elements
                    } else {
                        temp_writer.write_char('\n')?;
                    }
                    temp_writer.write_node_content(node)?;
                }
            }
        }

        // Get content and apply continuation indentation
        let item_content = temp_writer.into_string();
        if item_content.is_empty() {
            return Ok(());
        }

        // Apply indentation to continuation lines
        let formatted_content = self.apply_prefix(&item_content, continuation_indent, Some(""));
        self.buffer.push_str(&formatted_content);

        Ok(())
    }

    /// Write a thematic break
    pub fn write_thematic_break(&mut self) -> WriteResult<()> {
        let char = self.options.thematic_break_char;
        for _ in 0..3 {
            self.write_char(char)?;
        }
        Ok(())
    }

    /// Write an HTML block
    pub fn write_html_block(&mut self, content: &str) -> WriteResult<()> {
        self.buffer.push_str(content);

        // Context will handle trailing newline appropriately
        if self.buffer.ends_with('\n') {
            self.buffer.pop(); // Remove it so context can decide
        }

        Ok(())
    }

    /// Write a link reference definition
    pub fn write_link_reference_definition(
        &mut self,
        label: &str,
        destination: &str,
        title: &Option<EcoString>,
    ) -> WriteResult<()> {
        // Format: [label]: destination "optional title"
        self.write_char('[')?;
        self.write_str(label)?;
        self.write_str("]: ")?;
        self.write_str(destination)?;

        if let Some(title_text) = title {
            self.write_str(" \"")?;
            self.write_str(title_text)?;
            self.write_char('"')?;
        }

        // Don't add explicit trailing newline - let the context system handle it
        Ok(())
    }
}
