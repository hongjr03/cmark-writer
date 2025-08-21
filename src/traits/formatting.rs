//! Formatting and rendering traits
//!
//! This module provides a unified system for rendering content to different formats.
//! It combines the high-level format abstraction with concrete rendering implementations.

use crate::error::WriteResult;
use crate::writer::{CommonMarkWriter, HtmlWriter};

// ==== Core Rendering Traits ====

/// CommonMark rendering trait - using concrete types for dyn compatibility
pub trait CommonMarkRenderable: super::core::NodeContent {
    /// Render to CommonMark format
    fn render_commonmark(&self, writer: &mut CommonMarkWriter) -> WriteResult<()>;
}

/// HTML rendering trait - using concrete types for dyn compatibility
pub trait HtmlRenderable: super::core::NodeContent {
    /// Render to HTML format
    fn render_html(&self, writer: &mut HtmlWriter) -> WriteResult<()>;
}

// ==== High-Level Format Traits ====

/// Generic format trait - supports multiple output formats
pub trait Format<W> {
    /// Format self to the specified writer
    fn format(&self, writer: &mut W) -> WriteResult<()>;
}

/// Convenience trait for CommonMark format
pub trait ToCommonMark {
    /// Format to CommonMark
    fn to_commonmark(&self, writer: &mut CommonMarkWriter) -> WriteResult<()>;
}

/// Convenience trait for HTML format
pub trait ToHtml {
    /// Format to HTML
    fn to_html(&self, writer: &mut HtmlWriter) -> WriteResult<()>;
}

// ==== Automatic Implementations ====

/// Automatically implement ToCommonMark for types that implement `Format<CommonMarkWriter>`
impl<T> ToCommonMark for T
where
    T: Format<CommonMarkWriter>,
{
    fn to_commonmark(&self, writer: &mut CommonMarkWriter) -> WriteResult<()> {
        self.format(writer)
    }
}

/// Automatically implement ToHtml for types that implement `Format<HtmlWriter>`
impl<T> ToHtml for T
where
    T: Format<HtmlWriter>,
{
    fn to_html(&self, writer: &mut HtmlWriter) -> WriteResult<()> {
        self.format(writer)
    }
}

/// Bridge implementation: Format -> Renderable
impl<T> CommonMarkRenderable for T
where
    T: ToCommonMark + super::core::NodeContent,
{
    fn render_commonmark(&self, writer: &mut CommonMarkWriter) -> WriteResult<()> {
        self.to_commonmark(writer)
    }
}

impl<T> HtmlRenderable for T
where
    T: ToHtml + super::core::NodeContent,
{
    fn render_html(&self, writer: &mut HtmlWriter) -> WriteResult<()> {
        self.to_html(writer)
    }
}

// ==== Multi-Format Support ====

/// Multi-format node trait - unified interface for rendering to multiple formats
///
/// This trait provides a unified multi-format rendering interface for custom nodes.
/// All custom nodes should support at least CommonMark format.
pub trait MultiFormat: ToCommonMark {
    /// Check if HTML format is supported
    ///
    /// Returns false by default. Only types supporting HTML need to override this.
    fn supports_html(&self) -> bool {
        false
    }

    /// HTML rendering implementation
    ///
    /// By default, generates a comment indicating HTML is not supported.
    /// Types that support HTML should override this method.
    fn html_format(&self, writer: &mut HtmlWriter) -> WriteResult<()> {
        writer
            .raw_html(&format!(
                "<!-- HTML rendering not implemented for {} -->",
                std::any::type_name::<Self>()
            ))
            .map_err(Into::into)
    }

    /// Render to the appropriate format based on writer type
    fn render_multi<W>(&self, writer: &mut W) -> WriteResult<()>
    where
        W: std::any::Any + 'static,
    {
        if let Some(cm_writer) =
            (writer as &mut dyn std::any::Any).downcast_mut::<CommonMarkWriter>()
        {
            self.to_commonmark(cm_writer)
        } else if let Some(html_writer) =
            (writer as &mut dyn std::any::Any).downcast_mut::<HtmlWriter>()
        {
            if self.supports_html() {
                self.html_format(html_writer)
            } else {
                Err(crate::error::WriteError::custom(
                    "HTML format not supported for this node type",
                ))
            }
        } else {
            Err(crate::error::WriteError::custom("Unsupported writer type"))
        }
    }
}

/// Automatically implement MultiFormat for types that implement both ToCommonMark and ToHtml
impl<T> MultiFormat for T
where
    T: ToCommonMark + ToHtml,
{
    fn supports_html(&self) -> bool {
        true
    }

    fn html_format(&self, writer: &mut HtmlWriter) -> WriteResult<()> {
        self.to_html(writer)
    }
}
