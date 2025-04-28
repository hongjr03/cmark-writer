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

// 导出过程宏属性
pub use cmark_writer_macros::{coded_error, custom_error, custom_node};

pub mod ast;
pub mod error;
pub mod options;
pub mod writer;
