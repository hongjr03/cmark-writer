# Changelog

All notable changes to the cmark-writer project will be documented in this file.

## [0.7.9] - 2025-07-01

### Features

- **Flexible HtmlWriterOptions in CommonMarkWriter**: Added support for custom `HtmlWriterOptions` when rendering `HtmlElement` nodes in CommonMark mode
- **Runtime HtmlWriter Options Modification**: Added methods to modify `HtmlWriter` options at runtime for greater flexibility
- **Enhanced HtmlWriterOptions API**: Added builder-style methods and convenience functions for configuring HTML rendering options

### API Changes

#### WriterOptions
- Added `html_writer_options: Option<HtmlWriterOptions>` field to `WriterOptions`
- Added `html_writer_options()` method to `WriterOptions` for setting custom HTML rendering options
- Added `html_writer_options()` method to `WriterOptionsBuilder` for builder pattern support

#### HtmlWriter
- Added `set_options()` method for updating writer options at runtime
- Added `options()` and `options_mut()` methods for accessing current options
- Added `with_modified_options()` method for creating writers with modified options using closures

#### HtmlWriterOptions
- Added `with_code_block_prefix()` and `set_code_block_prefix()` methods
- Added `with_strict()` and `set_strict()` methods
- Added `with_gfm_enabled()` and `set_gfm_enabled()` methods (GFM feature)
- Added `with_gfm_disallowed_tags()` and `set_gfm_disallowed_tags()` methods (GFM feature)

### Behavior Changes

- **Default Behavior**: When `html_writer_options` is `None`, options are automatically derived from `CommonMarkWriter` options (backward compatible)
- **Custom HTML Options**: When `html_writer_options` is `Some(options)`, the custom options are used directly for HTML element rendering
- **Runtime Flexibility**: HtmlWriter options can now be modified during rendering for dynamic behavior

### Usage Examples

```rust
// Custom HTML options in CommonMark writer
let html_options = HtmlWriterOptions::default()
    .with_strict(false)
    .with_code_block_prefix(Some("highlight-"));

let writer_options = WriterOptions::default()
    .html_writer_options(Some(html_options));

let mut writer = CommonMarkWriter::with_options(writer_options);

// Runtime modification of HtmlWriter options
let mut html_writer = HtmlWriter::new();
html_writer.set_options(HtmlWriterOptions::default().with_strict(false));

// Builder pattern with custom HTML options
let options = WriterOptionsBuilder::new()
    .html_writer_options(Some(
        HtmlWriterOptions::default()
            .with_strict(false)
            .with_code_block_prefix(Some("lang-"))
    ))
    .build();
```

### Tests

- Added comprehensive test suite for custom HTML options functionality
- Tests cover default behavior, custom options, builder pattern, and runtime modifications
- All tests pass ensuring backward compatibility and new feature correctness

## [0.7.8] - 2025-06-29

### Features

- **HTML Fallback for Tables with Block Elements**: Added intelligent HTML fallback mechanism for tables containing block-level elements in soft mode
- Tables with block-level elements (paragraphs, code blocks, lists, etc.) now automatically fall back to HTML table rendering in soft mode (`strict: false`)
- Enhanced table rendering capabilities to support complex content that cannot be represented in Markdown table syntax

### API Changes

- Added `table_contains_block_elements()` method for detecting block-level elements in table content
- Added `write_table_as_html()` and `write_table_as_html_with_alignment()` methods for HTML table fallback
- Added `HtmlFallbackError` error variant for HTML fallback rendering failures

### Behavior Changes

- **Strict Mode** (`strict: true`): Tables with block elements now fail immediately with `InvalidStructure` error
- **Soft Mode** (`strict: false`): Tables with block elements automatically fall back to HTML output while preserving full content structure
- Tables with only inline elements continue to use standard Markdown table syntax regardless of mode

### Tests

- Added comprehensive test suite for HTML fallback functionality
- Tests cover strict mode error handling, soft mode HTML fallback, and inline-only table preservation
- All existing tests continue to pass, ensuring backward compatibility

### Bug Fixes

- Addressed issue where tables containing block-level elements would fail to render properly in CommonMark format
- Related to tinymist issue #1845 regarding typlite grid rendering with complex content

## [0.7.7] - 2025-06-16

### Features

- Added `trim_paragraph_trailing_hard_breaks` option to `WriterOptions` for controlling paragraph trailing hard break behavior
- New option allows users to choose whether to preserve or remove trailing hard breaks at the end of paragraphs
- Default behavior remains unchanged (trailing hard breaks are trimmed) for backward compatibility

### API Changes

- Added `trim_paragraph_trailing_hard_breaks: bool` field to `WriterOptions` (default: `true`)
- Added `trim_paragraph_trailing_hard_breaks()` method to `WriterOptionsBuilder`

### Tests

- Added comprehensive test suite for paragraph trailing hard break behavior
- Created example demonstrating the new functionality

### Bug Fixes

- Fixed tinymist issue #1818
- now users can control this behavior via options

## [0.7.6] - 2025-06-14

### Performance Improvements

