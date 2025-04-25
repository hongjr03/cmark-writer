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

fn main() {
    // Create a document
    let document = Node::Document(vec![
        Node::Heading {
            level: 1,
            content: vec![Node::Text("Hello CommonMark".to_string())],
        },
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
}
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
