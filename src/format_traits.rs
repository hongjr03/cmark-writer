//! 更科学的格式化trait设计
//!
//! 这个模块提供了基于标准Rust trait模式的渲染系统，
//! 使用泛型和关联类型来实现类型安全的多格式渲染。

use crate::error::WriteResult;
use crate::writer::{CommonMarkWriter, HtmlWriter};

/// 通用格式化trait - 支持多种输出格式
pub trait Format<W> {
    /// 将自身格式化到指定的writer
    fn format(&self, writer: &mut W) -> WriteResult<()>;
}

/// CommonMark格式标记trait
pub struct CommonMarkFormat;

/// HTML格式标记trait  
pub struct HtmlFormat;

/// 为CommonMark格式提供便捷trait
pub trait ToCommonMark {
    /// 格式化为CommonMark
    fn to_commonmark(&self, writer: &mut CommonMarkWriter) -> WriteResult<()>;
}

/// 为HTML格式提供便捷trait
pub trait ToHtml {
    /// 格式化为HTML
    fn to_html(&self, writer: &mut HtmlWriter) -> WriteResult<()>;

    /// 提供默认的HTML实现（可选）
    fn default_html(&self, writer: &mut HtmlWriter) -> WriteResult<()> {
        writer
            .raw_html(&format!(
                "<!-- HTML rendering not implemented for {} -->",
                std::any::type_name::<Self>()
            ))
            .map_err(Into::into)
    }
}

/// 自动为实现Format<CommonMarkWriter>的类型提供ToCommonMark
impl<T> ToCommonMark for T
where
    T: Format<CommonMarkWriter>,
{
    fn to_commonmark(&self, writer: &mut CommonMarkWriter) -> WriteResult<()> {
        self.format(writer)
    }
}

/// 自动为实现Format<HtmlWriter>的类型提供ToHtml  
impl<T> ToHtml for T
where
    T: Format<HtmlWriter>,
{
    fn to_html(&self, writer: &mut HtmlWriter) -> WriteResult<()> {
        self.format(writer)
    }
}

/// 支持多格式的节点trait - 手动实现以获得更好的控制
pub trait MultiFormat: ToCommonMark {
    /// 检查是否支持HTML格式
    fn supports_html(&self) -> bool;

    /// HTML渲染实现
    fn html_format(&self, writer: &mut HtmlWriter) -> WriteResult<()>;
}

/// 提供默认的HTML渲染辅助方法
pub fn default_html_render<T>(_item: &T, writer: &mut HtmlWriter) -> WriteResult<()> {
    writer
        .raw_html(&format!(
            "<!-- HTML rendering not implemented for {} -->",
            std::any::type_name::<T>()
        ))
        .map_err(Into::into)
}
