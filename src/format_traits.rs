//! 简化的格式化 trait 设计
//!
//! 这个模块提供了简洁的渲染系统，避免过度的抽象。

use crate::error::WriteResult;
use crate::writer::{CommonMarkWriter, HtmlWriter};

/// 通用格式化 trait - 支持多种输出格式
pub trait Format<W> {
    /// 将自身格式化到指定的 writer
    fn format(&self, writer: &mut W) -> WriteResult<()>;
}

/// 为 CommonMark 格式提供便捷 trait
pub trait ToCommonMark {
    /// 格式化为 CommonMark
    fn to_commonmark(&self, writer: &mut CommonMarkWriter) -> WriteResult<()>;
}

/// 为 HTML 格式提供便捷 trait
pub trait ToHtml {
    /// 格式化为 HTML
    fn to_html(&self, writer: &mut HtmlWriter) -> WriteResult<()>;
}

/// 自动为实现 Format<CommonMarkWriter>的类型提供 ToCommonMark
impl<T> ToCommonMark for T
where
    T: Format<CommonMarkWriter>,
{
    fn to_commonmark(&self, writer: &mut CommonMarkWriter) -> WriteResult<()> {
        self.format(writer)
    }
}

/// 自动为实现 Format<HtmlWriter>的类型提供 ToHtml
impl<T> ToHtml for T
where
    T: Format<HtmlWriter>,
{
    fn to_html(&self, writer: &mut HtmlWriter) -> WriteResult<()> {
        self.format(writer)
    }
}

/// 支持多格式的节点 trait - 简化版本
///
/// 这个 trait 为自定义节点提供统一的多格式渲染接口。
/// 所有自定义节点都应该至少支持 CommonMark 格式。
pub trait MultiFormat: ToCommonMark {
    /// 检查是否支持 HTML 格式
    ///
    /// 默认返回 false，只有支持 HTML 的类型需要重写返回 true
    fn supports_html(&self) -> bool {
        false
    }

    /// HTML 渲染实现
    ///
    /// 默认生成注释说明不支持 HTML。
    /// 支持 HTML 的类型应该重写此方法。
    fn html_format(&self, writer: &mut HtmlWriter) -> WriteResult<()> {
        writer
            .raw_html(&format!(
                "<!-- HTML rendering not implemented for {} -->",
                std::any::type_name::<Self>()
            ))
            .map_err(Into::into)
    }
}

/// 为同时实现 ToCommonMark 和 ToHtml 的类型自动实现 MultiFormat
impl<T> MultiFormat for T
where
    T: ToCommonMark + ToHtml,
{
    fn supports_html(&self) -> bool {
        true
    }

    fn html_format(&self, writer: &mut HtmlWriter) -> WriteResult<()> {
        self.to_html(writer)
    }
}
