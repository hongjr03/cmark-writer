# Core Concepts

This section covers the fundamental concepts of cmark-writer that you need to understand to use the library effectively.

## Key Components

cmark-writer is built around three primary components:

1. **AST Nodes**: These represent the different elements of a Markdown document, from basic text to complex structures like lists and tables.

2. **Writer**: The `CommonMarkWriter` takes AST nodes and serializes them to CommonMark-compliant text.

3. **Options**: Control the formatting behavior of the writer, allowing you to customize the output.

## Understanding the Workflow

The typical workflow when using cmark-writer is:

1. Build a document structure using the `Node` enum
2. Configure a `CommonMarkWriter` with desired options
3. Pass the document to the writer to generate CommonMark text

```rust
// 1. Build document structure
let document = Node::Document(vec![
    Node::heading(1, vec![Node::Text("Title".to_string())]),
    Node::Paragraph(vec![Node::Text("Content".to_string())]),
]);

// 2. Configure writer (with default options in this case)
let mut writer = CommonMarkWriter::new();

// 3. Generate CommonMark text
writer.write(&document).expect("Failed to write document");
let markdown = writer.into_string();
```

## Further Reading

Explore the following sections to learn more about each core component:

- [AST Nodes](./ast-nodes.md): Learn about the different node types and how to build document structures
- [Writer Interface](./writer-interface.md): Understand how the writer works and its capabilities
- [Formatting Options](./options.md): Discover how to customize the output
