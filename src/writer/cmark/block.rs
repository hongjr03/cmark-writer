//! Block-level element writing functionality.

use super::CommonMarkWriter;
use crate::ast::{CodeBlockType, HeadingType, ListItem, Node};
use crate::error::{WriteError, WriteResult};
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
        // 验证标题级别
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
                for node in content {
                    self.write_node_internal(node)?;
                }
                // Add trailing newline for ATX headings
                self.write_char('\n')?;
            }
            HeadingType::Setext => {
                for node in content {
                    self.write_node_internal(node)?;
                }
                self.write_char('\n')?;
                let underline_char = if level == 1 { '=' } else { '-' };
                let min_len = 3;
                for _ in 0..min_len {
                    self.write_char(underline_char)?;
                }
                // Add trailing newline for Setext headings
                self.write_char('\n')?;
            }
        }
        Ok(())
    }

    /// Write a paragraph node
    pub fn write_paragraph(&mut self, content: &[Node]) -> WriteResult<()> {
        self.write_paragraph_content(content)?;
        // Always add trailing newline for paragraphs
        self.write_char('\n')?;
        Ok(())
    }

    /// Write paragraph content without trailing newline
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
                self.write_node_internal(node)?;
            }
        } else {
            for node in content {
                self.write_node_internal(node)?;
            }
        }
        Ok(())
    }

    /// Write a blockquote node
    pub fn write_blockquote(&mut self, content: &[Node]) -> WriteResult<()> {
        // Create a temporary writer buffer to write all blockquote content
        let mut temp_writer = CommonMarkWriter::with_options(self.options.clone());

        // Write all content to temporary buffer
        for (i, node) in content.iter().enumerate() {
            if i > 0 {
                temp_writer.write_char('\n')?;
            }
            // Write all nodes uniformly
            temp_writer.write_node_internal(node)?;
        }

        // Get all content
        let all_content = temp_writer.into_string();

        // Apply blockquote prefix "> " uniformly
        let prefix = "> ";
        let formatted_content = self.apply_prefix(&all_content, prefix, Some(prefix));

        // Write formatted content
        self.buffer.push_str(&formatted_content);
        // Always add trailing newline for blockquotes
        self.write_char('\n')?;
        Ok(())
    }

    /// Write a thematic break (horizontal rule)
    pub fn write_thematic_break(&mut self) -> WriteResult<()> {
        let char = self.options.thematic_break_char;
        self.write_str(&format!("{}{}{}", char, char, char))?;
        self.write_char('\n')?;
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
            CodeBlockType::Indented => {
                let indent = "    ";
                let indented_content = self.apply_prefix(content, indent, Some(indent));
                self.buffer.push_str(&indented_content);
            }
            CodeBlockType::Fenced => {
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

                let fence_len = std::cmp::max(max_backticks + 1, 3);
                let fence = "`".repeat(fence_len);

                self.write_str(&fence)?;
                if let Some(lang) = language {
                    self.write_str(lang)?;
                }
                self.write_char('\n')?;

                self.buffer.push_str(content);
                if !content.ends_with('\n') {
                    self.write_char('\n')?;
                }

                self.write_str(&fence)?;
            }
        }

        // Always add trailing newline for code blocks
        self.write_char('\n')?;
        Ok(())
    }

    /// Write an unordered list node
    pub fn write_unordered_list(&mut self, items: &[ListItem]) -> WriteResult<()> {
        let list_marker = self.options.list_marker;
        let prefix = format!("{} ", list_marker);

        for (i, item) in items.iter().enumerate() {
            if i > 0 {
                self.write_char('\n')?;
            }
            self.write_list_item(item, &prefix)?;
        }

        // Always add trailing newline for lists
        self.write_char('\n')?;
        Ok(())
    }

    /// Write an ordered list node
    pub fn write_ordered_list(&mut self, start: u32, items: &[ListItem]) -> WriteResult<()> {
        // Track the current item number
        let mut current_number = start;

        for (i, item) in items.iter().enumerate() {
            if i > 0 {
                self.write_char('\n')?;
            }

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
        }

        // Always add trailing newline for ordered lists
        self.write_char('\n')?;
        Ok(())
    }

    /// Write a list item
    fn write_list_item(&mut self, item: &ListItem, prefix: &str) -> WriteResult<()> {
        match item {
            ListItem::Unordered { content } => {
                self.write_str(prefix)?;
                self.write_list_item_content(content, prefix.len())?;
            }
            ListItem::Ordered { number, content } => {
                if let Some(num) = number {
                    let custom_prefix = format!("{}. ", num);
                    self.write_str(&custom_prefix)?;
                    self.write_list_item_content(content, custom_prefix.len())?;
                } else {
                    self.write_str(prefix)?;
                    self.write_list_item_content(content, prefix.len())?;
                }
            }
            #[cfg(feature = "gfm")]
            ListItem::Task { status, content } => {
                // Only use task list syntax if GFM task lists are enabled
                if self.options.gfm_tasklists {
                    let checkbox = match status {
                        crate::ast::TaskListStatus::Checked => "[x] ",
                        crate::ast::TaskListStatus::Unchecked => "[ ] ",
                    };

                    // Use the original list marker (- or number) and append the checkbox
                    let task_prefix = format!("{}{}", prefix, checkbox);
                    self.write_str(&task_prefix)?;
                    self.write_list_item_content(content, task_prefix.len())?;
                } else {
                    // If GFM task lists are disabled, just write a normal list item
                    self.write_str(prefix)?;
                    self.write_list_item_content(content, prefix.len())?;
                }
            }
        }

        Ok(())
    }

    /// Write list item content
    fn write_list_item_content(&mut self, content: &[Node], prefix_len: usize) -> WriteResult<()> {
        if content.is_empty() {
            return Ok(());
        }

        let mut temp_writer = CommonMarkWriter::with_options(self.options.clone());

        for (i, node) in content.iter().enumerate() {
            if i > 0 {
                temp_writer.write_char('\n')?;
            }

            temp_writer.write_node_internal(node)?;
        }

        let all_content = temp_writer.into_string();

        let indent = " ".repeat(prefix_len);
        let formatted_content = self.apply_prefix(&all_content, &indent, Some(""));

        self.buffer.push_str(&formatted_content);

        Ok(())
    }

    /// Write an HTML block
    pub fn write_html_block(&mut self, content: &str) -> WriteResult<()> {
        self.buffer.push_str(content);
        // Always add trailing newline for HTML blocks if not already present
        if !content.ends_with('\n') {
            self.write_char('\n')?;
        }
        Ok(())
    }
}
