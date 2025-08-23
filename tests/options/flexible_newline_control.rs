//! Tests for flexible newline control API

use cmark_writer::ast::Node;
use cmark_writer::writer::{CommonMarkWriter, NewlineContext, NewlineStrategy, RenderingMode};

#[test]
fn test_basic_newline_context() {
    let writer = CommonMarkWriter::new();

    // Test that we can get current context
    let context = writer.current_context();
    assert_eq!(context.mode, RenderingMode::Block);
    assert_eq!(context.strategy, NewlineStrategy::Always);
    assert!(context.allows_blocks);
}

#[test]
fn test_push_pop_context() {
    let mut writer = CommonMarkWriter::new();

    // Push a new context
    let context = NewlineContext::pure_inline();
    writer.push_context(context);

    // Check current context
    let current = writer.current_context();
    assert_eq!(current.mode, RenderingMode::PureInline);
    assert_eq!(current.strategy, NewlineStrategy::None);
    assert!(!current.allows_blocks);

    // Pop context
    writer.pop_context();

    // Should be back to default block context
    let current = writer.current_context();
    assert_eq!(current.mode, RenderingMode::Block);
}

#[test]
fn test_with_context_temporary() {
    let mut writer = CommonMarkWriter::new();

    // Use temporary context
    let result = writer
        .with_temp_context(NewlineContext::pure_inline(), |w| {
            let context = w.current_context();
            assert_eq!(context.mode, RenderingMode::PureInline);
            Ok(42)
        })
        .unwrap();

    assert_eq!(result, 42);

    // Should be back to original context
    let context = writer.current_context();
    assert_eq!(context.mode, RenderingMode::Block);
}

#[test]
fn test_newline_strategy_decisions() {
    // Test different newline strategies

    // Always strategy
    let context = NewlineContext::custom(NewlineStrategy::Always, true);
    assert!(context.should_add_trailing_newline("content", None));
    assert!(context.should_add_trailing_newline("content\n", None));

    // None strategy
    let context = NewlineContext::custom(NewlineStrategy::None, true);
    assert!(!context.should_add_trailing_newline("content", None));
    assert!(!context.should_add_trailing_newline("content\n", None));

    // Conditional strategy
    let context = NewlineContext::custom(NewlineStrategy::Conditional, true);
    assert!(context.should_add_trailing_newline("content", None));
    assert!(!context.should_add_trailing_newline("content\n", None));
}

#[test]
fn test_context_validation() {
    let context = NewlineContext::pure_inline();

    // Inline node should be valid
    let inline_node = Node::Text("test".into());
    assert!(context.validate_node(&inline_node).is_ok());

    // Block node should be invalid
    let block_node = Node::Paragraph(vec![Node::Text("test".into())]);
    assert!(context.validate_node(&block_node).is_err());
}

#[test]
fn test_rendering_modes() {
    // Test different contexts can be created
    let block_ctx = NewlineContext::block();
    assert_eq!(block_ctx.mode, RenderingMode::Block);
    assert!(block_ctx.allows_blocks);

    let inline_ctx = NewlineContext::pure_inline();
    assert_eq!(inline_ctx.mode, RenderingMode::PureInline);
    assert!(!inline_ctx.allows_blocks);

    let table_ctx = NewlineContext::table_cell();
    assert_eq!(table_ctx.mode, RenderingMode::TableCell);
    assert!(!table_ctx.allows_blocks);

    let list_ctx = NewlineContext::list_item();
    assert_eq!(list_ctx.mode, RenderingMode::ListItem);
    assert!(list_ctx.allows_blocks);
}

#[test]
fn test_context_builder() {
    use cmark_writer::writer::context::NewlineContextBuilder;

    let context = NewlineContextBuilder::new()
        .mode(RenderingMode::InlineWithBlocks)
        .strategy(NewlineStrategy::Smart)
        .allow_blocks(true)
        .container_end(true)
        .custom_data("test-data".to_string())
        .build();

    assert_eq!(context.mode, RenderingMode::InlineWithBlocks);
    assert_eq!(context.strategy, NewlineStrategy::Smart);
    assert!(context.allows_blocks);
    assert!(context.is_container_end);
    assert_eq!(context.custom_data, Some("test-data".to_string()));
}
