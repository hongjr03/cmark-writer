//! Flexible newline control context for CommonMark writer.
//!
//! This module provides a sophisticated newline control system that allows fine-grained
//! control over trailing newlines based on content type, container context, and writing mode.

use crate::ast::Node;
use crate::error::{WriteError, WriteResult};

/// Newline control strategy for different writing scenarios
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NewlineStrategy {
    /// Never add trailing newlines
    None,
    /// Add trailing newline only if content doesn't already end with one
    Conditional,
    /// Always add trailing newline
    Always,
    /// Inherit from parent context
    Inherit,
    /// Smart mode: decide based on content and context
    Smart,
}

/// Content rendering mode that affects newline behavior
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RenderingMode {
    /// Standard block rendering (traditional behavior)
    Block,
    /// Inline rendering with block content support
    InlineWithBlocks,
    /// Pure inline rendering (no blocks allowed)
    PureInline,
    /// Table cell rendering
    TableCell,
    /// List item rendering
    ListItem,
    /// Custom container rendering
    Custom,
}

/// Context information for newline control decisions
#[derive(Debug, Clone)]
pub struct NewlineContext {
    /// Current rendering mode
    pub mode: RenderingMode,
    /// Newline strategy for this context
    pub strategy: NewlineStrategy,
    /// Whether this context allows block elements
    pub allows_blocks: bool,
    /// Whether we're at the end of a container
    pub is_container_end: bool,
    /// Parent context if nested
    pub parent: Option<Box<NewlineContext>>,
    /// Custom context data
    pub custom_data: Option<String>,
}

impl NewlineContext {
    /// Create a new block-level context
    pub fn block() -> Self {
        Self {
            mode: RenderingMode::Block,
            strategy: NewlineStrategy::Always,
            allows_blocks: true,
            is_container_end: false,
            parent: None,
            custom_data: None,
        }
    }

    /// Create a new inline context that can contain block elements
    pub fn inline_with_blocks() -> Self {
        Self {
            mode: RenderingMode::InlineWithBlocks,
            strategy: NewlineStrategy::Smart,
            allows_blocks: true,
            is_container_end: false,
            parent: None,
            custom_data: None,
        }
    }

    /// Create a new pure inline context
    pub fn pure_inline() -> Self {
        Self {
            mode: RenderingMode::PureInline,
            strategy: NewlineStrategy::None,
            allows_blocks: false,
            is_container_end: false,
            parent: None,
            custom_data: None,
        }
    }

    /// Create a table cell context
    pub fn table_cell() -> Self {
        Self {
            mode: RenderingMode::TableCell,
            strategy: NewlineStrategy::Smart,
            allows_blocks: false, // Standard tables don't allow block elements
            is_container_end: false,
            parent: None,
            custom_data: None,
        }
    }

    /// Create a list item context
    pub fn list_item() -> Self {
        Self {
            mode: RenderingMode::ListItem,
            strategy: NewlineStrategy::Conditional,
            allows_blocks: true,
            is_container_end: false,
            parent: None,
            custom_data: None,
        }
    }

    /// Create a custom context
    pub fn custom(strategy: NewlineStrategy, allows_blocks: bool) -> Self {
        Self {
            mode: RenderingMode::Custom,
            strategy,
            allows_blocks,
            is_container_end: false,
            parent: None,
            custom_data: None,
        }
    }

