use cmark_writer::error::WriteResult;
use cmark_writer::ToCommonMark;
use cmark_writer::{CommonMarkWriter, Format};

#[derive(Debug, Clone, PartialEq)]
struct SimpleNote {
    content: String,
}

impl Format<CommonMarkWriter> for SimpleNote {
    fn format(&self, w: &mut CommonMarkWriter) -> WriteResult<()> {
        w.write_str("Note: ")?;
        w.write_str(&self.content)?;
        Ok(())
    }
}

#[test]
fn test_simple_note_only_new_api() {
    let n = SimpleNote {
        content: "ok".into(),
    };
    let mut md = CommonMarkWriter::new();
    n.to_commonmark(&mut md).unwrap();
    assert!(md.into_string().starts_with("Note:"));
}
