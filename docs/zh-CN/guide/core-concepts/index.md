# 核心概念

本节介绍了使用 cmark-writer 库时需要了解的基本概念。

## 关键组件

cmark-writer 构建在三个主要组件之上：

1. **AST 节点**：这些代表 Markdown 文档的不同元素，从基本文本到像列表和表格这样的复杂结构。

2. **编写器**：`CommonMarkWriter` 接收 AST 节点并将其序列化为符合 CommonMark 规范的文本。

3. **选项**：控制编写器的格式化行为，允许您自定义输出。

## 了解工作流程

使用 cmark-writer 的典型工作流程是：

1. 使用 `Node` 枚举构建文档结构
2. 使用所需选项配置 `CommonMarkWriter`
3. 将文档传递给编写器以生成 CommonMark 文本

```rust
// 1. 构建文档结构
let document = Node::Document(vec![
    Node::heading(1, vec![Node::Text("标题".to_string())]),
    Node::Paragraph(vec![Node::Text("内容".to_string())]),
]);

// 2. 配置编写器（本例使用默认选项）
let mut writer = CommonMarkWriter::new();

// 3. 生成 CommonMark 文本
writer.write(&document).expect("无法写入文档");
let markdown = writer.into_string();
```

## 进一步阅读

探索以下部分，了解每个核心组件的更多信息：

- [AST 节点](./ast-nodes)：了解不同的节点类型以及如何构建文档结构
- [编写器接口](./writer-interface)：了解编写器的工作方式及其功能
- [格式化选项](./options)：了解如何自定义输出
