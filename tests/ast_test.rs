use cmark_rs::ast::{Alignment, ListItem, Node};

#[test]
fn test_alignment() {
    // Test alignment
    let alignment = Alignment::Center;
    assert_eq!(alignment, Alignment::Center);

    // Test different alignments
    let left_alignment = Alignment::Left;
    let right_alignment = Alignment::Right;

    assert_ne!(alignment, left_alignment);
    assert_ne!(alignment, right_alignment);
}

#[test]
fn test_node_equality() {
    // Test node equality
    let text1 = Node::Text("Hello".to_string());
    let text2 = Node::Text("Hello".to_string());
    let text3 = Node::Text("World".to_string());

    assert_eq!(text1, text2);
    assert_ne!(text1, text3);

    // Test complex node equality
    let heading1 = Node::Heading {
        level: 2,
        content: vec![Node::Text("Title".to_string())],
    };

    let heading2 = Node::Heading {
        level: 2,
        content: vec![Node::Text("Title".to_string())],
    };

    let heading3 = Node::Heading {
        level: 3, // Different level
        content: vec![Node::Text("Title".to_string())],
    };

    assert_eq!(heading1, heading2);
    assert_ne!(heading1, heading3);
}

#[test]
fn test_document_creation() {
    // Create a test document and verify its structure
    let document = Node::Document(vec![
        Node::Heading {
            level: 1,
            content: vec![Node::Text("Document Title".to_string())],
        },
        Node::Paragraph(vec![
            Node::Text("Paragraph with ".to_string()),
            Node::Emphasis(vec![Node::Text("emphasis".to_string())]),
            Node::Text(" and ".to_string()),
            Node::Strong(vec![Node::Text("strong".to_string())]),
            Node::Text(" text.".to_string()),
        ]),
    ]);

    // Check document structure
    if let Node::Document(children) = &document {
        assert_eq!(children.len(), 2);

        // Verify heading
        if let Node::Heading { level, content } = &children[0] {
            assert_eq!(*level, 1);
            assert_eq!(content.len(), 1);
            assert_eq!(content[0], Node::Text("Document Title".to_string()));
        } else {
            panic!("First child should be a heading");
        }

        // Verify paragraph
        if let Node::Paragraph(content) = &children[1] {
            assert_eq!(content.len(), 5);
            assert_eq!(content[0], Node::Text("Paragraph with ".to_string()));

            if let Node::Emphasis(emph_content) = &content[1] {
                assert_eq!(emph_content[0], Node::Text("emphasis".to_string()));
            } else {
                panic!("Second element should be emphasis");
            }

            if let Node::Strong(strong_content) = &content[3] {
                assert_eq!(strong_content[0], Node::Text("strong".to_string()));
            } else {
                panic!("Fourth element should be strong");
            }
        } else {
            panic!("Second child should be a paragraph");
        }
    } else {
        panic!("Document should be of type Document");
    }
}

#[test]
fn test_list_item() {
    // Test task list item
    let task_item = ListItem {
        content: vec![Node::Text("Task item".to_string())],
        is_task: true,
        task_completed: false,
    };

    assert!(task_item.is_task);
    assert!(!task_item.task_completed);
    assert_eq!(task_item.content.len(), 1);
    assert_eq!(task_item.content[0], Node::Text("Task item".to_string()));

    // Test completed task list item
    let completed_task = ListItem {
        content: vec![Node::Text("Completed task".to_string())],
        is_task: true,
        task_completed: true,
    };

    assert!(completed_task.is_task);
    assert!(completed_task.task_completed);

    // Test regular list item
    let regular_item = ListItem {
        content: vec![Node::Text("Regular item".to_string())],
        is_task: false,
        task_completed: false,
    };

    assert!(!regular_item.is_task);
}
