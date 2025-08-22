//! Tests for GFM formatting utilities

#![cfg(feature = "gfm")]

use cmark_writer::ast::Node;
use cmark_writer::gfm::formatting::*;

#[test]
fn test_strikethrough() {
    let content = vec![
        Node::Text("Hello ".into()),
        Node::Strong(vec![Node::Text("world".into())]),
    ];

    let strike_node = strikethrough(content.clone());

    match strike_node {
        Node::Strikethrough(children) => {
            assert_eq!(children, content);
        }
        _ => panic!("Expected Strikethrough node"),
    }
}

#[test]
fn test_strike_text() {
    let text = "Hello world";
    let strike_node = strike_text(text);

    match strike_node {
        Node::Strikethrough(children) => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                Node::Text(t) => assert_eq!(t, text),
                _ => panic!("Expected Text node"),
            }
        }
        _ => panic!("Expected Strikethrough node"),
    }
}

#[test]
fn test_strike_and_emphasize() {
    let text = "emphasized text";
    let node = strike_and_emphasize(text);

    match node {
        Node::Strikethrough(children) => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                Node::Emphasis(em_children) => {
                    assert_eq!(em_children.len(), 1);
                    match &em_children[0] {
                        Node::Text(t) => assert_eq!(t, text),
                        _ => panic!("Expected Text node inside Emphasis"),
                    }
                }
                _ => panic!("Expected Emphasis node inside Strikethrough"),
            }
        }
        _ => panic!("Expected Strikethrough node"),
    }
}

#[test]
fn test_strike_and_strong() {
    let text = "strong text";
    let node = strike_and_strong(text);

    match node {
        Node::Strikethrough(children) => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                Node::Strong(strong_children) => {
                    assert_eq!(strong_children.len(), 1);
                    match &strong_children[0] {
                        Node::Text(t) => assert_eq!(t, text),
                        _ => panic!("Expected Text node inside Strong"),
                    }
                }
                _ => panic!("Expected Strong node inside Strikethrough"),
            }
        }
        _ => panic!("Expected Strikethrough node"),
    }
}
