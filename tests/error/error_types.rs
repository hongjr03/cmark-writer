use cmark_writer::{CommonMarkWriter, Node, ToCommonMark, WriteError};
use cmark_writer::ast::HeadingType;

#[test]
fn invalid_heading_levels() {
    let mut w = CommonMarkWriter::new();
    let h0 = Node::Heading { level: 0, content: vec![Node::Text("Invalid".into())], heading_type: HeadingType::Atx };
    let e = h0.to_commonmark(&mut w).unwrap_err();
    assert!(matches!(e, WriteError::InvalidHeadingLevel(0)));

    let mut w = CommonMarkWriter::new();
    let h7 = Node::Heading { level: 7, content: vec![Node::Text("Invalid".into())], heading_type: HeadingType::Atx };
    let e = h7.to_commonmark(&mut w).unwrap_err();
    assert!(matches!(e, WriteError::InvalidHeadingLevel(7)));
}

#[test]
fn newline_in_inline_elements() {
    let mut w = CommonMarkWriter::new();
    let t = Node::Text("a\nb".into());
    assert!(matches!(t.to_commonmark(&mut w), Err(WriteError::NewlineInInlineElement(_))));
}

#[test]
fn custom_error_helpers_compile() {
    use cmark_writer::error::WriteError;
    let e = WriteError::custom("自定义");
    assert!(e.to_string().contains("Custom error"));
    let e2 = WriteError::custom_with_code("x", "CODE");
    assert!(e2.to_string().contains("[CODE]"));
}
