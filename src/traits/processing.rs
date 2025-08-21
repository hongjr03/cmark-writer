//! Node processing traits

use crate::error::WriteResult;

/// Node processor trait
pub trait NodeProcessor {
    /// Check if the node can be processed
    fn can_process(&self, node: &crate::ast::Node) -> bool;

    /// Process node and write to CommonMark
    fn process_commonmark(
        &self,
        writer: &mut crate::writer::CommonMarkWriter,
        node: &crate::ast::Node,
    ) -> WriteResult<()>;

    /// Process node and write to HTML
    fn process_html(
        &self,
        writer: &mut crate::writer::HtmlWriter,
        node: &crate::ast::Node,
    ) -> WriteResult<()>;

    /// Get processor priority
    fn priority(&self) -> u32 {
        0
    }
}

/// Block-level node processor
pub trait BlockNodeProcessor: NodeProcessor {
    /// Ensure block separation
    fn ensure_block_separation(&self, writer: &mut dyn super::core::Writer) -> WriteResult<()>;
}

/// Inline node processor
pub trait InlineNodeProcessor: NodeProcessor {
    /// Validate inline content
    fn validate_inline_content(&self, node: &crate::ast::Node) -> WriteResult<()>;
}
