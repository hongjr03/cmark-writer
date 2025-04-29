# Node API

The `Node` enum is the fundamental building block of the document structure in cmark-writer. It represents all possible elements in a CommonMark document.

## Node Types

The `Node` enum includes the following variants:

### Block Elements

Block-level elements form the structure of a document:

| Node Variant | Description | Example |
|--------------|-------------|---------|
| `Document` | Root container for all content | The entire Markdown document |
| `Heading` | Section heading (levels 1-6) | `# Heading` |
| `Paragraph` | Text paragraph | Normal text blocks |
| `BlockQuote` | Quoted content | `> Quoted text` |
| `CodeBlock` | Block of code with optional language | ````rust` |
| `ThematicBreak` | Horizontal rule | `---` |
| `OrderedList` | Numbered list | `1. Item` |
| `UnorderedList` | Bullet list | `- Item` |
| `Table` | Tabular data | A data table |
| `HtmlBlock` | Block-level HTML | `<div>` |
| `LinkReferenceDefinition` | Link reference | `[ref]: url` |

### Inline Elements

Inline elements appear within block elements:

| Node Variant | Description | Example |
|--------------|-------------|---------|
| `Text` | Plain text content | Normal text |
| `Emphasis` | Emphasized text | `*italic*` |
| `Strong` | Strongly emphasized text | `**bold**` |
| `InlineCode` | Code within text | `` `code` `` |
| `Link` | Hyperlink | `[text](url)` |
| `ReferenceLink` | Reference-style link | `[text][ref]` |
| `Image` | Image reference | `![alt](src)` |
| `Autolink` | URI/email in angle brackets | `<https://example.com>` |
| `HardBreak` | Hard line break | `\\` or two spaces |
| `SoftBreak` | Soft line break | A simple newline |
| `HtmlElement` | Inline HTML | `<span>` |

### GFM Extensions

When the `gfm` feature is enabled:

| Node Variant | Description | Example |
|--------------|-------------|---------|
| `Strikethrough` | Crossed-out text | `~~text~~` |
| `Table` with alignments | Table with column alignment | Left/center/right aligned columns |
| Task list items | Checkable items | `- [ ]` or `- [x]` |
| `ExtendedAutolink` | Auto-detected links | URLs without angle brackets |

## Creating Nodes

There are several ways to create nodes:

### Using Enum Variants Directly

```rust
let heading = Node::Heading {
    level: 1,
    content: vec![Node::Text("Title".to_string())],
    heading_type: HeadingType::Atx,
};
```

### Using Convenience Methods

```rust
let heading = Node::heading(1, vec![Node::Text("Title".to_string())]);
```

### Building Complex Structures

```rust
let document = Node::Document(vec![
    Node::heading(1, vec![Node::Text("Document Title".to_string())]),
    Node::Paragraph(vec![
        Node::Text("This is a paragraph with ".to_string()),
        Node::Strong(vec![Node::Text("bold".to_string())]),
        Node::Text(" and ".to_string()),
        Node::Emphasis(vec![Node::Text("italic".to_string())]),
        Node::Text(" text.".to_string()),
    ]),
    Node::UnorderedList(vec![
        ListItem::Unordered { 
            content: vec![Node::Paragraph(vec![Node::Text("Item 1".to_string())])] 
        },
        ListItem::Unordered { 
            content: vec![Node::Paragraph(vec![Node::Text("Item 2".to_string())])] 
        },
    ]),
]);
```

## Working with List Items

List items have their own types to support different list styles:

```rust
// Unordered list item
let unordered_item = ListItem::Unordered {
    content: vec![Node::Paragraph(vec![Node::Text("Bullet point".to_string())])],
};

// Ordered list item
let ordered_item = ListItem::Ordered {
    number: Some(1), // Optional explicit numbering
    content: vec![Node::Paragraph(vec![Node::Text("Numbered item".to_string())])],
};

// GFM task list item (with gfm feature)
#[cfg(feature = "gfm")]
let task_item = ListItem::Task {
    status: TaskListStatus::Unchecked,
    content: vec![Node::Paragraph(vec![Node::Text("Task to do".to_string())])],
};
```

## Tables

The library provides a fluent API for creating tables:

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

## Utility Methods

The `Node` enum provides useful methods for working with nodes:

### Type Checking

```rust
let heading = Node::heading(1, vec![Node::Text("Title".to_string())]);
let is_block = heading.is_block();  // true
let is_inline = heading.is_inline(); // false

let text = Node::Text("Hello".to_string());
let is_inline = text.is_inline();   // true
let is_block = text.is_block();     // false
```

### Custom Nodes

The `Node::Custom` variant allows you to implement custom node behavior by implementing the `CustomNode` trait:

```rust
use cmark_writer::{custom_node, ast::CustomNode};

#[custom_node]
#[derive(Debug, Clone, PartialEq)]
struct MyCustomNode {
    content: String,
}

// Use as: Node::Custom(Box::new(MyCustomNode { content: "Hello".to_string() }))
```

For more details on custom nodes, see the [Custom Nodes guide](/guide/advanced-usage/custom-nodes).
