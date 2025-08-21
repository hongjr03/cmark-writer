//! 演示新的统一接口：to_commonmark 和 to_html

use cmark_writer::ast::Node;
use cmark_writer::writer::{CommonMarkWriter, HtmlWriter};
use cmark_writer::{ToCommonMark, ToHtml};

fn main() {
    // 创建一个示例文档
    let document = Node::Document(vec![
        Node::heading(1, vec![Node::Text("新接口演示".into())]),
        Node::Paragraph(vec![
            Node::Text("这是使用新的统一接口 ".into()),
            Node::Strong(vec![Node::Text("to_commonmark".into())]),
            Node::Text(" 和 ".into()),
            Node::Strong(vec![Node::Text("to_html".into())]),
            Node::Text(" 的示例。".into()),
        ]),
        Node::UnorderedList(vec![
            cmark_writer::ast::ListItem::Unordered {
                content: vec![Node::Paragraph(vec![Node::Text("项目 1".into())])],
            },
            cmark_writer::ast::ListItem::Unordered {
                content: vec![Node::Paragraph(vec![Node::Text("项目 2".into())])],
            },
        ]),
    ]);

    // 使用新的 to_commonmark 接口
    println!("=== CommonMark 输出 ===");
    let mut md_writer = CommonMarkWriter::new();
    document.to_commonmark(&mut md_writer).unwrap();
    println!("{}", md_writer.into_string());

    // 使用新的 to_html 接口
    println!("=== HTML 输出 ===");
    let mut html_writer = HtmlWriter::new();
    document.to_html(&mut html_writer).unwrap();
    println!("{}", html_writer.into_string());

    println!("✅ 重构成功！新接口工作正常。");
}
