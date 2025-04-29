# CommonMarkWriter API

`CommonMarkWriter` 是 cmark-writer 库中负责将 AST 节点序列化为 CommonMark 文本的核心组件。本页详细介绍了它的方法和使用模式。

## 构造函数

```rust
/// 使用默认选项创建新的编写器
pub fn new() -> Self;

/// 使用指定的选项创建编写器
pub fn with_options(options: WriterOptions) -> Self;
```

## 核心方法

```rust
/// 将节点写入编写器
/// 返回 Result<(), WriteError> 以进行错误处理
pub fn write(&mut self, node: &Node) -> WriteResult<()>;

/// 将编写器转换为字符串并消耗它
pub fn into_string(self) -> String;

/// 重置编写器内部状态
pub fn reset(&mut self);
```

## 使用模式

### 基本用法

```rust
use cmark_writer::ast::Node;
use cmark_writer::writer::CommonMarkWriter;

// 创建编写器
let mut writer = CommonMarkWriter::new();

// 写入节点
writer.write(&Node::Text("Hello World".to_string()))
      .expect("写入失败");

// 获取输出
let markdown = writer.into_string();
println!("{}", markdown); // 输出：Hello World
```

### 使用自定义选项

```rust
use cmark_writer::ast::Node;
use cmark_writer::writer::CommonMarkWriter;
use cmark_writer::options::WriterOptionsBuilder;

// 使用自定义选项创建编写器
let options = WriterOptionsBuilder::new()
    .indent_spaces(2)
    .list_marker('*')
    .build();

let mut writer = CommonMarkWriter::with_options(options);

// 写入文档
let document = Node::Document(vec![
    Node::heading(1, vec![Node::Text("标题".to_string())]),
    Node::UnorderedList(vec![
        ListItem::Unordered { 
            content: vec![Node::Paragraph(vec![Node::Text("项目 1".to_string())])] 
        },
        ListItem::Unordered { 
            content: vec![Node::Paragraph(vec![Node::Text("项目 2".to_string())])] 
        },
    ]),
]);

writer.write(&document).expect("写入失败");
let markdown = writer.into_string();
```

### 错误处理

```rust
use cmark_writer::ast::Node;
use cmark_writer::writer::CommonMarkWriter;

let mut writer = CommonMarkWriter::new();

// 使用 match 处理可能的错误
match writer.write(&node) {
    Ok(_) => {
        println!("写入成功");
        let output = writer.into_string();
        // 使用输出...
    },
    Err(err) => {
        eprintln!("写入失败：{}", err);
        // 处理错误...
    }
}

// 或者使用 ? 运算符传播错误
fn process_document(document: &Node) -> Result<String, WriteError> {
    let mut writer = CommonMarkWriter::new();
    writer.write(document)?;
    Ok(writer.into_string())
}
```

## 内部处理流程

当调用 `write()` 方法时，`CommonMarkWriter` 会执行以下操作：

1. **检查节点类型** - 确定要应用的格式化规则
2. **应用格式选项** - 根据配置的 `WriterOptions` 格式化内容
3. **递归处理子节点** - 对容器节点的子节点应用相同的过程
4. **处理换行和缩进** - 根据 CommonMark 规范应用适当的空白

此过程确保生成的 Markdown 符合 CommonMark 规范，同时遵循用户指定的格式偏好。

## 与其他组件的关系

- **Node** - 提供要序列化的 AST 结构
- **WriterOptions** - 控制输出格式
- **WriteError** - 表示序列化过程中可能发生的错误
