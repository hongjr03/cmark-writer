# GFM 扩展

GitHub Flavored Markdown (GFM) 扩展了 CommonMark 规范，增加了特别适用于文档和协作的附加功能。当启用 `gfm` 功能时，cmark-writer 支持这些扩展。

## 启用 GFM 功能

要使用 GFM 功能，您需要：

1. 在 `Cargo.toml` 中为您的依赖项添加 `gfm` 功能：

    ```toml
    [dependencies]
    cmark-writer = { version = "0.6.2", features = ["gfm"] }
    ```

2. 在您的编写器中启用 GFM 选项：

    ```rust
    use cmark_writer::options::WriterOptionsBuilder;
    use cmark_writer::writer::CommonMarkWriter;

    // 启用所有 GFM 功能
    let options = WriterOptionsBuilder::new()
        .enable_gfm(true)
        .build();

    // 或启用特定的 GFM 功能
    let options = WriterOptionsBuilder::new()
        .gfm_tables(true)
        .gfm_strikethrough(true)
        .gfm_tasklists(true)
        .gfm_autolinks(true)
        .build();  // enable_gfm 自动设置为 true

    let writer = CommonMarkWriter::with_options(options);
    ```

## 带对齐的表格

GFM 表格支持使用 `:---`、`:---:` 和 `---:` 语法的列对齐。对齐可以是左对齐、居中或右对齐。

### 创建带对齐的表格

```rust
#[cfg(feature = "gfm")]
use cmark_writer::ast::{Node, TableAlignment};
use cmark_writer::writer::CommonMarkWriter;

#[cfg(feature = "gfm")]
fn create_aligned_table() {
    // 创建带有特定对齐方式的表格
    let table = Node::table_with_alignment(
        vec![
            Node::Text("左对齐".to_string()), 
            Node::Text("居中".to_string()), 
            Node::Text("右对齐".to_string())
        ],
        vec![
            TableAlignment::Left,     // :---
            TableAlignment::Center,   // :---:
            TableAlignment::Right,    // ---:
        ],
        vec![
            vec![
                Node::Text("数据 1".to_string()),
                Node::Text("数据 2".to_string()),
                Node::Text("数据 3".to_string()),
            ]
        ]
    );
    
    let mut writer = CommonMarkWriter::new();
    writer.write(&table).expect("写入表格失败");
    let markdown = writer.into_string();
    
    // 输出将包括对齐标记：
    // | 左对齐 | 居中 | 右对齐 |
    // | :--- | :----: | ----: |
    // | 数据 1 | 数据 2 | 数据 3 |
}
```

### 使用表格构建器

您也可以使用表格构建器获得更流畅的 API：

```rust
#[cfg(feature = "gfm")]
use cmark_writer::ast::tables::TableBuilder;
use cmark_writer::ast::Node;

#[cfg(feature = "gfm")]
fn build_aligned_table() {
    // 使用带对齐的 TableBuilder
    let table = TableBuilder::new()
        .headers(vec![
            Node::Text("左对齐".to_string()), 
            Node::Text("居中".to_string()), 
            Node::Text("右对齐".to_string())
        ])
        .alignments(vec![
            TableAlignment::Left,
            TableAlignment::Center,
            TableAlignment::Right,
        ])
        .add_row(vec![
            Node::Text("数据 1".to_string()),
            Node::Text("数据 2".to_string()),
            Node::Text("数据 3".to_string()),
        ])
        .build();
}
```

## 删除线

GFM 支持使用 `~~text~~` 语法的删除线文本。

```rust
#[cfg(feature = "gfm")]
use cmark_writer::ast::Node;
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;

#[cfg(feature = "gfm")]
fn strikethrough_example() {
    // 创建带有删除线文本的段落
    let paragraph = Node::Paragraph(vec![
        Node::Text("这段文本有 ".to_string()),
        Node::Strikethrough(vec![Node::Text("删除线".to_string())]),
        Node::Text(" 内容。".to_string()),
    ]);
    
    // 配置启用了 GFM 删除线的编写器
    let options = WriterOptionsBuilder::new()
        .gfm_strikethrough(true)
        .build();
    
    let mut writer = CommonMarkWriter::with_options(options);
    writer.write(&paragraph).expect("写入段落失败");
    let markdown = writer.into_string();
    
    // 输出：这段文本有 ~~删除线~~ 内容。
}
```

