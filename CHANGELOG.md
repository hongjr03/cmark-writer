# Changelog

All notable changes to the cmark-writer project will be documented in this file.

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
