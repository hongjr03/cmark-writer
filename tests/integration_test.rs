use cmark_writer::ast::{CodeBlockType, HeadingType, ListItem, Node};
use cmark_writer::writer::CommonMarkWriter;
use cmark_writer::ToCommonMark;

#[test]
fn test_simple_document() {
    // Create a simple document structure
    let document = Node::Document(vec![
        Node::Heading {
            level: 1,
            content: vec![Node::Text("Title".into())],
            heading_type: HeadingType::Atx,
        },
        Node::Paragraph(vec![
            Node::Text("Regular text ".into()),
            Node::Strong(vec![Node::Text("bold text".into())]),
            Node::Text(" regular text".into()),
        ]),
    ]);

    // Document is already a Node, so use it directly
    let node_document = document;

    // Write as CommonMark
    let mut writer = CommonMarkWriter::new();
    node_document
        .to_commonmark(&mut writer)
        .expect("Failed to write document");
    let result = writer.into_string();

    // Verify result, note that spacing handling is fixed
    let expected = "# Title\n\nRegular text **bold text** regular text\n";
    assert_eq!(result, expected);
}

#[test]
fn test_complex_document() {
    // Create a document containing various elements
    let document = Node::Document(vec![
        Node::Heading {
            level: 2,
            content: vec![Node::Text("List Example".into())],
            heading_type: HeadingType::Atx,
        },
        Node::UnorderedList(vec![
            ListItem::Unordered {
                content: vec![Node::Paragraph(vec![Node::Text("Item 1".into())])],
            },
            ListItem::Unordered {
                content: vec![Node::Paragraph(vec![Node::Text("Item 2".into())])],
            },
        ]),
        Node::CodeBlock {
            language: Some("rust".into()),
            content: "fn main() {\n    println!(\"Hello\");\n}".into(),
            block_type: CodeBlockType::Fenced,
        },
    ]);

    let node_document = document;

    let mut writer = CommonMarkWriter::new();
    node_document
        .to_commonmark(&mut writer)
        .expect("Failed to write document");
    let result = writer.into_string();

    let expected = "## List Example\n\n- Item 1\n- Item 2\n\n```rust\nfn main() {\n    println!(\"Hello\");\n}\n```\n";
    assert_eq!(result, expected);
}
