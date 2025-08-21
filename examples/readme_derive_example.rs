use cmark_writer::{CommonMarkWriter, Format, HtmlWriter, MultiFormat, ToCommonMark};
use ecow::EcoString;

// Simple custom node with automatic MultiFormat implementation
#[derive(Debug, Clone, PartialEq, cmark_writer::CommonMarkOnly)]
pub struct SimpleNote {
    pub content: EcoString,
}

// Only implement CommonMark format
impl Format<CommonMarkWriter> for SimpleNote {
    fn format(&self, writer: &mut CommonMarkWriter) -> cmark_writer::error::WriteResult<()> {
        writer.write_str("> **Note:** ")?;
        writer.write_str(&self.content)?;
        Ok(())
    }
}

fn main() -> cmark_writer::error::WriteResult<()> {
    // Usage - MultiFormat methods are automatically available
    let note = SimpleNote {
        content: "This is a note".into(),
    };

    // Check format support
    assert!(!note.supports_html()); // Returns false since only CommonMark is implemented
    println!("supports_html(): {}", note.supports_html());

    // CommonMark rendering works as expected
    let mut md = CommonMarkWriter::new();
    note.to_commonmark(&mut md)?;
    let md_output = md.into_string();
    assert_eq!(md_output, "> **Note:** This is a note");
    println!("CommonMark: {}", md_output);

    // HTML rendering provides a helpful fallback comment
    let mut html = HtmlWriter::new();
    note.html_format(&mut html)?;
    let html_output = html.into_string();
    assert!(html_output.contains("HTML rendering not implemented"));
    println!("HTML: {}", html_output);

    println!("✅ README 示例验证成功！");
    Ok(())
}
