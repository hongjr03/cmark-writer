# Traits Architecture Reorganization

This document describes the reorganization of the traits system in cmark-writer.

## Previous Structure

Before reorganization, we had two separate files:
- `src/traits.rs` - Core node and processing traits
- `src/format_traits.rs` - High-level formatting traits

This caused:
- Duplicate functionality with different naming conventions
- Confusion about which traits to use
- Split related functionality across files

## New Structure

The traits are now organized in a modular structure under `src/traits/`:

```
src/traits/
├── mod.rs              # Public re-exports
├── core.rs             # Core node content and writer traits
├── formatting.rs       # All formatting and rendering traits
├── processing.rs       # Node processing traits
├── utils.rs           # Utility traits (error handling, configuration)
├── legacy.rs          # Backup of original traits.rs
└── format_traits_legacy.rs  # Backup of original format_traits.rs
```

## Key Improvements

### 1. Unified Formatting Interface

All formatting traits are now in `formatting.rs`:
- `CommonMarkRenderable` / `HtmlRenderable` - Low-level rendering
- `ToCommonMark` / `ToHtml` - High-level convenience traits  
- `Format<W>` - Generic format trait
- `MultiFormat` - Multi-format support

### 2. Consistent Naming

- Rendering methods: `render_commonmark()`, `render_html()`
- Convenience methods: `to_commonmark()`, `to_html()`
- Automatic bridge implementations between levels

### 3. Clear Separation of Concerns

- **Core**: Basic node properties and writer interface
- **Formatting**: All rendering and format-related functionality
- **Processing**: Node processing and transformation
- **Utils**: Cross-cutting concerns like error handling

### 4. Better Trait Relationships

```rust
// Automatic implementations bridge different abstraction levels
impl<T> CommonMarkRenderable for T
where
    T: ToCommonMark + NodeContent
{
    fn render_commonmark(&self, writer: &mut CommonMarkWriter) -> WriteResult<()> {
        self.to_commonmark(writer)
    }
}
```

## Migration Guide

### For Users of `format_traits`

Old code:
```rust
use cmark_writer::format_traits::{Format, ToCommonMark, ToHtml, MultiFormat};
```

New code:
```rust
use cmark_writer::traits::{Format, ToCommonMark, ToHtml, MultiFormat};
```

### For Users of Core Traits

Old code:
```rust
use cmark_writer::traits::{CustomNode, NodeContent, Writer};
```

New code: 
```rust
use cmark_writer::traits::{CustomNode, NodeContent, Writer};
// No change needed - same interface, better organization
```

## Benefits

1. **Single Import Point**: All traits available from `crate::traits`
2. **Logical Organization**: Related traits grouped together
3. **Reduced Confusion**: Clear naming and consistent interface
4. **Better Maintainability**: Modular structure easier to maintain
5. **Backward Compatibility**: Public API remains the same

## Implementation Details

The reorganization maintains full backward compatibility by:
- Preserving all public trait interfaces
- Keeping automatic implementations
- Using comprehensive re-exports in `mod.rs`
- Maintaining the same functionality

Legacy files are preserved for reference and can be removed after confirming stability.