## 任务列表

GFM 任务列表是可以勾选或未勾选的复选框。

```rust
#[cfg(feature = "gfm")]
use cmark_writer::ast::{Node, ListItem, TaskListStatus};
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;

#[cfg(feature = "gfm")]
fn task_list_example() {
    // 创建带有任务项的无序列表
    let list = Node::UnorderedList(vec![
        // 未勾选的任务
        ListItem::Task {
            status: TaskListStatus::Unchecked,
            content: vec![Node::Paragraph(vec![
                Node::Text("未完成的任务".to_string())
            ])],
        },
        // 已勾选的任务
        ListItem::Task {
            status: TaskListStatus::Checked,
            content: vec![Node::Paragraph(vec![
                Node::Text("已完成的任务".to_string())
            ])],
        },
    ]);
    
    // 配置启用了 GFM 任务列表的编写器
    let options = WriterOptionsBuilder::new()
        .gfm_tasklists(true)
        .build();
    
    let mut writer = CommonMarkWriter::with_options(options);
    writer.write(&list).expect("写入列表失败");
    let markdown = writer.into_string();
    
    // 输出：
    // - [ ] 未完成的任务
    // - [x] 已完成的任务
}
```

## 扩展自动链接

GFM 自动检测 URL 和电子邮件地址，无需使用尖括号。

```rust
#[cfg(feature = "gfm")]
use cmark_writer::ast::Node;
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;

#[cfg(feature = "gfm")]
fn autolink_example() {
    // 创建带有扩展自动链接的段落
    let paragraph = Node::Paragraph(vec![
        Node::Text("查看 ".to_string()),
        Node::ExtendedAutolink("https://example.com".to_string()),
        Node::Text(" 获取更多信息。".to_string()),
    ]);
    
    // 配置启用了 GFM 自动链接的编写器
    let options = WriterOptionsBuilder::new()
        .gfm_autolinks(true)
        .build();
    
    let mut writer = CommonMarkWriter::with_options(options);
    writer.write(&paragraph).expect("写入段落失败");
    let markdown = writer.into_string();
    
    // 输出：查看 https://example.com 获取更多信息。
}
```

## HTML 安全性

GFM 提供额外的 HTML 安全功能，过滤掉潜在不安全的标签：

```rust
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;
use cmark_writer::ast::{Node, HtmlElement};

#[cfg(feature = "gfm")]
fn html_safety_example() {
    // 创建可能不安全的 HTML 元素
    let html = HtmlElement::new("script")
        .with_children(vec![Node::Text("alert('unsafe')".to_string())]);
    
    // 配置带有 HTML 过滤的编写器
    let options = WriterOptionsBuilder::new()
        .enable_gfm(true)
        .gfm_disallowed_html_tags(vec![
            "script".to_string(), 
            "iframe".to_string(),
            "object".to_string(),
        ])
        .build();
    
    let mut writer = CommonMarkWriter::with_options(options);
    writer.write(&Node::HtmlElement(html)).expect("写入 HTML 失败");
    let markdown = writer.into_string();
    
    // script 标签将被过滤掉或安全地转义
}
```

## 使用多个 GFM 功能

您可以在单个文档中组合多个 GFM 功能：

```rust
#[cfg(feature = "gfm")]
fn combined_gfm_example() {
    // 配置启用所有 GFM 功能的编写器
    let options = WriterOptionsBuilder::new()
        .enable_gfm(true)
        .build();
    
    let mut writer = CommonMarkWriter::with_options(options);
    
    // 现在您可以在文档中使用表格、删除线、任务列表和自动链接...
}
```
