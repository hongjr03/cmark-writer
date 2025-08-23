//! Inline element writing functionality.

use super::utils::{escape_str, CommonMarkEscapes};
use super::CommonMarkWriter;
use crate::ast::Node;
use crate::error::{WriteError, WriteResult};
use ecow::EcoString;
use log;

impl CommonMarkWriter {
    /// Writes text content with character escaping
    pub fn write_text_content(&mut self, content: &str) -> WriteResult<()> {
        if self.options.escape_special_chars {
            let escaped = escape_str::<CommonMarkEscapes>(content);
            self.write_str(&escaped)?
        } else {
            self.write_str(content)?
        }

        Ok(())
    }

    /// Writes inline code content
    pub fn write_code_content(&mut self, content: &str) -> WriteResult<()> {
        self.write_char('`')?;
        self.write_str(content)?;
        self.write_char('`')?;
        Ok(())
    }

    /// Write an emphasis (italic) node with custom delimiter
    pub fn write_emphasis(&mut self, content: &[Node]) -> WriteResult<()> {
        let delimiter = self.options.emphasis_char.to_string();
        self.write_delimited(content, &delimiter)
    }

    /// Write a strong emphasis (bold) node with custom delimiter
    pub fn write_strong(&mut self, content: &[Node]) -> WriteResult<()> {
        let char = self.options.strong_char;
        let delimiter = format!("{}{}", char, char);
        self.write_delimited(content, &delimiter)
    }

    /// Write a strikethrough node (GFM extension)
    #[cfg(feature = "gfm")]
    pub fn write_strikethrough(&mut self, content: &[Node]) -> WriteResult<()> {
        if !self.options.enable_gfm || !self.options.gfm_strikethrough {
            // If GFM strikethrough is disabled, just write the content without strikethrough
            for node in content.iter() {
                self.write_node_content(node)?;
            }
            return Ok(());
        }

        // Write content with ~~ delimiters
        self.write_delimited(content, "~~")
    }

    /// Write a link
    pub fn write_link(
        &mut self,
        url: &str,
        title: &Option<EcoString>,
        content: &[Node],
    ) -> WriteResult<()> {
        for node in content {
            self.check_no_newline(node, "Link content")?;
        }
        self.write_char('[')?;

        for node in content {
            self.write_node_content(node)?;
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
    pub fn write_image(
        &mut self,
        url: &str,
        title: &Option<EcoString>,
        alt: &[Node],
    ) -> WriteResult<()> {
        // Check for newlines in alt text content
        for node in alt {
            self.check_no_newline(node, "Image alt text")?;
        }

        self.write_str("![")?;

        // Write alt text content
        for node in alt {
            self.write_node_content(node)?;
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
    pub fn write_soft_break(&mut self) -> WriteResult<()> {
        self.write_char('\n')?;
        Ok(())
    }

    /// Write a hard line break
    pub fn write_hard_break(&mut self) -> WriteResult<()> {
        if self.options.hard_break_spaces {
            self.write_str("  \n")?;
        } else {
            self.write_str("\\\n")?;
        }
        Ok(())
    }

    /// Write an autolink (URI or email address wrapped in < and >)
    pub fn write_autolink(&mut self, url: &str, is_email: bool) -> WriteResult<()> {
        // Autolinks shouldn't contain newlines
        if url.contains('\n') {
            if self.is_strict_mode() {
                return Err(WriteError::NewlineInInlineElement(
                    "Autolink URL".to_string().into(),
                ));
            } else {
                log::warn!(
                    "Newline character found in autolink URL '{}'. Writing it as is, which might result in an invalid link. Strict mode is off.",
                    url
                );
                // Continue to write the URL as is, including the newline.
            }
        }

        // Write the autolink with < and > delimiters
        self.write_char('<')?;

        // For email autolinks, we don't need to add any prefix
        // For URI autolinks, ensure it has a scheme
        if !is_email && !url.contains(':') {
            // Default to https if no scheme is provided
            self.write_str("https://")?;
        }

        self.write_str(url)?;
        self.write_char('>')?;

        Ok(())
    }

    /// Write an extended autolink (GFM extension)
    #[cfg(feature = "gfm")]
    pub fn write_extended_autolink(&mut self, url: &str) -> WriteResult<()> {
        if !self.options.gfm_autolinks {
            // If GFM autolinks are disabled, write as plain text
            self.write_text_content(url)?;
            return Ok(());
        }

        // Autolinks shouldn't contain newlines
        if url.contains('\n') {
            if self.is_strict_mode() {
                // Or a specific gfm_autolinks_strict option if desired
                return Err(WriteError::NewlineInInlineElement(
                    "Extended Autolink URL".to_string().into(),
                ));
            } else {
                log::warn!(
                    "Newline character found in extended autolink URL '{}'. Writing it as is, which might result in an invalid link. Strict mode is off.",
                    url
                );
                // Continue to write the URL as is, including the newline.
            }
        }

        // Just write the URL as plain text for extended autolinks (no angle brackets)
        self.write_str(url)?;

        Ok(())
    }

    /// Write a reference link
    pub fn write_reference_link(&mut self, label: &str, content: &[Node]) -> WriteResult<()> {
        // Check for newlines in content
        for node in content {
            self.check_no_newline(node, "Reference Link Text")?;
        }

        // If content is empty or exactly matches the label (as plain text),
        // this is a shortcut reference link: [label]
        if content.is_empty() {
            self.write_char('[')?;
            self.write_str(label)?;
            self.write_char(']')?;
            return Ok(());
        }

        // Check if content is exactly the same as the label (to use shortcut syntax)
        let is_shortcut =
            content.len() == 1 && matches!(&content[0], Node::Text(text) if text == label);

        if is_shortcut {
            // Use shortcut reference link syntax: [label]
            self.write_char('[')?;
            self.write_str(label)?;
            self.write_char(']')?;
        } else {
            // Use full reference link syntax: [content][label]
            self.write_char('[')?;

            for node in content {
                self.write_node_content(node)?;
            }

            self.write_str("][")?;
            self.write_str(label)?;
            self.write_char(']')?;
        }

        Ok(())
    }

    /// Write an AST HtmlElement node as raw HTML string into the CommonMark output.
    pub fn write_html_element(&mut self, element: &crate::ast::HtmlElement) -> WriteResult<()> {
        if self.options.strict {
            if element.tag.contains('<') || element.tag.contains('>') {
                return Err(WriteError::InvalidHtmlTag(element.tag.clone()));
            }

            for attr in &element.attributes {
                if attr.name.contains('<') || attr.name.contains('>') {
                    return Err(WriteError::InvalidHtmlAttribute(attr.name.clone()));
                }
            }
        }

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

        html_writer.write_node_internal(&Node::HtmlElement(element.clone()))?;

        // Get the generated HTML
        let html_output = html_writer.into_string();

        // Otherwise write the raw HTML
        self.write_str(&html_output)
    }
}
