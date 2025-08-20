extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parse, parse::ParseStream, parse_macro_input, DeriveInput, Ident, LitBool, Token,
};

/// Parse custom_node attribute parameters
struct CustomNodeArgs {
    kind: Option<String>, // "block", "inline", "replaced", "void"
    html_impl: Option<bool>,
}

impl Parse for CustomNodeArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut kind = None;
        let mut html_impl = None;

        if input.is_empty() {
            return Ok(CustomNodeArgs { kind, html_impl });
        }

        loop {
            if input.is_empty() {
                break;
            }

            let ident: Ident = input.parse()?;

            if ident == "kind" {
                let _: Token![=] = input.parse()?;
                let value: syn::LitStr = input.parse()?;
                kind = Some(value.value());
            } else if ident == "html_impl" {
                let _: Token![=] = input.parse()?;
                let value: LitBool = input.parse()?;
                html_impl = Some(value.value);
            } else {
                return Err(syn::Error::new_spanned(
                    ident,
                    "Unknown attribute parameter. Use 'kind' or 'html_impl'",
                ));
            }

            // Handle optional comma separator
            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            }
        }

        Ok(CustomNodeArgs { kind, html_impl })
    }
}

/// Custom node attribute macro for implementing the new CustomNode trait architecture
///
/// This macro automatically implements the new CustomNode trait along with required
/// base traits (NodeContent, NodeClone, CommonMarkRenderable). Users can specify
/// whether the node is a block element using the `block` parameter and whether
/// it implements HTML rendering with the `html_impl` parameter.
///
/// # Example
///
/// ```rust
/// use cmark_writer_macros::custom_node;
/// use ecow::EcoString;
///
/// // Specified as an inline element with both CommonMark and HTML implementations
/// #[derive(Debug, Clone, PartialEq)]
/// #[custom_node(kind="inline", html_impl=true)]
/// struct HighlightNode {
///     content: EcoString,
///     color: EcoString,
/// }
///
/// impl HighlightNode {
///     // Required for CommonMark rendering
///     fn write_custom(&self, writer: &mut CommonMarkWriter) -> WriteResult<()> {
///         writer.write_str("<span style=\"background-color: ")?;
///         writer.write_str(&self.color)?;
///         writer.write_str("\">")?;
///         writer.write_str(&self.content)?;
///         writer.write_str("</span>")?;
///         Ok(())
///     }
///     
///     // Optional HTML rendering implementation
///     fn write_html_custom(&self, writer: &mut HtmlWriter) -> HtmlWriteResult<()> {
///         writer.start_tag("span")?;
///         writer.attribute("style", &format!("background-color: {}", self.color))?;
///         writer.finish_tag()?;
///         writer.text(&self.content)?;
///         writer.end_tag("span")?;
///         Ok(())
///     }
/// }
///
/// // Only CommonMark implementation, default HTML implementation
/// #[derive(Debug, Clone, PartialEq)]
/// #[custom_node(kind="block")]
/// struct AlertNode {
///     content: EcoString,
/// }
///
/// impl AlertNode {
///     fn write_custom(&self, writer: &mut CommonMarkWriter) -> WriteResult<()> {
///         writer.write_str("<div class=\"alert\">")?;
///         writer.write_str(&self.content)?;
///         writer.write_str("</div>")?;
///         Ok(())
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn custom_node(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(attr as CustomNodeArgs);
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    // Configure is_block implementation based on NodeKind
    let is_block_impl = if let Some(kind_str) = &args.kind {
        let node_kind = match kind_str.as_str() {
            "block" => quote! { ::cmark_writer::NodeKind::Block },
            "inline" => quote! { ::cmark_writer::NodeKind::Inline },
            "replaced" => quote! { ::cmark_writer::NodeKind::Replaced },
            "void" => quote! { ::cmark_writer::NodeKind::Void },
            _ => {
                return syn::Error::new_spanned(
                    name,
                    "Invalid kind. Use 'block', 'inline', 'replaced', or 'void'",
                )
                .to_compile_error()
                .into()
            }
        };
        quote! {
            fn is_block(&self) -> bool {
                #node_kind.is_block()
            }
        }
    } else {
        quote! {
            fn is_block(&self) -> bool {
                self.is_block_custom()
            }
        }
    };

    // Configure html_render implementation
    let html_render_impl = if args.html_impl.unwrap_or(false) {
        // When html_impl=true, expect user to implement write_html_custom method
        quote! {
            fn html_render(
                &self,
                writer: &mut ::cmark_writer::writer::HtmlWriter,
            ) -> ::cmark_writer::error::WriteResult<()> {
                self.write_html_custom(writer).map_err(::cmark_writer::error::WriteError::from)
            }
        }
    } else {
        // When html_impl is not set or false, use default implementation
        quote! {
            fn html_render(
                &self,
                writer: &mut ::cmark_writer::writer::HtmlWriter,
            ) -> ::cmark_writer::error::WriteResult<()> {
                use ::cmark_writer::traits::NodeContent;
                writer.raw_html(&format!(
                    "<!-- HTML rendering not implemented for Custom Node: {} -->",
                    self.type_name()
                )).map_err(::cmark_writer::error::WriteError::from)
            }
        }
    };

    let expanded = quote! {
        #input

        // Implement NodeContent trait
        impl ::cmark_writer::traits::NodeContent for #name {
            #is_block_impl

            fn type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }

        // Implement NodeClone trait
        impl ::cmark_writer::traits::NodeClone for #name {
            fn clone_box(&self) -> Box<dyn ::cmark_writer::traits::NodeContent> {
                Box::new(self.clone())
            }

            fn eq_box(&self, other: &dyn ::cmark_writer::traits::NodeContent) -> bool {
                if let Some(other) = other.as_any().downcast_ref::<Self>() {
                    self == other
                } else {
                    false
                }
            }
        }

        // Implement CommonMarkRenderable trait
        impl ::cmark_writer::traits::CommonMarkRenderable for #name {
            fn render_commonmark(
                &self,
                writer: &mut ::cmark_writer::writer::CommonMarkWriter,
            ) -> ::cmark_writer::error::WriteResult<()> {
                self.write_custom(writer)
            }
        }

        // Implement CustomNode trait
        impl ::cmark_writer::traits::CustomNode for #name {
            #html_render_impl

            fn supports_capability(&self, capability: &str) -> bool {
                match capability {
                    "commonmark" => true,
                    "html" => true, // Always true for macro-generated nodes
                    _ => false,
                }
            }
        }

        impl #name {
            pub fn matches(node: &dyn ::cmark_writer::traits::CustomNode) -> bool {
                use ::cmark_writer::traits::NodeContent;
                node.type_name() == std::any::type_name::<#name>() ||
                    node.as_any().downcast_ref::<#name>().is_some()
            }

            pub fn extract(node: Box<dyn ::cmark_writer::traits::CustomNode>) -> Option<#name> {
                use ::cmark_writer::traits::NodeContent;
                node.as_any().downcast_ref::<#name>().map(|n| n.clone())
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
