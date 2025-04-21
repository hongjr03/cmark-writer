use cmark_rs::ast::{ListItem, Node};
use cmark_rs::writer::CommonMarkWriter;

#[test]
fn test_simple_document() {
    // Create a simple document structure
    let document = Node::Document(vec![
        Node::Heading {
            level: 1,
            content: vec![Node::Text("Title".to_string())],
        },
        Node::Paragraph(vec![
            Node::Text("Regular text ".to_string()),
            Node::Strong(vec![Node::Text("bold text".to_string())]),
            Node::Text(" regular text".to_string()),
        ]),
    ]);
    
    // Write as CommonMark
    let mut writer = CommonMarkWriter::new();
    writer.write(&document).expect("Failed to write document");
    let result = writer.into_string();
    
    // Verify result, note that spacing handling is fixed
    let expected = "# Title\n\nRegular text **bold text** regular text";
    assert_eq!(result, expected);
}

#[test]
fn test_complex_document() {
    // Create a document containing various elements
    let document = Node::Document(vec![
        Node::Heading {
            level: 2,
            content: vec![Node::Text("List Example".to_string())],
        },
        Node::UnorderedList(vec![
            ListItem {
                content: vec![Node::Text("Item 1".to_string())],
                is_task: false,
                task_completed: false,
            },
            ListItem {
                content: vec![Node::Text("Item 2".to_string())],
                is_task: true,
                task_completed: true,
            },
        ]),
        Node::CodeBlock {
            language: Some("rust".to_string()),
            content: "fn main() {\n    println!(\"Hello\");\n}".to_string(),
        },
    ]);
    
    let mut writer = CommonMarkWriter::new();
    writer.write(&document).expect("Failed to write document");
    let result = writer.into_string();
    
    let expected = "## List Example\n\n- Item 1\n- [x] Item 2\n\n```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
    assert_eq!(result, expected);
}