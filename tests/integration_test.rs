use cmark_writer::ast::{BlockNode, InlineNode, ListItem, Node};
use cmark_writer::writer::CommonMarkWriter;

#[test]
fn test_simple_document() {
    // Create a simple document structure
    let document = BlockNode::Document(vec![
        BlockNode::Heading {
            level: 1,
            content: vec![InlineNode::Text("Title".to_string())],
        },
        BlockNode::Paragraph(vec![
            InlineNode::Text("Regular text ".to_string()),
            InlineNode::Strong(vec![InlineNode::Text("bold text".to_string())]),
            InlineNode::Text(" regular text".to_string()),
        ]),
    ]);

    // Convert to Node for API compatibility
    let node_document = Node::Block(document);

    // Write as CommonMark
    let mut writer = CommonMarkWriter::new();
    writer
        .write(&node_document)
        .expect("Failed to write document");
    let result = writer.into_string();

    // Verify result, note that spacing handling is fixed
    let expected = "# Title\n\nRegular text **bold text** regular text";
    assert_eq!(result, expected);
}

#[test]
fn test_complex_document() {
    // Create a document containing various elements
    let document = BlockNode::Document(vec![
        BlockNode::Heading {
            level: 2,
            content: vec![InlineNode::Text("List Example".to_string())],
        },
        BlockNode::UnorderedList(vec![
            ListItem::Regular {
                content: vec![BlockNode::Paragraph(vec![InlineNode::Text(
                    "Item 1".to_string(),
                )])],
            },
            ListItem::Task {
                completed: true,
                content: vec![BlockNode::Paragraph(vec![InlineNode::Text(
                    "Item 2".to_string(),
                )])],
            },
        ]),
        BlockNode::CodeBlock {
            language: Some("rust".to_string()),
            content: "fn main() {\n    println!(\"Hello\");\n}".to_string(),
        },
    ]);

    // Convert to Node for API compatibility
    let node_document = Node::Block(document);

    let mut writer = CommonMarkWriter::new();
    writer
        .write(&node_document)
        .expect("Failed to write document");
    let result = writer.into_string();

    let expected = "## List Example\n\n- Item 1\n- [x] Item 2\n\n```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
    assert_eq!(result, expected);
}
