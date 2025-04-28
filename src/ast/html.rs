//! HTML element definitions and utilities for CommonMark AST.
//!
//! This module contains definitions for HTML elements and attributes in the AST,
//! along with utilities for safely handling HTML content.

use super::Node;

/// HTML attribute
#[derive(Debug, Clone, PartialEq)]
pub struct HtmlAttribute {
    /// Attribute name
    pub name: String,
    /// Attribute value
    pub value: String,
}

/// HTML element
#[derive(Debug, Clone, PartialEq)]
pub struct HtmlElement {
    /// HTML tag name
    pub tag: String,
    /// HTML attributes
    pub attributes: Vec<HtmlAttribute>,
    /// Child nodes
    pub children: Vec<Node>,
    /// Whether this is a self-closing element
    pub self_closing: bool,
}

impl HtmlElement {
    /// Create a new HTML element
    pub fn new(tag: &str) -> Self {
        Self {
            tag: tag.to_string(),
            attributes: Vec::new(),
            children: Vec::new(),
            self_closing: false,
        }
    }

    /// Add an attribute to the HTML element
    pub fn with_attribute(mut self, name: &str, value: &str) -> Self {
        self.attributes.push(HtmlAttribute {
            name: name.to_string(),
            value: value.to_string(),
        });
        self
    }

    /// Add multiple attributes to the HTML element
    pub fn with_attributes(mut self, attrs: Vec<(&str, &str)>) -> Self {
        for (name, value) in attrs {
            self.attributes.push(HtmlAttribute {
                name: name.to_string(),
                value: value.to_string(),
            });
        }
        self
    }

    /// Add child nodes to the HTML element
    pub fn with_children(mut self, children: Vec<Node>) -> Self {
        self.children = children;
        self
    }

    /// Set whether the element is self-closing
    pub fn self_closing(mut self, is_self_closing: bool) -> Self {
        self.self_closing = is_self_closing;
        self
    }

    /// Check if this element's tag matches any in the provided list (case-insensitive)
    pub fn tag_matches_any(&self, tags: &[String]) -> bool {
        tags.iter().any(|tag| tag.eq_ignore_ascii_case(&self.tag))
    }
}

/// Safely escape HTML content
///
/// This function escapes the special HTML characters in a string
/// to ensure it is safe for inclusion in HTML content.
///
/// # Arguments
/// * `content` - The raw content to escape
///
/// # Returns
/// The escaped HTML content
pub fn escape_html(content: &str) -> String {
    content
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Creates a safe HTML node by filtering potentially unsafe elements
///
/// Processes an HTML element according to the provided filter rules.
/// If the element is considered unsafe (matches a disallowed tag),
/// it will be converted to escaped text.
///
/// # Arguments
/// * `element` - The original HTML element
/// * `disallowed_tags` - List of disallowed HTML tag names
///
/// # Returns
/// Either the original element as a Node::HtmlElement or
/// an escaped text representation as Node::Text
pub fn safe_html(element: HtmlElement, disallowed_tags: &[String]) -> Node {
    if element.tag_matches_any(disallowed_tags) {
        // Convert to escaped text
        let mut html_text = String::new();
        html_text.push_str(&format!("&lt;{}", element.tag));

        for attr in &element.attributes {
            html_text.push_str(&format!(" {}=\"{}\"", attr.name, attr.value));
        }

        if element.self_closing {
            html_text.push_str(" /&gt;");
        } else {
            html_text.push_str("&gt;");

            // Add children content (simplified approach)
            for child in &element.children {
                html_text.push_str(&format!("{}", child));
            }

            html_text.push_str(&format!("&lt;/{}&gt;", element.tag));
        }

        Node::Text(html_text)
    } else {
        Node::HtmlElement(element)
    }
}
