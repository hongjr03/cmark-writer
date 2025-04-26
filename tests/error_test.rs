use cmark_writer::ast::Node;
use cmark_writer::error::{WriteError, WriteResult};
use cmark_writer::writer::CommonMarkWriter;
use std::fmt::{self, Display};

#[test]
fn test_invalid_heading_level() {
    let mut writer = CommonMarkWriter::new();

    // Test heading level 0 (invalid)
    let invalid_heading_0 = Node::Heading {
        level: 0,
        content: vec![Node::Text("Invalid Heading".to_string())],
    };
    let result = writer.write(&invalid_heading_0);
    assert!(result.is_err());

    if let Err(WriteError::InvalidHeadingLevel(level)) = result {
        assert_eq!(level, 0);
    } else {
        panic!("Expected InvalidHeadingLevel error");
    }

    // Test heading level 7 (invalid)
    let mut writer = CommonMarkWriter::new();
    let invalid_heading_7 = Node::Heading {
        level: 7,
        content: vec![Node::Text("Invalid Heading".to_string())],
    };
    let result = writer.write(&invalid_heading_7);
    assert!(result.is_err());

    if let Err(WriteError::InvalidHeadingLevel(level)) = result {
        assert_eq!(level, 7);
    } else {
        panic!("Expected InvalidHeadingLevel error");
    }

    // Test heading level 6 (valid) - should not error
    let mut writer = CommonMarkWriter::new();
    let valid_heading = Node::Heading {
        level: 6,
        content: vec![Node::Text("Valid Heading".to_string())],
    };
    assert!(writer.write(&valid_heading).is_ok());
}

#[test]
fn test_newline_in_inline_element() {
    let mut writer = CommonMarkWriter::new();

    // Test newline in text
    let text_with_newline = Node::Text("Line 1\nLine 2".to_string());
    let result = writer.write(&text_with_newline);
    assert!(result.is_err());

    match result {
        Err(WriteError::NewlineInInlineElement(context)) => {
            assert_eq!(context, "Text");
        }
        _ => panic!("Expected NewlineInInlineElement error"),
    }

    // Test newline in emphasis
    let mut writer = CommonMarkWriter::new();
    let emphasis_with_newline = Node::Emphasis(vec![Node::Text("Line 1\nLine 2".to_string())]);
    let result = writer.write(&emphasis_with_newline);
    assert!(result.is_err());

    // Test newline in strong
    let mut writer = CommonMarkWriter::new();
    let strong_with_newline = Node::Strong(vec![Node::Text("Line 1\nLine 2".to_string())]);
    let result = writer.write(&strong_with_newline);
    assert!(result.is_err());

    // Test newline in inline code
    let mut writer = CommonMarkWriter::new();
    let code_with_newline = Node::InlineCode("Line 1\nLine 2".to_string());
    let result = writer.write(&code_with_newline);
    assert!(result.is_err());
}

#[test]
fn test_unsupported_node_type() {
    // Create a mock unsupported node type by creating a custom struct
    // that implements Display like Node but is not a valid Node variant
    struct MockUnsupportedNode;

    impl Display for MockUnsupportedNode {
        fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
            Ok(())
        }
    }

    // Create an instance to use for testing
    let _mock_node = MockUnsupportedNode;

    // Test that the error message matches what we expect
    let err = WriteError::UnsupportedNodeType;
    let error_message = format!("{}", err);
    assert_eq!(
        error_message,
        "Unsupported node type encountered during writing."
    );
}

#[test]
fn test_fmt_error_conversion() {
    // Test conversion from fmt::Error to WriteError
    let fmt_error = fmt::Error;
    let write_error = WriteError::from(fmt_error);

    match write_error {
        WriteError::FmtError(_) => (), // Success
        _ => panic!("Expected FmtError variant"),
    }
}

#[test]
fn test_error_display() {
    // Test that all error types can be displayed correctly
    let errors = vec![
        WriteError::InvalidHeadingLevel(0),
        WriteError::NewlineInInlineElement("Text".to_string()),
        WriteError::FmtError("test error".to_string()),
        WriteError::UnsupportedNodeType,
    ];

    for err in errors {
        let display_str = format!("{}", err);
        assert!(
            !display_str.is_empty(),
            "Error should have non-empty display: {:?}",
            err
        );
    }
}

#[test]
fn test_error_debug() {
    // Test that all error types can be debug formatted
    let errors = vec![
        WriteError::InvalidHeadingLevel(0),
        WriteError::NewlineInInlineElement("Text".to_string()),
        WriteError::FmtError("test error".to_string()),
        WriteError::UnsupportedNodeType,
    ];

    for err in errors {
        let debug_str = format!("{:?}", err);
        assert!(!debug_str.is_empty());
    }
}

#[test]
fn test_write_result_alias() {
    // Test that WriteResult alias works correctly
    let ok_result: WriteResult<()> = Ok(());
    assert!(ok_result.is_ok());

    let err_result: WriteResult<()> = Err(WriteError::UnsupportedNodeType);
    assert!(err_result.is_err());
}
