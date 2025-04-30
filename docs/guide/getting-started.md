# Getting Started

This guide will help you get started with cmark-writer quickly. We'll cover installation, basic usage, and a simple example.

## Installation

Add cmark-writer to your `Cargo.toml`:

```toml
[dependencies]
cmark-writer = "0.6.2"
```

If you need GitHub Flavored Markdown support, enable the `gfm` feature:

```toml
[dependencies]
cmark-writer = { version = "0.6.2", features = ["gfm"] }
```

## Basic Example

Here's a simple example that creates a Markdown document with a heading and a paragraph:

```rust
use cmark_writer::ast::Node;
use cmark_writer::writer::CommonMarkWriter;

fn main() {
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
}
```

This will produce:

```markdown
# Hello CommonMark

This is a simple **example**.
```

## Core Components

The library consists of these main components:

1. **AST Nodes** (`Node` enum): Represents different elements of a Markdown document
2. **Writer** (`CommonMarkWriter`): Serializes nodes to CommonMark text
3. **Options** (`WriterOptions`): Controls formatting behavior

## Next Steps

To learn more about cmark-writer:

- Explore [Core Concepts](./core-concepts/index.md) to understand the fundamentals
- Try the [Examples](./examples/index.md) to see more complex use cases
- Check the [API Reference](../api/index.md) for detailed documentation
