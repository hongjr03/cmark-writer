//! 新的节点处理器实现
//!
//! 使用新的trait架构重写的处理器系统

use crate::ast::Node;
use crate::error::{WriteError, WriteResult};
use crate::traits::{
    BlockNodeProcessor, ConfigurableProcessor, InlineNodeProcessor, NodeProcessor, Writer,
};

/// 块级处理器配置
#[derive(Debug, Clone)]
pub struct BlockProcessorConfig {
    /// 是否确保尾部换行
    pub ensure_trailing_newlines: bool,
    /// 块级分隔符
    pub block_separator: String,
}

impl Default for BlockProcessorConfig {
    fn default() -> Self {
        Self {
            ensure_trailing_newlines: true,
            block_separator: "\n\n".to_string(),
        }
    }
}

/// 内联处理器配置
#[derive(Debug, Clone)]
pub struct InlineProcessorConfig {
    /// 严格模式验证
    pub strict_validation: bool,
    /// 是否允许换行
    pub allow_newlines: bool,
}

impl Default for InlineProcessorConfig {
    fn default() -> Self {
        Self {
            strict_validation: true,
            allow_newlines: false,
        }
    }
}

/// 增强的块级节点处理器
#[derive(Debug)]
pub struct EnhancedBlockProcessor {
    config: BlockProcessorConfig,
}

impl EnhancedBlockProcessor {
    /// 创建新的块级处理器
    pub fn new() -> Self {
        Self {
            config: BlockProcessorConfig::default(),
        }
    }

    /// 使用自定义配置创建
    pub fn with_config(config: BlockProcessorConfig) -> Self {
        Self { config }
    }
}

impl Default for EnhancedBlockProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for EnhancedBlockProcessor {
    fn can_process(&self, node: &Node) -> bool {
        matches!(
            node,
            Node::Document(_)
                | Node::Heading { .. }
                | Node::Paragraph(_)
                | Node::BlockQuote(_)
                | Node::CodeBlock { .. }
                | Node::UnorderedList(_)
                | Node::OrderedList { .. }
                | Node::ThematicBreak
                | Node::Table { .. }
                | Node::HtmlBlock(_)
                | Node::LinkReferenceDefinition { .. }
        ) || matches!(node, Node::Custom(custom) if custom.is_block())
    }

    fn process_commonmark(
        &self,
        writer: &mut crate::writer::CommonMarkWriter,
        node: &Node,
    ) -> WriteResult<()> {
        match node {
            Node::Document(children) => {
                for (i, child) in children.iter().enumerate() {
                    if i > 0 {
                        writer.write_str("\n\n")?;
                    }
                    writer.write(child)?;
                }
                Ok(())
            }
            Node::Heading {
                level,
                content,
                heading_type,
            } => writer.write_heading(*level, content, heading_type),
            Node::Paragraph(content) => writer.write_paragraph(content),
            Node::BlockQuote(content) => writer.write_blockquote(content),
            Node::CodeBlock {
                language,
                content,
                block_type,
            } => writer.write_code_block(language, content, block_type),
            Node::UnorderedList(items) => writer.write_unordered_list(items),
            Node::OrderedList { start, items } => writer.write_ordered_list(*start, items),
            Node::ThematicBreak => writer.write_thematic_break(),
            #[cfg(feature = "gfm")]
            Node::Table {
                headers,
                alignments,
                rows,
            } => writer.write_table_with_alignment(headers, alignments, rows),
            #[cfg(not(feature = "gfm"))]
            Node::Table { headers, rows, .. } => writer.write_table(headers, rows),
            Node::HtmlBlock(content) => writer.write_html_block(content),
            Node::LinkReferenceDefinition {
                label,
                destination,
                title,
            } => writer.write_link_reference_definition(label, destination, title),
            Node::Custom(custom_node) if custom_node.is_block() => {
                custom_node.render_commonmark(writer)
            }
            _ => Err(WriteError::UnsupportedNodeType),
        }?;

        if self.config.ensure_trailing_newlines {
            writer.ensure_trailing_newline()?;
        }

        Ok(())
    }

    fn process_html(&self, writer: &mut crate::writer::HtmlWriter, node: &Node) -> WriteResult<()> {
        writer.write_node(node).map_err(WriteError::from)
    }

    fn priority(&self) -> u32 {
        100
    }
}

impl BlockNodeProcessor for EnhancedBlockProcessor {
    fn ensure_block_separation(&self, writer: &mut dyn Writer) -> WriteResult<()> {
        writer.write_str(&self.config.block_separator)
    }
}

impl ConfigurableProcessor for EnhancedBlockProcessor {
    type Config = BlockProcessorConfig;

    fn configure(&mut self, config: Self::Config) {
        self.config = config;
    }

    fn config(&self) -> &Self::Config {
        &self.config
    }
}

