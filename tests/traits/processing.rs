//! Tests for processing traits

use cmark_writer::{
    ast::Node,
    error::{WriteError, WriteResult},
    traits::processing::*,
    writer::{CommonMarkWriter, HtmlWriter},
};

// Mock processor for testing
#[derive(Debug)]
struct MockProcessor {
    can_process_result: bool,
    priority: u32,
}

impl MockProcessor {
    fn new(can_process: bool, priority: u32) -> Self {
        Self {
            can_process_result: can_process,
            priority,
        }
    }
}

impl NodeProcessor for MockProcessor {
    fn can_process(&self, _node: &Node) -> bool {
        self.can_process_result
    }

    fn process_commonmark(&self, writer: &mut CommonMarkWriter, _node: &Node) -> WriteResult<()> {
        writer.write_str("mock processor commonmark output")
    }

    fn process_html(&self, writer: &mut HtmlWriter, _node: &Node) -> WriteResult<()> {
        writer
            .raw_html("<!-- mock processor html output -->")
            .map_err(cmark_writer::error::WriteError::from)
    }

    fn priority(&self) -> u32 {
        self.priority
    }
}

// Mock block processor
#[derive(Debug)]
struct MockBlockProcessor {
    base: MockProcessor,
}

impl MockBlockProcessor {
    fn new() -> Self {
        Self {
            base: MockProcessor::new(true, 10),
        }
    }
}

impl NodeProcessor for MockBlockProcessor {
    fn can_process(&self, node: &Node) -> bool {
        self.base.can_process(node)
    }

    fn process_commonmark(&self, writer: &mut CommonMarkWriter, node: &Node) -> WriteResult<()> {
        self.base.process_commonmark(writer, node)
    }

    fn process_html(&self, writer: &mut HtmlWriter, node: &Node) -> WriteResult<()> {
        self.base.process_html(writer, node)
    }

    fn priority(&self) -> u32 {
        self.base.priority()
    }
}

impl BlockNodeProcessor for MockBlockProcessor {
    fn ensure_block_separation(
        &self,
        _writer: &mut dyn cmark_writer::traits::core::Writer,
    ) -> WriteResult<()> {
        // Mock implementation
        Ok(())
    }
}

// Mock inline processor
#[derive(Debug)]
struct MockInlineProcessor {
    base: MockProcessor,
    validation_result: WriteResult<()>,
}

impl MockInlineProcessor {
    fn new(validation_result: WriteResult<()>) -> Self {
        Self {
            base: MockProcessor::new(true, 5),
            validation_result,
        }
    }
}

impl NodeProcessor for MockInlineProcessor {
    fn can_process(&self, node: &Node) -> bool {
        self.base.can_process(node)
    }

    fn process_commonmark(&self, writer: &mut CommonMarkWriter, node: &Node) -> WriteResult<()> {
        self.base.process_commonmark(writer, node)
    }

    fn process_html(&self, writer: &mut HtmlWriter, node: &Node) -> WriteResult<()> {
        self.base.process_html(writer, node)
    }

    fn priority(&self) -> u32 {
        self.base.priority()
    }
}

impl InlineNodeProcessor for MockInlineProcessor {
    fn validate_inline_content(&self, _node: &Node) -> WriteResult<()> {
        match &self.validation_result {
            Ok(_) => Ok(()),
            Err(_) => Err(WriteError::custom("validation failed")),
        }
    }
}

#[test]
fn test_node_processor_can_process() {
    let processor = MockProcessor::new(true, 0);
    let node = Node::Text("test".into());

    assert!(processor.can_process(&node));

    let processor_false = MockProcessor::new(false, 0);
    assert!(!processor_false.can_process(&node));
}

#[test]
fn test_node_processor_priority() {
    let processor_high = MockProcessor::new(true, 100);
    let processor_low = MockProcessor::new(true, 1);
    let processor_default = MockProcessor::new(true, 0);

    assert_eq!(processor_high.priority(), 100);
    assert_eq!(processor_low.priority(), 1);
    assert_eq!(processor_default.priority(), 0);
}

#[test]
fn test_node_processor_commonmark() {
    let processor = MockProcessor::new(true, 0);
    let node = Node::Text("test".into());
    let mut writer = CommonMarkWriter::new();

    let result = processor.process_commonmark(&mut writer, &node);
    assert!(result.is_ok());

    let output = writer.into_string();
    assert!(output.contains("mock processor commonmark output"));
}

#[test]
fn test_node_processor_html() {
    let processor = MockProcessor::new(true, 0);
    let node = Node::Text("test".into());
    let mut writer = HtmlWriter::new();

    let result = processor.process_html(&mut writer, &node);
    assert!(result.is_ok());

    let output = writer.into_string();
    assert!(output.contains("mock processor html output"));
}

#[test]
fn test_block_node_processor() {
    let processor = MockBlockProcessor::new();
    let node = Node::Text("test".into());

    // Test basic functionality
    assert!(processor.can_process(&node));
    assert_eq!(processor.priority(), 10);

    // Test CommonMark processing
    let mut writer = CommonMarkWriter::new();
    let result = processor.process_commonmark(&mut writer, &node);
    assert!(result.is_ok());

    // Test HTML processing
    let mut html_writer = HtmlWriter::new();
    let result = processor.process_html(&mut html_writer, &node);
    assert!(result.is_ok());
}

#[test]
fn test_inline_node_processor_success() {
    let processor = MockInlineProcessor::new(Ok(()));
    let node = Node::Text("test".into());

    // Test basic functionality
    assert!(processor.can_process(&node));
    assert_eq!(processor.priority(), 5);

    // Test validation
    let result = processor.validate_inline_content(&node);
    assert!(result.is_ok());
}

#[test]
fn test_inline_node_processor_validation_error() {
    let error = WriteError::custom("validation failed");
    let processor = MockInlineProcessor::new(Err(error));
    let node = Node::Text("test".into());

    let result = processor.validate_inline_content(&node);
    assert!(result.is_err());
}
