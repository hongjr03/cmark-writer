# Writer Interface

The `CommonMarkWriter` is the core component responsible for serializing AST nodes to CommonMark-compliant text. This page explains how to use and interact with the writer.

## Basic Usage

Using the writer follows a simple pattern:

```rust
use cmark_writer::ast::Node;
use cmark_writer::writer::CommonMarkWriter;

// Create a writer with default options
let mut writer = CommonMarkWriter::new();

// Write a node to the writer
writer.write(&Node::Text("Hello world".to_string())).expect("Write failed");

// Get the output as a string
let output = writer.into_string();
```

## Creating a Writer

There are two ways to create a writer:

### With Default Options

```rust
let mut writer = CommonMarkWriter::new();
```

### With Custom Options

```rust
use cmark_writer::options::WriterOptions;

// Create with custom options
let options = WriterOptions {
    strict: true,
    hard_break_spaces: false,
    // ... other options
    ..Default::default()
};

let mut writer = CommonMarkWriter::with_options(options);
```

## Writing Methods

The main method you'll use is `write()`:

```rust
// Write a single node
writer.write(&node).expect("Failed to write");
```

This method recursively processes the node and its children, handling all the formatting according to the CommonMark specification.

## Getting the Output

After writing all nodes, retrieve the output:

```rust
// Get the final markdown output
let markdown = writer.into_string();
```

This consumes the writer, returning the formatted Markdown string.

## Error Handling

Writing operations return a `WriteResult<()>`, which is a type alias for `Result<(), WriteError>`. This allows you to handle formatting errors gracefully:

```rust
match writer.write(&node) {
    Ok(_) => {
        // Writing succeeded
        let output = writer.into_string();
        println!("Generated Markdown: {}", output);
    },
    Err(err) => {
        // Handle the error
        eprintln!("Failed to write node: {}", err);
    }
}
```

Common error types include:

- `WriteError::NewlineInInlineElement`: When inline elements contain newlines
- `WriteError::InvalidNesting`: When nodes are nested incorrectly
- `WriteError::UnsupportedNodeType`: When trying to write an unsupported node type

## Example: Building a Document

A complete example of building and writing a document:

```rust
use cmark_writer::ast::{Node, ListItem};
use cmark_writer::writer::CommonMarkWriter;

// Create a document structure
let document = Node::Document(vec![
    Node::heading(1, vec![Node::Text("Document Title".to_string())]),
    Node::Paragraph(vec![
        Node::Text("This is the introduction.".to_string())
    ]),
    Node::UnorderedList(vec![
        ListItem::Unordered { 
            content: vec![Node::Paragraph(vec![Node::Text("First point".to_string())])] 
        },
        ListItem::Unordered { 
            content: vec![Node::Paragraph(vec![Node::Text("Second point".to_string())])] 
        },
    ]),
]);

// Create a writer and generate the markdown
let mut writer = CommonMarkWriter::new();
writer.write(&document).expect("Failed to write document");
let markdown = writer.into_string();

println!("{}", markdown);
```

This would generate:

```markdown
# Document Title

This is the introduction.

- First point
- Second point
```
