use cmark_writer::ast::HeadingType;
use cmark_writer::coded_error;
use cmark_writer::custom_error;
use cmark_writer::CommonMarkWriter;
use cmark_writer::Node;
use cmark_writer::WriteError;
use cmark_writer::WriteResult;
use std::fmt::{self, Display};

#[test]
fn test_invalid_heading_level() {
    let mut writer = CommonMarkWriter::new();

    // Test heading level 0 (invalid)
    let invalid_heading_0 = Node::Heading {
        level: 0,
        content: vec![Node::Text("Invalid Heading".to_string())],
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
        content: vec![Node::Text("Invalid Heading".to_string())],
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
        content: vec![Node::Text("Valid Heading".to_string())],
        heading_type: HeadingType::Atx,
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

    let structure_err = WriteError::InvalidStructure("表格结构无效".to_string());
    assert_eq!(structure_err.to_string(), "Invalid structure: 表格结构无效");

    fn takes_error(_: &dyn Error) {}
    takes_error(&custom_err);
    takes_error(&coded_err);
    takes_error(&structure_err);
}

#[test]
fn test_custom_error_attribute() {
    // 使用属性宏定义自定义错误

    #[custom_error(format = "表格列数不匹配：{}")]
    struct TableColumnMismatchError(pub &'static str);

    #[custom_error(format = "表格空表头：{}")]
    struct TableEmptyHeaderError(pub &'static str);

    #[custom_error(format = "文档格式错误：{}")]
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

    let err3 = MarkdownSyntaxError(
        "缺少闭合代码块标记".to_string(),
        "CODE_BLOCK_UNCLOSED".to_string(),
    )
    .into_error();
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

    #[custom_error(format = "解析错误：{}")]
    struct ParseError(pub &'static str);

    #[coded_error]
    struct FormatError(pub String, pub String);

    #[custom_error(format = "渲染错误：{}")]
    struct RenderError(pub &'static str);

    let err1 = ValidationError(
        "数据验证失败".to_string(),
        "DATA_VALIDATION_FAILED".to_string(),
    )
    .into_error();
    assert_eq!(
        err1.to_string(),
        "Custom error [DATA_VALIDATION_FAILED]: 数据验证失败"
    );

    let err2 = ParseError("无法解析 Markdown").into_error();
    assert_eq!(
        err2.to_string(),
        "Invalid structure: 解析错误：无法解析 Markdown"
    );

    let err3 = FormatError("格式化失败".to_string(), "FORMAT_FAILED".to_string()).into_error();
    assert_eq!(err3.to_string(), "Custom error [FORMAT_FAILED]: 格式化失败");

    let err4 = RenderError("无法渲染表格").into_error();
    assert_eq!(
        err4.to_string(),
        "Invalid structure: 渲染错误：无法渲染表格"
    );
}
