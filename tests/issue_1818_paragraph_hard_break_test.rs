//! Test for issue #1818: paragraph 末尾出现 hard break
//! https://github.com/Myriad-Dreamin/tinymist/issues/1818

use cmark_writer::ast::Node;
use cmark_writer::writer::CommonMarkWriter;

#[test]
fn test_paragraph_trailing_hard_breaks_removed() {
    let paragraph = Node::Paragraph(vec![
        Node::Text("This is some text".into()),
        Node::HardBreak,
    ]);

    let mut writer = CommonMarkWriter::new();
    writer.write(&paragraph).unwrap();
    let result = writer.into_string();

    assert!(!result.ends_with("  \n"));
    assert!(!result.ends_with("\\\n"));
    assert!(result.starts_with("This is some text"));
    assert!(result == "This is some text\n");
}

#[test]
fn test_paragraph_multiple_trailing_hard_breaks_removed() {
    let paragraph = Node::Paragraph(vec![
        Node::Text("First line".into()),
        Node::HardBreak,
        Node::Text("Second line".into()),
        Node::HardBreak,
        Node::HardBreak,
        Node::HardBreak,
    ]);

    let mut writer = CommonMarkWriter::new();
    writer.write(&paragraph).unwrap();
    let result = writer.into_string();

    assert!(!result.ends_with("  \n"));
    assert!(!result.ends_with("\\\n"));
    assert!(result.contains("  \n") || result.contains("\\\n"));
    assert!(result.starts_with("First line"));
    assert!(result.contains("Second line"));
    assert!(result.trim_end().ends_with("Second line"));
}

#[test]
fn test_paragraph_internal_hard_breaks_preserved() {
    let paragraph = Node::Paragraph(vec![
        Node::Text("Line 1".into()),
        Node::HardBreak,
        Node::Text("Line 2".into()),
        Node::HardBreak,
        Node::Text("Line 3".into()),
    ]);

    let mut writer = CommonMarkWriter::new();
    writer.write(&paragraph).unwrap();
    let result = writer.into_string();

    assert!(result.contains("  \n") || result.contains("\\\n"));
    assert!(!result.ends_with("  \n"));
    assert!(!result.ends_with("\\\n"));
    assert!(result.trim_end().ends_with("Line 3"));
}

#[test]
fn test_paragraph_only_hard_breaks() {
    let paragraph = Node::Paragraph(vec![
        Node::HardBreak,
        Node::HardBreak,
        Node::HardBreak,
        Node::HardBreak,
    ]);

    let mut writer = CommonMarkWriter::new();
    writer.write(&paragraph).unwrap();
    let result = writer.into_string();

    assert!(result == "\n");
    assert!(!result.contains("  \n"));
    assert!(!result.contains("\\\n"));
}

#[test]
fn test_document_with_paragraphs_trailing_hard_breaks() {
    let document = Node::Document(vec![
        Node::Paragraph(vec![Node::Text("First paragraph".into()), Node::HardBreak]),
        Node::Paragraph(vec![
            Node::Text("Second paragraph".into()),
            Node::HardBreak,
            Node::HardBreak,
        ]),
    ]);

    let mut writer = CommonMarkWriter::new();
    writer.write(&document).unwrap();
    let result = writer.into_string();

    let lines: Vec<&str> = result.lines().collect();
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[0], "First paragraph");
    assert_eq!(lines[1], "");
    assert_eq!(lines[2], "Second paragraph");
}
