# cmark-writer

[![CI Status](https://github.com/hongjr03/cmark-writer/workflows/CI/badge.svg)](https://github.com/hongjr03/cmark-writer/actions)
[![Crates.io](https://img.shields.io/crates/v/cmark-writer.svg)](https://crates.io/crates/cmark-writer)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/cmark-writer.svg)](https://crates.io/crates/cmark-writer)

A CommonMark writer implementation in Rust.

## Usage

### Basic Example

```rust
use cmark_writer::ast::{Node, ListItem};
use cmark_writer::writer::CommonMarkWriter;

// Create a document
let document = Node::Document(vec![
    Node::heading(1, vec![Node::Text("Hello CommonMark".to_string())]),
    Node::Paragraph(vec![
        Node::Text("This is a simple ".to_string()),
        Node::Strong(vec![Node::Text("example".to_string())]),
        Node::Text(".".to_string()),
    ]),
]);

// Render to CommonMark
let mut writer = CommonMarkWriter::new();
writer.write(&document).expect("Failed to write document");
let markdown = writer.into_string();

println!("{}", markdown);
```

### Custom Formatting Options

You can customize the formatting behavior:

```rust
use cmark_writer::options::WriterOptions;
use cmark_writer::writer::CommonMarkWriter;
use cmark_writer::ast::Node;

// Create custom options
let options = WriterOptions {
    strict: true,                // Follow CommonMark spec strictly
    hard_break_spaces: false,    // Use backslash for line breaks
    indent_spaces: 2,            // Use 2 spaces for indentation
};

// Create writer with custom options
let mut writer = CommonMarkWriter::with_options(options);
writer.write(&Node::Text("Example".to_string())).unwrap();
```

## API Documentation

### Core Types

- `Node` - Represents various CommonMark node types
- `ListItem` - Represents list items, including task list items
- `CommonMarkWriter` - Converts nodes to CommonMark text
- `WriterOptions` - Customization options for the writer

### Creating Custom Nodes

You can extend the CommonMark syntax with your own custom nodes:

```rust
use cmark_writer::ast::{CustomNodeWriter, Node};
use cmark_writer::error::WriteResult;
use cmark_writer::custom_node;

// Define a custom highlight node
#[derive(Debug, Clone, PartialEq)]
#[custom_node]
struct HighlightNode {
    content: String,
    color: String,
}

// Implement the required methods
impl HighlightNode {
    // Custom node writing logic
    fn write_custom(&self, writer: &mut dyn CustomNodeWriter) -> WriteResult<()> {
        writer.write_str("<span style=\"background-color: ")?;
        writer.write_str(&self.color)?;
        writer.write_str("\">")?;
        writer.write_str(&self.content)?;
        writer.write_str("</span>")?;
        Ok(())
    }
    
    // Determine if it's a block or inline element
    fn is_block_custom(&self) -> bool {
        false // This is an inline element
    }
}

// Use your custom node
let document = Node::Document(vec![
    Node::Paragraph(vec![
        Node::Text("Here's some text with a ".to_string()),
        Node::Custom(Box::new(HighlightNode {
            content: "highlighted section".to_string(),
            color: "yellow".to_string(),
        })),
        Node::Text(".".to_string()),
    ]),
]);
```

### Creating Custom Errors

You can also define custom error types for your validation logic:

```rust
use cmark_writer::custom_error;
use cmark_writer::coded_error;
use cmark_writer::WriteError;

// Define a structure error with format string
#[custom_error(format = "Table structure error: {}")]
struct TableStructureError(pub &'static str);

// Define a coded error with error code
#[coded_error]
struct ValidationError(pub String, pub String);

// Use in your code
fn validate_table() -> Result<(), WriteError> {
    // Using a structure error
    return Err(TableStructureError("Rows don't match").into());
    
    // Using a coded error
    // return Err(ValidationError("Invalid alignment".to_string(), "INVALID_ALIGNMENT".to_string()).into());
}
```

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
