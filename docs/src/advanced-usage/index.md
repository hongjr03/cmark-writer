# Advanced Usage

This section covers more advanced topics for users who want to get the most out of cmark-writer. These features enable you to extend the library's functionality, handle errors effectively, and leverage GitHub Flavored Markdown extensions.

## Topics Covered

### [Custom Nodes](./custom-nodes.md)

Learn how to create your own custom node types with custom rendering logic, extending the library beyond its built-in functionality.

### [Error Handling](./error-handling.md)

Understand the error handling system in cmark-writer, including how to create custom error types and handle errors gracefully in your applications.

### [GFM Extensions](./gfm-extensions.md)

Explore the GitHub Flavored Markdown extensions available when the `gfm` feature is enabled, including tables with alignment, strikethrough, task lists, and extended autolinks.

## When to Use Advanced Features

Advanced features are particularly useful when:

- You need to represent document elements not covered by the standard CommonMark specification
- You're building a complex Markdown processing system with custom validation rules
- You want to ensure safety when processing user-provided content
- You need to extend cmark-writer's functionality for specific domain requirements

Each topic in this section includes practical examples and guidance on best practices.
