# Basic Document Example

This example demonstrates how to create a complete Markdown document with common elements like headings, paragraphs, lists, and formatting.

## Complete Example

```rust
use cmark_writer::ast::{Node, ListItem};
use cmark_writer::writer::CommonMarkWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a document with various elements
    let document = Node::Document(vec![
        // Heading level 1
        Node::heading(1, vec![Node::Text("Sample Document".to_string())]),
        
        // Paragraph with mixed formatting
        Node::Paragraph(vec![
            Node::Text("This is a paragraph with ".to_string()),
            Node::Strong(vec![Node::Text("bold".to_string())]),
            Node::Text(" and ".to_string()),
            Node::Emphasis(vec![Node::Text("italic".to_string())]),
            Node::Text(" text. It also includes a ".to_string()),
            Node::Link {
                url: "https://example.com".to_string(),
                title: Some("Example Website".to_string()),
                content: vec![Node::Text("link".to_string())],
            },
            Node::Text(".".to_string()),
        ]),
        
        // A blockquote
        Node::BlockQuote(vec![
            Node::Paragraph(vec![
                Node::Text("This is a blockquote with a nested ".to_string()),
                Node::Emphasis(vec![Node::Text("emphasized".to_string())]),
                Node::Text(" phrase.".to_string()),
            ]),
        ]),
        
        // A code block with language
        Node::CodeBlock {
            language: Some("rust".to_string()),
            content: "fn main() {\n    println!(\"Hello, world!\");\n}".to_string(),
            block_type: cmark_writer::ast::CodeBlockType::Fenced,
        },
        
        // Thematic break (horizontal rule)
        Node::ThematicBreak,
        
        // Heading level 2
        Node::heading(2, vec![Node::Text("Lists Example".to_string())]),
        
        // Unordered list
        Node::UnorderedList(vec![
            ListItem::Unordered { 
                content: vec![Node::Paragraph(vec![Node::Text("First item".to_string())])] 
            },
            ListItem::Unordered { 
                content: vec![
                    Node::Paragraph(vec![Node::Text("Second item with sub-list".to_string())]),
                    Node::UnorderedList(vec![
                        ListItem::Unordered { 
                            content: vec![Node::Paragraph(vec![Node::Text("Sub-item 1".to_string())])] 
                        },
                        ListItem::Unordered { 
                            content: vec![Node::Paragraph(vec![Node::Text("Sub-item 2".to_string())])] 
                        },
                    ]),
                ] 
            },
            ListItem::Unordered { 
                content: vec![Node::Paragraph(vec![Node::Text("Third item".to_string())])] 
            },
        ]),
        
        // Heading level 2
        Node::heading(2, vec![Node::Text("Ordered List Example".to_string())]),
        
        // Ordered list
        Node::OrderedList {
            start: 1,
            items: vec![
                ListItem::Ordered { 
                    number: None,
                    content: vec![Node::Paragraph(vec![Node::Text("First ordered item".to_string())])] 
                },
                ListItem::Ordered { 
                    number: None,
                    content: vec![Node::Paragraph(vec![Node::Text("Second ordered item".to_string())])] 
                },
                ListItem::Ordered { 
                    number: None,
                    content: vec![Node::Paragraph(vec![Node::Text("Third ordered item".to_string())])] 
                },
            ],
        },
        
        // A paragraph with inline code
        Node::Paragraph(vec![
            Node::Text("You can also include ".to_string()),
            Node::InlineCode("inline code".to_string()),
            Node::Text(" within a paragraph.".to_string()),
        ]),
        
        // Image example
        Node::Paragraph(vec![
            Node::Image {
                url: "https://example.com/image.jpg".to_string(),
                title: Some("Example Image".to_string()),
                alt: vec![Node::Text("An example image".to_string())],
            },
        ]),
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

## Output

The code above generates the following Markdown:

````markdown
# Sample Document

This is a paragraph with **bold** and *italic* text. It also includes a [link](https://example.com "Example Website").

> This is a blockquote with a nested *emphasized* phrase.

```rust
fn main() {
    println!("Hello, world!");
}
```

---

## Lists Example

- First item
- Second item with sub-list
  - Sub-item 1
  - Sub-item 2
- Third item

## Ordered List Example

1. First ordered item
2. Second ordered item
3. Third ordered item

You can also include `inline code` within a paragraph.

![An example image](https://example.com/image.jpg "Example Image")

````

## Key Points

- The root `Document` node serves as a container for all other nodes
- Block-level elements (headings, paragraphs, lists) can contain inline elements
- Nested structures like lists or blockquotes are represented through node nesting
- The writer handles proper indentation and formatting based on CommonMark rules
- For inline formatting, wrap text nodes in the appropriate container (Strong, Emphasis, etc.)

## Variations

You can customize the output format by using writer options:

```rust
use cmark_writer::options::WriterOptionsBuilder;

// Customize the formatting behavior
let options = WriterOptionsBuilder::new()
    .list_marker('*')          // Use * for unordered lists
    .thematic_break_char('_')  // Use ___ for horizontal rules
    .build();

let writer = CommonMarkWriter::with_options(options);
```
