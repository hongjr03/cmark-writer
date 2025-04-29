# 编写器接口

`CommonMarkWriter` 是负责将 AST 节点序列化为符合 CommonMark 规范文本的核心组件。本页解释了如何使用和与编写器交互。

## 基本用法

使用编写器遵循一个简单的模式：

```rust
use cmark_writer::ast::Node;
use cmark_writer::writer::CommonMarkWriter;

// 创建具有默认选项的编写器
let mut writer = CommonMarkWriter::new();

// 将节点写入编写器
writer.write(&Node::Text("Hello world".to_string())).expect("写入失败");

// 获取输出字符串
let output = writer.into_string();
```

## 创建编写器

有两种方式创建编写器：

### 使用默认选项

```rust
let mut writer = CommonMarkWriter::new();
```

### 使用自定义选项

```rust
use cmark_writer::options::WriterOptions;

// 使用自定义选项创建
let options = WriterOptions {
    strict: true,
    hard_break_spaces: false,
    // ... 其他选项
    ..Default::default()
};

let mut writer = CommonMarkWriter::with_options(options);
```

## 写入方法

您将使用的主要方法是 `write()`：

```rust
// 写入单个节点
writer.write(&node).expect("写入失败");
```

此方法递归处理节点及其子节点，根据 CommonMark 规范处理所有格式。

## 获取输出

写入所有节点后，获取输出：

```rust
// 获取最终的 Markdown 输出
let markdown = writer.into_string();
```

这会消耗编写器，返回格式化的 Markdown 字符串。

## 错误处理

写入操作返回 `WriteResult<()>`，这是 `Result<(), WriteError>` 的类型别名。这允许您优雅地处理格式错误：

```rust
match writer.write(&node) {
    Ok(_) => {
        // 写入成功
        let output = writer.into_string();
        println!("生成的 Markdown: {}", output);
    },
    Err(err) => {
        // 处理错误
        eprintln!("写入节点失败：{}", err);
    }
}
```

常见的错误类型包括：

- `WriteError::NewlineInInlineElement`：当行内元素包含换行符时
- `WriteError::InvalidNesting`：当节点嵌套不正确时
- `WriteError::UnsupportedNodeType`：当尝试写入不支持的节点类型时

## 示例：构建文档

构建和写入文档的完整示例：

```rust
use cmark_writer::ast::{Node, ListItem};
use cmark_writer::writer::CommonMarkWriter;

// 创建文档结构
let document = Node::Document(vec![
    Node::heading(1, vec![Node::Text("文档标题".to_string())]),
    Node::Paragraph(vec![
        Node::Text("这是介绍。".to_string())
    ]),
    Node::UnorderedList(vec![
        ListItem::Unordered { 
            content: vec![Node::Paragraph(vec![Node::Text("第一点".to_string())])] 
        },
        ListItem::Unordered { 
            content: vec![Node::Paragraph(vec![Node::Text("第二点".to_string())])] 
        },
    ]),
]);

// 创建编写器并生成 Markdown
let mut writer = CommonMarkWriter::new();
writer.write(&document).expect("写入文档失败");
let markdown = writer.into_string();

println!("{}", markdown);
```

这将生成：

```markdown
# 文档标题

这是介绍。

- 第一点
- 第二点
```
