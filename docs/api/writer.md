# CommonMarkWriter

The `CommonMarkWriter` is responsible for serializing AST nodes to CommonMark-compliant text. It's the main component that transforms your abstract syntax tree into readable Markdown.

## Creating a Writer

There are several ways to create a `CommonMarkWriter`:

```rust
use cmark_writer::writer::CommonMarkWriter;
use cmark_writer::options::WriterOptions;

// Default writer with standard options
let writer = CommonMarkWriter::new();

// Writer with custom options
let options = WriterOptions::builder()
    .soft_break("\n")
    .hard_break("  \n")
    .build();
let writer = CommonMarkWriter::with_options(options);
```

## Basic Usage

The basic workflow for using a writer involves:

1. Creating a writer instance
2. Writing nodes to it
3. Extracting the resulting Markdown string

```rust
use cmark_writer::{Node, CommonMarkWriter};

// Create document
let document = Node::Document(vec![
    Node::heading(1, vec![Node::Text("Hello".to_string())]),
    Node::Paragraph(vec![Node::Text("World".to_string())])
]);

// Write document to CommonMarkWriter
let mut writer = CommonMarkWriter::new();
writer.write(&document).expect("Failed to write document");

// Get resulting Markdown as a string
let markdown = writer.into_string();
println!("{}", markdown);
// Output:
// # Hello
// 
// World
```

## Writing Methods

### Writing Nodes

```rust
use cmark_writer::{Node, CommonMarkWriter};

let mut writer = CommonMarkWriter::new();

// Write a single node
let paragraph = Node::Paragraph(vec![Node::Text("Hello world".to_string())]);
writer.write(&paragraph).expect("Failed to write paragraph");

// Get the result
let markdown = writer.into_string();  // "Hello world\n"
```

### Writing Raw Text

In addition to nodes, you can write raw text directly:

```rust
use cmark_writer::CommonMarkWriter;

let mut writer = CommonMarkWriter::new();
writer.write_str("Hello ").unwrap();
writer.write_str("world!").unwrap();

let result = writer.into_string();  // "Hello world!"
```

## Customizing Output

The writer's behavior is controlled by `WriterOptions`. You can customize various aspects of the output format:

```rust
use cmark_writer::{CommonMarkWriter, options::WriterOptionsBuilder};

// Create custom options
let options = WriterOptionsBuilder::new()
    .list_marker('*')  // Use * for unordered lists
    .code_fence_char('~')  // Use ~~~ for code fences
    .build();

let writer = CommonMarkWriter::with_options(options);
```

For details on available options, see the [WriterOptions documentation](./options).

## Error Handling

Writing operations return a `WriteResult<T>` which is a type alias for `Result<T, WriteError>`. This allows for proper error handling:

```rust
use cmark_writer::{Node, CommonMarkWriter};

let node = Node::Document(vec![/* ... */]);
let mut writer = CommonMarkWriter::new();

match writer.write(&node) {
    Ok(_) => {
        let markdown = writer.into_string();
        // Use the generated Markdown...
    },
    Err(error) => {
        eprintln!("Failed to write document: {}", error);
        // Handle the error...
    }
}
```

## Working with Custom Nodes

The writer implements the `CustomNodeWriter` trait, which allows custom nodes to write their content:

```rust
use cmark_writer::{custom_node, ast::{CustomNode, CustomNodeWriter}};

#[custom_node]
#[derive(Debug, Clone, PartialEq)]
struct ColoredText {
    text: String,
    color: String,
}

impl CustomNode for ColoredText {
    fn write(&self, writer: &mut dyn CustomNodeWriter) -> WriteResult<()> {
        writer.write_str("<span style=\"color:")?;
        writer.write_str(&self.color)?;
        writer.write_str("\">")?;
        writer.write_str(&self.text)?;
        writer.write_str("</span>")?;
        Ok(())
    }
    
    // Other required trait methods...
}
```

## Display Implementation

The `Node` struct implements the `Display` trait using `CommonMarkWriter` internally, allowing for easy conversion to strings:

```rust
use cmark_writer::Node;
use std::fmt;

let node = Node::heading(1, vec![Node::Text("Title".to_string())]);

// Use Display implementation 
let markdown = format!("{}", node);  // "# Title\n"
```

## Thread Safety

`CommonMarkWriter` is not thread-safe by default. If you need to share a writer between threads, you'll need to implement appropriate synchronization.
