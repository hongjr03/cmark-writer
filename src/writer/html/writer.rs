use super::utils::{is_safe_attribute_name, is_safe_tag_name};
use super::{HtmlRenderOptions, HtmlWriteError, HtmlWriteResult};
#[cfg(feature = "gfm")]
use crate::ast::TaskListStatus;
use crate::ast::{ListItem, Node};
use log;
use std::io::{self, Write};

/// A writer for generating HTML output.
///
/// It buffers writes and provides methods for generating HTML tags, attributes, and text content,
/// ensuring proper escaping of special characters.
pub struct HtmlWriter<W: Write> {
    writer: W,
    buffer: String,
    tag_opened: bool, // Tracks if a start tag is opened (e.g. <tag) but not yet closed with > or />
}

impl<W: Write> HtmlWriter<W> {
    /// Creates a new `HtmlWriter` that writes to the given `writer`.
    pub fn new(writer: W) -> Self {
        HtmlWriter {
            writer,
            buffer: String::new(),
            tag_opened: false,
        }
    }

    fn ensure_tag_closed(&mut self) -> io::Result<()> {
        if self.tag_opened {
            self.buffer.push('>');
            self.tag_opened = false;
        }
        Ok(())
    }

    /// Writes the start of an HTML tag (e.g., initiates `<html>` or `<p`).
    /// Attributes can be added after this. Call `finish_tag` or write content/end_tag to close it.
    pub fn start_tag(&mut self, tag_name: &str) -> io::Result<()> {
        self.ensure_tag_closed()?; // Close any previously opened tag
        self.buffer.push('<');
        self.buffer.push_str(tag_name);
        self.tag_opened = true;
        Ok(())
    }

