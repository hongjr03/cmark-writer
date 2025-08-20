//! 展示更科学的自定义节点设计
//!
//! 这个示例展示了如何使用标准的 Rust trait 模式来实现
//! 类型安全、可扩展的多格式渲染系统。

use cmark_writer::error::WriteResult;
use cmark_writer::format_traits::default_html_render;
use cmark_writer::{CommonMarkWriter, HtmlWriter};
use cmark_writer::{Format, MultiFormat, ToCommonMark, ToHtml};
use ecow::EcoString;

/// 使用新的科学设计的高亮节点
#[derive(Debug, Clone, PartialEq)]
pub struct HighlightNode {
    pub content: EcoString,
    pub color: EcoString,
}

/// 为 CommonMark 格式实现 Format trait
impl Format<CommonMarkWriter> for HighlightNode {
    fn format(&self, writer: &mut CommonMarkWriter) -> WriteResult<()> {
        // 在 CommonMark 中使用 HTML 标签
        writer.write_str("<span style=\"background-color: ")?;
        writer.write_str(&self.color)?;
        writer.write_str("\">")?;
        writer.write_str(&self.content)?;
        writer.write_str("</span>")?;
        Ok(())
    }
}

/// 为 HTML 格式实现 Format trait  
impl Format<HtmlWriter> for HighlightNode {
    fn format(&self, writer: &mut HtmlWriter) -> WriteResult<()> {
        writer.start_tag("span")?;
        writer.attribute("style", &format!("background-color: {}", self.color))?;
        writer.finish_tag()?;
        writer.text(&self.content)?;
        writer.end_tag("span")?;
        Ok(())
    }
}

/// 实现 MultiFormat 以支持多种格式
impl MultiFormat for HighlightNode {
    fn supports_html(&self) -> bool {
        true
    }

    fn html_format(&self, writer: &mut HtmlWriter) -> WriteResult<()> {
        self.to_html(writer)
    }
}

/// 块级 CalloutBox 节点示例
#[derive(Debug, Clone, PartialEq)]
pub struct CalloutBox {
    pub title: EcoString,
    pub content: EcoString,
    pub level: CalloutLevel,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CalloutLevel {
    Info,
    Warning,
    Error,
}

impl CalloutLevel {
    fn as_str(&self) -> &'static str {
        match self {
            CalloutLevel::Info => "info",
            CalloutLevel::Warning => "warning",
            CalloutLevel::Error => "error",
        }
    }

    fn css_class(&self) -> &'static str {
        match self {
            CalloutLevel::Info => "callout-info",
            CalloutLevel::Warning => "callout-warning",
            CalloutLevel::Error => "callout-error",
        }
    }
}

/// CommonMark 格式实现
impl Format<CommonMarkWriter> for CalloutBox {
    fn format(&self, writer: &mut CommonMarkWriter) -> WriteResult<()> {
        writer.write_str("<div class=\"callout callout-")?;
        writer.write_str(self.level.as_str())?;
        writer.write_str("\">\n  <h4>")?;
        writer.write_str(&self.title)?;
        writer.write_str("</h4>\n  <p>")?;
        writer.write_str(&self.content)?;
        writer.write_str("</p>\n</div>")?;
        Ok(())
    }
}

/// HTML 格式实现  
impl Format<HtmlWriter> for CalloutBox {
    fn format(&self, writer: &mut HtmlWriter) -> WriteResult<()> {
        writer.start_tag("div")?;
        writer.attribute("class", &format!("callout {}", self.level.css_class()))?;
        writer.finish_tag()?;

        writer.start_tag("h4")?;
        writer.finish_tag()?;
        writer.text(&self.title)?;
        writer.end_tag("h4")?;

        writer.start_tag("p")?;
        writer.finish_tag()?;
        writer.text(&self.content)?;
        writer.end_tag("p")?;

        writer.end_tag("div")?;
        Ok(())
    }
}

/// CalloutBox 的 MultiFormat 实现
impl MultiFormat for CalloutBox {
    fn supports_html(&self) -> bool {
        true
    }

    fn html_format(&self, writer: &mut HtmlWriter) -> WriteResult<()> {
        self.to_html(writer)
    }
}

/// 只支持 CommonMark 的简单节点
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleNote {
    pub content: EcoString,
}

impl Format<CommonMarkWriter> for SimpleNote {
    fn format(&self, writer: &mut CommonMarkWriter) -> WriteResult<()> {
        writer.write_str("> **Note:** ")?;
        writer.write_str(&self.content)?;
        Ok(())
    }
}

/// SimpleNote 的 MultiFormat 实现 - 只支持 CommonMark
impl MultiFormat for SimpleNote {
    fn supports_html(&self) -> bool {
        false
    }

    fn html_format(&self, writer: &mut HtmlWriter) -> WriteResult<()> {
        default_html_render(self, writer)
    }
}

// SimpleNote 自动获得 ToCommonMark trait，但没有 HTML 支持

fn main() -> WriteResult<()> {
    // 创建节点实例
    let highlight = HighlightNode {
        content: "重要文本".into(),
        color: "yellow".into(),
    };

    let callout = CalloutBox {
        title: "重要提示".into(),
        content: "这是一个重要的信息。".into(),
        level: CalloutLevel::Warning,
    };

    let note = SimpleNote {
        content: "这是一个简单的备注。".into(),
    };

    // CommonMark 渲染
    println!("=== CommonMark 输出 ===");
    let mut md_writer = CommonMarkWriter::new();

    highlight.to_commonmark(&mut md_writer)?;
    md_writer.write_str("\n\n")?;

    callout.to_commonmark(&mut md_writer)?;
    md_writer.write_str("\n\n")?;

    note.to_commonmark(&mut md_writer)?;

    println!("{}", md_writer.into_string());

    // HTML 渲染
    println!("\n=== HTML 输出 ===");
    let mut html_writer = HtmlWriter::new();

    // 高亮节点支持 HTML
    highlight.to_html(&mut html_writer)?;
    html_writer.write_str("\n")?;

    // CalloutBox 支持 HTML
    callout.to_html(&mut html_writer)?;
    html_writer.write_str("\n")?;

    // SimpleNote 不支持 HTML，使用默认实现
    note.html_format(&mut html_writer)?;

    println!("{}", html_writer.into_string());

    // 检查格式支持
    println!("\n=== 格式支持检查 ===");
    println!("HighlightNode supports HTML: {}", highlight.supports_html());
    println!("CalloutBox supports HTML: {}", callout.supports_html());
    println!("SimpleNote supports HTML: {}", note.supports_html());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_commonmark() {
        let highlight = HighlightNode {
            content: "test".into(),
            color: "red".into(),
        };

        let mut writer = CommonMarkWriter::new();
        highlight.to_commonmark(&mut writer).unwrap();

        assert_eq!(
            writer.into_string(),
            "<span style=\"background-color: red\">test</span>"
        );
    }

    #[test]
    fn test_highlight_html() {
        let highlight = HighlightNode {
            content: "test".into(),
            color: "blue".into(),
        };

        let mut writer = HtmlWriter::new();
        highlight.to_html(&mut writer).unwrap();

        let html = writer.into_string();
        assert!(html.contains("<span"));
        assert!(html.contains("background-color: blue"));
        assert!(html.contains("test"));
        assert!(html.contains("</span>"));
    }

    #[test]
    fn test_format_support() {
        let highlight = HighlightNode {
            content: "test".into(),
            color: "green".into(),
        };

        let note = SimpleNote {
            content: "test".into(),
        };

        assert!(highlight.supports_html());
        assert!(!note.supports_html());
    }
}
