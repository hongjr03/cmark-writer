use cmark_writer::traits::{ConfigurableProcessor, InlineNodeProcessor};
use cmark_writer::writer::processors::{EnhancedInlineProcessor, InlineProcessorConfig};
use cmark_writer::{CommonMarkWriter, Node, ToCommonMark};

#[test]
fn inline_processor_rejects_newlines_by_default() {
    let p = EnhancedInlineProcessor::new();
    let mut w = CommonMarkWriter::new();
    let n = Node::Emphasis(vec![Node::Text("foo\nbar".into())]);
    // Using processor.validate_inline_content directly
    let err = InlineNodeProcessor::validate_inline_content(&p, &n).unwrap_err();
    assert!(format!("{}", err).contains("Newline"));
    // Writer path should also error
    assert!(n.to_commonmark(&mut w).is_err());
}

#[test]
fn inline_processor_allow_newlines_when_configured() {
    let mut p = EnhancedInlineProcessor::new();
    ConfigurableProcessor::configure(
        &mut p,
        InlineProcessorConfig {
            strict_validation: true,
            allow_newlines: true,
        },
    );
    // Should pass validation now
    let n = Node::Emphasis(vec![Node::Text("foo\nbar".into())]);
    assert!(InlineNodeProcessor::validate_inline_content(&p, &n).is_ok());
}
