use cmark_writer::ast::HeadingType;
use cmark_writer::coded_error;
use cmark_writer::custom_error;
use cmark_writer::custom_node;
use cmark_writer::CommonMarkWriter;
use cmark_writer::WriteResult;
use cmark_writer::{CustomNodeWriter, Node};

// 使用属性宏定义自定义错误
#[custom_error(format = "表格行列不匹配：{}")]
pub struct TableRowColumnMismatchError(pub &'static str);

#[custom_error(format = "表格空表头：{}")]
pub struct TableEmptyHeaderError(pub &'static str);

#[coded_error]
pub struct TableAlignmentError(pub String, pub String);

// 一个简单的自定义节点示例：表示高亮文本
#[derive(Debug, PartialEq, Clone)]
#[custom_node]
struct HighlightNode {
    content: String,
    color: String,
}

// 实现 HighlightNode 所需的方法
impl HighlightNode {
    fn write_custom(&self, writer: &mut dyn CustomNodeWriter) -> WriteResult<()> {
        // 实现自定义写入逻辑
        writer.write_str("<span style=\"background-color: ")?;
        writer.write_str(&self.color)?;
        writer.write_str("\">")?;
        writer.write_str(&self.content)?;
        writer.write_str("</span>")?;
        Ok(())
    }

    fn is_block_custom(&self) -> bool {
        false // 这是一个内联元素
    }
}

// 自定义块级节点实现示例
#[derive(Debug, PartialEq, Clone)]
#[custom_node]
struct CalloutNode {
    title: String,
    content: String,
    style: String, // 例如：note, warning, danger
}

// 实现 CalloutNode 所需的方法
impl CalloutNode {
    fn write_custom(&self, writer: &mut dyn CustomNodeWriter) -> WriteResult<()> {
        writer.write_str("<div class=\"callout callout-")?;
        writer.write_str(&self.style)?;
        writer.write_str("\">\n")?;

        writer.write_str("  <h4>")?;
        writer.write_str(&self.title)?;
        writer.write_str("</h4>\n")?;

        writer.write_str("  <p>")?;
        writer.write_str(&self.content)?;
        writer.write_str("</p>\n")?;

        writer.write_str("</div>")?;
        Ok(())
    }

    fn is_block_custom(&self) -> bool {
        true // 这是一个块级元素
    }
}

#[test]
fn test_highlight_node_attribute() {
    let mut writer = CommonMarkWriter::new();
    let highlight = Node::Custom(Box::new(HighlightNode {
        content: "Highlighted text".to_string(),
        color: "yellow".to_string(),
    }));

    writer.write(&highlight).unwrap();
    assert_eq!(
        writer.into_string(),
        "<span style=\"background-color: yellow\">Highlighted text</span>"
    );
}

#[test]
fn test_callout_block_attribute() {
    let mut writer = CommonMarkWriter::new();
    let callout = Node::Custom(Box::new(CalloutNode {
        title: "Important note".to_string(),
        content: "This is an important message.".to_string(),
        style: "warning".to_string(),
    }));

    writer.write(&callout).unwrap();
    let expected = "<div class=\"callout callout-warning\">\n  <h4>Important note</h4>\n  <p>This is an important message.</p>\n</div>\n";
    assert_eq!(writer.into_string(), expected);
}

#[test]
fn test_custom_errors_attribute() {
    let err1 = TableRowColumnMismatchError("第 3 行有 4 列，但表头只有 3 列").into_error();
    assert_eq!(
        err1.to_string(),
        "Invalid structure: 表格行列不匹配：第 3 行有 4 列，但表头只有 3 列"
    );

    let err2 = TableEmptyHeaderError("表格必须包含至少一个表头").into_error();
    assert_eq!(
        err2.to_string(),
        "Invalid structure: 表格空表头：表格必须包含至少一个表头"
    );

    let err3 = TableAlignmentError(
        "无效的表格对齐方式".to_string(),
        "INVALID_ALIGNMENT".to_string(),
    )
    .into_error();
    assert_eq!(
        err3.to_string(),
        "Custom error [INVALID_ALIGNMENT]: 无效的表格对齐方式"
    );
}

// 演示在文档中使用多个自定义节点
#[test]
fn test_multiple_custom_nodes_in_document() {
    let mut writer = CommonMarkWriter::new();

    // 创建包含多种自定义节点的文档
    let document = Node::Document(vec![
        Node::Heading {
            level: 1,
            content: vec![Node::Text("使用属性宏的示例文档".to_string())],
            heading_type: HeadingType::Atx,
        },
        Node::Paragraph(vec![
            Node::Text("这是一个包含".to_string()),
            Node::Custom(Box::new(HighlightNode {
                content: "高亮文本".to_string(),
                color: "yellow".to_string(),
            })),
            Node::Text("的段落。".to_string()),
        ]),
        Node::Custom(Box::new(CalloutNode {
            title: "重要提示".to_string(),
            content: "使用属性宏可以简化代码并提高可读性。".to_string(),
            style: "info".to_string(),
        })),
        Node::Paragraph(vec![Node::Text("文档结束。".to_string())]),
    ]);

    writer.write(&document).unwrap();

    let expected = "# 使用属性宏的示例文档\n\n这是一个包含<span style=\"background-color: yellow\">高亮文本</span>的段落。\n\n<div class=\"callout callout-info\">\n  <h4>重要提示</h4>\n  <p>使用属性宏可以简化代码并提高可读性。</p>\n</div>\n\n文档结束。\n";
    assert_eq!(writer.into_string(), expected);
}
