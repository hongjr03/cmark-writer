# AST Nodes

The Abstract Syntax Tree (AST) nodes are the building blocks of your Markdown document. The `Node` enum represents all possible elements in a CommonMark document.

## Node Types

The `Node` enum includes various variants for different Markdown elements:

### Block Elements

These elements form the structure of the document:

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

### Inline Elements

These elements appear within block elements:

| Node Variant | Description | Example |
|--------------|-------------|---------|
| `Text` | Plain text content | Normal text |
| `Emphasis` | Emphasized text | `*italic*` |
| `Strong` | Strongly emphasized text | `**bold**` |
| `InlineCode` | Code within text | `` `code` `` |
| `Link` | Hyperlink | `[text](url)` |
| `Image` | Image reference | `![alt](src)` |
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

You can create nodes in several ways:

### Direct Enum Construction

```rust
// Using enum variant directly
let heading = Node::Heading {
    level: 1,
    content: vec![Node::Text("Title".to_string())],
    heading_type: HeadingType::Atx,
};
```

### Convenience Methods

```rust
// Using convenience methods
let heading = Node::heading(1, vec![Node::Text("Title".to_string())]);
```

### Building Complex Structures

Nodes can be nested to create complex documents:

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
