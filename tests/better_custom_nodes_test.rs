use cmark_writer::{CommonMarkWriter, Format, HtmlWriter, MultiFormat, ToCommonMark, ToHtml};
use ecow::EcoString;

// 引入示例中的节点定义，或在此最小复刻以验证新用法
#[derive(Debug, Clone, PartialEq)]
pub struct HighlightNode {
    pub content: EcoString,
    pub color: EcoString,
}

impl Format<CommonMarkWriter> for HighlightNode {
    fn format(&self, w: &mut CommonMarkWriter) -> cmark_writer::error::WriteResult<()> {
        w.write_str("<span style=\"background-color: ")?;
        w.write_str(&self.color)?;
        w.write_str("\">")?;
        w.write_str(&self.content)?;
        w.write_str("</span>")?;
        Ok(())
    }
}

impl Format<HtmlWriter> for HighlightNode {
    fn format(&self, w: &mut HtmlWriter) -> cmark_writer::error::WriteResult<()> {
        w.start_tag("span")?;
        w.attribute("style", &format!("background-color: {}", self.color))?;
        w.finish_tag()?;
        w.text(&self.content)?;
        w.end_tag("span")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, cmark_writer::CommonMarkOnly)]
pub struct SimpleNote {
    pub content: EcoString,
}

impl Format<CommonMarkWriter> for SimpleNote {
    fn format(&self, w: &mut CommonMarkWriter) -> cmark_writer::error::WriteResult<()> {
        w.write_str("> **Note:** ")?;
        w.write_str(&self.content)?;
        Ok(())
    }
}

#[test]
fn test_highlight_commonmark() {
    let node = HighlightNode {
        content: "test".into(),
        color: "red".into(),
    };
    let mut w = CommonMarkWriter::new();
    node.to_commonmark(&mut w).unwrap();
    assert_eq!(
        w.into_string(),
        "<span style=\"background-color: red\">test</span>"
    );
}

#[test]
fn test_highlight_html() {
    let node = HighlightNode {
        content: "test".into(),
        color: "blue".into(),
    };
    let mut w = HtmlWriter::new();
    node.to_html(&mut w).unwrap();
    let s = w.into_string();
    assert!(s.contains("<span"));
    assert!(s.contains("background-color: blue"));
    assert!(s.contains("test"));
    assert!(s.contains("</span>"));
}

#[test]
fn test_simple_note_commonmark_only() {
    let node = SimpleNote {
        content: "仅支持 MD".into(),
    };
    let mut md = CommonMarkWriter::new();
    node.to_commonmark(&mut md).unwrap();
    assert_eq!(md.into_string(), "> **Note:** 仅支持 MD");

    assert!(!node.supports_html());
    let mut html = HtmlWriter::new();
    node.html_format(&mut html).unwrap();
    let out = html.into_string();
    assert!(out.contains("HTML rendering not implemented"));
}
