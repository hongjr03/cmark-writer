# 基础文档示例

本示例演示如何创建包含标题、段落、列表和格式化等常见元素的完整 Markdown 文档。

## 完整示例

```rust
use cmark_writer::ast::{Node, ListItem};
use cmark_writer::writer::CommonMarkWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建包含各种元素的文档
    let document = Node::Document(vec![
        // 一级标题
        Node::heading(1, vec![Node::Text("示例文档".to_string())]),
        
        // 带有混合格式的段落
        Node::Paragraph(vec![
            Node::Text("这是一个包含 ".to_string()),
            Node::Strong(vec![Node::Text("粗体".to_string())]),
            Node::Text(" 和 ".to_string()),
            Node::Emphasis(vec![Node::Text("斜体".to_string())]),
            Node::Text(" 文本的段落。它还包含一个 ".to_string()),
            Node::Link {
                url: "https://example.com".to_string(),
                title: Some("示例网站".to_string()),
                content: vec![Node::Text("链接".to_string())],
            },
            Node::Text("。".to_string()),
        ]),
        
        // 块引用
        Node::BlockQuote(vec![
            Node::Paragraph(vec![
                Node::Text("这是一个带有嵌套 ".to_string()),
                Node::Emphasis(vec![Node::Text("强调".to_string())]),
                Node::Text(" 短语的块引用。".to_string()),
            ]),
        ]),
        
        // 带有语言的代码块
        Node::CodeBlock {
            language: Some("rust".to_string()),
            content: "fn main() {\n    println!(\"Hello, world!\");\n}".to_string(),
            block_type: cmark_writer::ast::CodeBlockType::Fenced,
        },
        
        // 分隔线（水平线）
        Node::ThematicBreak,
        
        // 二级标题
        Node::heading(2, vec![Node::Text("列表示例".to_string())]),
        
        // 无序列表
        Node::UnorderedList(vec![
            ListItem::Unordered { 
                content: vec![Node::Paragraph(vec![Node::Text("第一项".to_string())])] 
            },
            ListItem::Unordered { 
                content: vec![
                    Node::Paragraph(vec![Node::Text("带有子列表的第二项".to_string())]),
                    Node::UnorderedList(vec![
                        ListItem::Unordered { 
                            content: vec![Node::Paragraph(vec![Node::Text("子项 1".to_string())])] 
                        },
                        ListItem::Unordered { 
                            content: vec![Node::Paragraph(vec![Node::Text("子项 2".to_string())])] 
                        },
                    ]),
                ] 
            },
            ListItem::Unordered { 
                content: vec![Node::Paragraph(vec![Node::Text("第三项".to_string())])] 
            },
        ]),
        
        // 二级标题
        Node::heading(2, vec![Node::Text("有序列表示例".to_string())]),
        
        // 有序列表
        Node::OrderedList {
            start: 1,
            items: vec![
                ListItem::Ordered { 
                    number: None,
                    content: vec![Node::Paragraph(vec![Node::Text("第一个有序项".to_string())])] 
                },
                ListItem::Ordered { 
                    number: None,
                    content: vec![Node::Paragraph(vec![Node::Text("第二个有序项".to_string())])] 
                },
                ListItem::Ordered { 
                    number: None,
                    content: vec![Node::Paragraph(vec![Node::Text("第三个有序项".to_string())])] 
                },
            ],
        },
        
        // 带有行内代码的段落
        Node::Paragraph(vec![
            Node::Text("您还可以在段落中包含 ".to_string()),
            Node::InlineCode("行内代码".to_string()),
            Node::Text("。".to_string()),
        ]),
        
        // 图片示例
        Node::Paragraph(vec![
            Node::Image {
                url: "https://example.com/image.jpg".to_string(),
                title: Some("示例图片".to_string()),
                alt: vec![Node::Text("一个示例图片".to_string())],
            },
        ]),
    ]);
    
    // 创建编写器并渲染文档
    let mut writer = CommonMarkWriter::new();
    writer.write(&document)?;
    let markdown = writer.into_string();
    
    // 打印生成的 markdown
    println!("{}", markdown);
    
    Ok(())
}
```

## 输出

上面的代码生成以下 Markdown：

````markdown
# 示例文档

这是一个包含 **粗体** 和 *斜体* 文本的段落。它还包含一个 [链接](https://example.com "示例网站")。

> 这是一个带有嵌套 *强调* 短语的块引用。

```rust
fn main() {
    println!("Hello, world!");
}
```

---

## 列表示例

- 第一项
- 带有子列表的第二项
  - 子项 1
  - 子项 2
- 第三项

## 有序列表示例

1. 第一个有序项
2. 第二个有序项
3. 第三个有序项

您还可以在段落中包含 `行内代码`。

![一个示例图片](https://example.com/image.jpg "示例图片")

````

## 要点

- 根 `Document` 节点作为所有其他节点的容器
- 块级元素（标题、段落、列表）可以包含行内元素
- 嵌套结构如列表或块引用通过节点嵌套表示
- 编写器根据 CommonMark 规则处理适当的缩进和格式化
- 对于行内格式化，将文本节点包装在适当的容器中（Strong、Emphasis 等）

## 变体

您可以使用编写器选项自定义输出格式：

```rust
use cmark_writer::options::WriterOptionsBuilder;

// 自定义格式化行为
let options = WriterOptionsBuilder::new()
    .list_marker('*')          // 使用 * 作为无序列表的标记
    .thematic_break_char('_')  // 使用 ___ 作为水平线
    .build();

let writer = CommonMarkWriter::with_options(options);
```
