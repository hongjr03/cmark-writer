# 介绍

**cmark-writer** 是一个用于将 CommonMark AST 节点序列化为 CommonMark 格式的 Rust 库。它提供了一个完整的实现，用于以编程方式编写 Markdown 文档，并完全符合 [CommonMark 规范](https://spec.commonmark.org/)。

## 特性

- **符合规范**：符合 CommonMark 规范，提供可靠的输出
- **灵活**：用于创建和操作 Markdown 文档的丰富 API
- **可定制**：各种格式化选项控制输出
- **可扩展**：支持自定义节点类型和编写行为
- **GFM 支持**：可选的 GitHub Flavored Markdown 扩展，包括表格、删除线、任务列表和自动链接

## 为什么选择 cmark-writer？

虽然 Rust 中有很多 Markdown 解析器，但 cmark-writer 特别专注于写入/序列化方面。这使它非常适合：

- 以编程方式生成 Markdown 内容
- 创建文档系统
- 构建 Markdown 导出功能
- 实现 Markdown 转换工具
- 处理基于 AST 的内容编辑

## 项目状态

cmark-writer 处于积极维护状态并已准备好用于生产环境。它被设计为 Markdown 处理管道中的可靠组件。

## 许可证

cmark-writer 使用 MIT 许可证，适用于开源和商业应用。