    /// Writes an HTML attribute (e.g., `class="example"`).
    /// Must be called after `start_tag` and before `finish_tag`, `text`, or `end_tag`.
    pub fn attribute(&mut self, key: &str, value: &str) -> io::Result<()> {
        if !self.tag_opened {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "attribute called without an open tag",
            ));
        }
        self.buffer.push(' ');
        self.buffer.push_str(key);
        self.buffer.push_str("=\"");
        escape_html_to_buffer(value, &mut self.buffer);
        self.buffer.push('"');
        Ok(())
    }

    /// Finishes an open start tag by writing `>`.
    pub fn finish_tag(&mut self) -> io::Result<()> {
        if self.tag_opened {
            self.buffer.push('>');
            self.tag_opened = false;
        }
        Ok(())
    }

    /// Writes the end of an HTML tag (e.g., `</html>`, `</p>`).
    /// This also ensures any opened start tag is finished.
    pub fn end_tag(&mut self, tag_name: &str) -> io::Result<()> {
        self.ensure_tag_closed()?;
        self.buffer.push_str("</");
        self.buffer.push_str(tag_name);
        self.buffer.push('>');
        Ok(())
    }

    /// Writes text content, escaping special HTML characters.
    /// This also ensures any opened start tag is finished.
    pub fn text(&mut self, text: &str) -> io::Result<()> {
        self.ensure_tag_closed()?;
        escape_html_to_buffer(text, &mut self.buffer);
        Ok(())
    }

    /// Writes a self-closing HTML tag (e.g., `<img />`, `<br />`).
    /// If attributes are needed, use `start_tag`, `attribute` calls, then `finish_self_closing_tag`.
    pub fn self_closing_tag(&mut self, tag_name: &str) -> io::Result<()> {
        self.ensure_tag_closed()?; // Close any previously opened tag.
        self.buffer.push('<');
        self.buffer.push_str(tag_name);
        self.buffer.push_str(" />");
        // self.tag_opened remains false as this tag is now complete.
        Ok(())
    }

    /// Finishes an open start tag as a self-closing tag by writing ` />`.
    pub fn finish_self_closing_tag(&mut self) -> io::Result<()> {
        if self.tag_opened {
            self.buffer.push_str(" />");
            self.tag_opened = false;
        }
        // Else: error or no-op? If no tag was opened, this is a usage error.
        // return Err(io::Error::new(io::ErrorKind::Other, "finish_self_closing_tag called without an open tag"));
        Ok(())
    }

    /// Writes a raw HTML string to the buffer without any escaping.
    /// This should be used with caution, only with HTML that is known to be safe.
    /// This also ensures any opened start tag is finished.
    pub fn raw_html(&mut self, html: &str) -> io::Result<()> {
        self.ensure_tag_closed()?;
        self.buffer.push_str(html);
        Ok(())
    }

    /// Writes a CommonMark AST `Node` to HTML using the provided options.
    /// This is the main rendering method for converting AST nodes to HTML.
    pub fn write_node(&mut self, node: &Node, options: &HtmlRenderOptions) -> HtmlWriteResult<()> {
        match node {
            Node::Document(children) => {
                for child in children {
                    self.write_node(child, options)?;
                }
                Ok(())
            }
            Node::Paragraph(children) => {
                self.start_tag("p")?;
                self.finish_tag()?;
                for child in children {
                    self.write_node(child, options)?;
                }
                self.end_tag("p")?;
                Ok(())
            }
            Node::Text(text) => {
                self.text(text)?;
                Ok(())
            }
            Node::Heading { level, content, .. } => {
                let tag_name = format!("h{}", level);
                self.start_tag(&tag_name)?;
                self.finish_tag()?;
                for child in content {
                    self.write_node(child, options)?;
                }
                self.end_tag(&tag_name)?;
                Ok(())
            }
            Node::Emphasis(children) => {
                self.start_tag("em")?;
                self.finish_tag()?;
                for child in children {
                    self.write_node(child, options)?;
                }
                self.end_tag("em")?;
                Ok(())
            }
            Node::Strong(children) => {
                self.start_tag("strong")?;
                self.finish_tag()?;
                for child in children {
                    self.write_node(child, options)?;
                }
                self.end_tag("strong")?;
                Ok(())
            }
            Node::ThematicBreak => {
                self.self_closing_tag("hr")?;
                self.raw_html("\n")?;
                Ok(())
            }
            Node::InlineCode(code) => {
                self.start_tag("code")?;
                self.finish_tag()?;
                self.text(code)?;
                self.end_tag("code")?;
                Ok(())
            }
            Node::CodeBlock {
                language, content, ..
            } => {
                self.start_tag("pre")?;
                if let Some(prefix) = &options.code_block_language_class_prefix {
                    if let Some(lang) = language {
                        if !lang.is_empty() {
                            self.attribute("class", &format!("{}{}", prefix, lang))?;
                        }
                    }
                }
                self.finish_tag()?;

                self.start_tag("code")?;
                self.finish_tag()?;

                self.text(content)?;
                self.end_tag("code")?;
                self.end_tag("pre")?;
                Ok(())
            }
            Node::HtmlBlock(block_content) => {
                self.raw_html(block_content)?;
                Ok(())
            }
            Node::HtmlElement(element) => {
                #[cfg(feature = "gfm")]
                if options.enable_gfm
                    && options
                        .gfm_disallowed_html_tags
                        .iter()
                        .any(|tag| tag.eq_ignore_ascii_case(&element.tag))
                {
                    self.textualize_full_element(element, options)?;
                    return Ok(());
                }

                if !is_safe_tag_name(&element.tag) {
                    if options.strict {
                        return Err(HtmlWriteError::InvalidHtmlTag(element.tag.clone()));
                    } else {
                        log::warn!(
                            "Invalid HTML tag name '{}' encountered. Textualizing entire element in non-strict mode.",
                            element.tag
                        );
                        self.textualize_full_element(element, options)?;
                        return Ok(());
                    }
                }
                self.start_tag(&element.tag)?;
                for attr in &element.attributes {
                    if !is_safe_attribute_name(&attr.name) {
                        if options.strict {
                            return Err(HtmlWriteError::InvalidHtmlAttribute(attr.name.clone()));
                        } else {
                            log::warn!(
                                "Invalid HTML attribute name '{}' in tag '{}' encountered. Textualizing attribute in non-strict mode.",
                                attr.name, element.tag
                            );
                            self.text(" ")?;
                            self.text(&attr.name)?;
                            self.text("=")?;
                            self.text("\"")?;
                            self.text(&attr.value)?;
                            self.text("\"")?;
                            continue;
                        }
                    }
                    self.attribute(&attr.name, &attr.value)?;
                }
                if element.self_closing {
                    self.finish_self_closing_tag()?;
                } else {
                    self.finish_tag()?;
                    for child in &element.children {
                        self.write_node(child, options)?;
                    }
                    self.end_tag(&element.tag)?;
                }
                Ok(())
            }
            Node::SoftBreak => {
                self.raw_html("\n")?;
                Ok(())
            }
            Node::HardBreak => {
                self.self_closing_tag("br")?;
                self.raw_html("\n")?;
                Ok(())
            }
            Node::Link {
                url,
                title,
                content,
            } => {
                self.start_tag("a")?;
                self.attribute("href", url)?;
                if let Some(title_str) = title {
                    self.attribute("title", title_str)?;
                }
                self.finish_tag()?;
                for child in content {
                    self.write_node(child, options)?;
                }
                self.end_tag("a")?;
                Ok(())
            }
            Node::Image { url, title, alt } => {
                self.start_tag("img")?;
                self.attribute("src", url)?;

                let mut alt_text_buffer = String::new();
                render_nodes_to_plain_text(alt, &mut alt_text_buffer, options);
                self.attribute("alt", &alt_text_buffer)?;

                if let Some(t) = title {
                    if !t.is_empty() {
                        self.attribute("title", t)?;
                    }
                }
                self.finish_self_closing_tag()?;
                Ok(())
            }
            Node::BlockQuote(children) => {
                self.start_tag("blockquote")?;
                self.finish_tag()?;
                for child in children {
                    self.write_node(child, options)?;
                }
                self.end_tag("blockquote")?;
                Ok(())
            }
            Node::OrderedList { start, items } => {
                self.start_tag("ol")?;
                if *start != 1 {
                    self.attribute("start", &start.to_string())?;
                }
                self.finish_tag()?;
                for item in items {
                    self.write_list_item(item, options)?;
                }
                self.end_tag("ol")?;
                Ok(())
            }
            Node::UnorderedList(items) => {
                self.start_tag("ul")?;
                self.finish_tag()?;
                for item in items {
                    self.write_list_item(item, options)?;
                }
                self.end_tag("ul")?;
                Ok(())
            }
            #[cfg(feature = "gfm")]
            Node::Strikethrough(children) => {
                self.start_tag("del")?;
                self.finish_tag()?;
                for child in children {
                    self.write_node(child, options)?;
                }
                self.end_tag("del")?;
                Ok(())
            }
            Node::Table {
                headers,
                #[cfg(feature = "gfm")]
                alignments,
                rows,
            } => {
                self.start_tag("table")?;
                self.finish_tag()?;

                self.start_tag("thead")?;
                self.finish_tag()?;
                self.start_tag("tr")?;
                self.finish_tag()?;
                for (i_idx, header_node) in headers.iter().enumerate() {
                    self.start_tag("th")?;
                    #[cfg(feature = "gfm")]
                    {
                        if i_idx < alignments.len() {
                            match alignments[i_idx] {
                                crate::ast::TableAlignment::Left => {
                                    self.attribute("style", "text-align: left;")?
                                }
                                crate::ast::TableAlignment::Center => {
                                    self.attribute("style", "text-align: center;")?
                                }
                                crate::ast::TableAlignment::Right => {
                                    self.attribute("style", "text-align: right;")?
                                }
                                crate::ast::TableAlignment::None => {}
                            }
                        }
                    }
                    #[cfg(not(feature = "gfm"))]
                    let _ = i_idx;

                    self.finish_tag()?;
                    self.write_node(header_node, options)?;
                    self.end_tag("th")?;
                }
                self.end_tag("tr")?;
                self.end_tag("thead")?;

                self.start_tag("tbody")?;
                self.finish_tag()?;
                for row_nodes in rows {
                    self.start_tag("tr")?;
                    self.finish_tag()?;
                    for (c_idx, cell_node) in row_nodes.iter().enumerate() {
                        self.start_tag("td")?;
                        #[cfg(feature = "gfm")]
                        {
                            if c_idx < alignments.len() {
                                match alignments[c_idx] {
                                    crate::ast::TableAlignment::Left => {
                                        self.attribute("style", "text-align: left;")?
                                    }
                                    crate::ast::TableAlignment::Center => {
                                        self.attribute("style", "text-align: center;")?
                                    }
                                    crate::ast::TableAlignment::Right => {
                                        self.attribute("style", "text-align: right;")?
                                    }
                                    crate::ast::TableAlignment::None => {}
                                }
                            }
                        }
                        #[cfg(not(feature = "gfm"))]
                        let _ = c_idx;

                        self.finish_tag()?;
                        self.write_node(cell_node, options)?;
                        self.end_tag("td")?;
                    }
                    self.end_tag("tr")?;
                }
                self.end_tag("tbody")?;
                self.end_tag("table")?;
                Ok(())
            }
            Node::Autolink { url, is_email } => {
                self.start_tag("a")?;
                let href = if *is_email && !url.starts_with("mailto:") {
                    format!("mailto:{}", url)
                } else {
                    url.clone()
                };
                self.attribute("href", &href)?;
                self.finish_tag()?;
                self.text(url)?;
                self.end_tag("a")?;
                Ok(())
            }
            Node::ExtendedAutolink(url) => {
                self.start_tag("a")?;
                self.attribute("href", url)?;
                self.finish_tag()?;
                self.text(url)?;
                self.end_tag("a")?;
                Ok(())
            }
            Node::LinkReferenceDefinition { .. } => Ok(()),
            Node::ReferenceLink { label, content } => {
                if options.strict {
                    Err(HtmlWriteError::UnsupportedNodeType(format!(
                        "Unresolved reference link '{}' encountered in strict mode.",
                        label
                    )))
                } else {
                    log::warn!(
                        "Unresolved reference link '{}' encountered. Rendering as plain text.",
                        label
                    );
                    self.text("[")?;
                    if content.is_empty() {
                        self.text(label)?;
                    } else {
                        for child in content {
                            self.write_node(
                                child,
                                &HtmlRenderOptions {
                                    strict: false,
                                    ..options.clone()
                                },
                            )?;
                        }
                    }
                    self.text("][")?;
                    self.text(label)?;
                    self.text("]")?;
                    Ok(())
                }
            }
            Node::Custom(custom_node) => {
                match custom_node.to_html_string(options) {
                    Ok(html_string) => self.raw_html(&html_string)?,
                    Err(e) => return Err(e),
                }
                Ok(())
            }
            #[cfg(not(feature = "gfm"))]
            other_node => Err(HtmlWriteError::UnsupportedNodeType(format!(
                "Node type {:?} is not supported for HTML conversion.",
                other_node
            ))),
        }
    }

    fn write_list_item(
        &mut self,
        list_item: &ListItem, // Correct type from ast::ListItem
        options: &HtmlRenderOptions,
    ) -> HtmlWriteResult<()> {
        self.start_tag("li")?;

        #[cfg(feature = "gfm")]
        if let ListItem::Task { status, .. } = list_item {
            if options.enable_gfm {
                let class_name = if *status == TaskListStatus::Checked {
                    "task-list-item task-list-item-checked"
                } else {
                    "task-list-item task-list-item-unchecked"
                };
                self.attribute("class", class_name)?;
            }
        }
        self.finish_tag()?;

        let item_content: &Vec<Node> = match list_item {
            ListItem::Unordered { content } => content,
            ListItem::Ordered { content, .. } => content,
            #[cfg(feature = "gfm")]
            ListItem::Task { content, .. } => content,
        };

        #[cfg(feature = "gfm")]
        if let ListItem::Task { status, .. } = list_item {
            if options.enable_gfm {
                self.start_tag("input")?;
                self.attribute("type", "checkbox")?;
                self.attribute("disabled", "")?;
                if *status == TaskListStatus::Checked {
                    self.attribute("checked", "")?;
                }
                self.finish_self_closing_tag()?;
                self.raw_html(" ")?; // Space after checkbox before content
            }
        }

        // Write content directly without wrapping in <p> for task list items
        for child_node in item_content {
            self.write_node(child_node, options)?;
        }

        self.end_tag("li")?;
        Ok(())
    }

    /// Helper method to render an entire HTML element (tag, attributes, children) as escaped text.
    /// This is used when a tag is disallowed (e.g., by GFM rules or due to unsafe characters in non-strict mode).
    fn textualize_full_element(
        &mut self,
        element: &crate::ast::HtmlElement,
        options: &HtmlRenderOptions,
    ) -> HtmlWriteResult<()> {
        self.text("<")?;
        self.text(&element.tag)?;
        for attr in &element.attributes {
            self.text(" ")?;
            self.text(&attr.name)?;
            self.text("=")?;
            self.text("\"")?;
            self.text(&attr.value)?;
            self.text("\"")?;
        }
        if element.self_closing {
            self.text(" />")?;
        } else {
            self.text(">")?;
            for child in &element.children {
                self.write_node(child, options)?;
            }
            self.text("</")?;
            self.text(&element.tag)?;
            self.text(">")?;
        }
        Ok(())
    }

    /// Flushes the buffer to the writer.
    pub fn flush(&mut self) -> io::Result<()> {
        if !self.buffer.is_empty() {
            let result = self.writer.write_all(self.buffer.as_bytes());
            self.buffer.clear();
            result?
        }
        Ok(())
    }
}

