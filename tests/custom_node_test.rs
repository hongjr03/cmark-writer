use cmark_writer::ast::{CustomNode, CustomNodeWriter, Node};
use cmark_writer::writer::CommonMarkWriter;
use cmark_writer::WriteResult;
use std::any::Any;

// A simple custom node example: representing highlighted text
#[derive(Debug, PartialEq)]
struct HighlightNode {
    content: String,
    color: String,
}

// Implement the CustomNode trait
impl CustomNode for HighlightNode {
    fn write(&self, writer: &mut dyn CustomNodeWriter) -> cmark_writer::error::WriteResult<()> {
        // Implement custom writing logic
        writer.write_str("<span style=\"background-color: ")?;
        writer.write_str(&self.color)?;
        writer.write_str("\">")?;
        writer.write_str(&self.content)?;
        writer.write_str("</span>")?;
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn CustomNode> {
        Box::new(Self {
            content: self.content.clone(),
            color: self.color.clone(),
        })
    }

    fn eq_box(&self, other: &dyn CustomNode) -> bool {
        // Try to downcast other to HighlightNode
        if let Some(other) = other.as_any().downcast_ref::<HighlightNode>() {
            self == other
        } else {
            false
        }
    }

    fn is_block(&self) -> bool {
        false // This is an inline element
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// Example of a custom block-level node implementation
#[derive(Debug, PartialEq)]
struct CalloutNode {
    title: String,
    content: String,
    style: String, // e.g.: note, warning, danger
}

impl CustomNode for CalloutNode {
    fn write(&self, writer: &mut dyn CustomNodeWriter) -> cmark_writer::error::WriteResult<()> {
        writer.write_str("<div class=\"callout callout-")?;
        writer.write_str(&self.style)?;
        writer.write_str("\">\n")?;

        writer.write_str("  <h4>")?;
        writer.write_str(&self.title)?;
        writer.write_str("</h4>\n")?;

        writer.write_str("  <p>")?;
        writer.write_str(&self.content)?;
        writer.write_str("</p>\n")?;

        writer.write_str("</div>")?;
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn CustomNode> {
        Box::new(Self {
            title: self.title.clone(),
            content: self.content.clone(),
            style: self.style.clone(),
        })
    }

    fn eq_box(&self, other: &dyn CustomNode) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<CalloutNode>() {
            self == other
        } else {
            false
        }
    }

    fn is_block(&self) -> bool {
        true // This is a block-level element
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[test]
fn test_highlight_node() {
    let mut writer = CommonMarkWriter::new();
    let highlight = Node::Custom(Box::new(HighlightNode {
        content: "Highlighted text".to_string(),
        color: "yellow".to_string(),
    }));

    writer.write(&highlight).unwrap();
    assert_eq!(
        writer.into_string(),
        "<span style=\"background-color: yellow\">Highlighted text</span>"
    );
}

#[test]
fn test_callout_block() {
    let mut writer = CommonMarkWriter::new();
    let callout = Node::Custom(Box::new(CalloutNode {
        title: "Important note".to_string(),
        content: "This is an important message.".to_string(),
        style: "warning".to_string(),
    }));

    writer.write(&callout).unwrap();
    let expected = "<div class=\"callout callout-warning\">\n  <h4>Important note</h4>\n  <p>This is an important message.</p>\n</div>";
    assert_eq!(writer.into_string(), expected);
}

#[test]
fn test_custom_node_in_paragraph() {
    let mut writer = CommonMarkWriter::new();
    let paragraph = Node::Paragraph(vec![
        Node::Text("This is regular text with ".to_string()),
        Node::Custom(Box::new(HighlightNode {
            content: "highlighted text".to_string(),
            color: "yellow".to_string(),
        })),
        Node::Text(" mixed together.".to_string()),
    ]);

    writer.write(&paragraph).unwrap();
    assert_eq!(
        writer.into_string(),
        "This is regular text with <span style=\"background-color: yellow\">highlighted text</span> mixed together."
    );
}

#[test]
fn test_custom_block_in_document() {
    let mut writer = CommonMarkWriter::new();
    let document = Node::Document(vec![
        Node::Heading {
            level: 1,
            content: vec![Node::Text("Document Title".to_string())],
        },
        Node::Paragraph(vec![Node::Text("This is a paragraph.".to_string())]),
        Node::Custom(Box::new(CalloutNode {
            title: "Important Information".to_string(),
            content: "Please pay attention to this content.".to_string(),
            style: "info".to_string(),
        })),
        Node::Paragraph(vec![Node::Text("Another paragraph.".to_string())]),
    ]);

    writer.write(&document).unwrap();
    let expected = "# Document Title\n\nThis is a paragraph.\n\n<div class=\"callout callout-info\">\n  <h4>Important Information</h4>\n  <p>Please pay attention to this content.</p>\n</div>\n\nAnother paragraph.";
    assert_eq!(writer.into_string(), expected);
}

/// A Figure custom node that can contain any block node as its body
/// and has a caption. This allows for advanced document structures like
/// figures with numbered captions, images with descriptions, etc.
#[derive(Debug, PartialEq, Clone)]
struct FigureNode {
    /// The main content of the figure, can be any block node
    body: Box<Node>,
    /// The caption text for the figure
    caption: String,
    /// Optional ID for referencing
    id: Option<String>,
}

impl CustomNode for FigureNode {
    fn write(&self, writer: &mut dyn CustomNodeWriter) -> WriteResult<()> {
        // Start the figure element with optional ID
        writer.write_str("<figure")?;
        if let Some(id) = &self.id {
            writer.write_str(" id=\"")?;
            writer.write_str(id)?;
            writer.write_str("\"")?;
        }
        writer.write_str(">\n")?;

        // Create a temporary CommonMarkWriter to render the body node
        let mut body_writer = CommonMarkWriter::new();
        // We need to downcast to access the write method
        let body_writer_ptr: &mut dyn CustomNodeWriter = &mut body_writer;

        // Render the body content using its native renderer
        // This allows any block node to be properly rendered inside the figure
        match &*self.body {
            Node::Paragraph(content) => {
                for node in content {
                    // Write paragraph content directly without wrapping in <p> tags
                    // since we're already in a figure element
                    self.write_node(node, body_writer_ptr)?;
                }
            }
            node => {
                // For any other block node, use its own rendering logic
                self.write_node(node, body_writer_ptr)?;
            }
        }

        // Get the rendered body content
        let body_content = body_writer.into_string();

        // Write the body content to the main writer
        writer.write_str(&body_content)?;
        writer.write_str("\n")?;

        // Add the caption
        writer.write_str("  <figcaption>")?;
        writer.write_str(&self.caption)?;
        writer.write_str("</figcaption>\n")?;

        // Close the figure element
        writer.write_str("</figure>")?;

        Ok(())
    }

    fn clone_box(&self) -> Box<dyn CustomNode> {
        Box::new(Self {
            body: self.body.clone(),
            caption: self.caption.clone(),
            id: self.id.clone(),
        })
    }

    fn eq_box(&self, other: &dyn CustomNode) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<FigureNode>() {
            self == other
        } else {
            false
        }
    }

    fn is_block(&self) -> bool {
        true // Figure is always a block-level element
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl FigureNode {
    // Helper method to write a node to the provided writer
    fn write_node(&self, node: &Node, writer: &mut dyn CustomNodeWriter) -> WriteResult<()> {
        // We need to use a temporary CommonMarkWriter to render the node
        let mut temp_writer = CommonMarkWriter::new();
        temp_writer.write(node)?;
        let content = temp_writer.into_string();
        writer.write_str(&content)?;
        Ok(())
    }
}

#[test]
fn test_figure_with_image() {
    let mut writer = CommonMarkWriter::new();

    // Create a figure containing an image
    let figure = Node::Custom(Box::new(FigureNode {
        body: Box::new(Node::Paragraph(vec![Node::Image {
            url: "sample.jpg".to_string(),
            title: Some("Sample image".to_string()),
            alt: vec![Node::Text("A sample image".to_string())],
        }])),
        caption: "Figure 1: Sample illustration".to_string(),
        id: Some("fig1".to_string()),
    }));

    writer.write(&figure).unwrap();

    let expected = "<figure id=\"fig1\">\n![A sample image](sample.jpg \"Sample image\")\n  <figcaption>Figure 1: Sample illustration</figcaption>\n</figure>";
    assert_eq!(writer.into_string(), expected);
}

#[test]
fn test_figure_with_code_block() {
    let mut writer = CommonMarkWriter::new();

    // Create a figure containing a code block
    let figure = Node::Custom(Box::new(FigureNode {
        body: Box::new(Node::CodeBlock {
            language: Some("rust".to_string()),
            content: "fn main() {\n    println!(\"Hello, world!\");\n}".to_string(),
        }),
        caption: "Figure 2: Rust Hello World example".to_string(),
        id: None,
    }));

    writer.write(&figure).unwrap();

    let expected = "<figure>\n```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```\n  <figcaption>Figure 2: Rust Hello World example</figcaption>\n</figure>";
    assert_eq!(writer.into_string(), expected);
}

#[test]
fn test_figure_with_table() {
    let mut writer = CommonMarkWriter::new();

    // Create a figure containing a table
    use cmark_writer::ast::Alignment;

    let figure = Node::Custom(Box::new(FigureNode {
        body: Box::new(Node::Table {
            headers: vec![
                Node::Text("Name".to_string()),
                Node::Text("Value".to_string()),
            ],
            rows: vec![
                vec![
                    Node::Text("Item 1".to_string()),
                    Node::Text("100".to_string()),
                ],
                vec![
                    Node::Text("Item 2".to_string()),
                    Node::Text("200".to_string()),
                ],
            ],
            alignments: vec![Alignment::Left, Alignment::Right],
        }),
        caption: "Figure 3: Sample data table".to_string(),
        id: Some("data-table".to_string()),
    }));

    writer.write(&figure).unwrap();

    let expected = "<figure id=\"data-table\">\n| Name | Value |\n| :--- | ---: |\n| Item 1 | 100 |\n| Item 2 | 200 |\n\n  <figcaption>Figure 3: Sample data table</figcaption>\n</figure>";
    assert_eq!(writer.into_string(), expected);
}

#[test]
fn test_figure_in_document() {
    let mut writer = CommonMarkWriter::new();

    // Create a document with a figure inside
    let document = Node::Document(vec![
        Node::Heading {
            level: 1,
            content: vec![Node::Text("Document with Figures".to_string())],
        },
        Node::Paragraph(vec![Node::Text(
            "This document demonstrates using figures.".to_string(),
        )]),
        Node::Custom(Box::new(FigureNode {
            body: Box::new(Node::BlockQuote(vec![Node::Paragraph(vec![Node::Text(
                "This is a quote inside a figure.".to_string(),
            )])])),
            caption: "Figure 1: An important quote".to_string(),
            id: Some("quote-fig".to_string()),
        })),
        Node::Paragraph(vec![Node::Text("Text after the figure.".to_string())]),
    ]);

    writer.write(&document).unwrap();

    let expected = String::from("# Document with Figures\n\n")
        + "This document demonstrates using figures.\n\n"
        + "<figure id=\"quote-fig\">\n"
        + "> This is a quote inside a figure.\n"
        + "  <figcaption>Figure 1: An important quote</figcaption>\n"
        + "</figure>\n\n"
        + "Text after the figure.";

    assert_eq!(writer.into_string(), expected);
}
