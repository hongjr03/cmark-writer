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
use cmark_writer::ToCommonMark;

// Create a document
let document = Node::Document(vec![
    Node::heading(1, vec![Node::Text("Hello CommonMark".into())]),
    Node::Paragraph(vec![
        Node::Text("This is a simple ".into()),
        Node::Strong(vec![Node::Text("example".into())]),
        Node::Text(".".into()),
    ]),
]);

// Render to CommonMark
let mut writer = CommonMarkWriter::new();
document.to_commonmark(&mut writer).expect("Failed to write document");
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
        Node::Text("Name".into()),
        Node::Text("Age".into())
    ])
    .add_row(vec![
        Node::Text("John".into()),
        Node::Text("30".into()),
    ])
    .add_row(vec![
        Node::Text("Alice".into()),
        Node::Text("25".into()),
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

## HTML Writing

The library provides dedicated HTML writing capabilities through the `HtmlWriter` class:

```rust
use cmark_writer::{HtmlWriter, HtmlWriterOptions, Node, ToHtml};

// Create HTML writer with custom options
let options = HtmlWriterOptions {
    strict: true,
    code_block_language_class_prefix: Some("language-".into()),
    #[cfg(feature = "gfm")]
    enable_gfm: true,
    #[cfg(feature = "gfm")]
    gfm_disallowed_html_tags: vec!["script".into()],
};

let mut writer = HtmlWriter::with_options(options);

// Write some nodes
let paragraph = Node::Paragraph(vec![Node::Text("Hello HTML".into())]);
paragraph.to_html(&mut writer).unwrap();

// Get resulting HTML
let html = writer.into_string();
assert_eq!(html, "<p>Hello HTML</p>\n");
```

## Custom Nodes

The recommended way to build custom nodes is via standard Rust traits. Implement Format for each writer you want to support, and optionally MultiFormat for capability checks and HTML fallback.

```rust
use cmark_writer::{Format, ToCommonMark, ToHtml, MultiFormat};
use cmark_writer::{CommonMarkWriter, HtmlWriter, WriteResult};
use cmark_writer::format_traits::default_html_render;
use ecow::EcoString;

// Inline custom node with CommonMark + HTML implementations
#[derive(Debug, Clone, PartialEq)]
pub struct HighlightNode {
    content: EcoString,
    color: EcoString,
}

impl Format<CommonMarkWriter> for HighlightNode {
    fn format(&self, writer: &mut CommonMarkWriter) -> WriteResult<()> {
        writer.write_str("<span style=\"background-color: ")?;
        writer.write_str(&self.color)?;
        writer.write_str("\">")?;
        writer.write_str(&self.content)?;
        writer.write_str("</span>")?;
        Ok(())
    }
}

impl Format<HtmlWriter> for HighlightNode {
    fn format(&self, writer: &mut HtmlWriter) -> WriteResult<()> {
        writer.start_tag("span")?;
        writer.attribute("style", &format!("background-color: {}", self.color))?;
        writer.finish_tag()?;
        writer.text(&self.content)?;
        writer.end_tag("span")?;
        Ok(())
    }
}

impl MultiFormat for HighlightNode {
    fn supports_html(&self) -> bool { true }
    fn html_format(&self, writer: &mut HtmlWriter) -> WriteResult<()> { self.to_html(writer) }
}

// Block-level custom node example
#[derive(Debug, Clone, PartialEq)]
pub struct CalloutBox {
    title: EcoString,
    content: EcoString,
}

impl Format<CommonMarkWriter> for CalloutBox {
    fn format(&self, writer: &mut CommonMarkWriter) -> WriteResult<()> {
        writer.write_str("<div class=\"callout\">\n  <h4>")?;
        writer.write_str(&self.title)?;
        writer.write_str("</h4>\n  <p>")?;
        writer.write_str(&self.content)?;
        writer.write_str("</p>\n</div>")?;
        Ok(())
    }
}

impl Format<HtmlWriter> for CalloutBox {
    fn format(&self, writer: &mut HtmlWriter) -> WriteResult<()> {
        writer.start_tag("div")?; writer.finish_tag()?;
        writer.start_tag("h4")?; writer.finish_tag()?; writer.text(&self.title)?; writer.end_tag("h4")?;
        writer.start_tag("p")?;  writer.finish_tag()?; writer.text(&self.content)?; writer.end_tag("p")?;
        writer.end_tag("div")?;
        Ok(())
    }
}

impl MultiFormat for CalloutBox {
    fn supports_html(&self) -> bool { true }
    fn html_format(&self, writer: &mut HtmlWriter) -> WriteResult<()> { self.to_html(writer) }
}

// Only CommonMark support with graceful HTML fallback
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleNote { content: EcoString }

impl Format<CommonMarkWriter> for SimpleNote {
    fn format(&self, writer: &mut CommonMarkWriter) -> WriteResult<()> {
        writer.write_str("> **Note:** ")?;
        writer.write_str(&self.content)?;
        Ok(())
    }
}

impl MultiFormat for SimpleNote {
    fn supports_html(&self) -> bool { false }
    fn html_format(&self, writer: &mut HtmlWriter) -> WriteResult<()> { default_html_render(self, writer) }
}

// Usage
let highlight = HighlightNode { content: "important".into(), color: "yellow".into() };
let mut md = CommonMarkWriter::new();
let mut html = HtmlWriter::new();
highlight.to_commonmark(&mut md).unwrap();
highlight.to_html(&mut html).unwrap();
assert!(highlight.supports_html());
```

This approach provides:

- Type safety and clear format boundaries
- Easy extensibility for new formats
- Consistent, idiomatic Rust traits across the API

## Custom Error Handling

The library provides convenient macros for creating structured custom errors:

```rust
use cmark_writer::{coded_error, structure_error, WriteError};

// Structure error - for invalid document structure
#[structure_error(format = "表格列数不匹配：{}")]
struct TableColumnMismatchError(pub &'static str);

// Coded error - for custom errors with error codes
#[coded_error]
struct MarkdownSyntaxError(pub String, pub String);

// Usage examples
fn validate_table() -> Result<(), WriteError> {
    // Create structure error
    let err = TableColumnMismatchError("第 3 行有 4 列，但表头只有 3 列").into_error();
    // Result: "Invalid structure: 表格列数不匹配：第 3 行有 4 列，但表头只有 3 列"
    
    // Create coded error
    let err = MarkdownSyntaxError(
        "缺少闭合代码块标记".into(), 
        "CODE_BLOCK_UNCLOSED".into()
    ).into_error();
    // Result: "Custom error [CODE_BLOCK_UNCLOSED]: 缺少闭合代码块标记"
    
    Ok(())
}

// Convert to standard WriteError
let write_err: WriteError = TableColumnMismatchError("错误示例").into();
assert!(matches!(write_err, WriteError::InvalidStructure(_)));
```

The error macros provide:

- **`#[structure_error]`**: For document structure validation errors
- **`#[coded_error]`**: For custom errors with error codes and messages
- Automatic conversion to `WriteError` types
- Consistent error formatting and display

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