/// An extension trait for `Write` to provide a convenient `write_str` method.
pub trait WriteExt: Write {
    /// Writes a string slice to the writer.
    fn write_str(&mut self, s: &str) -> io::Result<usize> {
        self.write(s.as_bytes())
    }
}

impl<W: Write> WriteExt for W {}

// Helper function to escape HTML to a provided string buffer
fn escape_html_to_buffer(text: &str, buffer: &mut String) {
    for ch in text.chars() {
        match ch {
            '&' => buffer.push_str("&amp;"),
            '<' => buffer.push_str("&lt;"),
            '>' => buffer.push_str("&gt;"),
            '"' => buffer.push_str("&quot;"),
            '\'' => buffer.push_str("&#39;"),
            _ => buffer.push(ch),
        }
    }
}

// Helper function to render AST nodes to a plain text string for alt attributes
fn render_nodes_to_plain_text(nodes: &[Node], buffer: &mut String, _options: &HtmlRenderOptions) {
    for node in nodes {
        match node {
            Node::Text(text) => buffer.push_str(text),
            Node::Emphasis(children) | Node::Strong(children) => {
                render_nodes_to_plain_text(children, buffer, _options);
            }
            Node::Link { content, .. } => {
                render_nodes_to_plain_text(content, buffer, _options);
            }
            Node::Image { alt, .. } => {
                // Nested image in alt? Render its alt text.
                render_nodes_to_plain_text(alt, buffer, _options);
            }
            Node::InlineCode(code) => buffer.push_str(code),
            Node::SoftBreak => buffer.push(' '), // Replace soft breaks with a space
            Node::HardBreak => buffer.push(' '), // Replace hard breaks with a space (alt text is usually single line)
            Node::HtmlElement(element) => {
                // For HTML elements, try to get text content if any, or ignore.
                // This is a simplification; proper textualization of HTML can be complex.
                // Based on CommonMark Dingus, HTML tags are typically stripped.
                if !element.children.is_empty() {
                    render_nodes_to_plain_text(&element.children, buffer, _options);
                }
            }
            Node::Autolink { url, .. } => buffer.push_str(url),
            Node::ExtendedAutolink(url) => buffer.push_str(url),
            // Paragraphs and other block-level elements are unlikely/invalid directly in alt text.
            // If they appear, recurse to find any text, but this is non-standard.
            Node::Paragraph(children)
            | Node::BlockQuote(children)
            | Node::Heading {
                content: children, ..
            } => {
                render_nodes_to_plain_text(children, buffer, _options);
            }
            // Other node types are generally ignored for plain text alt representation.
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_simple_html_generation() {
        let mut buffer = Cursor::new(Vec::new());
        let mut html_writer = HtmlWriter::new(&mut buffer);

        html_writer.start_tag("html").unwrap();
        html_writer.finish_tag().unwrap(); // Explicitly finish tag
        html_writer.start_tag("body").unwrap();
        html_writer.finish_tag().unwrap();
        html_writer.start_tag("h1").unwrap();
        html_writer.finish_tag().unwrap();
        html_writer.text("Hello & <world>!").unwrap();
        html_writer.end_tag("h1").unwrap();
        html_writer.end_tag("body").unwrap();
        html_writer.end_tag("html").unwrap();
        html_writer.flush().unwrap();

        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert_eq!(
            output,
            "<html><body><h1>Hello &amp; &lt;world&gt;!</h1></body></html>"
        );
    }

    #[test]
    fn test_text_escaping() {
        let mut buffer = Cursor::new(Vec::new());
        let mut html_writer = HtmlWriter::new(&mut buffer);
        // Text implicitly closes any open tag, so no explicit finish_tag needed before it.
        html_writer
            .text("Text with \"quotes\" and 'apostrophes' & special <chars>.")
            .unwrap();
        html_writer.flush().unwrap();
        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert_eq!(
            output,
            "Text with &quot;quotes&quot; and &#39;apostrophes&#39; &amp; special &lt;chars&gt;."
        );
    }

    #[test]
    fn test_attributes() {
        let mut buffer = Cursor::new(Vec::new());
        let mut html_writer = HtmlWriter::new(&mut buffer);

        html_writer.start_tag("p").unwrap();
        html_writer.attribute("class", "greeting").unwrap();
        html_writer.attribute("id", "main-greeting").unwrap();
        html_writer.finish_tag().unwrap(); // Finish tag after attributes
        html_writer.text("Hello with attributes!").unwrap();
        html_writer.end_tag("p").unwrap();
        html_writer.flush().unwrap();

        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert_eq!(
            output,
            "<p class=\"greeting\" id=\"main-greeting\">Hello with attributes!</p>"
        );
    }

    #[test]
    fn test_self_closing_tag() {
        let mut buffer = Cursor::new(Vec::new());
        let mut html_writer = HtmlWriter::new(&mut buffer);

        html_writer.self_closing_tag("br").unwrap();
        html_writer.flush().unwrap();
        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert_eq!(output, "<br />");
    }

    #[test]
    fn test_self_closing_tag_with_attributes() {
        let mut buffer = Cursor::new(Vec::new());
        let mut html_writer = HtmlWriter::new(&mut buffer);

        html_writer.start_tag("img").unwrap();
        html_writer.attribute("src", "image.png").unwrap();
        html_writer
            .attribute("alt", "An example image with <special> chars & quotes \"")
            .unwrap();
        html_writer.finish_self_closing_tag().unwrap(); // Finish as self-closing
        html_writer.flush().unwrap();

        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert_eq!(output, "<img src=\"image.png\" alt=\"An example image with &lt;special&gt; chars &amp; quotes &quot;\" />");
    }

    #[test]
    fn test_mixed_content() {
        let mut buffer = Cursor::new(Vec::new());
        let mut html_writer = HtmlWriter::new(&mut buffer);

        html_writer.start_tag("div").unwrap();
        html_writer.attribute("id", "container").unwrap();
        // text() will call ensure_tag_closed -> finish_tag()
        html_writer.text("Some leading text.").unwrap();

        html_writer.start_tag("p").unwrap();
        html_writer.text("A paragraph inside the div.").unwrap();
        html_writer.end_tag("p").unwrap();

        html_writer.self_closing_tag("hr").unwrap();

        html_writer.start_tag("span").unwrap();
        // No attributes, text will close it.
        html_writer.text("More text.").unwrap();
        html_writer.end_tag("span").unwrap();

        html_writer.end_tag("div").unwrap();
        html_writer.flush().unwrap();

        let expected = "<div id=\"container\">Some leading text.<p>A paragraph inside the div.</p><hr /><span>More text.</span></div>";
        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_sequential_tags_without_content() {
        let mut buffer = Cursor::new(Vec::new());
        let mut html_writer = HtmlWriter::new(&mut buffer);

        html_writer.start_tag("div").unwrap();
        html_writer.finish_tag().unwrap();
        html_writer.start_tag("span").unwrap();
        html_writer.finish_tag().unwrap();
        html_writer.end_tag("span").unwrap();
        html_writer.end_tag("div").unwrap();
        html_writer.flush().unwrap();

        assert_eq!(
            String::from_utf8(buffer.into_inner()).unwrap(),
            "<div><span></span></div>"
        );
    }

    #[test]
    fn test_empty_tag() {
        let mut buffer = Cursor::new(Vec::new());
        let mut html_writer = HtmlWriter::new(&mut buffer);

        html_writer.start_tag("p").unwrap();
        html_writer.finish_tag().unwrap();
        html_writer.end_tag("p").unwrap();
        html_writer.flush().unwrap();

        assert_eq!(String::from_utf8(buffer.into_inner()).unwrap(), "<p></p>");
    }

    #[test]
    fn test_ensure_tag_closed_on_new_start_tag() {
        let mut buffer = Cursor::new(Vec::new());
        let mut html_writer = HtmlWriter::new(&mut buffer);

        html_writer.start_tag("div").unwrap(); // <div
        html_writer.attribute("class", "outer").unwrap(); // <div class="outer"
        html_writer.start_tag("p").unwrap(); // Should close div: <div class="outer"><p
        html_writer.text("hello").unwrap(); // <div class="outer"><p>hello
        html_writer.end_tag("p").unwrap(); // <div class="outer"><p>hello</p>
        html_writer.end_tag("div").unwrap(); // <div class="outer"><p>hello</p></div>
        html_writer.flush().unwrap();

        let expected = "<div class=\"outer\"><p>hello</p></div>";
        assert_eq!(String::from_utf8(buffer.into_inner()).unwrap(), expected);
    }
}
