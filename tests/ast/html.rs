//! Tests for HTML element AST structures

use cmark_writer::{
    ast::{HtmlAttribute, HtmlElement, Node},
    HeadingType,
};

#[test]
fn test_html_attribute_creation() {
    let attr = HtmlAttribute {
        name: "class".into(),
        value: "test-class".into(),
    };

    assert_eq!(attr.name, "class");
    assert_eq!(attr.value, "test-class");
}

#[test]
fn test_html_element_new() {
    let element = HtmlElement::new("div");

    assert_eq!(element.tag, "div");
    assert!(element.attributes.is_empty());
    assert!(element.children.is_empty());
    assert!(!element.self_closing);
}

#[test]
fn test_html_element_with_attribute() {
    let element = HtmlElement::new("div")
        .with_attribute("id", "main")
        .with_attribute("class", "container");

    assert_eq!(element.attributes.len(), 2);
    assert_eq!(element.attributes[0].name, "id");
    assert_eq!(element.attributes[0].value, "main");
    assert_eq!(element.attributes[1].name, "class");
    assert_eq!(element.attributes[1].value, "container");
}

#[test]
fn test_html_element_with_attributes() {
    let element = HtmlElement::new("input").with_attributes(vec![
        ("type", "text"),
        ("name", "username"),
        ("placeholder", "Enter username"),
    ]);

    assert_eq!(element.attributes.len(), 3);
    assert_eq!(element.attributes[0].name, "type");
    assert_eq!(element.attributes[0].value, "text");
    assert_eq!(element.attributes[1].name, "name");
    assert_eq!(element.attributes[1].value, "username");
    assert_eq!(element.attributes[2].name, "placeholder");
    assert_eq!(element.attributes[2].value, "Enter username");
}

#[test]
fn test_html_element_with_children() {
    let children = vec![
        Node::Text("Hello ".into()),
        Node::Strong(vec![Node::Text("world".into())]),
        Node::Text("!".into()),
    ];

    let element = HtmlElement::new("p").with_children(children.clone());

    assert_eq!(element.children, children);
}

#[test]
fn test_html_element_self_closing() {
    let element = HtmlElement::new("img")
        .with_attribute("src", "image.jpg")
        .self_closing(true);

    assert!(element.self_closing);

    let element = HtmlElement::new("div").self_closing(false);
    assert!(!element.self_closing);
}

#[test]
fn test_html_element_tag_matches_any() {
    let element = HtmlElement::new("div");

    let tags = vec!["span".into(), "div".into(), "p".into()];
    assert!(element.tag_matches_any(&tags));

    let tags = vec!["span".into(), "p".into()];
    assert!(!element.tag_matches_any(&tags));

    // Test case-insensitive matching
    let element = HtmlElement::new("DIV");
    let tags = vec!["div".into(), "span".into()];
    assert!(element.tag_matches_any(&tags));

    let element = HtmlElement::new("div");
    let tags = vec!["DIV".into(), "SPAN".into()];
    assert!(element.tag_matches_any(&tags));
}

#[test]
fn test_html_element_fluent_api() {
    let element = HtmlElement::new("section")
        .with_attribute("id", "content")
        .with_attributes(vec![("class", "main"), ("data-role", "content")])
        .with_children(vec![
            Node::Heading {
                level: 1,
                content: vec![Node::Text("Title".into())],
                heading_type: HeadingType::Atx,
            },
            Node::Paragraph(vec![Node::Text("Content paragraph.".into())]),
        ])
        .self_closing(false);

    assert_eq!(element.tag, "section");
    assert_eq!(element.attributes.len(), 3);
    assert_eq!(element.children.len(), 2);
    assert!(!element.self_closing);
}

#[test]
fn test_html_element_clone_and_partial_eq() {
    let element1 = HtmlElement::new("p")
        .with_attribute("class", "text")
        .with_children(vec![Node::Text("Hello".into())]);

    let element2 = element1.clone();
    assert_eq!(element1, element2);

    let element3 = HtmlElement::new("p")
        .with_attribute("class", "different")
        .with_children(vec![Node::Text("Hello".into())]);

    assert_ne!(element1, element3);
}

#[test]
fn test_html_attribute_clone_and_partial_eq() {
    let attr1 = HtmlAttribute {
        name: "id".into(),
        value: "test".into(),
    };

    let attr2 = attr1.clone();
    assert_eq!(attr1, attr2);

    let attr3 = HtmlAttribute {
        name: "id".into(),
        value: "different".into(),
    };

    assert_ne!(attr1, attr3);
}