- Use EcoString instead of traditional string types for better performance
- Optimized string processing and memory allocation

### Dependencies

- Added ecow dependency

## [0.7.5] - 2025-05-23

### Changes

- Refactored text escaping logic to use a trait-based approach which has better performance.

## [0.7.4] - 2025-05-23

### Changes

- Make `options` field of `CommonMarkWriter` and `HtmlWriter` public.

## [0.7.3] - 2025-05-19

### Changes

- Refactored HtmlElement writing logic to delegate to HtmlWriter.

## [0.7.2] - 2025-05-19

### Bug Fixes

- Fixed wrong behavior of HTML self-closing tag writing.

## [0.7.1] - 2025-05-19

### Changes

- Refactored HtmlWriter.

### Bug Fixes

- Fixed HTML attribute value escaping in CommonMarkWriter

### Documentation

- Updated README.md to include examples of new custom macros

## [0.7.0] - 2025-05-19

### Features

- Added HTML writer functionality for more flexible HTML output
- Added `HtmlRenderOptions` for controlling HTML rendering
- Improved error handling mechanism with more detailed error messages

### Improvements

- Enhanced test cases with warning tests
- Removed outdated documentation

## [0.6.3] - 2025-04-30

### Features

- Added new `escape_special_chars` option to WriterOptions to control text escaping (default is false)
- Enhanced text handling with optional character escaping

## [0.6.2] - 2025-04-30

### Features

- Added support for custom node type matching with `is_custom_type` and `as_custom_type` helper methods
- Enhanced `CustomNode` trait with `type_name` method for better type identification
- Updated `custom_node` attribute macro to support `block=true/false` parameter for simpler node definition
- Added `as_any_mut` method to `CustomNode` trait for mutable access to custom nodes

### Documentation

- Added documentation for custom node pattern matching in README and guide
- Created example tests for custom node pattern matching

## [0.6.1] - 2025-04-28

### Features

- Added GitHub Flavored Markdown (GFM) support with strikethrough, task lists, and tables

### Bug Fixes

- Corrected import path for safe_html in GFM HTML handling
- Removed default attribute from Node enum and implemented Default manually

### Documentation

- Removed ListItem description from Core Types section in README

### Changes

- Refactored error handling into a single file
- Updated version to 0.6.1 in Cargo.toml and Cargo.lock

### CI

- Enhanced CI workflow with multi-Rust version support and additional linting steps
- Added Codecov token and enabled failure on coverage errors

## [0.6.0] - 2025-04-28

### Features

- Added CodeBlockType and HeadingType enums for better type safety
- Exported CodeBlockType and HeadingType in public API
- Added `Node::heading` and `Node::code_block` methods to create nodes with default styles
- Improved WriteError with custom error handling and structure error support
- Implemented procedural macros for all CommonMark nodes
- Added custom error macro for simplified error handling
- Added LinkReferenceDefinition, ReferenceLink and Autolink nodes
- Implemented full CommonMark specification compliance
- Removed non-standard CommonMark elements

### Documentation

- Updated README to include examples of new custom macros
- Improved documentation with more idiomatic heading function examples

### Changes

- Refactored codebase to use procedural macros instead of manual structure construction
- Updated dependencies in Cargo files
- Updated project version to 0.6.0

## [0.5.0] - 2025-04-27

### Features

- Implemented custom node macros and refactored project architecture

### Documentation

- Updated README.md, removed main function and improved example clarity

## [0.4.0] - 2025-04-26

### Features

- Added support for custom nodes in AST
- Implemented writing logic for custom nodes in CommonMark

### Changes

- Updated project version to 0.4.0

## [0.3.0] - 2025-04-26

### Features

- Refactored AST structure to unify node types
- Enhanced error handling in CommonMark writer

### Tests

- Added comprehensive tests for error handling and writing functionality

### Documentation

- Updated documentation to use README format

## [0.2.0] - 2025-04-25

### Features

- Added error handling mechanisms
- Added formatting options for CommonMark writer

### Documentation

- Reordered imports for clarity in README.md
- Updated README.md content

## [0.1.5] - 2025-04-25

### Features

- Simplified paragraph writing logic

## [0.1.4] - 2025-04-24

### Features

- Refactored AST structure
- Updated tests to use BlockNode and InlineNode

## [0.1.3] - 2025-04-24

### Features

- ~~Updated write_inline method to add space between inline nodes~~ [Deleted]

## [0.1.2] - 2025-04-24

### Features

- Added Inline node type
- Added corresponding write method in CommonMarkWriter

## [0.1.1] - 2025-04-24

### Features

- Added support for strikethrough text
- Added support for custom HTML elements in CommonMarkWriter

### Changes

- Added package metadata to Cargo.toml

## [0.1.0] - 2025-04-22

### Features

- Initial release of cmark-writer (renamed from cmark-rs)

### Bug Fixes

- Added newline checks to CommonMarkWriter and corresponding tests
- Enhanced code block fence detection in CommonMarkWriter
- Updated references from cmark-rs to cmark-writer in documentation and examples
- Fixed inline and list indents

### Build

- Set up CI pipeline
- Applied clippy and fmt fixes
