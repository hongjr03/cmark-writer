//! Tests for core traits

use cmark_writer::error::WriteResult;
use cmark_writer::traits::core::*;
use cmark_writer::writer::{CommonMarkWriter, HtmlWriter};
use std::any::Any;
use std::collections::HashMap;

// Mock custom node for testing
#[derive(Debug, Clone, PartialEq)]
struct MockCustomNode {
    content: String,
    is_block: bool,
}

impl cmark_writer::traits::core::NodeContent for MockCustomNode {
    fn is_block(&self) -> bool {
        self.is_block
    }

    fn type_name(&self) -> &'static str {
        "MockCustomNode"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl cmark_writer::traits::core::NodeClone for MockCustomNode {
    fn clone_box(&self) -> Box<dyn NodeContent> {
        Box::new(self.clone())
    }

    fn eq_box(&self, other: &dyn NodeContent) -> bool {
        if let Some(other_mock) = other.as_any().downcast_ref::<MockCustomNode>() {
            self == other_mock
        } else {
            false
        }
    }
}

impl cmark_writer::traits::formatting::CommonMarkRenderable for MockCustomNode {
    fn render_commonmark(&self, writer: &mut CommonMarkWriter) -> WriteResult<()> {
        writer.write_str(&format!("[Mock: {}]", self.content))
    }
}

impl cmark_writer::traits::core::CustomNode for MockCustomNode {
    fn html_render(&self, writer: &mut HtmlWriter) -> WriteResult<()> {
        writer
            .raw_html(&format!("<mock>{}</mock>", self.content))
            .map_err(cmark_writer::error::WriteError::from)
    }

    fn attributes(&self) -> Option<&HashMap<String, String>> {
        None
    }

    fn supports_capability(&self, capability: &str) -> bool {
        matches!(capability, "commonmark" | "html" | "custom")
    }
}

#[test]
fn test_node_content_trait() {
    let block_node = MockCustomNode {
        content: "Block content".to_string(),
        is_block: true,
    };

    let inline_node = MockCustomNode {
        content: "Inline content".to_string(),
        is_block: false,
    };

    assert!(block_node.is_block());
    assert!(!inline_node.is_block());
    assert_eq!(block_node.type_name(), "MockCustomNode");
    assert_eq!(inline_node.type_name(), "MockCustomNode");
}

#[test]
fn test_node_clone_trait() {
    let original = MockCustomNode {
        content: "Test content".to_string(),
        is_block: false,
    };

    let cloned = original.clone_box();
    assert!(original.eq_box(cloned.as_ref()));

    let different = MockCustomNode {
        content: "Different content".to_string(),
        is_block: false,
    };

    assert!(!original.eq_box(&different));
}

#[test]
fn test_custom_node_capabilities() {
    let node = MockCustomNode {
        content: "Test".to_string(),
        is_block: false,
    };

    assert!(node.supports_capability("commonmark"));
    assert!(node.supports_capability("html"));
    assert!(node.supports_capability("custom"));
    assert!(!node.supports_capability("unknown"));
}

#[test]
fn test_custom_node_attributes() {
    let node = MockCustomNode {
        content: "Test".to_string(),
        is_block: false,
    };

    assert!(node.attributes().is_none());
}

#[test]
fn test_writer_trait_commonmark() {
    let mut writer = CommonMarkWriter::new();

    assert!(writer.write_str("Hello").is_ok());
    assert!(writer.write_char(' ').is_ok());
    assert!(writer.write_fmt(format_args!("World")).is_ok());

    let output = writer.into_string();
    assert_eq!(output, "Hello World");
}

#[test]
fn test_writer_trait_html() {
    let mut writer = HtmlWriter::new();

    assert!(writer.write_str("Hello").is_ok());
    assert!(writer.write_char(' ').is_ok());
    assert!(writer.write_fmt(format_args!("World")).is_ok());

    let output = writer.into_string();
    assert_eq!(output, "Hello World");
}

#[test]
fn test_any_casting() {
    let mut node = MockCustomNode {
        content: "Test".to_string(),
        is_block: false,
    };

    // Test immutable cast
    let any_ref = node.as_any();
    let cast_back = any_ref.downcast_ref::<MockCustomNode>();
    assert!(cast_back.is_some());
    assert_eq!(cast_back.unwrap().content, "Test");

    // Test mutable cast
    let any_mut = node.as_any_mut();
    let cast_back_mut = any_mut.downcast_mut::<MockCustomNode>();
    assert!(cast_back_mut.is_some());
    cast_back_mut.unwrap().content = "Modified".to_string();

    assert_eq!(node.content, "Modified");
}
