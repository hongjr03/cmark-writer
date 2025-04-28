use cmark_writer::ast::{HeadingType, Node};
use cmark_writer::writer::CommonMarkWriter;

#[test]
fn test_setext_heading() {
    // 创建一级 Setext 标题节点
    let heading_level1 = Node::Heading {
        level: 1,
        content: vec![Node::Text("这是一级 Setext 标题".to_string())],
        heading_type: HeadingType::Setext,
    };

    let mut writer = CommonMarkWriter::new();
    writer.write(&heading_level1).unwrap();

    // Setext 一级标题应该使用 = 字符作为下划线
    let expected_level1 = "这是一级 Setext 标题\n===\n";
    assert_eq!(writer.into_string(), expected_level1);

    // 创建二级 Setext 标题节点
    let heading_level2 = Node::Heading {
        level: 2,
        content: vec![Node::Text("这是二级 Setext 标题".to_string())],
        heading_type: HeadingType::Setext,
    };

    let mut writer = CommonMarkWriter::new();
    writer.write(&heading_level2).unwrap();

    // Setext 二级标题应该使用 - 字符作为下划线
    let expected_level2 = "这是二级 Setext 标题\n---\n";
    assert_eq!(writer.into_string(), expected_level2);
}

#[test]
fn test_complex_setext_heading() {
    // 创建含有多个内联元素的 Setext 标题
    let complex_heading = Node::Heading {
        level: 1,
        content: vec![
            Node::Text("带有 ".to_string()),
            Node::Emphasis(vec![Node::Text("强调".to_string())]),
            Node::Text(" 和 ".to_string()),
            Node::Strong(vec![Node::Text("加粗".to_string())]),
            Node::Text(" 的 Setext 标题".to_string()),
        ],
        heading_type: HeadingType::Setext,
    };

    let mut writer = CommonMarkWriter::new();
    writer.write(&complex_heading).unwrap();

    let expected = "带有 _强调_ 和 **加粗** 的 Setext 标题\n===\n";
    assert_eq!(writer.into_string(), expected);
}

#[test]
fn test_compare_atx_and_setext() {
    // ATX 标题
    let atx_heading = Node::Heading {
        level: 1,
        content: vec![Node::Text("ATX 形式的标题".to_string())],
        heading_type: HeadingType::Atx,
    };

    let mut writer = CommonMarkWriter::new();
    writer.write(&atx_heading).unwrap();
    let atx_result = writer.into_string();

    // Setext 标题
    let setext_heading = Node::Heading {
        level: 1,
        content: vec![Node::Text("Setext 形式的标题".to_string())],
        heading_type: HeadingType::Setext,
    };

    let mut writer = CommonMarkWriter::new();
    writer.write(&setext_heading).unwrap();
    let setext_result = writer.into_string();

    // 验证两种形式确实不同
    assert_eq!(atx_result, "# ATX 形式的标题\n");
    assert_eq!(setext_result, "Setext 形式的标题\n===\n");
}

#[test]
fn test_setext_heading_in_document() {
    // 创建一个包含多种标题的文档
    let document = Node::Document(vec![
        Node::Heading {
            level: 1,
            content: vec![Node::Text("文档标题 (ATX)".to_string())],
            heading_type: HeadingType::Atx,
        },
        Node::Paragraph(vec![Node::Text("这是一段介绍性文字。".to_string())]),
        Node::Heading {
            level: 2,
            content: vec![Node::Text("第一部分 (Setext)".to_string())],
            heading_type: HeadingType::Setext,
        },
        Node::Paragraph(vec![Node::Text(
            "这部分内容使用 Setext 风格的标题。".to_string(),
        )]),
        Node::Heading {
            level: 2,
            content: vec![Node::Text("第二部分 (ATX)".to_string())],
            heading_type: HeadingType::Atx,
        },
        Node::Paragraph(vec![Node::Text(
            "这部分内容使用 ATX 风格的标题。".to_string(),
        )]),
    ]);

    let mut writer = CommonMarkWriter::new();
    writer.write(&document).unwrap();

    let expected = "\
# 文档标题 (ATX)

这是一段介绍性文字。

第一部分 (Setext)
---

这部分内容使用 Setext 风格的标题。

## 第二部分 (ATX)

这部分内容使用 ATX 风格的标题。
";

    assert_eq!(writer.into_string(), expected);
}
