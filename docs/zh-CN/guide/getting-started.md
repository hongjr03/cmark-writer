# 快速开始

本指南将帮助您快速上手 cmark-writer。我们将介绍安装、基本用法和一个简单的示例。

## 安装

在您的 `Cargo.toml` 中添加 cmark-writer：

```toml
[dependencies]
cmark-writer = "0.6.1"
```

如果您需要 GitHub Flavored Markdown 支持，请启用 `gfm` 功能：

```toml
[dependencies]
cmark-writer = { version = "0.6.1", features = ["gfm"] }
```

## 基本示例

这里是一个简单的示例，创建了一个带有标题和段落的 Markdown 文档：

```rust
use cmark_writer::ast::Node;
use cmark_writer::writer::CommonMarkWriter;

fn main() {
    // 创建文档
    let document = Node::Document(vec![
        Node::heading(1, vec![Node::Text("Hello CommonMark".to_string())]),
        Node::Paragraph(vec![
            Node::Text("这是一个简单的 ".to_string()),
            Node::Strong(vec![Node::Text("示例".to_string())]),
            Node::Text("。".to_string()),
        ]),
    ]);

    // 渲染为 CommonMark
    let mut writer = CommonMarkWriter::new();
    writer.write(&document).expect("Failed to write document");
    let markdown = writer.into_string();

    println!("{}", markdown);
}
```

这将生成：

```markdown
# Hello CommonMark

这是一个简单的 **示例**。
```

## 核心组件

该库由这些主要组件组成：

1. **AST 节点**（`Node` 枚举）：表示 Markdown 文档的不同元素
2. **编写器**（`CommonMarkWriter`）：将节点序列化为 CommonMark 文本
3. **选项**（`WriterOptions`）：控制格式化行为

## 下一步

要了解有关 cmark-writer 的更多信息：

- 探索[核心概念](/guide/core-concepts/index)以了解基础知识
- 尝试[示例](/guide/examples/index)以查看更复杂的用例
- 查看[API 参考](/api/index)获取详细文档
