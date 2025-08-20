//! 新的自定义节点实现
//!
//! 这个模块提供了基于新 trait 架构的自定义节点实现

use crate::error::{WriteError, WriteResult};
use crate::traits::{CommonMarkRenderable, CustomNode, NodeClone, NodeContent};
use std::any::Any;
use std::collections::HashMap;

/// 节点类型枚举，用于表示节点的显示和行为特性
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeKind {
    /// 块级元素 - 占据整行，前后通常需要空行分隔
    Block,
    /// 内联元素 - 可以与其他内容在同一行显示
    Inline,
    /// 替换元素 - 可以根据上下文表现为块级或内联（如图片）
    Replaced,
    /// 空元素 - 不占用显示空间，用于元数据或标记
    Void,
}

impl NodeKind {
    /// 检查是否为块级元素
    pub fn is_block(&self) -> bool {
        matches!(self, NodeKind::Block | NodeKind::Void)
    }

    /// 检查是否为内联元素
    pub fn is_inline(&self) -> bool {
        matches!(self, NodeKind::Inline | NodeKind::Replaced)
    }

    /// 检查是否可以包含其他节点
    pub fn can_contain_content(&self) -> bool {
        !matches!(self, NodeKind::Void)
    }
}

/// 通用自定义节点实现
#[derive(Debug, Clone, PartialEq)]
pub struct GenericCustomNode {
    /// 节点类型标识符
    pub node_type: String,
    /// 节点种类（块级、内联等）
    pub kind: NodeKind,
    /// 节点内容
    pub content: String,
    /// 自定义属性
    pub attributes: HashMap<String, String>,
    /// CommonMark 渲染函数
    pub commonmark_renderer:
        fn(&GenericCustomNode, &mut crate::writer::CommonMarkWriter) -> WriteResult<()>,
    /// HTML 渲染函数（可选）
    pub html_renderer:
        Option<fn(&GenericCustomNode, &mut crate::writer::HtmlWriter) -> WriteResult<()>>,
}

impl NodeContent for GenericCustomNode {
    fn is_block(&self) -> bool {
        self.kind.is_block()
    }

    fn type_name(&self) -> &'static str {
        "GenericCustomNode"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl NodeClone for GenericCustomNode {
    fn clone_box(&self) -> Box<dyn NodeContent> {
        Box::new(self.clone())
    }

    fn eq_box(&self, other: &dyn NodeContent) -> bool {
        other.as_any().downcast_ref::<GenericCustomNode>() == Some(self)
    }
}

impl CommonMarkRenderable for GenericCustomNode {
    fn render_commonmark(&self, writer: &mut crate::writer::CommonMarkWriter) -> WriteResult<()> {
        (self.commonmark_renderer)(self, writer)
    }
}

impl CustomNode for GenericCustomNode {
    fn html_render(&self, writer: &mut crate::writer::HtmlWriter) -> WriteResult<()> {
        if let Some(renderer) = self.html_renderer {
            renderer(self, writer)
        } else {
            writer
                .raw_html(&format!(
                    "<!-- HTML rendering not implemented for {} -->",
                    self.node_type
                ))
                .map_err(WriteError::from)
        }
    }

    fn attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
    }

    fn supports_capability(&self, capability: &str) -> bool {
        match capability {
            "commonmark" => true,
            "html" => self.html_renderer.is_some(),
            _ => false,
        }
    }
}

impl GenericCustomNode {
    /// 创建新的通用自定义节点
    pub fn new<S: Into<String>>(
        node_type: S,
        kind: NodeKind,
        content: S,
        commonmark_renderer: fn(
            &GenericCustomNode,
            &mut crate::writer::CommonMarkWriter,
        ) -> WriteResult<()>,
    ) -> Self {
        Self {
            node_type: node_type.into(),
            kind,
            content: content.into(),
            attributes: HashMap::new(),
            commonmark_renderer,
            html_renderer: None,
        }
    }

    /// 添加 HTML 渲染器
    pub fn with_html_renderer(
        mut self,
        renderer: fn(&GenericCustomNode, &mut crate::writer::HtmlWriter) -> WriteResult<()>,
    ) -> Self {
        self.html_renderer = Some(renderer);
        self
    }

    /// 添加属性
    pub fn with_attribute<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// 获取内容
    pub fn content(&self) -> &str {
        &self.content
    }

    /// 获取节点类型
    pub fn node_type(&self) -> &str {
        &self.node_type
    }
}

// 实现 Box<dyn CustomNode>的 Clone
impl Clone for Box<dyn CustomNode> {
    fn clone(&self) -> Self {
        // 尝试 downcast 到已知类型
        if let Some(generic) = self.as_any().downcast_ref::<GenericCustomNode>() {
            Box::new(generic.clone())
        } else {
            // 如果无法转换，创建一个空的 GenericCustomNode
            Box::new(GenericCustomNode::new(
                "unknown",
                NodeKind::Inline,
                "",
                |_node, writer| writer.write_str("<!-- Unknown custom node -->"),
            ))
        }
    }
}

// 实现 Box<dyn CustomNode>的 PartialEq
impl PartialEq for Box<dyn CustomNode> {
    fn eq(&self, other: &Self) -> bool {
        self.eq_box(&**other)
    }
}

/// 简单文本自定义节点 - 示例实现
#[derive(Debug, Clone, PartialEq)]
pub struct TextCustomNode {
    /// 节点文本内容
    pub content: String,
    /// 节点种类（块级、内联等）
    pub kind: NodeKind,
}

impl NodeContent for TextCustomNode {
    fn is_block(&self) -> bool {
        self.kind.is_block()
    }

    fn type_name(&self) -> &'static str {
        "TextCustomNode"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl NodeClone for TextCustomNode {
    fn clone_box(&self) -> Box<dyn NodeContent> {
        Box::new(self.clone())
    }

    fn eq_box(&self, other: &dyn NodeContent) -> bool {
        other.as_any().downcast_ref::<TextCustomNode>() == Some(self)
    }
}

impl CommonMarkRenderable for TextCustomNode {
    fn render_commonmark(&self, writer: &mut crate::writer::CommonMarkWriter) -> WriteResult<()> {
        writer.write_str(&self.content)
    }
}

impl CustomNode for TextCustomNode {
    fn html_render(&self, writer: &mut crate::writer::HtmlWriter) -> WriteResult<()> {
        writer.text(&self.content).map_err(WriteError::from)
    }
}

impl TextCustomNode {
    /// 创建新的文本自定义节点
    pub fn new<S: Into<String>>(content: S, kind: NodeKind) -> Self {
        Self {
            content: content.into(),
            kind,
        }
    }
}

// 重新导出 CustomNode trait 以保持向后兼容
pub use crate::traits::CustomNode as NewCustomNode;
