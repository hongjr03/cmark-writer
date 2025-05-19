# cmark-writer

[![CI Status](https://github.com/hongjr03/cmark-writer/workflows/CI/badge.svg)](https://github.com/hongjr03/cmark-writer/actions)
[![Crates.io](https://img.shields.io/crates/v/cmark-writer.svg)](https://crates.io/crates/cmark-writer)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/cmark-writer.svg)](https://crates.io/crates/cmark-writer)
[![Codecov](https://codecov.io/gh/hongjr03/cmark-writer/branch/master/graph/badge.svg)](https://codecov.io/gh/hongjr03/cmark-writer)

A CommonMark writer implementation in Rust.

## Basic Usage

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

## Custom Options

```rust
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;

// Use builder pattern for custom options
let options = WriterOptionsBuilder::new()
    .strict(true)
    .hard_break_spaces(false)
    .indent_spaces(2)
    .build();

let mut writer = CommonMarkWriter::with_options(options);
```

## Table Support

```rust
use cmark_writer::ast::{Node, tables::TableBuilder};

// Create tables with the builder pattern
let table = TableBuilder::new()
    .headers(vec![
        Node::Text("Name".to_string()), 
        Node::Text("Age".to_string())
    ])
    .add_row(vec![
        Node::Text("John".to_string()),
        Node::Text("30".to_string()),
    ])
    .add_row(vec![
        Node::Text("Alice".to_string()),
        Node::Text("25".to_string()),
    ])
    .build();
```

## GitHub Flavored Markdown (GFM)

Enable GFM features by adding to your `Cargo.toml`:

```toml
[dependencies]
cmark-writer = { version = "0.7.0", features = ["gfm"] }
```

GFM Support:

- Tables with column alignment
- Strikethrough text
- Task lists
- Extended autolinks
- HTML element filtering

## Custom Nodes

```rust
use cmark_writer::ast::{CustomNodeWriter, Node};
use cmark_writer::error::WriteResult;
use cmark_writer::custom_node;

#[derive(Debug, Clone, PartialEq)]
#[custom_node]
struct HighlightNode {
    content: String,
    color: String,
}

impl HighlightNode {
    fn write_custom(&self, writer: &mut dyn CustomNodeWriter) -> WriteResult<()> {
        writer.write_str("<span style=\"background-color: ")?;
        writer.write_str(&self.color)?;
        writer.write_str("\">")?;
        writer.write_str(&self.content)?;
        writer.write_str("</span>")?;
        Ok(())
    }
    
    fn is_block_custom(&self) -> bool {
        false
    }
}
```

## Development

```bash
# Build
cargo build

# Run tests
cargo test
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Feel free to submit a Pull Request.
