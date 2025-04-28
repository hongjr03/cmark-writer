//! GFM HTML handling and filtering
//!
//! This module provides utilities for handling HTML in GitHub Flavored Markdown,
//! including filtering of potentially unsafe HTML tags according to GFM specifications.

use crate::ast::{safe_html, HtmlElement, Node};

/// Default list of HTML tags disallowed in GitHub Flavored Markdown
///
/// These tags are considered unsafe according to GFM specifications:
/// https://github.github.com/gfm/#disallowed-raw-html-extension-
pub fn default_disallowed_tags() -> Vec<String> {
    vec![
        "title".to_string(),
        "textarea".to_string(),
        "style".to_string(),
        "xmp".to_string(),
        "iframe".to_string(),
        "noembed".to_string(),
        "noframes".to_string(),
        "script".to_string(),
        "plaintext".to_string(),
    ]
}

/// Creates a GFM-safe HTML node by filtering disallowed tags
///
/// This is a convenience wrapper around the generic safe_html function
/// that uses the default GFM disallowed tags list.
///
/// # Arguments
/// * `element` - The original HTML element
///
/// # Returns
/// Either the original element or an escaped text representation
pub fn gfm_safe_html(element: HtmlElement) -> Node {
    safe_html(element, &default_disallowed_tags())
}

/// Process a node tree and make all HTML elements GFM-safe
///
/// This function recursively processes all nodes in a tree,
/// making HTML elements safe according to GFM specifications.
///
/// # Arguments
/// * `node` - The node tree to process
///
/// # Returns
/// A new node tree with safe HTML elements
pub fn make_html_gfm_safe(node: &Node) -> Node {
    match node {
        Node::HtmlElement(element) => gfm_safe_html(element.clone()),
        Node::Document(children) => {
            Node::Document(children.iter().map(make_html_gfm_safe).collect())
        }
        Node::Paragraph(children) => {
            Node::Paragraph(children.iter().map(make_html_gfm_safe).collect())
        }
        Node::BlockQuote(children) => {
            Node::BlockQuote(children.iter().map(make_html_gfm_safe).collect())
        }
        Node::Heading {
            level,
            content,
            heading_type,
        } => Node::Heading {
            level: *level,
            content: content.iter().map(make_html_gfm_safe).collect(),
            heading_type: heading_type.clone(),
        },
        Node::Emphasis(children) => {
            Node::Emphasis(children.iter().map(make_html_gfm_safe).collect())
        }
        Node::Strong(children) => Node::Strong(children.iter().map(make_html_gfm_safe).collect()),
        Node::Strikethrough(children) => {
            Node::Strikethrough(children.iter().map(make_html_gfm_safe).collect())
        }
        Node::Link {
            url,
            title,
            content,
        } => Node::Link {
            url: url.clone(),
            title: title.clone(),
            content: content.iter().map(make_html_gfm_safe).collect(),
        },
        Node::Image { url, title, alt } => Node::Image {
            url: url.clone(),
            title: title.clone(),
            alt: alt.iter().map(make_html_gfm_safe).collect(),
        },
        // For other node types that don't contain HTML elements, simply clone them
        _ => node.clone(),
    }
}
