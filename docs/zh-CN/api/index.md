# API 参考

本节提供 cmark-writer 库的 API 参考文档，详细介绍了主要类型、结构体和方法。

## 核心组件

cmark-writer 的 API 主要由以下核心组件组成：

### Node

`Node` 枚举是构成 Markdown 文档的基础构建块。它表示各种类型的 CommonMark 元素，如段落、标题、列表等。

[查看 Node 文档](./node)

### CommonMarkWriter

`CommonMarkWriter` 是负责将 AST 节点序列化为 CommonMark 文本的主要组件。它提供了用于生成 Markdown 输出的核心功能。

[查看 CommonMarkWriter 文档](./writer)

### WriterOptions

`WriterOptions` 结构体控制 Markdown 输出的格式化行为。通过这些选项，您可以自定义输出的各个方面。

[查看 WriterOptions 文档](./options)

## 使用模式

以下是使用 cmark-writer API 的常见模式：

1. **构建文档结构**：使用 `Node` 枚举及其变体创建 AST
2. **配置编写器**：使用 `WriterOptions` 设置格式化首选项
3. **生成输出**：使用 `CommonMarkWriter` 将 AST 转换为 Markdown 文本

```rust
use cmark_writer::ast::Node;
use cmark_writer::writer::CommonMarkWriter;
use cmark_writer::options::WriterOptionsBuilder;

// 1. 构建文档
let document = Node::Document(vec![
    Node::heading(1, vec![Node::Text("标题".to_string())]),
    Node::Paragraph(vec![Node::Text("内容".to_string())]),
]);

// 2. 配置编写器
let options = WriterOptionsBuilder::new()
    .list_marker('*')
    .build();
let mut writer = CommonMarkWriter::with_options(options);

// 3. 生成输出
writer.write(&document).expect("写入失败");
let markdown = writer.into_string();
```

有关每个组件的详细文档，请参阅相应的子页面。
