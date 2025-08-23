//! Table writing functionality.

use super::CommonMarkWriter;
use crate::ast::Node;
use crate::error::{WriteError, WriteResult};
use log;

#[cfg(feature = "gfm")]
use crate::ast::TableAlignment;

impl CommonMarkWriter {
    /// Check if a table contains any block-level elements in headers or cells
    pub(super) fn table_contains_block_elements(headers: &[Node], rows: &[Vec<Node>]) -> bool {
        // Check headers for block elements
        if headers.iter().any(|node| node.is_block()) {
            return true;
        }

        // Check all cells in all rows for block elements
        rows.iter()
            .any(|row| row.iter().any(|node| node.is_block()))
    }

    /// Write a table
    pub fn write_table(&mut self, headers: &[Node], rows: &[Vec<Node>]) -> WriteResult<()> {
        // Check if table contains block elements
        if Self::table_contains_block_elements(headers, rows) {
            if self.is_strict_mode() {
                // In strict mode, fail immediately if block elements are present
                return Err(WriteError::InvalidStructure(
                    "Table contains block-level elements which are not allowed in strict mode"
                        .to_string()
                        .into(),
                ));
            } else {
                // In soft mode, fallback to HTML
                log::info!(
                    "Table contains block-level elements, falling back to HTML output in soft mode"
                );
                return self.write_table_as_html(headers, rows);
            }
        }

        // Write header
        self.write_char('|')?;
        for header in headers {
            self.check_no_newline(header, "Table Header")?;
            self.write_char(' ')?;
            self.write_node_content(header)?;
            self.write_str(" |")?;
        }
        self.write_char('\n')?;

        // Write alignment row (default to centered if no alignments provided)
        self.write_char('|')?;
        for _ in 0..headers.len() {
            self.write_str(" --- |")?;
        }
        self.write_char('\n')?;

        // Write table content
        for row in rows {
            self.write_char('|')?;
            for cell in row {
                self.check_no_newline(cell, "Table Cell")?;
                self.write_char(' ')?;
                self.write_node_content(cell)?;
                self.write_str(" |")?;
            }
            self.write_char('\n')?;
        }

        // Don't add extra trailing newline - let the context system handle it
        Ok(())
    }

    #[cfg(feature = "gfm")]
    /// Write a table with alignment (GFM extension)
    pub fn write_table_with_alignment(
        &mut self,
        headers: &[Node],
        alignments: &[TableAlignment],
        rows: &[Vec<Node>],
    ) -> WriteResult<()> {
        // Only use alignment when GFM tables are enabled
        if !self.options.gfm_tables {
            return self.write_table(headers, rows);
        }

        // Check if table contains block elements
        if Self::table_contains_block_elements(headers, rows) {
            if self.is_strict_mode() {
                // In strict mode, fail immediately if block elements are present
                return Err(WriteError::InvalidStructure(
                    "GFM table contains block-level elements which are not allowed in strict mode"
                        .to_string()
                        .into(),
                ));
            } else {
                // In soft mode, fallback to HTML
                log::info!("GFM table contains block-level elements, falling back to HTML output in soft mode");
                return self.write_table_as_html_with_alignment(headers, alignments, rows);
            }
        }

        // Write header
        self.write_char('|')?;
        for header in headers {
            self.check_no_newline(header, "Table Header")?;
            self.write_char(' ')?;
            self.write_node_content(header)?;
            self.write_str(" |")?;
        }
        self.write_char('\n')?;

        // Write alignment row
        self.write_char('|')?;

        // Use provided alignments, or default to center if not enough alignments provided
        for i in 0..headers.len() {
            let alignment = if i < alignments.len() {
                &alignments[i]
            } else {
                &TableAlignment::Center
            };

            match alignment {
                TableAlignment::Left => self.write_str(" :--- |")?,
                TableAlignment::Center => self.write_str(" :---: |")?,
                TableAlignment::Right => self.write_str(" ---: |")?,
                TableAlignment::None => self.write_str(" --- |")?,
            }
        }

        self.write_char('\n')?;

        // Write table content
        for row in rows {
            self.write_char('|')?;
            for cell in row {
                self.check_no_newline(cell, "Table Cell")?;
                self.write_char(' ')?;
                self.write_node_content(cell)?;
                self.write_str(" |")?;
            }
            self.write_char('\n')?;
        }

        // Don't add extra trailing newline - let the context system handle it
        Ok(())
    }
}
