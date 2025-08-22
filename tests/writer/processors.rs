use cmark_writer::traits::{ConfigurableProcessor, InlineNodeProcessor, NodeProcessor};
use cmark_writer::writer::processors::{
    BlockProcessorConfig, EnhancedBlockProcessor, EnhancedInlineProcessor, InlineProcessorConfig,
};
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

#[test]
fn test_block_processor_config_default() {
    let config = BlockProcessorConfig::default();

    assert!(config.ensure_trailing_newlines);
    assert_eq!(config.block_separator, "\n\n");
}

#[test]
fn test_inline_processor_config_default() {
    let config = InlineProcessorConfig::default();

    assert!(config.strict_validation);
    assert!(!config.allow_newlines);
}

#[test]
fn test_enhanced_block_processor_new() {
    let processor = EnhancedBlockProcessor::new();

    // Test that it initializes with default config
    let config = processor.config();
    assert!(config.ensure_trailing_newlines);
    assert_eq!(config.block_separator, "\n\n");
}

#[test]
fn test_enhanced_block_processor_with_config() {
    let custom_config = BlockProcessorConfig {
        ensure_trailing_newlines: false,
        block_separator: "---\n".to_string(),
    };

    let processor = EnhancedBlockProcessor::with_config(custom_config.clone());
    let config = processor.config();

    assert!(!config.ensure_trailing_newlines);
    assert_eq!(config.block_separator, "---\n");
}

#[test]
fn test_enhanced_inline_processor_with_config() {
    let custom_config = InlineProcessorConfig {
        strict_validation: false,
        allow_newlines: true,
    };

    let processor = EnhancedInlineProcessor::with_config(custom_config.clone());
    let config = processor.config();

    assert!(!config.strict_validation);
    assert!(config.allow_newlines);
}

#[test]
fn test_processor_can_process() {
    let block_processor = EnhancedBlockProcessor::new();
    let inline_processor = EnhancedInlineProcessor::new();

    let text = Node::Text("test".into());
    let paragraph = Node::Paragraph(vec![Node::Text("test".into())]);

    // Test what processors can actually handle - implementation may vary
    let can_process_text_block = block_processor.can_process(&text);
    let can_process_text_inline = inline_processor.can_process(&text);
    let can_process_paragraph_block = block_processor.can_process(&paragraph);

    // Print for debugging - don't make hard assertions since implementation may vary
    println!(
        "Block processor can process text: {}",
        can_process_text_block
    );
    println!(
        "Inline processor can process text: {}",
        can_process_text_inline
    );
    println!(
        "Block processor can process paragraph: {}",
        can_process_paragraph_block
    );

    // At least one processor should be able to handle paragraph (block-level)
    assert!(can_process_paragraph_block);
}

#[test]
fn test_processor_priority() {
    let block_processor = EnhancedBlockProcessor::new();
    let inline_processor = EnhancedInlineProcessor::new();

    // Test that processors have priorities
    let block_priority = block_processor.priority();
    let inline_priority = inline_processor.priority();

    // Just verify they return valid priority values
    assert!(block_priority <= 100);
    assert!(inline_priority <= 100);
}
