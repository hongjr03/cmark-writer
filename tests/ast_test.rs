use cmark_writer::ast::{Alignment, BlockNode, InlineNode, ListItem, Node};

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
    // Test inline node equality
    let text1 = InlineNode::Text("Hello".to_string());
    let text2 = InlineNode::Text("Hello".to_string());
    let text3 = InlineNode::Text("World".to_string());

    assert_eq!(text1, text2);
    assert_ne!(text1, text3);

    // Convert to Node type
    let node1 = Node::Inline(text1);
    let node2 = Node::Inline(text2);
    let node3 = Node::Inline(text3);

    assert_eq!(node1, node2);
    assert_ne!(node1, node3);

    // Test complex node equality
    let heading1 = BlockNode::Heading {
        level: 2,
        content: vec![InlineNode::Text("Title".to_string())],
    };

    let heading2 = BlockNode::Heading {
        level: 2,
        content: vec![InlineNode::Text("Title".to_string())],
    };

    let heading3 = BlockNode::Heading {
        level: 3, // Different level
        content: vec![InlineNode::Text("Title".to_string())],
    };

    assert_eq!(heading1, heading2);
    assert_ne!(heading1, heading3);

    // Convert to Node type
    let node_heading1 = Node::Block(heading1);
    let node_heading2 = Node::Block(heading2);
    let node_heading3 = Node::Block(heading3);

    assert_eq!(node_heading1, node_heading2);
    assert_ne!(node_heading1, node_heading3);
}

#[test]
fn test_document_creation() {
    // Create a test document and verify its structure
    let document = BlockNode::Document(vec![
        BlockNode::Heading {
            level: 1,
            content: vec![InlineNode::Text("Document Title".to_string())],
        },
        BlockNode::Paragraph(vec![
            InlineNode::Text("Paragraph with ".to_string()),
            InlineNode::Emphasis(vec![InlineNode::Text("emphasis".to_string())]),
            InlineNode::Text(" and ".to_string()),
            InlineNode::Strong(vec![InlineNode::Text("strong".to_string())]),
            InlineNode::Text(" text.".to_string()),
        ]),
    ]);

    // Check document structure
    if let BlockNode::Document(children) = &document {
        assert_eq!(children.len(), 2);

        // Verify heading
        if let BlockNode::Heading { level, content } = &children[0] {
            assert_eq!(*level, 1);
            assert_eq!(content.len(), 1);
            assert_eq!(content[0], InlineNode::Text("Document Title".to_string()));
        } else {
            panic!("First child should be a heading");
        }

        // Verify paragraph
        if let BlockNode::Paragraph(content) = &children[1] {
            assert_eq!(content.len(), 5);
            assert_eq!(content[0], InlineNode::Text("Paragraph with ".to_string()));

            if let InlineNode::Emphasis(emph_content) = &content[1] {
                assert_eq!(emph_content[0], InlineNode::Text("emphasis".to_string()));
            } else {
                panic!("Second element should be emphasis");
            }

            if let InlineNode::Strong(strong_content) = &content[3] {
                assert_eq!(strong_content[0], InlineNode::Text("strong".to_string()));
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
    let task_item = ListItem::Task {
        completed: false,
        content: vec![BlockNode::Paragraph(vec![InlineNode::Text(
            "Task item".to_string(),
        )])],
    };

    if let ListItem::Task { completed, content } = &task_item {
        assert!(!completed);
        assert_eq!(content.len(), 1);
        if let BlockNode::Paragraph(text) = &content[0] {
            assert_eq!(text[0], InlineNode::Text("Task item".to_string()));
        } else {
            panic!("Content should be a paragraph");
        }
    } else {
        panic!("Should be a task list item");
    }

    // Test completed task list item
    let completed_task = ListItem::Task {
        completed: true,
        content: vec![BlockNode::Paragraph(vec![InlineNode::Text(
            "Completed task".to_string(),
        )])],
    };

    if let ListItem::Task {
        completed,
        content: _,
    } = &completed_task
    {
        assert!(*completed);
    } else {
        panic!("Should be a task list item");
    }

    // Test regular list item
    let regular_item = ListItem::Regular {
        content: vec![BlockNode::Paragraph(vec![InlineNode::Text(
            "Regular item".to_string(),
        )])],
    };

    if let ListItem::Regular { content } = &regular_item {
        assert_eq!(content.len(), 1);
        if let BlockNode::Paragraph(text) = &content[0] {
            assert_eq!(text[0], InlineNode::Text("Regular item".to_string()));
        } else {
            panic!("Content should be a paragraph");
        }
    } else {
        panic!("Should be a regular list item");
    }
}
