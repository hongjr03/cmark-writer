//! Tests for HTML error types

use cmark_writer::error::WriteError;
use cmark_writer::writer::html::error::*;
use std::error::Error;
use std::io;

#[test]
fn test_html_write_error_io() {
    let io_err = io::Error::new(io::ErrorKind::BrokenPipe, "Broken pipe");
    let html_err = HtmlWriteError::Io(io_err);

    assert!(html_err.to_string().contains("HTML I/O error"));
    assert!(html_err.to_string().contains("Broken pipe"));
}

#[test]
fn test_html_write_error_unsupported_node_type() {
    let err = HtmlWriteError::UnsupportedNodeType("CustomNode".to_string());

    assert!(err.to_string().contains("HTML conversion not supported"));
    assert!(err.to_string().contains("CustomNode"));
}

#[test]
fn test_html_write_error_invalid_structure() {
    let err = HtmlWriteError::InvalidStructure("Invalid nesting".to_string());

    assert!(err.to_string().contains("Invalid structure"));
    assert!(err.to_string().contains("Invalid nesting"));
}

#[test]
fn test_html_write_error_invalid_html_tag() {
    let err = HtmlWriteError::InvalidHtmlTag("invalid-tag".to_string());

    assert!(err.to_string().contains("Invalid HTML tag name"));
    assert!(err.to_string().contains("invalid-tag"));
}

#[test]
fn test_html_write_error_invalid_html_attribute() {
    let err = HtmlWriteError::InvalidHtmlAttribute("invalid-attr".to_string());

    assert!(err.to_string().contains("Invalid HTML attribute name"));
    assert!(err.to_string().contains("invalid-attr"));
}

#[test]
fn test_html_write_error_custom_node_error() {
    let err = HtmlWriteError::CustomNodeError("Custom error message".to_string());

    assert!(err.to_string().contains("Error writing custom node"));
    assert!(err.to_string().contains("Custom error message"));
}

#[test]
fn test_html_write_error_source() {
    let io_err = io::Error::new(io::ErrorKind::BrokenPipe, "Broken pipe");
    let html_err = HtmlWriteError::Io(io_err);

    assert!(html_err.source().is_some());

    let other_err = HtmlWriteError::UnsupportedNodeType("Test".to_string());
    assert!(other_err.source().is_none());
}

#[test]
fn test_html_write_error_into_write_error() {
    // Test Io conversion
    let io_err = io::Error::new(io::ErrorKind::BrokenPipe, "Broken pipe");
    let html_err = HtmlWriteError::Io(io_err);
    let write_err = html_err.into_write_error();

    match write_err {
        WriteError::IoError(_) => {} // Expected
        _ => panic!("Expected IoError"),
    }

    // Test UnsupportedNodeType conversion
    let html_err = HtmlWriteError::UnsupportedNodeType("TestNode".to_string());
    let write_err = html_err.into_write_error();

    match write_err {
        WriteError::Custom { message, code } => {
            assert!(message.contains("HTML writer error"));
            assert!(message.contains("TestNode"));
            assert!(code.is_none());
        }
        _ => panic!("Expected Custom error"),
    }

    // Test InvalidStructure conversion
    let html_err = HtmlWriteError::InvalidStructure("Bad structure".to_string());
    let write_err = html_err.into_write_error();

    match write_err {
        WriteError::InvalidStructure(msg) => {
            assert_eq!(msg, "Bad structure");
        }
        _ => panic!("Expected InvalidStructure"),
    }

    // Test InvalidHtmlTag conversion
    let html_err = HtmlWriteError::InvalidHtmlTag("bad-tag".to_string());
    let write_err = html_err.into_write_error();

    match write_err {
        WriteError::InvalidHtmlTag(tag) => {
            assert_eq!(tag, "bad-tag");
        }
        _ => panic!("Expected InvalidHtmlTag"),
    }

    // Test InvalidHtmlAttribute conversion
    let html_err = HtmlWriteError::InvalidHtmlAttribute("bad-attr".to_string());
    let write_err = html_err.into_write_error();

    match write_err {
        WriteError::InvalidHtmlAttribute(attr) => {
            assert_eq!(attr, "bad-attr");
        }
        _ => panic!("Expected InvalidHtmlAttribute"),
    }

    // Test CustomNodeError conversion
    let html_err = HtmlWriteError::CustomNodeError("Custom error".to_string());
    let write_err = html_err.into_write_error();

    match write_err {
        WriteError::Custom { message, code } => {
            assert!(message.contains("Custom node error"));
            assert!(message.contains("Custom error"));
            assert!(code.is_none());
        }
        _ => panic!("Expected Custom error"),
    }
}

#[test]
fn test_io_error_conversion() {
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "Permission denied");
    let html_err: HtmlWriteError = io_err.into();

    match html_err {
        HtmlWriteError::Io(err) => {
            assert_eq!(err.kind(), io::ErrorKind::PermissionDenied);
            assert!(err.to_string().contains("Permission denied"));
        }
        _ => panic!("Expected Io error"),
    }
}

#[test]
fn test_html_write_result_type() {
    let error: HtmlWriteResult<String> =
        Err(HtmlWriteError::UnsupportedNodeType("Test".to_string()));
    assert!(error.is_err());
}

#[test]
fn test_error_debug_format() {
    let err = HtmlWriteError::UnsupportedNodeType("TestNode".to_string());
    let debug_str = format!("{:?}", err);

    assert!(debug_str.contains("UnsupportedNodeType"));
    assert!(debug_str.contains("TestNode"));
}
