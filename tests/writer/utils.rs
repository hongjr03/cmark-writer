use cmark_writer::writer::cmark::{escape_str, CommonMarkEscapes};

#[test]
fn escape_str_commonmark() {
    let s = "* _ [ ] < > ` \\";
    let escaped = escape_str::<CommonMarkEscapes>(s);
    assert_eq!(escaped, "\\* \\_ \\[ \\] \\< \\> \\` \\\\");
}
