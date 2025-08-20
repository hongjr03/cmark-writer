use cmark_writer::error::WriteResult;
use cmark_writer::{CommonMarkWriter, Format, HtmlWriter, MultiFormat, ToCommonMark, ToHtml};
use ecow::EcoString;

#[derive(Debug, Clone, PartialEq)]
struct HighlightNode {
    content: EcoString,
    color: EcoString,
}

impl Format<CommonMarkWriter> for HighlightNode {
    fn format(&self, w: &mut CommonMarkWriter) -> WriteResult<()> {
        w.write_str("<span style=\"background-color: ")?;
        w.write_str(&self.color)?;
        w.write_str("\">")?;
        w.write_str(&self.content)?;
        w.write_str("</span>")?;
        Ok(())
    }
}

impl Format<HtmlWriter> for HighlightNode {
    fn format(&self, w: &mut HtmlWriter) -> WriteResult<()> {
        w.start_tag("span")?;
        w.attribute("style", &format!("background-color: {}", self.color))?;
        w.finish_tag()?;
        w.text(&self.content)?;
        w.end_tag("span")?;
        Ok(())
    }
}

impl MultiFormat for HighlightNode {
    fn supports_html(&self) -> bool {
        true
    }
    fn html_format(&self, w: &mut HtmlWriter) -> WriteResult<()> {
        self.to_html(w)
    }
}

#[test]
fn test_highlight_new_api_again() {
    let node = HighlightNode {
        content: "Again".into(),
        color: "yellow".into(),
    };

    let mut md = CommonMarkWriter::new();
    node.to_commonmark(&mut md).unwrap();
    assert!(md.into_string().contains("background-color: yellow"));

    let mut html = HtmlWriter::new();
    node.to_html(&mut html).unwrap();
    let s = html.into_string();
    assert!(s.contains("<span"));
    assert!(s.contains("Again"));
}
