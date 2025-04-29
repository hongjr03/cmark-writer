# WriterOptions

The `WriterOptions` struct controls how the Markdown output is formatted by `CommonMarkWriter`. It provides a way to customize various aspects of the generated CommonMark text.

## Creating Options

The recommended way to create options is using the builder pattern:

```rust
use cmark_writer::options::WriterOptionsBuilder;

let options = WriterOptionsBuilder::new()
    .soft_break("\n")
    .hard_break("  \n")
    .list_marker('*')
    .code_fence_char('`')
    .build();
```

## Available Options

### Text Formatting

| Option | Description | Default | Example |
|--------|-------------|---------|---------|
| `soft_break` | String to use for soft line breaks | `"\n"` | `"\n"` |
| `hard_break` | String to use for hard line breaks | `"  \n"` | `"  \n"` |
| `html_escape` | Whether to escape HTML special characters | `true` | `true` |

### List Formatting

| Option | Description | Default | Example |
|--------|-------------|---------|---------|
| `list_marker` | Character to use for unordered lists | `'-'` | `'*'`, `'-'`, `'+'` |
| `ordered_list_marker` | Character to use after numbers in ordered lists | `'.'` | `'.'`, `')'` |

### Code Blocks

| Option | Description | Default | Example |
|--------|-------------|---------|---------|
| `code_fence_char` | Character to use for code fences | ``'`'`` | ``'`'``, `'~'` |
| `code_fence_length` | Number of fence characters | `3` | `3`, `4` |

### Spacing

| Option | Description | Default | Example |
|--------|-------------|---------|---------|
| `heading_spacing` | Whether to add a space after heading markers | `true` | `# Heading` vs `#Heading` |
| `list_item_spacing` | Whether to add a space after list markers | `true` | `- Item` vs `-Item` |

### Tables (GFM)

These options are available when the `gfm` feature is enabled:

| Option | Description | Default | Example |
|--------|-------------|---------|---------|
| `table_cell_padding` | Minimum number of spaces for table cell padding | `1` | `| Cell |` vs `|Cell|` |
| `align_table_pipes` | Whether to align table pipes vertically | `true` | Aligned vs non-aligned pipes |

### Advanced Options

| Option | Description | Default | Example |
|--------|-------------|---------|---------|
| `preserve_reference_definitions` | Whether to preserve reference definitions when writing | `true` | `[ref]: url` |
| `end_with_newline` | Whether to ensure output ends with a newline | `true` | Document with trailing newline |

## Usage Examples

### Customizing List Formatting

```rust
use cmark_writer::{Node, CommonMarkWriter, options::WriterOptionsBuilder};

// Create options with asterisks for unordered lists
let options = WriterOptionsBuilder::new()
    .list_marker('*')
    .build();

let list = Node::UnorderedList(vec![/* ... */]);
let mut writer = CommonMarkWriter::with_options(options);
writer.write(&list).expect("Failed to write list");

// Result will use asterisks instead of hyphens:
// * Item 1
// * Item 2
```

### Customizing Code Blocks

```rust
use cmark_writer::{Node, CommonMarkWriter, options::WriterOptionsBuilder};

// Create options with tildes for code fences
let options = WriterOptionsBuilder::new()
    .code_fence_char('~')
    .code_fence_length(4)
    .build();

let code_block = Node::code_block(Some("rust".to_string()), "fn main() {}".to_string());
let mut writer = CommonMarkWriter::with_options(options);
writer.write(&code_block).expect("Failed to write code block");

// Result will use tildes instead of backticks:
// ~~~~rust
// fn main() {}
// ~~~~
```

### GFM Table Formatting

When the `gfm` feature is enabled:

```rust
use cmark_writer::{Node, CommonMarkWriter, options::WriterOptionsBuilder};

// Create options for table formatting
let options = WriterOptionsBuilder::new()
    .table_cell_padding(2)  // More spacing in cells
    .align_table_pipes(true)
    .build();

let table = /* ... */;
let mut writer = CommonMarkWriter::with_options(options);
writer.write(&table).expect("Failed to write table");

// Result will have additional padding in table cells
// |  Header 1  |  Header 2  |
// |------------|------------|
// |  Data 1    |  Data 2    |
```

## Default Options

The default options are designed to produce clean, widely-compatible Markdown:

```rust
use cmark_writer::options::{WriterOptions, WriterOptionsBuilder};

// These are equivalent:
let default1 = WriterOptions::default();
let default2 = WriterOptionsBuilder::new().build();
```