    /// Set the newline strategy
    pub fn with_strategy(mut self, strategy: NewlineStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Set whether blocks are allowed
    pub fn with_blocks_allowed(mut self, allows_blocks: bool) -> Self {
        self.allows_blocks = allows_blocks;
        self
    }

    /// Set container end flag
    pub fn with_container_end(mut self, is_container_end: bool) -> Self {
        self.is_container_end = is_container_end;
        self
    }

    /// Set parent context
    pub fn with_parent(mut self, parent: NewlineContext) -> Self {
        self.parent = Some(Box::new(parent));
        self
    }

    /// Set custom data
    pub fn with_custom_data(mut self, data: String) -> Self {
        self.custom_data = Some(data);
        self
    }

    /// Determine if a trailing newline should be added for given content
    pub fn should_add_trailing_newline(&self, content: &str, node: Option<&Node>) -> bool {
        match self.strategy {
            NewlineStrategy::None => false,
            NewlineStrategy::Always => true,
            NewlineStrategy::Conditional => !content.ends_with('\n'),
            NewlineStrategy::Inherit => {
                if let Some(parent) = &self.parent {
                    parent.should_add_trailing_newline(content, node)
                } else {
                    // Default behavior if no parent
                    match self.mode {
                        RenderingMode::Block => true,
                        RenderingMode::InlineWithBlocks => !content.ends_with('\n'),
                        _ => false,
                    }
                }
            }
            NewlineStrategy::Smart => self.smart_newline_decision(content, node),
        }
    }

    /// Smart newline decision based on content and context
    fn smart_newline_decision(&self, content: &str, node: Option<&Node>) -> bool {
        // If content already ends with newline, don't add another unless we're at container end
        if content.ends_with('\n') && !self.is_container_end {
            return false;
        }

        match self.mode {
            RenderingMode::Block => true,
            RenderingMode::InlineWithBlocks => {
                // If we have a node and it's a block, add newline
                if let Some(node) = node {
                    if node.is_block() {
                        return true;
                    }
                }
                // For mixed inline/block content, add newline if at container end
                self.is_container_end && !content.ends_with('\n')
            }
            RenderingMode::PureInline => false,
            RenderingMode::TableCell => {
                // In table cells, only add newline if explicitly at container end
                self.is_container_end && !content.ends_with('\n')
            }
            RenderingMode::ListItem => {
                // In list items, add newline conditionally
                !content.ends_with('\n')
            }
            RenderingMode::Custom => {
                // For custom contexts, use conditional logic
                !content.ends_with('\n')
            }
        }
    }

    /// Check if block elements are allowed in this context
    pub fn allows_block_elements(&self) -> bool {
        self.allows_blocks
    }

    /// Get the effective newline strategy (resolving inheritance)
    pub fn effective_strategy(&self) -> NewlineStrategy {
        if self.strategy == NewlineStrategy::Inherit {
            if let Some(parent) = &self.parent {
                parent.effective_strategy()
            } else {
                NewlineStrategy::Conditional
            }
        } else {
            self.strategy
        }
    }

    /// Validate if a node is allowed in this context
    pub fn validate_node(&self, node: &Node) -> WriteResult<()> {
        if !self.allows_blocks && node.is_block() {
            return Err(WriteError::InvalidStructure(
                format!(
                    "Block-level node {:?} not allowed in {:?} context",
                    node.type_name(),
                    self.mode
                )
                .into(),
            ));
        }
        Ok(())
    }
}

impl Default for NewlineContext {
    fn default() -> Self {
        Self::block()
    }
}

/// Builder for creating flexible newline contexts
pub struct NewlineContextBuilder {
    context: NewlineContext,
}

impl NewlineContextBuilder {
    /// Start building a new context
    pub fn new() -> Self {
        Self {
            context: NewlineContext::default(),
        }
    }

    /// Set the rendering mode
    pub fn mode(mut self, mode: RenderingMode) -> Self {
        self.context.mode = mode;
        self
    }

    /// Set the newline strategy
    pub fn strategy(mut self, strategy: NewlineStrategy) -> Self {
        self.context.strategy = strategy;
        self
    }

    /// Set whether blocks are allowed
    pub fn allow_blocks(mut self, allow: bool) -> Self {
        self.context.allows_blocks = allow;
        self
    }

    /// Set container end flag
    pub fn container_end(mut self, is_end: bool) -> Self {
        self.context.is_container_end = is_end;
        self
    }

    /// Set parent context
    pub fn parent(mut self, parent: NewlineContext) -> Self {
        self.context.parent = Some(Box::new(parent));
        self
    }

    /// Set custom data
    pub fn custom_data(mut self, data: String) -> Self {
        self.context.custom_data = Some(data);
        self
    }

    /// Build the context
    pub fn build(self) -> NewlineContext {
        self.context
    }
}

impl Default for NewlineContextBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_context() {
        let ctx = NewlineContext::block();
        assert_eq!(ctx.mode, RenderingMode::Block);
        assert_eq!(ctx.strategy, NewlineStrategy::Always);
        assert!(ctx.allows_blocks);
    }

    #[test]
    fn test_inline_with_blocks_context() {
        let ctx = NewlineContext::inline_with_blocks();
        assert_eq!(ctx.mode, RenderingMode::InlineWithBlocks);
        assert_eq!(ctx.strategy, NewlineStrategy::Smart);
        assert!(ctx.allows_blocks);
    }

    #[test]
    fn test_smart_newline_decision() {
        let ctx = NewlineContext::inline_with_blocks();

        // Content without trailing newline should NOT get one unless at container end
        assert!(!ctx.should_add_trailing_newline("content", None));

        // Content at container end should get newline if missing
        let ctx_at_end = ctx.with_container_end(true);
        assert!(ctx_at_end.should_add_trailing_newline("content", None));

        // Content with trailing newline shouldn't get another unless at container end
        assert!(!ctx_at_end.should_add_trailing_newline("content\n", None));
    }

    #[test]
    fn test_builder_pattern() {
        let ctx = NewlineContextBuilder::new()
            .mode(RenderingMode::Custom)
            .strategy(NewlineStrategy::Smart)
            .allow_blocks(false)
            .container_end(true)
            .build();

        assert_eq!(ctx.mode, RenderingMode::Custom);
        assert_eq!(ctx.strategy, NewlineStrategy::Smart);
        assert!(!ctx.allows_blocks);
        assert!(ctx.is_container_end);
    }
}
