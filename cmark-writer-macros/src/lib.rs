extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Custom node attribute macro, replaces the original derive_custom_node! macro
///
/// This macro automatically implements the CustomNode trait, users only need to implement
/// write_custom and is_block_custom methods
///
/// # Example
///
/// ```rust
/// use cmark_writer_macros::custom_node;
///
/// #[derive(Debug, Clone, PartialEq)]
/// #[custom_node]
/// struct HighlightNode {
///     content: String,
///     color: String,
/// }
///
/// impl HighlightNode {
///     fn write_custom(&self, writer: &mut dyn CustomNodeWriter) -> WriteResult<()> {
///         writer.write_str("<span style=\"background-color: ")?;
///         writer.write_str(&self.color)?;
///         writer.write_str("\">")?;
///         writer.write_str(&self.content)?;
///         writer.write_str("</span>")?;
///         Ok(())
///     }
///     
///     fn is_block_custom(&self) -> bool {
///         false // This is an inline element
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn custom_node(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        #input

        impl ::cmark_writer::ast::CustomNode for #name {
            fn write(
                &self,
                writer: &mut dyn ::cmark_writer::ast::CustomNodeWriter,
            ) -> ::cmark_writer::error::WriteResult<()> {
                self.write_custom(writer)
            }

            fn clone_box(&self) -> Box<dyn ::cmark_writer::ast::CustomNode> {
                Box::new(self.clone())
            }

            fn eq_box(&self, other: &dyn ::cmark_writer::ast::CustomNode) -> bool {
                if let Some(other) = other.as_any().downcast_ref::<Self>() {
                    self == other
                } else {
                    false
                }
            }

            fn is_block(&self) -> bool {
                self.is_block_custom()
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };

    TokenStream::from(expanded)
}

/// Custom error attribute macro, replaces the struct form errors in the original define_custom_errors! macro
///
/// # Example
///
/// ```rust
/// use cmark_writer_macros::structure_error;
///
/// #[structure_error(format = "Table column mismatch: {}")]
/// struct TableColumnMismatchError(pub &'static str);
/// ```
#[proc_macro_attribute]
pub fn structure_error(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_str = attr.to_string();
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    // Parse format attribute
    let format = if attr_str.starts_with("format") {
        let format_str = attr_str
            .replace("format", "")
            .replace("=", "")
            .trim()
            .trim_matches('"')
            .to_string();
        format_str
    } else {
        // Default error message if format not specified
        "{}".to_string()
    };

    let expanded = quote! {
        #input

        impl #name {
            pub fn new(message: &'static str) -> Self {
                Self(message)
            }

            pub fn into_error(self) -> ::cmark_writer::error::WriteError {
                let mut error_factory = ::cmark_writer::error::StructureError::new(#format);

                let arg = self.0.to_string();
                error_factory = error_factory.arg(arg);

                <::cmark_writer::error::StructureError as ::cmark_writer::error::CustomErrorFactory>::create_error(&error_factory)
            }
        }

        impl From<#name> for ::cmark_writer::error::WriteError {
            fn from(factory: #name) -> Self {
                factory.into_error()
            }
        }

        impl ::cmark_writer::error::CustomErrorFactory for #name {
            fn create_error(&self) -> ::cmark_writer::error::WriteError {
                let mut error_factory = ::cmark_writer::error::StructureError::new(#format);

                let arg = self.0.to_string();
                error_factory = error_factory.arg(arg);

                <::cmark_writer::error::StructureError as ::cmark_writer::error::CustomErrorFactory>::create_error(&error_factory)
            }
        }
    };

    TokenStream::from(expanded)
}

/// Custom coded error attribute macro, replaces the coded form errors in the original define_custom_errors! macro
///
/// # Example
///
/// ```rust
/// use cmark_writer_macros::coded_error;
///
/// #[coded_error]
/// struct MarkdownSyntaxError(pub &'static str, pub &'static str);
/// ```
#[proc_macro_attribute]
pub fn coded_error(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        #input

        impl #name {
            pub fn new(message: &str, code: &str) -> Self {
                Self(message.to_string(), code.to_string())
            }

            pub fn into_error(self) -> ::cmark_writer::error::WriteError {
                let coded_error = ::cmark_writer::error::CodedError::new(self.0, self.1);
                <::cmark_writer::error::CodedError as ::cmark_writer::error::CustomErrorFactory>::create_error(&coded_error)
            }
        }

        impl From<#name> for ::cmark_writer::error::WriteError {
            fn from(factory: #name) -> Self {
                factory.into_error()
            }
        }

        impl ::cmark_writer::error::CustomErrorFactory for #name {
            fn create_error(&self) -> ::cmark_writer::error::WriteError {
                let coded_error = ::cmark_writer::error::CodedError::new(self.0.clone(), self.1.clone());
                <::cmark_writer::error::CodedError as ::cmark_writer::error::CustomErrorFactory>::create_error(&coded_error)
            }
        }
    };

    TokenStream::from(expanded)
}
