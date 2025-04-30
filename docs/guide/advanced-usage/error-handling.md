# Error Handling

cmark-writer provides a comprehensive error handling system that helps you identify and resolve issues when generating Markdown content. This page covers the error types, handling strategies, and how to create custom errors.

## Error Types

The library uses the `WriteError` enum as its primary error type:

```rust
pub enum WriteError {
    // Standard errors
    NewlineInInlineElement(String),
    InvalidNesting(String),
    UnsupportedNodeType,
    IoError(std::io::Error),
    // Custom errors
    CustomError(Box<dyn CustomErrorFactory>),
    StructureError(Box<dyn StructureError>),
    CodedError(Box<dyn CodedError>),
}
```

Common error scenarios include:

- **Invalid content**: Like newlines in inline elements (`NewlineInInlineElement`)
- **Improper nesting**: When nodes are nested incorrectly (`InvalidNesting`)
- **Unsupported operations**: When trying to use unsupported features (`UnsupportedNodeType`)
- **I/O errors**: When writing to files or streams fails (`IoError`)
- **Custom errors**: Application-specific errors you define (`CustomError`, `StructureError`, `CodedError`)

## Basic Error Handling

When using the writer, you can handle errors with standard Rust patterns:

```rust
use cmark_writer::ast::Node;
use cmark_writer::writer::CommonMarkWriter;

let node = Node::Text("Hello".to_string());
let mut writer = CommonMarkWriter::new();

match writer.write(&node) {
    Ok(_) => {
        println!("Successfully wrote node");
        let output = writer.into_string();
        // Use the output...
    },
    Err(err) => {
        eprintln!("Failed to write node: {}", err);
        // Handle the error appropriately...
    }
}
```

You can also use the `?` operator for more concise error handling:

```rust
fn write_document(document: &Node) -> Result<String, cmark_writer::error::WriteError> {
    let mut writer = CommonMarkWriter::new();
    writer.write(document)?;
    Ok(writer.into_string())
}
```

## Creating Custom Errors

cmark-writer provides several ways to define your own error types:

### 1. Structure Errors

Use the `#[structure_error]` attribute macro for simple formatted errors:

```rust
use cmark_writer::custom_error;
use cmark_writer::WriteError;

// Define a structure error with format string
#[structure_error(format = "Table structure error: {}")]
struct TableStructureError(pub &'static str);

// Using the error
fn validate_table_rows(rows: &[Vec<Node>]) -> Result<(), WriteError> {
    if rows.is_empty() {
        return Err(TableStructureError("Table must have at least one row").into());
    }
    // More validation...
    Ok(())
}
```

### 2. Coded Errors

Use the `#[coded_error]` attribute macro for errors with error codes:

```rust
use cmark_writer::coded_error;
use cmark_writer::WriteError;

// Define a coded error with error code
#[coded_error]
struct ValidationError(pub String, pub String);

// Using the error
fn validate_content(content: &str) -> Result<(), WriteError> {
    if content.contains("<script>") {
        return Err(ValidationError("Content contains unsafe script tags".to_string(), 
                                 "UNSAFE_CONTENT_ERROR".to_string()).into());
    }
    Ok(())
}
```

### 3. Custom Error Factory

For more complex errors, implement the `CustomErrorFactory` trait:

```rust
use std::fmt;
use cmark_writer::error::{CustomErrorFactory, WriteError};

struct ComplexError {
    message: String,
    line: usize,
    column: usize,
}

impl fmt::Display for ComplexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error at line {}, column {}: {}", 
               self.line, self.column, self.message)
    }
}

impl fmt::Debug for ComplexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl CustomErrorFactory for ComplexError {
    fn message(&self) -> &str {
        &self.message
    }
    
    fn construct_error(self: Box<Self>) -> WriteError {
        WriteError::CustomError(self)
    }
}

// Using the error
fn process_at_position(content: &str, line: usize, column: usize) -> Result<(), WriteError> {
    if content.is_empty() {
        let error = ComplexError {
            message: "Empty content is not allowed".to_string(),
            line,
            column,
        };
        return Err(WriteError::CustomError(Box::new(error)));
    }
    Ok(())
}
```

## Result Extensions

The library provides the `WriteResultExt` trait with useful methods for working with results:

```rust
use cmark_writer::error::WriteResultExt;

fn process_content() -> Result<(), WriteError> {
    let result = possibly_failing_operation()
        .context("Failed during content processing")?;
    
    Ok(())
}
```

## Error Conversion

The library's error types implement `From` for easy conversion:

```rust
// Converting from std::io::Error
fn write_to_file(content: &str, path: &str) -> Result<(), WriteError> {
    let mut file = std::fs::File::create(path)
        .map_err(WriteError::from)?;
    
    // More operations...
    Ok(())
}
```

## Best Practices

1. **Be specific**: Use the most specific error type for each situation
2. **Add context**: Include helpful details in error messages
3. **Propagate appropriately**: Use `?` to propagate errors when appropriate
4. **Handle gracefully**: Provide meaningful recovery options when possible
5. **Test error paths**: Ensure your error handling logic works correctly
