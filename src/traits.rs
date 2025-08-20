//! 重构后的核心trait定义
//!
//! 这个模块提供了科学合理的trait层次结构，遵循SOLID原则并提供更好的关注点分离。

use crate::error::{WriteError, WriteResult};
use ecow::EcoString;
use std::any::Any;

/// 核心节点内容trait - 只关注基本属性
pub trait NodeContent: std::fmt::Debug + Send + Sync {
    /// 是否为块级元素
    fn is_block(&self) -> bool;

    /// 获取类型名称用于模式匹配
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    /// 转换为Any进行类型转换
    fn as_any(&self) -> &dyn Any;

    /// 转换为可变Any进行类型转换
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// 节点克隆和相等性trait
pub trait NodeClone: NodeContent {
    /// 克隆节点到Box中
    fn clone_box(&self) -> Box<dyn NodeContent>;

    /// 检查与另一个节点的相等性
    fn eq_box(&self, other: &dyn NodeContent) -> bool;
}

/// CommonMark渲染trait - 使用具体类型确保dyn兼容性
pub trait CommonMarkRenderable: NodeContent {
    /// 渲染到CommonMark格式
    fn render_commonmark(&self, writer: &mut crate::writer::CommonMarkWriter) -> WriteResult<()>;
}

/// HTML渲染trait - 使用具体类型确保dyn兼容性
pub trait HtmlRenderable: NodeContent {
    /// 渲染到HTML格式
    fn render_html(&self, writer: &mut crate::writer::HtmlWriter) -> WriteResult<()>;
}

/// 自定义节点trait - 现在dyn兼容
pub trait CustomNode: NodeClone + CommonMarkRenderable {
    /// HTML渲染的默认实现
    fn html_render(&self, writer: &mut crate::writer::HtmlWriter) -> WriteResult<()> {
        // 使用HtmlWriter的raw_html方法
        writer
            .raw_html(&format!(
                "<!-- HTML rendering not implemented for {} -->",
                self.type_name()
            ))
            .map_err(WriteError::from)
    }

    /// 获取自定义属性
    fn attributes(&self) -> Option<&std::collections::HashMap<String, String>> {
        None
    }

    /// 检查是否支持特定功能
    fn supports_capability(&self, capability: &str) -> bool {
        match capability {
            "commonmark" => true,
            "html" => false,
            _ => false,
        }
    }
}

/// 输出写入器trait - 简化设计确保dyn兼容性
pub trait Writer {
    /// 写入字符串
    fn write_str(&mut self, s: &str) -> WriteResult<()>;

    /// 写入字符
    fn write_char(&mut self, c: char) -> WriteResult<()>;

    /// 写入格式化内容
    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> WriteResult<()> {
        self.write_str(&args.to_string())
    }
}

/// 节点处理器trait
pub trait NodeProcessor {
    /// 检查是否可以处理该节点
    fn can_process(&self, node: &crate::ast::Node) -> bool;

    /// 处理节点并写入CommonMark
    fn process_commonmark(
        &self,
        writer: &mut crate::writer::CommonMarkWriter,
        node: &crate::ast::Node,
    ) -> WriteResult<()>;

    /// 处理节点并写入HTML
    fn process_html(
        &self,
        writer: &mut crate::writer::HtmlWriter,
        node: &crate::ast::Node,
    ) -> WriteResult<()>;

    /// 获取处理器优先级
    fn priority(&self) -> u32 {
        0
    }
}

/// 块级节点处理器
pub trait BlockNodeProcessor: NodeProcessor {
    /// 确保块级分隔
    fn ensure_block_separation(&self, writer: &mut dyn Writer) -> WriteResult<()>;
}

/// 内联节点处理器
pub trait InlineNodeProcessor: NodeProcessor {
    /// 验证内联内容
    fn validate_inline_content(&self, node: &crate::ast::Node) -> WriteResult<()>;
}

/// 错误上下文trait
pub trait ErrorContext<T> {
    /// 添加上下文信息到错误
    fn with_context<S: Into<EcoString>>(self, context: S) -> Result<T, WriteError>;

    /// 使用闭包添加上下文信息
    fn with_context_fn<F, S>(self, f: F) -> Result<T, WriteError>
    where
        F: FnOnce() -> S,
        S: Into<EcoString>;
}

/// 错误工厂trait
pub trait ErrorFactory<E> {
    /// 创建错误
    fn create_error(&self) -> E;

    /// 带上下文创建错误
    fn create_error_with_context<S: Into<EcoString>>(&self, _context: S) -> E {
        self.create_error()
    }
}

/// 可配置处理器trait
pub trait ConfigurableProcessor {
    /// 配置类型
    type Config;

    /// 应用配置
    fn configure(&mut self, config: Self::Config);

    /// 获取当前配置
    fn config(&self) -> &Self::Config;
}

// 为Result实现ErrorContext
impl<T> ErrorContext<T> for Result<T, WriteError> {
    fn with_context<S: Into<EcoString>>(self, context: S) -> Result<T, WriteError> {
        self.map_err(|e| {
            let context_str = context.into();
            WriteError::custom(format!("{}: {}", context_str, e))
        })
    }

    fn with_context_fn<F, S>(self, f: F) -> Result<T, WriteError>
    where
        F: FnOnce() -> S,
        S: Into<EcoString>,
    {
        self.map_err(|e| {
            let context_str = f().into();
            WriteError::custom(format!("{}: {}", context_str, e))
        })
    }
}

// 为CommonMarkWriter实现Writer trait
impl Writer for crate::writer::CommonMarkWriter {
    fn write_str(&mut self, s: &str) -> WriteResult<()> {
        self.write_str(s)
    }

    fn write_char(&mut self, c: char) -> WriteResult<()> {
        self.write_char(c)
    }
}

// 为HtmlWriter实现Writer trait
impl Writer for crate::writer::HtmlWriter {
    fn write_str(&mut self, s: &str) -> WriteResult<()> {
        self.write_str(s).map_err(WriteError::from)
    }

    fn write_char(&mut self, c: char) -> WriteResult<()> {
        self.write_str(&c.to_string()).map_err(WriteError::from)
    }
}
