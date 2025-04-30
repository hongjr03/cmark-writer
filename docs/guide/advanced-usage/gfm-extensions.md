# GFM Extensions

GitHub Flavored Markdown (GFM) extends the CommonMark specification with additional features that are particularly useful for documentation and collaboration. cmark-writer supports these extensions when the `gfm` feature is enabled.

## Enabling GFM Features

To use GFM features, you need to:

1. Add the `gfm` feature to your dependency in `Cargo.toml`:

    ```toml
    [dependencies]
    cmark-writer = { version = "0.6.2", features = ["gfm"] }
    ```

2. Enable GFM options in your writer:

    ```rust
    use cmark_writer::options::WriterOptionsBuilder;
    use cmark_writer::writer::CommonMarkWriter;

    // Enable all GFM features
    let options = WriterOptionsBuilder::new()
        .enable_gfm(true)
        .build();

    // Or enable specific GFM features
    let options = WriterOptionsBuilder::new()
        .gfm_tables(true)
        .gfm_strikethrough(true)
        .gfm_tasklists(true)
        .gfm_autolinks(true)
        .build();  // enable_gfm is automatically set to true

    let writer = CommonMarkWriter::with_options(options);
    ```

## Tables with Alignment

GFM tables support column alignment using `:---`, `:---:`, and `---:` syntax. The alignment can be left, center, or right.

### Creating Tables with Alignment

```rust
#[cfg(feature = "gfm")]
use cmark_writer::ast::{Node, TableAlignment};
use cmark_writer::writer::CommonMarkWriter;

#[cfg(feature = "gfm")]
fn create_aligned_table() {
    // Create a table with specific alignments
    let table = Node::table_with_alignment(
        vec![
            Node::Text("Left".to_string()), 
            Node::Text("Center".to_string()), 
            Node::Text("Right".to_string())
        ],
        vec![
            TableAlignment::Left,     // :---
            TableAlignment::Center,   // :---:
            TableAlignment::Right,    // ---:
        ],
        vec![
            vec![
                Node::Text("Data 1".to_string()),
                Node::Text("Data 2".to_string()),
                Node::Text("Data 3".to_string()),
            ]
        ]
    );
    
    let mut writer = CommonMarkWriter::new();
    writer.write(&table).expect("Failed to write table");
    let markdown = writer.into_string();
    
    // Output will include alignment markers:
    // | Left | Center | Right |
    // | :--- | :----: | ----: |
    // | Data 1 | Data 2 | Data 3 |
}
```

### Using the Table Builder

You can also use the table builder for a more fluent API:

```rust
#[cfg(feature = "gfm")]
use cmark_writer::ast::tables::TableBuilder;
use cmark_writer::ast::Node;

#[cfg(feature = "gfm")]
fn build_aligned_table() {
    // Using the TableBuilder with alignment
    let table = TableBuilder::new()
        .headers(vec![
            Node::Text("Left".to_string()), 
            Node::Text("Center".to_string()), 
            Node::Text("Right".to_string())
        ])
        .alignments(vec![
            TableAlignment::Left,
            TableAlignment::Center,
            TableAlignment::Right,
        ])
        .add_row(vec![
            Node::Text("Data 1".to_string()),
            Node::Text("Data 2".to_string()),
            Node::Text("Data 3".to_string()),
        ])
        .build();
}
```

## Strikethrough

GFM supports strikethrough text using the `~~text~~` syntax.

```rust
#[cfg(feature = "gfm")]
use cmark_writer::ast::Node;
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;

#[cfg(feature = "gfm")]
fn strikethrough_example() {
    // Create a paragraph with strikethrough text
    let paragraph = Node::Paragraph(vec![
        Node::Text("This text has ".to_string()),
        Node::Strikethrough(vec![Node::Text("strikethrough".to_string())]),
        Node::Text(" content.".to_string()),
    ]);
    
    // Configure writer with GFM strikethrough enabled
    let options = WriterOptionsBuilder::new()
        .gfm_strikethrough(true)
        .build();
    
    let mut writer = CommonMarkWriter::with_options(options);
    writer.write(&paragraph).expect("Failed to write paragraph");
    let markdown = writer.into_string();
    
    // Output: This text has ~~strikethrough~~ content.
}
```

## Task Lists

GFM task lists are checkboxes that can be checked or unchecked.

```rust
#[cfg(feature = "gfm")]
use cmark_writer::ast::{Node, ListItem, TaskListStatus};
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;

#[cfg(feature = "gfm")]
fn task_list_example() {
    // Create an unordered list with task items
    let list = Node::UnorderedList(vec![
        // Unchecked task
        ListItem::Task {
            status: TaskListStatus::Unchecked,
            content: vec![Node::Paragraph(vec![
                Node::Text("Incomplete task".to_string())
            ])],
        },
        // Checked task
        ListItem::Task {
            status: TaskListStatus::Checked,
            content: vec![Node::Paragraph(vec![
                Node::Text("Completed task".to_string())
            ])],
        },
    ]);
    
    // Configure writer with GFM task lists enabled
    let options = WriterOptionsBuilder::new()
        .gfm_tasklists(true)
        .build();
    
    let mut writer = CommonMarkWriter::with_options(options);
    writer.write(&list).expect("Failed to write list");
    let markdown = writer.into_string();
    
    // Output:
    // - [ ] Incomplete task
    // - [x] Completed task
}
```

## Extended Autolinks

GFM automatically detects URLs and email addresses without requiring angle brackets.

```rust
#[cfg(feature = "gfm")]
use cmark_writer::ast::Node;
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;

#[cfg(feature = "gfm")]
fn autolink_example() {
    // Create a paragraph with an extended autolink
    let paragraph = Node::Paragraph(vec![
        Node::Text("Check out ".to_string()),
        Node::ExtendedAutolink("https://example.com".to_string()),
        Node::Text(" for more information.".to_string()),
    ]);
    
    // Configure writer with GFM autolinks enabled
    let options = WriterOptionsBuilder::new()
        .gfm_autolinks(true)
        .build();
    
    let mut writer = CommonMarkWriter::with_options(options);
    writer.write(&paragraph).expect("Failed to write paragraph");
    let markdown = writer.into_string();
    
    // Output: Check out https://example.com for more information.
}
```

## HTML Safety

GFM provides additional HTML safety features to filter out potentially unsafe tags:

```rust
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;
use cmark_writer::ast::{Node, HtmlElement};

#[cfg(feature = "gfm")]
fn html_safety_example() {
    // Create an HTML element that might be unsafe
    let html = HtmlElement::new("script")
        .with_children(vec![Node::Text("alert('unsafe')".to_string())]);
    
    // Configure writer with HTML filtering
    let options = WriterOptionsBuilder::new()
        .enable_gfm(true)
        .gfm_disallowed_html_tags(vec![
            "script".to_string(), 
            "iframe".to_string(),
            "object".to_string(),
        ])
        .build();
    
    let mut writer = CommonMarkWriter::with_options(options);
    writer.write(&Node::HtmlElement(html)).expect("Failed to write HTML");
    let markdown = writer.into_string();
    
    // The script tag will be filtered out or safely escaped
}
```

## Using Multiple GFM Features

You can combine multiple GFM features in a single document:

```rust
#[cfg(feature = "gfm")]
fn combined_gfm_example() {
    // Configure writer with all GFM features
    let options = WriterOptionsBuilder::new()
        .enable_gfm(true)
        .build();
    
    let mut writer = CommonMarkWriter::with_options(options);
    
    // Now you can use tables, strikethrough, task lists, and autolinks
    // in your document...
}
