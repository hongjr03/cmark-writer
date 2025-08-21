use cmark_writer::error::WriteResult;
use cmark_writer::{CommonMarkWriter, Format, HtmlWriter, ToCommonMark, ToHtml};
use ecow::EcoString;

#[derive(Debug, Clone, PartialEq)]
struct MyHighlight {
    content: EcoString,
    color: EcoString,
}

impl Format<CommonMarkWriter> for MyHighlight {
    fn format(&self, w: &mut CommonMarkWriter) -> WriteResult<()> {
        w.write_str("<span style=\"background-color: ")?;
        w.write_str(&self.color)?;
        w.write_str("\">")?;
        w.write_str(&self.content)?;
        w.write_str("</span>")?;
        Ok(())
    }
}

impl Format<HtmlWriter> for MyHighlight {
    fn format(&self, w: &mut HtmlWriter) -> WriteResult<()> {
        w.start_tag("span")?;
        w.attribute("style", &format!("background-color: {}", self.color))?;
        w.finish_tag()?;
        w.text(&self.content)?;
        w.end_tag("span")?;
        Ok(())
    }
}

#[test]
fn test_my_highlight_basic() {
    let n = MyHighlight {
        content: "X".into(),
        color: "yellow".into(),
    };
    let mut md = CommonMarkWriter::new();
    n.to_commonmark(&mut md).unwrap();
    assert!(md.into_string().contains("X"));
}
