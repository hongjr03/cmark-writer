# Tables Example

This example demonstrates how to create and format tables using cmark-writer, including both standard tables and GFM-enabled tables with alignment options.

## Basic Table Example

```rust
use cmark_writer::ast::Node;
use cmark_writer::ast::tables::TableBuilder;
use cmark_writer::writer::CommonMarkWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple table using the TableBuilder
    let table = TableBuilder::new()
        .headers(vec![
            Node::Text("Name".to_string()), 
            Node::Text("Age".to_string()), 
            Node::Text("Occupation".to_string())
        ])
        .add_row(vec![
            Node::Text("Alice".to_string()),
            Node::Text("30".to_string()),
            Node::Text("Engineer".to_string()),
        ])
        .add_row(vec![
            Node::Text("Bob".to_string()),
            Node::Text("25".to_string()),
            Node::Text("Designer".to_string()),
        ])
        .add_row(vec![
            Node::Text("Charlie".to_string()),
            Node::Text("35".to_string()),
            Node::Text("Doctor".to_string()),
        ])
        .build();
    
    // Create a document containing the table
    let document = Node::Document(vec![
        Node::heading(1, vec![Node::Text("Basic Table Example".to_string())]),
        Node::Paragraph(vec![Node::Text("Here's a simple table:".to_string())]),
        table,
    ]);
    
    // Create a writer and render the document
    let mut writer = CommonMarkWriter::new();
    writer.write(&document)?;
    let markdown = writer.into_string();
    
    // Print the generated markdown
    println!("{}", markdown);
    
    Ok(())
}
```

This produces the following output:

```markdown
# Basic Table Example

Here's a simple table:

| Name | Age | Occupation |
| --- | --- | --- |
| Alice | 30 | Engineer |
| Bob | 25 | Designer |
| Charlie | 35 | Doctor |
```

## GFM Tables with Alignment

When the `gfm` feature is enabled, you can specify column alignment:

```rust
#[cfg(feature = "gfm")]
use cmark_writer::ast::{Node, TableAlignment};
use cmark_writer::ast::tables::TableBuilder;
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;

#[cfg(feature = "gfm")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a table with specific alignments
    let table = TableBuilder::new()
        .headers(vec![
            Node::Text("Left Aligned".to_string()),
            Node::Text("Center Aligned".to_string()),
            Node::Text("Right Aligned".to_string()),
        ])
        .alignments(vec![
            TableAlignment::Left,    // :---
            TableAlignment::Center,  // :---:
            TableAlignment::Right,   // ---:
        ])
        .add_row(vec![
            Node::Text("Text".to_string()),
            Node::Text("Text".to_string()),
            Node::Text("Text".to_string()),
        ])
        .add_row(vec![
            Node::Text("Longer text here".to_string()),
            Node::Text("Centered content".to_string()),
            Node::Text("12.34".to_string()),
        ])
        .build();
    
    // Create a document containing the table
    let document = Node::Document(vec![
        Node::heading(1, vec![Node::Text("GFM Table with Alignment".to_string())]),
        Node::Paragraph(vec![
            Node::Text("This table uses GFM alignment features:".to_string())
        ]),
        table,
    ]);
    
    // Configure writer with GFM tables enabled
    let options = WriterOptionsBuilder::new()
        .gfm_tables(true)
        .build();
    
    let mut writer = CommonMarkWriter::with_options(options);
    writer.write(&document)?;
    let markdown = writer.into_string();
    
    // Print the generated markdown
    println!("{}", markdown);
    
    Ok(())
}

#[cfg(not(feature = "gfm"))]
fn main() {
    println!("This example requires the 'gfm' feature to be enabled");
}
```

With the `gfm` feature enabled, this produces:

```markdown
# GFM Table with Alignment

This table uses GFM alignment features:

| Left Aligned | Center Aligned | Right Aligned |
| :--- | :---: | ---: |
| Text | Text | Text |
| Longer text here | Centered content | 12.34 |
```

## Advanced Table Formatting

You can include formatting inside table cells:

```rust
use cmark_writer::ast::{Node, ListItem};
use cmark_writer::ast::tables::TableBuilder;
use cmark_writer::writer::CommonMarkWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a table with formatted content in cells
    let table = TableBuilder::new()
        .headers(vec![
            Node::Text("Item".to_string()),
            Node::Text("Description".to_string()),
            Node::Text("Status".to_string()),
        ])
        .add_row(vec![
            Node::Text("Feature 1".to_string()),
            Node::Paragraph(vec![
                Node::Text("Basic feature with ".to_string()),
                Node::Strong(vec![Node::Text("important".to_string())]),
                Node::Text(" aspects".to_string()),
            ]),
            Node::Text("Complete".to_string()),
        ])
        .add_row(vec![
            Node::Text("Feature 2".to_string()),
            Node::Paragraph(vec![
                Node::Text("Complex feature with ".to_string()),
                Node::Emphasis(vec![Node::Text("specialized".to_string())]),
                Node::Text(" components".to_string()),
            ]),
            Node::Text("In Progress".to_string()),
        ])
        .build();
    
    // Create a document with the table
    let document = Node::Document(vec![
        Node::heading(1, vec![Node::Text("Formatted Table Cells".to_string())]),
        table,
    ]);
    
    // Render the document
    let mut writer = CommonMarkWriter::new();
    writer.write(&document)?;
    let markdown = writer.into_string();
    
    println!("{}", markdown);
    
    Ok(())
}
```

This produces:

```markdown
# Formatted Table Cells

| Item | Description | Status |
| --- | --- | --- |
| Feature 1 | Basic feature with **important** aspects | Complete |
| Feature 2 | Complex feature with *specialized* components | In Progress |
```

## Best Practices for Tables

1. **Keep tables simple**: Avoid overly complex formatting in table cells
2. **Maintain consistency**: Use similar structure across all rows
3. **Align data appropriately**: Use alignment to improve readability (numbers right-aligned, text left-aligned)
4. **Use headers effectively**: Make column headers clear and descriptive
5. **Consider width**: Be mindful of table width for better rendering on different devices