/// 增强的内联节点处理器
#[derive(Debug)]
pub struct EnhancedInlineProcessor {
    config: InlineProcessorConfig,
}

impl EnhancedInlineProcessor {
    /// 创建新的内联处理器
    pub fn new() -> Self {
        Self {
            config: InlineProcessorConfig::default(),
        }
    }

    /// 使用自定义配置创建
    pub fn with_config(config: InlineProcessorConfig) -> Self {
        Self { config }
    }
}

impl Default for EnhancedInlineProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeProcessor for EnhancedInlineProcessor {
    fn can_process(&self, node: &Node) -> bool {
        matches!(
            node,
            Node::Text(_)
                | Node::Emphasis(_)
                | Node::Strong(_)
                | Node::InlineCode(_)
                | Node::Link { .. }
                | Node::Image { .. }
                | Node::Autolink { .. }
                | Node::ReferenceLink { .. }
                | Node::HtmlElement(_)
                | Node::SoftBreak
                | Node::HardBreak
        ) || matches!(node, Node::Custom(custom) if !custom.is_block())
            || (cfg!(feature = "gfm")
                && matches!(node, Node::Strikethrough(_) | Node::ExtendedAutolink(_)))
    }

    fn process_commonmark(
        &self,
        writer: &mut crate::writer::CommonMarkWriter,
        node: &Node,
    ) -> WriteResult<()> {
        if self.config.strict_validation {
            self.validate_inline_content(node)?;
        }

        match node {
            Node::Text(content) => writer.write_text_content(content),
            Node::Emphasis(content) => writer.write_emphasis(content),
            Node::Strong(content) => writer.write_strong(content),
            #[cfg(feature = "gfm")]
            Node::Strikethrough(content) => writer.write_strikethrough(content),
            Node::InlineCode(content) => writer.write_code_content(content),
            Node::Link {
                url,
                title,
                content,
            } => writer.write_link(url, title, content),
            Node::Image { url, title, alt } => writer.write_image(url, title, alt),
            Node::Autolink { url, is_email } => writer.write_autolink(url, *is_email),
            #[cfg(feature = "gfm")]
            Node::ExtendedAutolink(url) => writer.write_extended_autolink(url),
            Node::ReferenceLink { label, content } => writer.write_reference_link(label, content),
            Node::HtmlElement(element) => writer.write_html_element(element),
            Node::SoftBreak => writer.write_soft_break(),
            Node::HardBreak => writer.write_hard_break(),
            Node::Custom(custom_node) if !custom_node.is_block() => {
                custom_node.render_commonmark(writer)
            }
            _ => Err(WriteError::UnsupportedNodeType),
        }
    }

    fn process_html(&self, writer: &mut crate::writer::HtmlWriter, node: &Node) -> WriteResult<()> {
        writer.write_node(node).map_err(WriteError::from)
    }

    fn priority(&self) -> u32 {
        50
    }
}

impl InlineNodeProcessor for EnhancedInlineProcessor {
    fn validate_inline_content(&self, node: &Node) -> WriteResult<()> {
        if !self.config.allow_newlines && !matches!(node, Node::SoftBreak | Node::HardBreak) {
            // 验证逻辑 - 检查是否包含换行符
            match node {
                Node::Text(content) => {
                    if content.contains('\n') {
                        return Err(WriteError::NewlineInInlineElement(
                            format!("Text node: {}", content).into(),
                        ));
                    }
                }
                _ => {} // 其他类型的验证可以在这里添加
            }
        }
        Ok(())
    }
}

impl ConfigurableProcessor for EnhancedInlineProcessor {
    type Config = InlineProcessorConfig;

    fn configure(&mut self, config: Self::Config) {
        self.config = config;
    }

    fn config(&self) -> &Self::Config {
        &self.config
    }
}

/// 自定义节点处理器
#[derive(Debug, Default)]
pub struct CustomNodeProcessor;

impl NodeProcessor for CustomNodeProcessor {
    fn can_process(&self, node: &Node) -> bool {
        matches!(node, Node::Custom(_))
    }

    fn process_commonmark(
        &self,
        writer: &mut crate::writer::CommonMarkWriter,
        node: &Node,
    ) -> WriteResult<()> {
        match node {
            Node::Custom(custom_node) => {
                custom_node.render_commonmark(writer)?;

                if custom_node.is_block() {
                    writer.ensure_trailing_newline()?;
                }

                Ok(())
            }
            _ => Err(WriteError::UnsupportedNodeType),
        }
    }

    fn process_html(&self, writer: &mut crate::writer::HtmlWriter, node: &Node) -> WriteResult<()> {
        match node {
            Node::Custom(custom_node) => custom_node.html_render(writer),
            _ => Err(WriteError::UnsupportedNodeType),
        }
    }

    fn priority(&self) -> u32 {
        200 // 高优先级处理自定义节点
    }
}
