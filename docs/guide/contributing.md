# Contributing Guide

We welcome contributions to cmark-writer! This guide outlines the process for contributing to the project.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** to your local machine
3. **Set up the development environment**:

   ```bash
   git clone https://github.com/YOUR-USERNAME/cmark-writer.git
   cd cmark-writer
   cargo build
   cargo test
   ```

## Development Workflow

### Making Changes

1. Create a new branch for your feature or bugfix:

   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes following the coding standards
3. Add tests for your changes where applicable
4. Update documentation as needed

### Testing

Before submitting, make sure all tests pass:

```bash
# Run all tests
cargo test

# Run specific tests
cargo test -- test_name

# Run tests with the gfm feature enabled
cargo test --features gfm
```

### Documentation

When adding new features, please update the documentation:

- Add inline code documentation with doc comments
- Update the README.md if necessary
- Consider adding examples

## Pull Request Process

1. **Push your changes** to your fork on GitHub
2. **Create a Pull Request** from your branch to the main repository
3. **Describe your changes** in the PR description, including:
   - What the PR adds or fixes
   - Any breaking changes
   - Testing approach
4. **Address review feedback** if requested

## Coding Standards

- Follow the Rust style guidelines
- Use meaningful variable and function names
- Add doc comments to public APIs
- Keep functions focused on a single responsibility
- Format code with `cargo fmt` before committing

## Bug Reports and Feature Requests

If you find a bug or have a feature request, please open an issue on GitHub:

- For bugs, include steps to reproduce, expected behavior, and actual behavior
- For feature requests, describe the feature and why it would be valuable

## License

By contributing to cmark-writer, you agree that your contributions will be licensed under the project's MIT License.
