# AST 节点

抽象语法树 (AST) 节点是您的 Markdown 文档的基础构建块。`Node` 枚举表示 CommonMark 文档中所有可能的元素。

## 节点类型

`Node` 枚举包含不同 Markdown 元素的各种变体：

### 块级元素

这些元素构成了文档的结构：

| 节点变体 | 描述 | 示例 |
|--------------|-------------|---------|
| `Document` | 所有内容的根容器 | 整个 Markdown 文档 |
| `Heading` | 章节标题（级别 1-6） | `# 标题` |
| `Paragraph` | 文本段落 | 普通文本块 |
| `BlockQuote` | 引用内容 | `> 引用文本` |
| `CodeBlock` | 带有可选语言的代码块 | ````rust` |
| `ThematicBreak` | 水平分割线 | `---` |
| `OrderedList` | 有序列表 | `1. 项目` |
| `UnorderedList` | 无序列表 | `- 项目` |
| `Table` | 表格数据 | 数据表 |

### 行内元素

这些元素出现在块级元素内：

| 节点变体 | 描述 | 示例 |
|--------------|-------------|---------|
| `Text` | 纯文本内容 | 普通文本 |
| `Emphasis` | 强调文本 | `*斜体*` |
| `Strong` | 高度强调文本 | `**粗体**` |
| `InlineCode` | 文本中的代码 | `` `代码` `` |
| `Link` | 超链接 | `[文本](url)` |
| `Image` | 图片引用 | `![替代文本](src)` |
| `HardBreak` | 硬换行 | `\\` 或两个空格 |
| `SoftBreak` | 软换行 | 简单的换行 |
| `HtmlElement` | 行内 HTML | `<span>` |

### GFM 扩展

当启用 `gfm` 功能时：

| 节点变体 | 描述 | 示例 |
|--------------|-------------|---------|
| `Strikethrough` | 删除线文本 | `~~文本~~` |
| 带对齐方式的 `Table` | 带列对齐的表格 | 左/中/右对齐列 |
| 任务列表项 | 可勾选的项目 | `- [ ]` 或 `- [x]` |
| `ExtendedAutolink` | 自动检测链接 | 无需尖括号的 URL |

## 创建节点

您可以通过多种方式创建节点：

### 直接枚举构造

```rust
// 直接使用枚举变体
let heading = Node::Heading {
    level: 1,
    content: vec![Node::Text("标题".to_string())],
    heading_type: HeadingType::Atx,
};
```

### 便捷方法

```rust
// 使用便捷方法
let heading = Node::heading(1, vec![Node::Text("标题".to_string())]);
```

### 构建复杂结构

节点可以嵌套以创建复杂的文档：

```rust
let document = Node::Document(vec![
    Node::heading(1, vec![Node::Text("文档标题".to_string())]),
    Node::Paragraph(vec![
        Node::Text("这是一个包含 ".to_string()),
        Node::Strong(vec![Node::Text("粗体".to_string())]),
        Node::Text(" 和 ".to_string()),
        Node::Emphasis(vec![Node::Text("斜体".to_string())]),
        Node::Text(" 文本的段落。".to_string()),
    ]),
    Node::UnorderedList(vec![
        ListItem::Unordered { 
            content: vec![Node::Paragraph(vec![Node::Text("项目 1".to_string())])] 
        },
        ListItem::Unordered { 
            content: vec![Node::Paragraph(vec![Node::Text("项目 2".to_string())])] 
        },
    ]),
]);
```

## 使用列表项

列表项有自己的类型以支持不同的列表样式：

```rust
// 无序列表项
let unordered_item = ListItem::Unordered {
    content: vec![Node::Paragraph(vec![Node::Text("项目符号".to_string())])],
};

// 有序列表项
let ordered_item = ListItem::Ordered {
    number: Some(1), // 可选的显式编号
    content: vec![Node::Paragraph(vec![Node::Text("编号项目".to_string())])],
};

// GFM 任务列表项（使用 gfm 功能）
#[cfg(feature = "gfm")]
let task_item = ListItem::Task {
    status: TaskListStatus::Unchecked,
    content: vec![Node::Paragraph(vec![Node::Text("要做的任务".to_string())])],
};
```
