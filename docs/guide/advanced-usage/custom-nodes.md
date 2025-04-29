# Custom Nodes

cmark-writer allows you to extend its functionality by creating custom node types. This feature is useful when you need to represent document elements that aren't part of the standard CommonMark specification.

## Creating Custom Nodes

To create a custom node, you need to:

1. Define a struct or enum for your custom node
2. Implement the `CustomNode` trait for your type
3. Apply the `#[custom_node]` attribute to your type
4. Create instances of your custom node wrapped in `Node::Custom`

### Basic Example

Here's a simple example of creating a custom highlight node:

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
```

### Using Custom Nodes

Once defined, you can use your custom node in documents:

```rust
use cmark_writer::writer::CommonMarkWriter;

// Create a document with a custom node
let document = Node::Document(vec![
    Node::Paragraph(vec![
        Node::Text("This text contains a ".to_string()),
        Node::Custom(Box::new(HighlightNode {
            content: "highlighted section".to_string(),
            color: "yellow".to_string(),
        })),
        Node::Text(".".to_string()),
    ]),
]);

// Write the document
let mut writer = CommonMarkWriter::new();
writer.write(&document).expect("Failed to write document");
let markdown = writer.into_string();
```

## Custom Node Interface

The `CustomNode` trait requires implementing several methods:

```rust
pub trait CustomNode: Debug + Send + Sync {
    // Required by #[custom_node] macro:
    fn write(&self, writer: &mut dyn CustomNodeWriter) -> WriteResult<()>;
    fn is_block(&self) -> bool;
    fn clone_custom(&self) -> Box<dyn CustomNode>;
    fn eq_custom(&self, other: &dyn CustomNode) -> bool;
}
```

The `#[custom_node]` attribute automatically implements these methods by delegating to:

- `write_custom(&self, writer: &mut dyn CustomNodeWriter) -> WriteResult<()>`
- `is_block_custom(&self) -> bool`

You only need to implement these two methods.

## The CustomNodeWriter Interface

The `CustomNodeWriter` trait provides methods for writing content:

```rust
pub trait CustomNodeWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result;
    fn write_char(&mut self, c: char) -> fmt::Result;
}
```

Use these methods in your `write_custom` implementation to produce output.

## More Complex Example

Here's a more complex example that creates a colored box node:

```rust
#[derive(Debug, Clone, PartialEq)]
#[custom_node]
struct ColorBoxNode {
    content: Vec<Node>,
    background_color: String,
    border_color: Option<String>,
}

impl ColorBoxNode {
    fn write_custom(&self, writer: &mut dyn CustomNodeWriter) -> WriteResult<()> {
        // Start HTML for the colored box
        writer.write_str("<div style=\"background-color: ")?;
        writer.write_str(&self.background_color)?;
        
        if let Some(border) = &self.border_color {
            writer.write_str("; border: 1px solid ")?;
            writer.write_str(border)?;
        }
        
        writer.write_str("; padding: 10px;\">\n")?;
        
        // For a complex node that contains other nodes,
        // you would typically convert this writer to CommonMarkWriter
        // and use it to write child nodes. This requires more advanced
        // implementation that is beyond this simple example.
        
        writer.write_str("</div>")?;
        
        Ok(())
    }
    
    fn is_block_custom(&self) -> bool {
        true // This is a block element
    }
}
```

## Best Practices

When creating custom nodes:

1. **Clear Responsibility**: Each custom node should have a single, well-defined purpose
2. **Proper Nesting**: Respect block/inline distinctions when nesting custom nodes
3. **Error Handling**: Use appropriate error handling in your `write_custom` method
4. **Documentation**: Document your custom nodes thoroughly for users
5. **Testing**: Write tests to ensure your custom nodes render correctly
