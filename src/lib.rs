#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub use crate::ast::{
    CodeBlockType, CustomNode, CustomNodeWriter, HeadingType, HtmlAttribute, HtmlElement, ListItem,
    Node,
};
pub use crate::error::{
    CodedError, CustomErrorFactory, StructureError, WriteError, WriteResult, WriteResultExt,
};
pub use crate::options::WriterOptions;
pub use crate::writer::CommonMarkWriter;

pub mod ast;
pub mod error;
pub mod macros;
pub mod options;
pub mod writer;
