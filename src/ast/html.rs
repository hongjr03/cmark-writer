//! HTML element definitions for the CommonMark AST.

use super::node::Node;

/// Represents an HTML attribute, containing name and value
#[derive(Debug, Clone, PartialEq)]
pub struct HtmlAttribute {
    /// Attribute name
    pub name: String,
    /// Attribute value
    pub value: String,
}

/// Represents an HTML element, containing tag name, attributes, and child nodes
#[derive(Debug, Clone, PartialEq)]
pub struct HtmlElement {
    /// Element tag name
    pub tag: String,
    /// Element attributes
    pub attributes: Vec<HtmlAttribute>,
    /// Element child nodes (can only contain inline nodes)
    pub children: Vec<Node>,
    /// Whether it's a self-closing tag (e.g. <img />)
    pub self_closing: bool,
}
