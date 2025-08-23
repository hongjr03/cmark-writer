//! Main CommonMark writer struct and core functionality.

use crate::ast::{CustomNode, Node};
use crate::error::{WriteError, WriteResult};
use crate::options::WriterOptions;
use crate::writer::context::{NewlineContext, NewlineStrategy, RenderingMode};
use ecow::EcoString;
use std::fmt;

/// CommonMark writer with flexible newline control
///
/// This writer uses a context-based system for intelligent newline handling,
/// allowing fine-grained control over formatting in different scenarios.
#[derive(Debug)]
pub struct CommonMarkWriter {
    /// Writer options
    pub options: WriterOptions,
    /// Buffer for storing the output text
    pub(super) buffer: EcoString,
    /// Current rendering context
    context: NewlineContext,
}

impl CommonMarkWriter {
    /// Create a new CommonMark writer with default options
    ///
    /// # Example
    ///
    /// ```
    /// use cmark_writer::writer::CommonMarkWriter;
    /// use cmark_writer::ast::Node;
    /// use cmark_writer::ToCommonMark;
    ///
    /// let mut writer = CommonMarkWriter::new();
    /// Node::Text("Hello".into()).to_commonmark(&mut writer).unwrap();
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
    ///     ..Default::default()       // Other options can be set as needed
    /// };
    /// let writer = CommonMarkWriter::with_options(options);
    /// ```
    pub fn with_options(options: WriterOptions) -> Self {
        Self {
            options,
            buffer: EcoString::new(),
            context: NewlineContext::block(),
        }
    }

    /// Create a writer with a specific rendering context
    pub fn with_context(options: WriterOptions, context: NewlineContext) -> Self {
        Self {
            options,
            buffer: EcoString::new(),
            context,
        }
    }

    /// Whether the writer is in strict mode
    pub(super) fn is_strict_mode(&self) -> bool {
        self.options.strict
    }

    /// Apply a specific prefix to multi-line text, used for handling container node indentation
    ///
    /// # Parameters
    ///
    /// * `content` - The multi-line content to process
    /// * `prefix` - The prefix to apply to each line
    /// * `first_line_prefix` - The prefix to apply to the first line (can be different from other lines)
    ///
    /// # Returns
    ///
    /// Returns a string with applied indentation
    pub(super) fn apply_prefix(
        &self,
        content: &str,
        prefix: &str,
        first_line_prefix: Option<&str>,
    ) -> EcoString {
        if content.is_empty() {
            return EcoString::new();
        }

        let mut result = EcoString::new();
        let lines: Vec<&str> = content.lines().collect();

        if !lines.is_empty() {
            let actual_prefix = first_line_prefix.unwrap_or(prefix);
            result.push_str(actual_prefix);
            result.push_str(lines[0]);
        }

        for line in &lines[1..] {
            result.push('\n');
            result.push_str(prefix);
            result.push_str(line);
        }

        result
    }

    /// Write document children with proper spacing
    pub(super) fn write_document_children(&mut self, children: &[Node]) -> WriteResult<()> {
        for (i, node) in children.iter().enumerate() {
            if i > 0 {
                self.write_node_separator(&children[i - 1], node)?;
            }

            // For the last child, be selective about trailing newlines
            if i == children.len() - 1 {
                // If it's a block element, add trailing newline
                if node.is_block() {
                    self.write_node(node)?;
                } else {
                    // For inline elements, don't add trailing newline
                    self.write_node_content(node)?;
                }
            } else {
                self.write_node(node)?;
            }
        }
        Ok(())
    }

