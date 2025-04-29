# Node API

`Node` 枚举是 cmark-writer 的核心类型，表示 CommonMark 文档中的各种元素。本页详细介绍了 `Node` 的变体和相关方法。

## 枚举变体

`Node` 枚举包含以下主要变体：

### 块级元素

| 变体 | 说明 |
|------|------|
| `Document(Vec<Node>)` | 文档根节点，包含子节点 |
| `Heading { level: u8, content: Vec<Node>, heading_type: HeadingType }` | 标题元素 (# 语法) |
| `Paragraph(Vec<Node>)` | 段落，包含行内子节点 |
| `BlockQuote(Vec<Node>)` | 块引用 (> 语法) |
| `CodeBlock { language: Option<String>, content: String, block_type: CodeBlockType }` | 代码块 |
| `ThematicBreak` | 分隔线 (---, ***, ___) |
| `UnorderedList(Vec<ListItem>)` | 无序列表 |
| `OrderedList { start: u32, items: Vec<ListItem> }` | 有序列表 |
| `Custom(Box<dyn CustomNode>)` | 自定义节点 |

### 行内元素

| 变体 | 说明 |
|------|------|
| `Text(String)` | 文本内容 |
| `Emphasis(Vec<Node>)` | 强调 (*斜体*) |
| `Strong(Vec<Node>)` | 加粗 (**粗体**) |
| `InlineCode(String)` | 行内代码 (`代码`) |
| `SoftBreak` | 软换行 |
| `HardBreak` | 硬换行 |
| `Link { url: String, title: Option<String>, content: Vec<Node> }` | 链接 |
| `Image { url: String, title: Option<String>, alt: Vec<Node> }` | 图片 |
| `HtmlElement(HtmlElement)` | HTML 元素 |

### GFM 扩展 (需要 `gfm` 功能)

| 变体 | 说明 |
|------|------|
| `Strikethrough(Vec<Node>)` | 删除线 (~~删除线~~) |
| `Table { ... }` | 表格 |
| `ExtendedAutolink(String)` | 扩展自动链接 |

## 常用方法

### 便捷构造函数

```rust
// 创建标题节点
pub fn heading(level: u8, content: Vec<Node>) -> Self;

// 创建代码块节点
pub fn code_block(language: Option<String>, content: String) -> Self;

// 创建表格节点 (需要 gfm 功能)
#[cfg(feature = "gfm")]
pub fn table(headers: Vec<Node>, rows: Vec<Vec<Node>>) -> Self;

// 创建带对齐的表格节点 (需要 gfm 功能)
#[cfg(feature = "gfm")]
pub fn table_with_alignment(
    headers: Vec<Node>,
    alignments: Vec<TableAlignment>,
    rows: Vec<Vec<Node>>
) -> Self;
```

## 示例用法

### 创建基本文档

```rust
use cmark_writer::ast::Node;
use cmark_writer::writer::CommonMarkWriter;

// 构建文档结构
let document = Node::Document(vec![
    Node::heading(1, vec![Node::Text("标题".to_string())]),
    Node::Paragraph(vec![
        Node::Text("带有 ".to_string()),
        Node::Strong(vec![Node::Text("粗体".to_string())]),
        Node::Text(" 文本的段落。".to_string()),
    ]),
]);

// 序列化为 CommonMark
let mut writer = CommonMarkWriter::new();
writer.write(&document).expect("写入失败");
let markdown = writer.into_string();
```

### 创建列表

```rust
use cmark_writer::ast::{Node, ListItem};

// 无序列表
let unordered_list = Node::UnorderedList(vec![
    ListItem::Unordered { 
        content: vec![Node::Paragraph(vec![Node::Text("项目 1".to_string())])] 
    },
    ListItem::Unordered { 
        content: vec![Node::Paragraph(vec![Node::Text("项目 2".to_string())])] 
    },
]);

// 有序列表
let ordered_list = Node::OrderedList {
    start: 1,
    items: vec![
        ListItem::Ordered { 
            number: None,
            content: vec![Node::Paragraph(vec![Node::Text("项目 A".to_string())])] 
        },
        ListItem::Ordered { 
            number: None,
            content: vec![Node::Paragraph(vec![Node::Text("项目 B".to_string())])] 
        },
    ],
};
```

## 相关类型

* `ListItem` - 表示列表项的枚举
* `HtmlElement` - 表示 HTML 元素的结构体
* `TableAlignment` - 表格列对齐方式的枚举
* `HeadingType` - 标题类型的枚举 (Atx: #, Setext: ===)
* `CodeBlockType` - 代码块类型的枚举 (Fenced: ```, Indented: 4 空格)
