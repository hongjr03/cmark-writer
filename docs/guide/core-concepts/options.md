# Formatting Options

cmark-writer provides various options to customize the output formatting through the `WriterOptions` struct. This page explains the available options and how to use them.

## Available Options

Here are the key options available for customizing the CommonMark output:

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `strict` | `bool` | `false` | Follow CommonMark spec strictly |
| `hard_break_spaces` | `bool` | `true` | Use spaces for hard breaks (vs. backslash) |
| `indent_spaces` | `u8` | `4` | Number of spaces for indentation |
| `list_marker` | `char` | `-` | Marker character for unordered lists |
| `thematic_break_char` | `char` | `-` | Character for thematic breaks |
| `enable_gfm` | `bool` | `false` | Enable GitHub Flavored Markdown |
| `gfm_tables` | `bool` | `false` | Enable GFM tables |
| `gfm_tasklists` | `bool` | `false` | Enable GFM task lists |
| `gfm_strikethrough` | `bool` | `false` | Enable GFM strikethrough |
| `gfm_autolinks` | `bool` | `false` | Enable GFM extended autolinks |
| `gfm_disallowed_html_tags` | `Vec<String>` | Empty | HTML tags to filter out |

## Using Options

There are two ways to configure options:

### Direct Struct Initialization

Create options by directly initializing the struct:

```rust
use cmark_writer::options::WriterOptions;
use cmark_writer::writer::CommonMarkWriter;

let options = WriterOptions {
    strict: true,
    hard_break_spaces: false,
    indent_spaces: 2,
    list_marker: '*',
    ..Default::default()
};

let writer = CommonMarkWriter::with_options(options);
```

### Builder Pattern

Or use the more convenient builder pattern:

```rust
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;

let options = WriterOptionsBuilder::new()
    .strict(true)
    .hard_break_spaces(false)
    .indent_spaces(2)
    .list_marker('*')
    .build();

let writer = CommonMarkWriter::with_options(options);
```

## Option Examples

### Standard CommonMark Mode

Strict mode follows the CommonMark specification precisely:

```rust
let options = WriterOptionsBuilder::new()
    .strict(true)
    .build();
```

### Custom Formatting

Change the visual style of the output:

```rust
let options = WriterOptionsBuilder::new()
    .indent_spaces(2)           // Use 2 spaces for indentation
    .list_marker('*')           // Use * for bullet points
    .thematic_break_char('_')   // Use ___ for horizontal rules
    .build();
```

### Enabling GFM Features

Enable GitHub Flavored Markdown extensions:

```rust
let options = WriterOptionsBuilder::new()
    .gfm_tables(true)
    .gfm_strikethrough(true)
    .gfm_tasklists(true)
    .build();  // enable_gfm is automatically set to true
```

Or enable all GFM features at once:

```rust
let options = WriterOptionsBuilder::new()
    .enable_gfm(true)  // Enables all GFM features
    .build();
```

### HTML Safety

When using GFM, you can filter out potentially unsafe HTML:

```rust
let options = WriterOptionsBuilder::new()
    .enable_gfm(true)
    .gfm_disallowed_html_tags(vec![
        "script".to_string(),
        "iframe".to_string(),
        "object".to_string(),
    ])
    .build();
```

## Effect of Options

Here are some examples showing how different options affect the output:

### Hard Break Spaces

```rust
// With hard_break_spaces = true (default)
// Two spaces at end of line become a hard break
"Line with hard break  \nNext line"

// With hard_break_spaces = false
// Backslash used for hard breaks
"Line with hard break\\\nNext line"
```

### Indentation

```rust
// With indent_spaces = 4 (default)
">     Indented blockquote content"

// With indent_spaces = 2
">   Indented blockquote content"
```

### List Markers

```rust
// With list_marker = '-' (default)
"- Item 1
- Item 2"

// With list_marker = '*'
"* Item 1
* Item 2"
```
