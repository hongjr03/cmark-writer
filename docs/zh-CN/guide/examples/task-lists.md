# GFM 任务列表示例

本示例演示如何创建 GitHub Flavored Markdown 任务列表，它们本质上是可以被勾选或未勾选的复选框。

## 基本任务列表示例

```rust
#[cfg(feature = "gfm")]
use cmark_writer::ast::{Node, ListItem, TaskListStatus};
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;

#[cfg(feature = "gfm")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建带有任务列表的文档
    let document = Node::Document(vec![
        Node::heading(1, vec![Node::Text("项目任务".to_string())]),
        
        Node::Paragraph(vec![
            Node::Text("以下任务需要完成：".to_string())
        ]),
        
        // 带有任务项的无序列表
        Node::UnorderedList(vec![
            // 未勾选的任务
            ListItem::Task {
                status: TaskListStatus::Unchecked,
                content: vec![Node::Paragraph(vec![
                    Node::Text("实现功能 X".to_string())
                ])],
            },
            
            // 已勾选的任务
            ListItem::Task {
                status: TaskListStatus::Checked,
                content: vec![Node::Paragraph(vec![
                    Node::Text("编写文档".to_string())
                ])],
            },
            
            // 另一个未勾选的任务
            ListItem::Task {
                status: TaskListStatus::Unchecked,
                content: vec![Node::Paragraph(vec![
                    Node::Text("在各种平台上测试".to_string())
                ])],
            },
            
            // 嵌套任务
            ListItem::Task {
                status: TaskListStatus::Unchecked,
                content: vec![
                    Node::Paragraph(vec![Node::Text("部署到生产环境".to_string())]),
                    // 嵌套任务列表
                    Node::UnorderedList(vec![
                        ListItem::Task {
                            status: TaskListStatus::Checked,
                            content: vec![Node::Paragraph(vec![
                                Node::Text("准备暂存环境".to_string())
                            ])],
                        },
                        ListItem::Task {
                            status: TaskListStatus::Unchecked,
                            content: vec![Node::Paragraph(vec![
                                Node::Text("配置 CI/CD 流水线".to_string())
                            ])],
                        },
                    ]),
                ],
            },
        ]),
    ]);
    
    // 配置启用 GFM 任务列表的编写器
    let options = WriterOptionsBuilder::new()
        .gfm_tasklists(true)
        .build();
    
    let mut writer = CommonMarkWriter::with_options(options);
    writer.write(&document)?;
    let markdown = writer.into_string();
    
    // 打印生成的 markdown
    println!("{}", markdown);
    
    Ok(())
}

#[cfg(not(feature = "gfm"))]
fn main() {
    println!("此示例需要启用 'gfm' 功能");
}
```

启用 `gfm` 功能后，这将生成：

```markdown
# 项目任务

以下任务需要完成：

- [ ] 实现功能 X
- [x] 编写文档
- [ ] 在各种平台上测试
- [ ] 部署到生产环境
  - [x] 准备暂存环境
  - [ ] 配置 CI/CD 流水线
```

## 混合列表类型示例

您可以将任务列表项与常规列表项混合使用：

```rust
#[cfg(feature = "gfm")]
use cmark_writer::ast::{Node, ListItem, TaskListStatus};
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;

#[cfg(feature = "gfm")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建带有混合列表类型的文档
    let document = Node::Document(vec![
        Node::heading(1, vec![Node::Text("混合列表示例".to_string())]),
        
        // 带有任务和常规项的无序列表
        Node::UnorderedList(vec![
            // 常规列表项
            ListItem::Unordered {
                content: vec![Node::Paragraph(vec![
                    Node::Text("这是一个常规列表项".to_string())
                ])],
            },
            
            // 任务列表项
            ListItem::Task {
                status: TaskListStatus::Unchecked,
                content: vec![Node::Paragraph(vec![
                    Node::Text("这是一个任务列表项".to_string())
                ])],
            },
            
            // 另一个常规列表项
            ListItem::Unordered {
                content: vec![Node::Paragraph(vec![
                    Node::Text("另一个常规项".to_string())
                ])],
            },
            
            // 已完成的任务
            ListItem::Task {
                status: TaskListStatus::Checked,
                content: vec![Node::Paragraph(vec![
                    Node::Text("一个已完成的任务".to_string())
                ])],
            },
        ]),
        
        // 也适用于有序列表
        Node::heading(2, vec![Node::Text("使用有序列表".to_string())]),
        
        Node::OrderedList {
            start: 1,
            items: vec![
                // 常规有序项
                ListItem::Ordered {
                    number: None,
                    content: vec![Node::Paragraph(vec![
                        Node::Text("第一个有序项".to_string())
                    ])],
                },
                
                // 有序列表中的任务项
                ListItem::Task {
                    status: TaskListStatus::Unchecked,
                    content: vec![Node::Paragraph(vec![
                        Node::Text("有序列表中的任务".to_string())
                    ])],
                },
                
                // 另一个常规有序项
                ListItem::Ordered {
                    number: None,
                    content: vec![Node::Paragraph(vec![
                        Node::Text("另一个有序项".to_string())
                    ])],
                },
            ],
        },
    ]);
    
    // 配置启用 GFM 任务列表的编写器
    let options = WriterOptionsBuilder::new()
        .gfm_tasklists(true)
        .build();
    
    let mut writer = CommonMarkWriter::with_options(options);
    writer.write(&document)?;
    let markdown = writer.into_string();
    
    // 打印生成的 markdown
    println!("{}", markdown);
    
    Ok(())
}
```

启用 GFM 后，这将生成：

```markdown
# 混合列表示例

- 这是一个常规列表项
- [ ] 这是一个任务列表项
- 另一个常规项
- [x] 一个已完成的任务

## 使用有序列表

1. 第一个有序项
2. [ ] 有序列表中的任务
3. 另一个有序项
```

## 任务列表的最佳实践

1. **将任务列表用于可操作的项目**：任务列表最适合跟踪待办事项，而不是一般信息
2. **保持任务描述简洁**：简短、清晰的描述在任务列表中效果最佳
3. **使用嵌套来表示层级任务**：将相关的子任务归类在父任务下
4. **与其他 Markdown 元素结合使用**：任务列表描述可以包含其他格式，如强调或链接
5. **注意状态表示**：已勾选的项目通常代表已完成的任务，请一致地使用它们

## 实现注意事项

在实现任务列表时：

- 记得在 `Cargo.toml` 中启用 `gfm` 功能
- 使用 `WriterOptionsBuilder` 来启用 GFM 任务列表
- 任务列表可以嵌套在其他列表中
- 任务项在有序和无序列表中都适用
