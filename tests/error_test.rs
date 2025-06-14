use cmark_writer::ast::HeadingType;
use cmark_writer::coded_error;
use cmark_writer::structure_error;
use cmark_writer::CommonMarkWriter;
use cmark_writer::Node;
use cmark_writer::WriteError;
use cmark_writer::WriteResult;
use cmark_writer::WriterOptions;
use std::fmt::{self, Display};

#[test]
fn test_invalid_heading_level() {
    let mut writer = CommonMarkWriter::new();

    // Test heading level 0 (invalid)
    let invalid_heading_0 = Node::Heading {
        level: 0,
        content: vec![Node::Text("Invalid Heading".into())],
        heading_type: HeadingType::Atx,
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
        content: vec![Node::Text("Invalid Heading".into())],
        heading_type: HeadingType::Atx,
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
        content: vec![Node::Text("Valid Heading".into())],
        heading_type: HeadingType::Atx,
    };
    assert!(writer.write(&valid_heading).is_ok());
}

#[test]
fn test_newline_in_inline_element() {
    let mut writer = CommonMarkWriter::new();

    // Test newline in text
    let text_with_newline = Node::Text("Line 1\nLine 2".into());
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
    let emphasis_with_newline = Node::Emphasis(vec![Node::Text("Line 1\nLine 2".into())]);
    let result = writer.write(&emphasis_with_newline);
    assert!(result.is_err());

    // Test newline in strong
    let mut writer = CommonMarkWriter::new();
    let strong_with_newline = Node::Strong(vec![Node::Text("Line 1\nLine 2".into())]);
    let result = writer.write(&strong_with_newline);
    assert!(result.is_err());

    // Test newline in inline code
    let mut writer = CommonMarkWriter::new();
    let code_with_newline = Node::InlineCode("Line 1\nLine 2".into());
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
        WriteError::NewlineInInlineElement("Text".into()),
        WriteError::FmtError("test error".into()),
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
        WriteError::NewlineInInlineElement("Text".into()),
        WriteError::FmtError("test error".into()),
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

#[test]
fn test_custom_errors() {
    use cmark_writer::error::WriteError;
    use std::error::Error;

    let custom_err = WriteError::custom("这是一个自定义错误");
    assert_eq!(custom_err.to_string(), "Custom error: 这是一个自定义错误");

    let coded_err =
        WriteError::custom_with_code("表格行单元格数与表头数不匹配", "TABLE_STRUCTURE_ERROR");
    assert_eq!(
        coded_err.to_string(),
        "Custom error [TABLE_STRUCTURE_ERROR]: 表格行单元格数与表头数不匹配"
    );

    let structure_err = WriteError::InvalidStructure("表格结构无效".into());
    assert_eq!(structure_err.to_string(), "Invalid structure: 表格结构无效");

    fn takes_error(_: &dyn Error) {}
    takes_error(&custom_err);
    takes_error(&coded_err);
    takes_error(&structure_err);
}

#[test]
fn test_custom_error_attribute() {
    // 使用属性宏定义自定义错误

    #[structure_error(format = "表格列数不匹配：{}")]
    struct TableColumnMismatchError(pub &'static str);

    #[structure_error(format = "表格空表头：{}")]
    struct TableEmptyHeaderError(pub &'static str);

    #[structure_error(format = "文档格式错误：{}")]
    struct DocumentFormatError(pub &'static str);

    #[coded_error]
    struct MarkdownSyntaxError(pub String, pub String);

    let err1 = TableColumnMismatchError("第 3 行有 4 列，但表头只有 3 列").into_error();
    assert_eq!(
        err1.to_string(),
        "Invalid structure: 表格列数不匹配：第 3 行有 4 列，但表头只有 3 列"
    );

    let err2 = TableEmptyHeaderError("表格必须包含至少一个表头").into_error();
    assert_eq!(
        err2.to_string(),
        "Invalid structure: 表格空表头：表格必须包含至少一个表头"
    );

    let err3 =
        MarkdownSyntaxError("缺少闭合代码块标记".into(), "CODE_BLOCK_UNCLOSED".into()).into_error();
    assert_eq!(
        err3.to_string(),
        "Custom error [CODE_BLOCK_UNCLOSED]: 缺少闭合代码块标记"
    );

    let err4 = DocumentFormatError("文档超过最大嵌套深度").into_error();
    assert_eq!(
        err4.to_string(),
        "Invalid structure: 文档格式错误：文档超过最大嵌套深度"
    );

    let err5: WriteError = TableColumnMismatchError("错误示例").into();
    assert!(matches!(err5, WriteError::InvalidStructure(_)));
}

#[test]
fn test_mixed_order_custom_errors() {
    // 使用属性宏定义多个自定义错误，顺序混合

    #[coded_error]
    struct ValidationError(pub String, pub String);

    #[structure_error(format = "解析错误：{}")]
    struct ParseError(pub &'static str);

    #[coded_error]
    struct FormatError(pub String, pub String);

    #[structure_error(format = "渲染错误：{}")]
    struct RenderError(pub &'static str);

    let err1 = ValidationError("数据验证失败".into(), "DATA_VALIDATION_FAILED".into()).into_error();
    assert_eq!(
        err1.to_string(),
        "Custom error [DATA_VALIDATION_FAILED]: 数据验证失败"
    );

    let err2 = ParseError("无法解析 Markdown").into_error();
    assert_eq!(
        err2.to_string(),
        "Invalid structure: 解析错误：无法解析 Markdown"
    );

    let err3 = FormatError("格式化失败".into(), "FORMAT_FAILED".into()).into_error();
    assert_eq!(err3.to_string(), "Custom error [FORMAT_FAILED]: 格式化失败");

    let err4 = RenderError("无法渲染表格").into_error();
    assert_eq!(
        err4.to_string(),
        "Invalid structure: 渲染错误：无法渲染表格"
    );
}

// Helper to initialize logger for tests.
// Call this at the beginning of each test or in a common setup function if needed.
fn init_logger() {
    // Using try_init() to avoid panic if logger is already initialized,
    // which can happen if tests are run in parallel or multiple times.
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn test_invalid_heading_level_strict() {
    init_logger();
    let options = WriterOptions {
        strict: true,
        ..Default::default()
    };
    let mut writer = CommonMarkWriter::with_options(options);
    let node = Node::Heading {
        level: 0, // Invalid level
        content: vec![Node::Text("Test".into())],
        heading_type: HeadingType::Atx,
    };
    match writer.write(&node) {
        Err(WriteError::InvalidHeadingLevel(level)) => assert_eq!(level, 0),
        _ => panic!("Expected InvalidHeadingLevel error"),
    }
}

#[test]
fn test_invalid_heading_level_non_strict() {
    init_logger();
    let options = WriterOptions {
        strict: false,
        ..Default::default()
    };
    let mut writer = CommonMarkWriter::with_options(options);
    let node = Node::Heading {
        level: 0, // Invalid level
        content: vec![Node::Text("Test".into())],
        heading_type: HeadingType::Atx,
    };
    assert!(writer.write(&node).is_ok());
    // In non-strict, level 0 should be clamped to 1.
    assert_eq!(writer.into_string(), "# Test\n");
    // Manually check stderr for log: "Invalid heading level: 0. Corrected to 1..."
}

#[test]
fn test_invalid_heading_level_7_non_strict() {
    init_logger();
    let options = WriterOptions {
        strict: false,
        ..Default::default()
    };
    let mut writer = CommonMarkWriter::with_options(options);
    let node = Node::Heading {
        level: 7, // Invalid level
        content: vec![Node::Text("Test".into())],
        heading_type: HeadingType::Atx,
    };
    assert!(writer.write(&node).is_ok());
    // In non-strict, level 7 should be clamped to 6.
    assert_eq!(writer.into_string(), "###### Test\n");
    // Manually check stderr for log: "Invalid heading level: 7. Corrected to 6..."
}

#[test]
fn test_newline_in_link_text_strict() {
    init_logger();
    let options = WriterOptions {
        strict: true,
        ..Default::default()
    };
    let mut writer = CommonMarkWriter::with_options(options);
    let node = Node::Link {
        url: "http://example.com".into(),
        title: None,
        content: vec![Node::Text("Link\nText".into())], // Newline in link text
    };
    match writer.write(&node) {
        Err(WriteError::NewlineInInlineElement(context)) => assert_eq!(context, "Link content"),
        _ => panic!("Expected NewlineInInlineElement error for link text"),
    }
}

#[test]
fn test_newline_in_link_text_non_strict() {
    init_logger();
    let options = WriterOptions {
        strict: false,
        ..Default::default()
    };
    let mut writer = CommonMarkWriter::with_options(options);
    let node = Node::Link {
        url: "http://example.com".into(),
        title: None,
        content: vec![Node::Text("Link\nText".into())], // Newline in link text
    };
    assert!(writer.write(&node).is_ok());
    // Output will contain the newline as per current non-strict behavior
    assert_eq!(writer.into_string(), "[Link\nText](http://example.com)");
    // Manually check stderr for log: "Newline character found in inline element 'Link Text'..."
}

// TODO: Add test for UnsupportedNodeType if a stable way to construct/mock one exists.
// For example, if Node enum had a test-only variant:
// #[cfg(test)]
// TestOnlyUnsupported,
//
// Then you could write:
// #[test]
// fn test_unsupported_node_type_strict() {
//     init_logger();
//     let options = WriterOptions { strict: true, ..Default::default() };
//     let mut writer = CommonMarkWriter::with_options(options);
//     let node = Node::TestOnlyUnsupported; // Hypothetical
//     match writer.write(&node) {
//         Err(WriteError::UnsupportedNodeType) => { /* Expected */ }
//         _ => panic!("Expected UnsupportedNodeType error"),
//     }
// }
//
// #[test]
// fn test_unsupported_node_type_non_strict() {
//     init_logger();
//     let options = WriterOptions { strict: false, ..Default::default() };
//     let mut writer = CommonMarkWriter::with_options(options);
//     let node = Node::TestOnlyUnsupported; // Hypothetical
//     assert!(writer.write(&node).is_ok());
//     assert_eq!(writer.into_string(), ""); // Or placeholder if you decide to write one
//     // Manually check stderr for log: "Unsupported node type encountered and skipped..."
// }
