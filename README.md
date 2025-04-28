# cmark-writer

[![CI Status](https://github.com/hongjr03/cmark-writer/workflows/CI/badge.svg)](https://github.com/hongjr03/cmark-writer/actions)
[![Crates.io](https://img.shields.io/crates/v/cmark-writer.svg)](https://crates.io/crates/cmark-writer)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/cmark-writer.svg)](https://crates.io/crates/cmark-writer)
[![Codecov](https://codecov.io/gh/hongjr03/cmark-writer/branch/master/graph/badge.svg)](https://codecov.io/gh/hongjr03/cmark-writer)

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

You can customize the formatting behavior using the options builder pattern:

```rust
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;
use cmark_writer::ast::Node;

// Create custom options using the builder pattern
let options = WriterOptionsBuilder::new()
    .strict(true)                // Follow CommonMark spec strictly
    .hard_break_spaces(false)    // Use backslash for line breaks
    .indent_spaces(2)            // Use 2 spaces for indentation
    .build();

// Create writer with custom options
let mut writer = CommonMarkWriter::with_options(options);
writer.write(&Node::Text("Example".to_string())).unwrap();
```

Alternatively, you can use the struct initialization syntax:

```rust
use cmark_writer::options::WriterOptions;
use cmark_writer::writer::CommonMarkWriter;
use cmark_writer::ast::Node;

// Create custom options using struct initialization
let options = WriterOptions {
    strict: true,                // Follow CommonMark spec strictly
    hard_break_spaces: false,    // Use backslash for line breaks
    indent_spaces: 2,            // Use 2 spaces for indentation
    ..Default::default()         // Other options can be set here
};

// Create writer with custom options
let mut writer = CommonMarkWriter::with_options(options);
writer.write(&Node::Text("Example".to_string())).unwrap();
```

### Tables

The library provides a fluent API for creating tables, even without the GFM feature enabled:

```rust
use cmark_writer::ast::{Node, tables::TableBuilder};

// Create a simple table using the builder pattern
let table = TableBuilder::new()
    .headers(vec![
        Node::Text("Name".to_string()), 
        Node::Text("Age".to_string())
    ])
    .add_row(vec![
        Node::Text("Alice".to_string()),
        Node::Text("30".to_string()),
    ])
    .add_row(vec![
        Node::Text("Bob".to_string()),
        Node::Text("25".to_string()),
    ])
    .build();

// Or use the convenience function
let simple_table = cmark_writer::ast::tables::simple_table(
    vec![Node::Text("Header".to_string())],
    vec![vec![Node::Text("Data".to_string())]]
);
```

Alternatively, you can use the direct struct initialization approach:

```rust
use cmark_writer::ast::Node;

// Create a table using struct initialization
let table = Node::Table {
    headers: vec![
        Node::Text("Name".to_string()),
        Node::Text("Age".to_string()),
    ],
    #[cfg(feature = "gfm")]
    alignments: vec![
        cmark_writer::ast::TableAlignment::Left,
        cmark_writer::ast::TableAlignment::Left,
    ],
    rows: vec![
        vec![
            Node::Text("Alice".to_string()),
            Node::Text("30".to_string()),
        ],
        vec![
            Node::Text("Bob".to_string()),
            Node::Text("25".to_string()),
        ],
    ],
};
```

When the GFM feature is enabled, additional table alignment options become available.

### Safe HTML Handling

The library provides utilities for safely handling HTML content:

```rust
use cmark_writer::ast::{HtmlElement, Node};

// Escape HTML special characters in attributes
let script_element = HtmlElement::new("div")
    .with_attribute("data-content", "alert('hello')")
    .with_children(vec![Node::Text("Safe content".to_string())]);

// HTML attributes are automatically escaped
let mut writer = cmark_writer::writer::CommonMarkWriter::new();
writer.write(&Node::HtmlElement(script_element)).unwrap();
let html = writer.into_string();
// This will produce safe HTML with escaped attributes
```

### GitHub Flavored Markdown (GFM)

The library supports GitHub Flavored Markdown extensions as an optional feature. This includes:

- Tables with column alignment (`:---`, `:---:`, `---:`)
- Strikethrough text (`~~text~~`)
- Task lists (`- [ ]` and `- [x]`)
- Extended autolinks (without angle brackets)
- HTML element filtering (blocking potentially unsafe tags)

To use GFM features, first enable the feature in your `Cargo.toml`:

```toml
[dependencies]
cmark-writer = { version = "0.6.0", features = ["gfm"] }
```

#### Basic GFM Usage

```rust
// Note: this example requires the "gfm" feature to be enabled
#[cfg(feature = "gfm")]
mod example {
    use cmark_writer::writer::CommonMarkWriter;
    use cmark_writer::ast::Node;
    use cmark_writer::options::WriterOptionsBuilder;
    
    pub fn demo() {
        // Create writer options with GFM features enabled using the builder pattern
        let options = WriterOptionsBuilder::new()
            .gfm_tasklists(true)
            .gfm_strikethrough(true)
            .build();  // enable_gfm is automatically set when any GFM feature is enabled
        
        let mut writer = CommonMarkWriter::with_options(options);
        
        // Create a task list item (would use gfm::tasks if available)
        let document = Node::Document(vec![
            Node::Paragraph(vec![
                Node::Text("This is a task list example".to_string())
            ])
        ]);
        
        writer.write(&document).expect("Failed to write document");
        let markdown = writer.into_string();
        println!("{}", markdown);
    }
}
```

#### GFM Tables with the Table Builder

```rust
// Note: this example requires the "gfm" feature to be enabled
#[cfg(feature = "gfm")]
mod example {
    use cmark_writer::ast::Node;
    use cmark_writer::ast::tables::TableBuilder;
    
    pub fn demo() {
        // Create a table with standard alignment
        let table = TableBuilder::new()
            .headers(vec![
                Node::Text("Left".to_string()), 
                Node::Text("Center".to_string()), 
                Node::Text("Right".to_string())
            ])
            .add_row(vec![
                Node::Text("Data 1".to_string()),
                Node::Text("Data 2".to_string()),
                Node::Text("Data 3".to_string()),
            ])
            .build();
    
        // With GFM enabled, you could use alignment features
        #[cfg(feature = "gfm")]
        {
            use cmark_writer::ast::TableAlignment;
            
            let _aligned_table = TableBuilder::new()
                .headers(vec![Node::Text("Header".to_string())])
                .add_row(vec![Node::Text("Content".to_string())])
                .build();
        }
    }
}
```

#### Task Lists and Strikethrough

```rust
// Note: this example requires the "gfm" feature to be enabled
#[cfg(feature = "gfm")]
mod example {
    use cmark_writer::ast::Node;
    use cmark_writer::options::WriterOptions;
    use cmark_writer::writer::CommonMarkWriter;
    
    pub fn demo() {
        // Create a task list using GFM options
        let options = WriterOptions {
            enable_gfm: true,
            gfm_tasklists: true,
            gfm_strikethrough: true,
            ..Default::default()
        };
        
        // Create task-like content
        let completed_task = Node::Paragraph(vec![
            Node::Text("Completed task".to_string())
        ]);
        
        // Create strikethrough-like content
        let strike_text = Node::Paragraph(vec![
            Node::Text("This text would be crossed out".to_string())
        ]);
        
        // With GFM enabled, this would render as task lists and strikethrough
        let mut writer = CommonMarkWriter::with_options(options);
        writer.write(&completed_task).expect("Failed to write");
        writer.write(&strike_text).expect("Failed to write");
    }
}
```

#### GFM HTML Safety

GFM provides additional HTML safety features:

```rust
// Note: this example requires the "gfm" feature to be enabled
#[cfg(feature = "gfm")]
mod example {
    use cmark_writer::ast::{Node, HtmlElement};
    use cmark_writer::options::WriterOptions;
    use cmark_writer::writer::CommonMarkWriter;
    
    pub fn demo() {
        // Create a document with potentially unsafe HTML
        let document = Node::Document(vec![
            Node::HtmlElement(HtmlElement::new("script")
                .with_children(vec![Node::Text("alert('unsafe')".to_string())]))
        ]);
        
        // Enable GFM with HTML filtering
        let options = WriterOptions {
            enable_gfm: true,
            gfm_disallowed_html_tags: vec!["script".to_string(), "iframe".to_string()],
            ..Default::default()
        };
        
        // This will automatically filter out unsafe HTML elements
        let mut writer = CommonMarkWriter::with_options(options);
        writer.write(&document).expect("Failed to write");
    }
}
```

#### Customizing GFM Features

```rust
// Note: this example requires the "gfm" feature to be enabled
#[cfg(feature = "gfm")]
mod example {
    use cmark_writer::options::WriterOptionsBuilder;
    use cmark_writer::writer::CommonMarkWriter;
    
    pub fn demo() {
        // Enable specific GFM features using the builder pattern
        let writer = CommonMarkWriter::with_options(
            WriterOptionsBuilder::new()
                .gfm_tables(true)
                .gfm_strikethrough(true)
                .build()  // enable_gfm is automatically set
        );
        
        // You can also customize the list of disallowed HTML tags
        let custom_html_safety = WriterOptionsBuilder::new()
            .gfm_disallowed_html_tags(vec![
                "script".to_string(), 
                "iframe".to_string()
            ])
            .build();  // enable_gfm is automatically set
        
        let _writer = CommonMarkWriter::with_options(custom_html_safety);
    }
}
```

## API Documentation

### Core Types

- `Node` - Represents various CommonMark node types
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
