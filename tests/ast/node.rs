use cmark_writer::ast::{HeadingType, ListItem, Node};

#[test]
fn node_equality() {
    let text1 = Node::Text("Hello".into());
    let text2 = Node::Text("Hello".into());
    let text3 = Node::Text("World".into());
    assert_eq!(text1, text2);
    assert_ne!(text1, text3);

    let h1 = Node::Heading { level: 2, content: vec![Node::Text("Title".into())], heading_type: HeadingType::Atx };
    let h2 = Node::Heading { level: 2, content: vec![Node::Text("Title".into())], heading_type: HeadingType::Atx };
    let h3 = Node::Heading { level: 3, content: vec![Node::Text("Title".into())], heading_type: HeadingType::Atx };
    assert_eq!(h1, h2);
    assert_ne!(h1, h3);
}

#[test]
fn document_construction() {
    let doc = Node::Document(vec![
        Node::Heading { level: 1, content: vec![Node::Text("Document Title".into())], heading_type: HeadingType::Atx },
        Node::Paragraph(vec![
            Node::Text("Paragraph with ".into()),
            Node::Emphasis(vec![Node::Text("emphasis".into())]),
            Node::Text(" and ".into()),
            Node::Strong(vec![Node::Text("strong".into())]),
            Node::Text(" text.".into()),
        ]),
    ]);

    if let Node::Document(children) = &doc {
        assert_eq!(children.len(), 2);
        if let Node::Heading { level, content, .. } = &children[0] {
            assert_eq!(*level, 1);
            assert_eq!(content[0], Node::Text("Document Title".into()));
        } else { panic!("First child should be a heading"); }

        if let Node::Paragraph(content) = &children[1] {
            assert_eq!(content.len(), 5);
        } else { panic!("Second child should be a paragraph"); }
    } else {
        panic!("Document should be of type Document");
    }
}

#[test]
fn list_items() {
    let unordered_item = ListItem::Unordered { content: vec![Node::Paragraph(vec![Node::Text("Unordered item".into())])]};
    if let ListItem::Unordered { content } = &unordered_item {
        assert_eq!(content.len(), 1);
    } else { panic!("Should be unordered"); }

    let ordered_item = ListItem::Ordered { number: Some(3), content: vec![Node::Paragraph(vec![Node::Text("Ordered item".into())])]};
    if let ListItem::Ordered { number, content } = &ordered_item {
        assert_eq!(*number, Some(3));
        assert_eq!(content.len(), 1);
    } else { panic!("Should be ordered"); }
}

#[test]
fn node_type_checks() {
    let heading = Node::Heading { level: 1, content: vec![Node::Text("Heading".into())], heading_type: HeadingType::Atx };
    assert!(heading.is_block());
    assert!(!heading.is_inline());

    let text = Node::Text("Hello".into());
    assert!(text.is_inline());
    assert!(!text.is_block());
}

#[test]
fn constructors() {
    let heading = Node::heading(2, vec![Node::Text("标题".into())]);
    if let Node::Heading { level, content, heading_type } = &heading {
        assert_eq!(*level, 2);
        assert_eq!(content[0], Node::Text("标题".into()));
        assert_eq!(*heading_type, HeadingType::Atx);
    } else { panic!("expected heading"); }

    let rust_code = Node::code_block(Some("rust".into()), "fn main() {}\n".into());
    if let Node::CodeBlock { language, content, block_type } = &rust_code {
        assert_eq!(*language, Some("rust".into()));
        assert_eq!(*content, "fn main() {}\n".to_string());
        assert!(matches!(*block_type, cmark_writer::ast::CodeBlockType::Fenced));
    } else { panic!("expected code block"); }
}