    /// Write node content without context-aware newline handling
    /// This is called by write_node() which handles the newline logic
    pub fn write_node_content(&mut self, node: &Node) -> WriteResult<()> {
        // 处理自定义节点
        if let Node::Custom(custom_node) = node {
            // Ensure that CustomNode trait requires render_commonmark method
            return custom_node.render_commonmark(self);
        }

        // 处理文档节点
        if let Node::Document(children) = node {
            return self.write_document_children(children);
        }

        // 在严格模式下检查内联元素中的换行符
        if self.options.strict
            && !node.is_block()
            && !matches!(node, Node::SoftBreak | Node::HardBreak)
        {
            match node {
                Node::Text(content) => {
                    if content.contains('\n') {
                        return Err(WriteError::NewlineInInlineElement("Text".into()));
                    }
                }
                Node::InlineCode(content) => {
                    if content.contains('\n') {
                        return Err(WriteError::NewlineInInlineElement("InlineCode".into()));
                    }
                }
                Node::Emphasis(children) | Node::Strong(children) => {
                    for child in children {
                        if let Node::Text(content) = child {
                            if content.contains('\n') {
                                return Err(WriteError::NewlineInInlineElement(
                                    "Text in formatting".into(),
                                ));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // Delegate to specific writing methods
        match node {
            // Block elements
            Node::Heading {
                level,
                content,
                heading_type,
            } => self.write_heading(*level, content, heading_type),
            Node::Paragraph(content) => self.write_paragraph(content),
            Node::BlockQuote(content) => self.write_blockquote(content),
            Node::CodeBlock {
                language,
                content,
                block_type,
            } => self.write_code_block(language, content, block_type),
            Node::UnorderedList(items) => self.write_unordered_list(items),
            Node::OrderedList { start, items } => self.write_ordered_list(items, *start, true), // Default to tight
            Node::ThematicBreak => self.write_thematic_break(),

            // Inline elements
            Node::Text(content) => self.write_text_content(content),
            Node::Emphasis(content) => self.write_emphasis(content),
            Node::Strong(content) => self.write_strong(content),
            Node::InlineCode(content) => self.write_code_content(content),
            Node::Link {
                url,
                title,
                content,
            } => self.write_link(url, title, content),
            Node::Image { url, title, alt } => self.write_image(url, title, alt),
            Node::SoftBreak => self.write_soft_break(),
            Node::HardBreak => self.write_hard_break(),
            Node::Autolink { url, is_email } => self.write_autolink(url, *is_email),
            Node::ReferenceLink { label, content } => self.write_reference_link(label, content),
            Node::LinkReferenceDefinition {
                label,
                destination,
                title,
            } => self.write_link_reference_definition(label, destination, title),

            // HTML elements
            Node::HtmlBlock(content) => self.write_html_block(content),
            Node::HtmlElement(element) => self.write_html_element(element),

            // Table elements
            #[cfg(feature = "gfm")]
            Node::Table {
                headers,
                alignments,
                rows,
            } => self.write_table_with_alignment(headers, alignments, rows),
            #[cfg(not(feature = "gfm"))]
            Node::Table { headers, rows, .. } => self.write_table(headers, rows),

            // GFM-specific elements
            #[cfg(feature = "gfm")]
            Node::Strikethrough(content) => self.write_strikethrough(content),
            #[cfg(feature = "gfm")]
            Node::ExtendedAutolink(url) => self.write_extended_autolink(url),

            // Custom nodes
            Node::Custom(custom_node) => self.write_custom_node(custom_node),

            _ => {
                log::warn!("Unsupported node type encountered and skipped: {:?}", node);
                Ok(())
            }
        }
    }

    /// Write a custom node using its implementation
    #[allow(clippy::borrowed_box)]
    pub(super) fn write_custom_node(&mut self, node: &Box<dyn CustomNode>) -> WriteResult<()> {
        node.render_commonmark(self)
    }

    /// Check if the inline node contains a newline character and return an error if it does
    pub(super) fn check_no_newline(&self, node: &Node, context: &str) -> WriteResult<()> {
        if Self::node_contains_newline(node) {
            if self.is_strict_mode() {
                return Err(WriteError::NewlineInInlineElement(
                    context.to_string().into(),
                ));
            } else {
                log::warn!(
                    "Newline character found in inline element '{}', but non-strict mode allows it (output may be affected).",
                    context
                );
            }
        }
        Ok(())
    }

    /// Check if the inline node contains a newline character recursively
    pub(super) fn node_contains_newline(node: &Node) -> bool {
        match node {
            Node::Text(s) | Node::InlineCode(s) => s.contains('\n'),
            Node::Emphasis(children) | Node::Strong(children) => {
                children.iter().any(Self::node_contains_newline)
            }
            #[cfg(feature = "gfm")]
            Node::Strikethrough(children) => children.iter().any(Self::node_contains_newline),
            Node::HtmlElement(element) => element.children.iter().any(Self::node_contains_newline),
            Node::Link { content, .. } => content.iter().any(Self::node_contains_newline),
            Node::Image { alt, .. } => alt.iter().any(Self::node_contains_newline),
            Node::SoftBreak | Node::HardBreak => true,
            // Custom nodes are handled separately
            Node::Custom(_) => false,
            _ => false,
        }
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
    /// use cmark_writer::ToCommonMark;
    ///
    /// let mut writer = CommonMarkWriter::new();
    /// Node::Text("Hello".into()).to_commonmark(&mut writer).unwrap();
    /// let result = writer.into_string();
    /// assert_eq!(result, "Hello");
    /// ```
    pub fn into_string(self) -> EcoString {
        self.buffer
    }

    /// Write a string to the output buffer
    ///
    /// This method is provided for custom node implementations to use
    pub fn write_str(&mut self, s: &str) -> WriteResult<()> {
        self.buffer.push_str(s);
        Ok(())
    }

    /// Write a character to the output buffer
    ///
    /// This method is provided for custom node implementations to use
    pub fn write_char(&mut self, c: char) -> WriteResult<()> {
        self.buffer.push(c);
        Ok(())
    }

    /// Get current rendering context
    pub fn context(&self) -> &NewlineContext {
        &self.context
    }

    /// Get current rendering context (alias for backwards compatibility with tests)
    pub fn current_context(&self) -> &NewlineContext {
        &self.context
    }

    /// Set new rendering context
    pub fn set_context(&mut self, context: NewlineContext) {
        self.context = context;
    }

    /// Push a new context (for stack-based context management in tests)
    pub fn push_context(&mut self, context: NewlineContext) {
        // For simplicity in tests, just replace current context
        // In the future, we could implement a real stack if needed
        self.context = context;
    }

    /// Pop the current context (for stack-based context management in tests)
    pub fn pop_context(&mut self) -> Option<NewlineContext> {
        // Return to default context
        let old = std::mem::take(&mut self.context);
        Some(old)
    }

    /// Execute a closure with a temporary context
    pub fn with_temporary_context<F, R>(&mut self, context: NewlineContext, f: F) -> WriteResult<R>
    where
        F: FnOnce(&mut Self) -> WriteResult<R>,
    {
        let original_context = std::mem::replace(&mut self.context, context);
        let result = f(self);
        self.context = original_context;
        result
    }

    /// Execute a closure with a temporary context (alias for examples)
    pub fn with_temp_context<F, R>(&mut self, context: NewlineContext, f: F) -> WriteResult<R>
    where
        F: FnOnce(&mut Self) -> WriteResult<R>,
    {
        self.with_temporary_context(context, f)
    }

    /// Write a single node with context-aware formatting
    pub fn write_node(&mut self, node: &Node) -> WriteResult<()> {
        // Handle document nodes specially - they manage their own newlines
        if let Node::Document(children) = node {
            return self.write_document_children(children);
        }

        // Validate node is allowed in current context
        self.context.validate_node(node)?;

        // Remember buffer state before writing
        let buffer_start = self.buffer.len();

        // Write the actual node content
        self.write_node_content(node)?;

        // Get the content that was just written
        let new_content = &self.buffer[buffer_start..];

        // Apply context-aware trailing newline logic
        if self
            .context
            .should_add_trailing_newline(new_content, Some(node))
        {
            self.write_char('\n')?;
        }

        Ok(())
    }

    /// Write multiple nodes with intelligent spacing
    pub fn write_nodes(&mut self, nodes: &[Node]) -> WriteResult<()> {
        for (i, node) in nodes.iter().enumerate() {
            if i > 0 {
                self.write_node_separator(&nodes[i - 1], node)?;
            }
            self.write_node(node)?;
        }
        Ok(())
    }

    /// Write multiple nodes with intelligent spacing (alias for examples)
    pub fn write_nodes_with_context(&mut self, nodes: &[Node]) -> WriteResult<()> {
        self.write_nodes(nodes)
    }

    /// Write content with a specific node context (for examples)
    pub fn write_content_with_context(
        &mut self,
        _node: &Node,
        content_fn: impl FnOnce(&mut Self) -> WriteResult<()>,
    ) -> WriteResult<()> {
        content_fn(self)
    }

    /// Write separator between nodes based on context
    fn write_node_separator(&mut self, prev_node: &Node, current_node: &Node) -> WriteResult<()> {
        match self.context.mode {
            RenderingMode::Block => {
                // Traditional block spacing
                if prev_node.is_block() && current_node.is_block() {
                    self.ensure_double_newline()?;
                }
            }
            RenderingMode::InlineWithBlocks => {
                // Smart spacing for mixed content
                if prev_node.is_block() || current_node.is_block() {
                    self.ensure_single_newline()?;
                }
            }
            RenderingMode::PureInline => {
                // No automatic spacing
            }
            RenderingMode::TableCell => {
                // Space separation
                if !self.buffer.ends_with(' ') && !self.buffer.is_empty() {
                    self.write_char(' ')?;
                }
            }
            RenderingMode::ListItem => {
                // Conditional newlines for list items
                if prev_node.is_block() && current_node.is_block() {
                    self.ensure_single_newline()?;
                }
            }
            RenderingMode::Custom => {
                // Custom logic based on strategy
                if (prev_node.is_block() || current_node.is_block())
                    && self.context.strategy == NewlineStrategy::Always
                {
                    self.ensure_single_newline()?;
                }
            }
        }
        Ok(())
    }

    /// Ensure buffer ends with a single newline
    fn ensure_single_newline(&mut self) -> WriteResult<()> {
        if !self.buffer.ends_with('\n') {
            self.write_char('\n')?;
        }
        Ok(())
    }

    /// Ensure buffer ends with a double newline
    fn ensure_double_newline(&mut self) -> WriteResult<()> {
        if self.buffer.ends_with("\n\n") {
            // Already has double newline
        } else if self.buffer.ends_with('\n') {
            self.write_char('\n')?;
        } else {
            self.write_str("\n\n")?;
        }
        Ok(())
    }

    /// Helper function for writing content with delimiters
    pub(super) fn write_delimited(&mut self, content: &[Node], delimiter: &str) -> WriteResult<()> {
        self.write_str(delimiter)?;

        // Use pure inline context for delimited content (like emphasis, strong, etc.)
        let original_context = std::mem::replace(&mut self.context, NewlineContext::pure_inline());

        for node in content {
            self.write_node_content(node)?;
        }

        self.context = original_context;
        self.write_str(delimiter)?;
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
        let result = if self.is_block() {
            writer.write_node(self)
        } else {
            // For inline elements, don't add automatic trailing newlines
            writer.write_node_content(self)
        };
        match result {
            Ok(_) => write!(f, "{}", writer.into_string()),
            Err(e) => write!(f, "Error writing Node: {}", e),
        }
    }
}
