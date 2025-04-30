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

You can also specify whether a node is a block element directly in the attribute:

```rust
// Define a block-level custom node using the attribute parameter
#[derive(Debug, Clone, PartialEq)]
#[custom_node(block=true)]
struct AlertBoxNode {
    content: String,
    level: AlertLevel,
}

// No need to implement is_block_custom anymore
impl AlertBoxNode {
    fn write_custom(&self, writer: &mut dyn CustomNodeWriter) -> WriteResult<()> {
        // Implementation...
        Ok(())
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

## Pattern Matching on Custom Nodes

One challenge with custom nodes is pattern matching, as they are stored behind a trait object (`Box<dyn CustomNode>`). cmark-writer provides convenient helper methods for matching and handling custom node types:

### Using Helper Methods

The `Node` enum provides helper methods for checking and extracting custom node types:

```rust
// Check if a node is a specific custom type
if node.is_custom_type::<HighlightNode>() {
    // Get a reference to the typed node
    let highlight = node.as_custom_type::<HighlightNode>().unwrap();
    // Work with the strongly typed node
    println!("Found highlight with color: {}", highlight.color);
}
```

You can also check the type name of a custom node:

```rust
match node {
    Node::Custom(custom) => {
        if HighlightNode::matches(&**custom) {
            if let Some(highlight) = custom.as_any().downcast_ref::<HighlightNode>() {
                // Handle HighlightNode
                println!("Highlight color: {}", highlight.color);
            }
        } else if AlertBoxNode::matches(&**custom) {
            if let Some(alert) = custom.as_any().downcast_ref::<AlertBoxNode>() {
                // Handle AlertBoxNode
                println!("Alert level: {:?}", alert.level);
            }
        }
    },
    _ => {
        // Handle other node types
    }
}
```

For more elegant matching, you can use `if node.is_custom_type()` as a match guard:

```rust
match node {
    Node::Paragraph(p) => {
        // Handle paragraph
    },
    node if node.is_custom_type::<HighlightNode>() => {
        let highlight = node.as_custom_type::<HighlightNode>().unwrap();
        // Handle highlight node
        println!("Highlight color: {}", highlight.color);
    },
    node if node.is_custom_type::<AlertBoxNode>() => {
        let alert = node.as_custom_type::<AlertBoxNode>().unwrap();
        // Handle alert node
        println!("Alert level: {:?}", alert.level);
    },
    _ => {
        // Handle other nodes
    }
}
```

These pattern matching utilities make working with custom nodes almost as convenient as working with regular enum variants.
