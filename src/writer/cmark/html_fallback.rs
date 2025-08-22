//! HTML fallback handling for tables with block elements.

use super::CommonMarkWriter;
use crate::ast::Node;
use crate::error::{WriteError, WriteResult};

#[cfg(feature = "gfm")]
use crate::ast::TableAlignment;

impl CommonMarkWriter {
    /// Write a table as HTML (fallback for tables with block-level elements)
    pub(super) fn write_table_as_html(
        &mut self,
        headers: &[Node],
        rows: &[Vec<Node>],
    ) -> WriteResult<()> {
        use crate::writer::html::{HtmlWriter, HtmlWriterOptions};

        let html_options = if let Some(ref custom_options) = self.options.html_writer_options {
            custom_options.clone()
        } else {
            HtmlWriterOptions {
                strict: self.options.strict,
                code_block_language_class_prefix: Some("language-".into()),
                #[cfg(feature = "gfm")]
                enable_gfm: self.options.enable_gfm,
                #[cfg(feature = "gfm")]
                gfm_disallowed_html_tags: self.options.gfm_disallowed_html_tags.clone(),
            }
        };

        let mut html_writer = HtmlWriter::with_options(html_options);

        // Create table node for HTML writer
        let table_node = Node::Table {
            headers: headers.to_vec(),
            #[cfg(feature = "gfm")]
            alignments: vec![],
            rows: rows.to_vec(),
        };

        html_writer.write_node_internal(&table_node).map_err(|_| {
            WriteError::HtmlFallbackError("Failed to write table as HTML".to_string().into())
        })?;

        let html_output = html_writer.into_string();
        self.buffer.push_str(&html_output);

        Ok(())
    }

    #[cfg(feature = "gfm")]
    /// Write a GFM table with alignment as HTML (fallback for tables with block-level elements)
    pub(super) fn write_table_as_html_with_alignment(
        &mut self,
        headers: &[Node],
        alignments: &[TableAlignment],
        rows: &[Vec<Node>],
    ) -> WriteResult<()> {
        use crate::writer::html::{HtmlWriter, HtmlWriterOptions};

        let html_options = if let Some(ref custom_options) = self.options.html_writer_options {
            custom_options.clone()
        } else {
            HtmlWriterOptions {
                strict: self.options.strict,
                code_block_language_class_prefix: Some("language-".into()),
                #[cfg(feature = "gfm")]
                enable_gfm: self.options.enable_gfm,
                #[cfg(feature = "gfm")]
                gfm_disallowed_html_tags: self.options.gfm_disallowed_html_tags.clone(),
            }
        };

        let mut html_writer = HtmlWriter::with_options(html_options);

        // Create table node for HTML writer
        let table_node = Node::Table {
            headers: headers.to_vec(),
            alignments: alignments.to_vec(),
            rows: rows.to_vec(),
        };

        html_writer.write_node_internal(&table_node).map_err(|_| {
            WriteError::HtmlFallbackError("Failed to write GFM table as HTML".to_string().into())
        })?;

        let html_output = html_writer.into_string();
        self.buffer.push_str(&html_output);

        Ok(())
    }
}
