#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub use crate::ast::{Alignment, CustomNode, CustomNodeWriter, HtmlAttribute, HtmlElement, ListItem, Node};
pub use crate::error::{WriteError, WriteResult};
pub use crate::options::WriterOptions;
pub use crate::writer::CommonMarkWriter;

pub mod ast;
pub mod error;
pub mod options;
pub mod writer;
