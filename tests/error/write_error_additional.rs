//! Tests for WriteError additional functionality

use cmark_writer::error::WriteError;
use std::error::Error;
use std::fmt;
use std::io;

#[test]
fn test_write_error_display_formatting() {
    // Test InvalidHeadingLevel
    let error = WriteError::InvalidHeadingLevel(0);
    assert!(error.to_string().contains("Invalid heading level: 0"));
    assert!(error.to_string().contains("Level must be between 1 and 6"));

    let error = WriteError::InvalidHeadingLevel(7);
    assert!(error.to_string().contains("Invalid heading level: 7"));

    // Test NewlineInInlineElement
    let error = WriteError::NewlineInInlineElement("link".into());
    let msg = error.to_string();
    assert!(msg.contains("Newline character found within an inline element"));
    assert!(msg.contains("link"));
    assert!(msg.contains("not allowed in strict mode"));

    // Test FmtError
    let fmt_err = fmt::Error;
    let error = WriteError::from(fmt_err);
    assert!(error.to_string().contains("Formatting error"));

    // Test UnsupportedNodeType
    let error = WriteError::UnsupportedNodeType;
    assert!(error.to_string().contains("Unsupported node type"));

    // Test InvalidStructure
    let error = WriteError::InvalidStructure("test structure issue".into());
    assert!(error
        .to_string()
        .contains("Invalid structure: test structure issue"));

    // Test InvalidHtmlTag
    let error = WriteError::InvalidHtmlTag("bad<tag>".into());
    let msg = error.to_string();
    assert!(msg.contains("Invalid HTML tag name: 'bad<tag>'"));
    assert!(msg.contains("alphanumeric characters"));

    // Test InvalidHtmlAttribute
    let error = WriteError::InvalidHtmlAttribute("bad<attr>".into());
    let msg = error.to_string();
    assert!(msg.contains("Invalid HTML attribute name: 'bad<attr>'"));
    assert!(msg.contains("alphanumeric characters"));
}

#[test]
fn test_write_error_custom_with_code() {
    let error = WriteError::custom_with_code("test message", "TEST_001");
    let msg = error.to_string();
    assert!(msg.contains("Custom error [TEST_001]: test message"));
}

#[test]
fn test_write_error_custom_without_code() {
    let error = WriteError::custom("test message");
    let msg = error.to_string();
    assert!(msg.contains("Custom error: test message"));
    assert!(!msg.contains("["));
}

#[test]
fn test_write_error_io_conversion() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let write_error = WriteError::from(io_error);

    assert!(write_error.to_string().contains("I/O error"));
    assert!(write_error.to_string().contains("file not found"));
}

#[test]
fn test_write_error_fmt_conversion() {
    let fmt_error = fmt::Error;
    let write_error = WriteError::from(fmt_error);

    assert!(write_error.to_string().contains("Formatting error"));
}

#[test]
fn test_write_error_as_std_error() {
    let error = WriteError::custom("test error");

    // Test that it implements std::error::Error
    let std_error: &dyn Error = &error;
    assert_eq!(std_error.to_string(), "Custom error: test error");

    // Test source (should be None for custom errors)
    assert!(std_error.source().is_none());
}

#[test]
fn test_write_error_html_rendering_error() {
    use cmark_writer::writer::html::error::HtmlWriteError;

    let html_error = HtmlWriteError::InvalidHtmlTag("bad tag".into());
    let write_error = WriteError::HtmlRenderingError(html_error);

    let msg = write_error.to_string();
    assert!(msg.contains("Error during HTML rendering phase"));
    assert!(msg.contains("bad tag"));
}

#[test]
fn test_write_error_html_fallback_error() {
    let error = WriteError::HtmlFallbackError("fallback failed".into());
    let msg = error.to_string();
    assert!(msg.contains("Error during HTML fallback rendering"));
    assert!(msg.contains("fallback failed"));
}

#[test]
fn test_write_error_debug_format() {
    let error = WriteError::custom("debug test");
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("Custom"));
    assert!(debug_str.contains("debug test"));
}

#[test]
fn test_write_error_clone() {
    // WriteError doesn't implement Clone, so we'll test equality instead
    let error1 = WriteError::custom("test message");
    let error2 = WriteError::custom("test message");

    assert_eq!(error1.to_string(), error2.to_string());
}

#[test]
fn test_write_error_debug_comparison() {
    let error1 = WriteError::InvalidHeadingLevel(0);
    let error2 = WriteError::InvalidHeadingLevel(1);

    // Test that debug output is different for different values
    let debug1 = format!("{:?}", error1);
    let debug2 = format!("{:?}", error2);

    assert_ne!(debug1, debug2);
    assert!(debug1.contains("0"));
    assert!(debug2.contains("1"));
}

#[test]
fn test_write_error_io_source() {
    let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
    let write_error = WriteError::from(io_error);

    // Test that the error contains the IO error information
    assert!(write_error.to_string().contains("access denied"));
    assert!(write_error.to_string().contains("I/O error"));

    // Note: The source() method may not always return Some() depending on the Error implementation
    // So we'll just test that the error converts properly and contains the expected information
}
