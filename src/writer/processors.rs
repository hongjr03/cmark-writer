//! Node processor implementations for CommonMark writer.
//!
//! This module contains the node processing strategies for handling different types of nodes.

use crate::ast::Node;
use crate::error::{WriteError, WriteResult};

// CommonMarkWriter is imported via super to avoid circular dependencies
use super::common_mark::CommonMarkWriter;

/// Private trait for node processing strategy
pub(crate) trait NodeProcessor {
    /// Process a node and write its content
    fn process(&self, writer: &mut CommonMarkWriter, node: &Node) -> WriteResult<()>;
}

/// Strategy for processing block nodes
pub(crate) struct BlockNodeProcessor;

/// Strategy for processing inline nodes
pub(crate) struct InlineNodeProcessor;

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
            Node::Custom(custom_node) if custom_node.is_block() => {
                writer.write_custom_node(custom_node)
            }
            _ => Err(WriteError::UnsupportedNodeType),
        }
    }
}

impl NodeProcessor for InlineNodeProcessor {
    fn process(&self, writer: &mut CommonMarkWriter, node: &Node) -> WriteResult<()> {
        // Check for newlines in inline nodes in strict mode
        if writer.is_strict_mode() && !matches!(node, Node::SoftBreak | Node::HardBreak) {
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
            Node::Custom(custom_node) if !custom_node.is_block() => {
                writer.write_custom_node(custom_node)
            }
            _ => Err(WriteError::UnsupportedNodeType),
        }
    }
}
