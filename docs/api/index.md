# API Reference

This section provides API reference documentation for the cmark-writer library, detailing the main types, structs, and methods.

## Core Components

The cmark-writer API consists of the following core components:

### Node

The `Node` enum represents the basic building blocks that make up a Markdown document. It represents various types of CommonMark elements such as paragraphs, headings, lists, and more.

[View Node documentation](./node)

### CommonMarkWriter

`CommonMarkWriter` is the main component responsible for serializing AST nodes into CommonMark text. It provides the core functionality for generating Markdown output.

[View CommonMarkWriter documentation](./writer)

### WriterOptions

The `WriterOptions` struct controls the formatting behavior of Markdown output. Through these options, you can customize various aspects of the output.

[View WriterOptions documentation](./options)

## Usage Patterns

Common patterns for using the cmark-writer API:

1. **Build Document Structure**: Create an AST using the `Node` enum and its variants
2. **Configure Writer**: Set formatting preferences using `WriterOptions`
3. **Generate Output**: Convert the AST to Markdown text using `CommonMarkWriter`

```rust
use cmark_writer::ast::Node;
use cmark_writer::writer::CommonMarkWriter;
use cmark_writer::options::WriterOptionsBuilder;

// 1. Build document
let document = Node::Document(vec![
    Node::heading(1, vec![Node::Text("Title".to_string())]),
    Node::Paragraph(vec![Node::Text("Content".to_string())]),
]);

// 2. Configure writer
let options = WriterOptionsBuilder::new()
    .list_marker('*')
    .build();
let mut writer = CommonMarkWriter::with_options(options);

// 3. Generate output
writer.write(&document).expect("Failed to write");
let markdown = writer.into_string();
```

For detailed documentation on each component, refer to the corresponding sub-pages.
