use cmark_writer::{CommonMarkWriter, Format, HtmlWriter, ToCommonMark, ToHtml};
use ecow::EcoString;

#[derive(Debug, Clone, PartialEq)]
struct HighlightNode {
    content: EcoString,
    color: EcoString,
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

#[test]
fn test_highlight_new_api() {
    let node = HighlightNode {
        content: "X".into(),
        color: "yellow".into(),
    };
    let mut md = CommonMarkWriter::new();
    node.to_commonmark(&mut md).unwrap();
    assert_eq!(
        md.into_string(),
        "<span style=\"background-color: yellow\">X</span>"
    );

    let mut html = HtmlWriter::new();
    node.to_html(&mut html).unwrap();
    let s = html.into_string();
    assert!(s.contains("<span"));
    assert!(s.contains("background-color: yellow"));
    assert!(s.contains("X"));
}
