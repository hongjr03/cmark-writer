# cmark-rs

[![CI Status](https://github.com/hongjr03/cmark-rs/workflows/CI/badge.svg)](https://github.com/hongjr03/cmark-rs/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A CommonMark writer implementation in Rust.

## Usage

### Basic Example

```rust
use cmark_rs::ast::{Node, ListItem};
use cmark_rs::writer::CommonMarkWriter;

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
use cmark_rs::writer::{CommonMarkWriter, WriterOptions};
use cmark_rs::ast::Node;

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
