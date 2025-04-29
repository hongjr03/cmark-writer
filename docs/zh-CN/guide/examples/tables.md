# 表格示例

本示例演示如何使用 cmark-writer 创建和格式化表格，包括标准表格和带有对齐选项的 GFM 启用表格。

## 基本表格示例

```rust
use cmark_writer::ast::Node;
use cmark_writer::ast::tables::TableBuilder;
use cmark_writer::writer::CommonMarkWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 使用 TableBuilder 创建一个简单的表格
    let table = TableBuilder::new()
        .headers(vec![
            Node::Text("姓名".to_string()), 
            Node::Text("年龄".to_string()), 
            Node::Text("职业".to_string())
        ])
        .add_row(vec![
            Node::Text("张三".to_string()),
            Node::Text("30".to_string()),
            Node::Text("工程师".to_string()),
        ])
        .add_row(vec![
            Node::Text("李四".to_string()),
            Node::Text("25".to_string()),
            Node::Text("设计师".to_string()),
        ])
        .add_row(vec![
            Node::Text("王五".to_string()),
            Node::Text("35".to_string()),
            Node::Text("医生".to_string()),
        ])
        .build();
    
    // 创建包含表格的文档
    let document = Node::Document(vec![
        Node::heading(1, vec![Node::Text("基本表格示例".to_string())]),
        Node::Paragraph(vec![Node::Text("这是一个简单的表格：".to_string())]),
        table,
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

这将生成以下输出：

```markdown
# 基本表格示例

这是一个简单的表格：

| 姓名 | 年龄 | 职业 |
| --- | --- | --- |
| 张三 | 30 | 工程师 |
| 李四 | 25 | 设计师 |
| 王五 | 35 | 医生 |
```

## 带对齐的 GFM 表格

当启用 `gfm` 功能时，您可以指定列对齐方式：

```rust
#[cfg(feature = "gfm")]
use cmark_writer::ast::{Node, TableAlignment};
use cmark_writer::ast::tables::TableBuilder;
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;

#[cfg(feature = "gfm")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建带有特定对齐方式的表格
    let table = TableBuilder::new()
        .headers(vec![
            Node::Text("左对齐".to_string()),
            Node::Text("居中对齐".to_string()),
            Node::Text("右对齐".to_string()),
        ])
        .alignments(vec![
            TableAlignment::Left,    // :---
            TableAlignment::Center,  // :---:
            TableAlignment::Right,   // ---:
        ])
        .add_row(vec![
            Node::Text("文本".to_string()),
            Node::Text("文本".to_string()),
            Node::Text("文本".to_string()),
        ])
        .add_row(vec![
            Node::Text("这里是较长的文本".to_string()),
            Node::Text("居中的内容".to_string()),
            Node::Text("12.34".to_string()),
        ])
        .build();
    
    // 创建包含表格的文档
    let document = Node::Document(vec![
        Node::heading(1, vec![Node::Text("带对齐的 GFM 表格".to_string())]),
        Node::Paragraph(vec![
            Node::Text("此表格使用 GFM 对齐功能：".to_string())
        ]),
        table,
    ]);
    
    // 配置启用 GFM 表格的编写器
    let options = WriterOptionsBuilder::new()
        .gfm_tables(true)
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

启用 `gfm` 功能后，将生成：

```markdown
# 带对齐的 GFM 表格

此表格使用 GFM 对齐功能：

| 左对齐 | 居中对齐 | 右对齐 |
| :--- | :---: | ---: |
| 文本 | 文本 | 文本 |
| 这里是较长的文本 | 居中的内容 | 12.34 |
```

## 高级表格格式化

您可以在表格单元格中包含格式化：

```rust
use cmark_writer::ast::{Node, ListItem};
use cmark_writer::ast::tables::TableBuilder;
use cmark_writer::writer::CommonMarkWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建带有格式化内容的表格单元格
    let table = TableBuilder::new()
        .headers(vec![
            Node::Text("项目".to_string()),
            Node::Text("描述".to_string()),
            Node::Text("状态".to_string()),
        ])
        .add_row(vec![
            Node::Text("功能 1".to_string()),
            Node::Paragraph(vec![
                Node::Text("带有 ".to_string()),
                Node::Strong(vec![Node::Text("重要".to_string())]),
                Node::Text(" 方面的基本功能".to_string()),
            ]),
            Node::Text("已完成".to_string()),
        ])
        .add_row(vec![
            Node::Text("功能 2".to_string()),
            Node::Paragraph(vec![
                Node::Text("带有 ".to_string()),
                Node::Emphasis(vec![Node::Text("专业化".to_string())]),
                Node::Text(" 组件的复杂功能".to_string()),
            ]),
            Node::Text("进行中".to_string()),
        ])
        .build();
    
    // 创建带有表格的文档
    let document = Node::Document(vec![
        Node::heading(1, vec![Node::Text("格式化表格单元格".to_string())]),
        table,
    ]);
    
    // 渲染文档
    let mut writer = CommonMarkWriter::new();
    writer.write(&document)?;
    let markdown = writer.into_string();
    
    println!("{}", markdown);
    
    Ok(())
}
```

这将生成：

```markdown
# 格式化表格单元格

| 项目 | 描述 | 状态 |
| --- | --- | --- |
| 功能 1 | 带有 **重要** 方面的基本功能 | 已完成 |
| 功能 2 | 带有 *专业化* 组件的复杂功能 | 进行中 |
```

## 表格的最佳实践

1. **保持表格简洁**：避免在表格单元格中使用过于复杂的格式
2. **保持一致性**：在所有行中使用类似的结构
3. **适当对齐数据**：使用对齐以提高可读性（数字右对齐，文本左对齐）
4. **有效使用标题**：使列标题清晰和描述性
5. **考虑宽度**：注意表格宽度，以便在不同设备上更好地呈现
