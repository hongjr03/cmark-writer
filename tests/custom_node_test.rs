use cmark_writer::ast::{CustomNodeWriter, Node};
use cmark_writer::writer::CommonMarkWriter;
use cmark_writer::WriteResult;

// A simple custom node example: representing highlighted text
#[derive(Debug, PartialEq, Clone)]
struct HighlightNode {
    content: String,
    color: String,
}

// Using macro to implement CustomNode trait
cmark_writer::derive_custom_node!(HighlightNode);

// Implementing required methods for HighlightNode
impl HighlightNode {
    fn write_custom(
        &self,
        writer: &mut dyn CustomNodeWriter,
    ) -> cmark_writer::error::WriteResult<()> {
        // Implement custom writing logic
        writer.write_str("<span style=\"background-color: ")?;
        writer.write_str(&self.color)?;
        writer.write_str("\">")?;
        writer.write_str(&self.content)?;
        writer.write_str("</span>")?;
        Ok(())
    }

    fn is_block_custom(&self) -> bool {
        false // This is an inline element
    }
}

// Example of a custom block-level node implementation
#[derive(Debug, PartialEq, Clone)]
struct CalloutNode {
    title: String,
    content: String,
    style: String, // e.g.: note, warning, danger
}

// Using macro to implement CustomNode trait
cmark_writer::derive_custom_node!(CalloutNode);

// Implementing required methods for CalloutNode
impl CalloutNode {
    fn write_custom(
        &self,
        writer: &mut dyn CustomNodeWriter,
    ) -> cmark_writer::error::WriteResult<()> {
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

    fn is_block_custom(&self) -> bool {
        true // This is a block-level element
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

// Using macro to implement CustomNode trait
cmark_writer::derive_custom_node!(FigureNode);

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

    fn write_custom(&self, writer: &mut dyn CustomNodeWriter) -> WriteResult<()> {
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

    fn is_block_custom(&self) -> bool {
        true // Figure is always a block-level element
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

#[test]
fn test_derive_custom_node_macro() {
    use cmark_writer::ast::{CustomNodeWriter, Node};
    use cmark_writer::error::WriteResult;
    use cmark_writer::writer::CommonMarkWriter;

    // A simple alert box custom node using the macro
    #[derive(Debug, Clone, PartialEq)]
    struct AlertBox {
        message: String,
        level: String, // info, warning, error
    }

    // Use the macro to implement CustomNode trait
    cmark_writer::derive_custom_node!(AlertBox);

    // Implement the required methods for AlertBox
    impl AlertBox {
        fn write_custom(&self, writer: &mut dyn CustomNodeWriter) -> WriteResult<()> {
            writer.write_str("<div class=\"alert alert-")?;
            writer.write_str(&self.level)?;
            writer.write_str("\">\n")?;
            writer.write_str("  <p>")?;
            writer.write_str(&self.message)?;
            writer.write_str("</p>\n")?;
            writer.write_str("</div>")?;
            Ok(())
        }

        fn is_block_custom(&self) -> bool {
            true // This is a block element
        }
    }

    // Create an instance of our custom node
    let alert = Node::Custom(Box::new(AlertBox {
        message: "This is an important alert message.".to_string(),
        level: "warning".to_string(),
    }));

    // Test rendering the custom node
    let mut writer = CommonMarkWriter::new();
    writer.write(&alert).unwrap();

    let expected =
        "<div class=\"alert alert-warning\">\n  <p>This is an important alert message.</p>\n</div>";
    assert_eq!(writer.into_string(), expected);

    // Test using the custom node in a document
    let document = Node::Document(vec![
        Node::Heading {
            level: 1,
            content: vec![Node::Text("Document with Alert".to_string())],
        },
        Node::Paragraph(vec![Node::Text("Text before alert.".to_string())]),
        Node::Custom(Box::new(AlertBox {
            message: "This is an important alert message.".to_string(),
            level: "warning".to_string(),
        })),
        Node::Paragraph(vec![Node::Text("Text after alert.".to_string())]),
    ]);

    let mut writer = CommonMarkWriter::new();
    writer.write(&document).unwrap();

    let expected = "# Document with Alert\n\nText before alert.\n\n<div class=\"alert alert-warning\">\n  <p>This is an important alert message.</p>\n</div>\n\nText after alert.";
    assert_eq!(writer.into_string(), expected);
}
