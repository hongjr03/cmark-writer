# 错误处理

cmark-writer 提供了一个全面的错误处理系统，可帮助您在生成 Markdown 内容时识别和解决问题。本页涵盖了错误类型、处理策略以及如何创建自定义错误。

## 错误类型

该库使用 `WriteError` 枚举作为其主要错误类型：

```rust
pub enum WriteError {
    // 标准错误
    NewlineInInlineElement(String),
    InvalidNesting(String),
    UnsupportedNodeType,
    IoError(std::io::Error),
    // 自定义错误
    CustomError(Box<dyn CustomErrorFactory>),
    StructureError(Box<dyn StructureError>),
    CodedError(Box<dyn CodedError>),
}
```

常见的错误场景包括：

- **无效内容**：如行内元素中的换行符（`NewlineInInlineElement`）
- **不正确的嵌套**：当节点嵌套不正确时（`InvalidNesting`）
- **不支持的操作**：当尝试使用不支持的功能时（`UnsupportedNodeType`）
- **I/O 错误**：当写入文件或流失败时（`IoError`）
- **自定义错误**：您定义的应用程序特定错误（`CustomError`、`StructureError`、`CodedError`）

## 基本错误处理

使用编写器时，您可以使用标准 Rust 模式处理错误：

```rust
use cmark_writer::ast::Node;
use cmark_writer::writer::CommonMarkWriter;

let node = Node::Text("Hello".to_string());
let mut writer = CommonMarkWriter::new();

match writer.write(&node) {
    Ok(_) => {
        println!("成功写入节点");
        let output = writer.into_string();
        // 使用输出...
    },
    Err(err) => {
        eprintln!("写入节点失败：{}", err);
        // 适当处理错误...
    }
}
```

您也可以使用 `?` 运算符进行更简洁的错误处理：

```rust
fn write_document(document: &Node) -> Result<String, cmark_writer::error::WriteError> {
    let mut writer = CommonMarkWriter::new();
    writer.write(document)?;
    Ok(writer.into_string())
}
```

## 创建自定义错误

cmark-writer 提供了几种定义自己的错误类型的方式：

### 1. 结构错误

使用 `#[custom_error]` 属性宏定义简单的格式化错误：

```rust
use cmark_writer::custom_error;
use cmark_writer::WriteError;

// 使用格式字符串定义结构错误
#[custom_error(format = "表格结构错误：{}")]
struct TableStructureError(pub &'static str);

// 使用该错误
fn validate_table_rows(rows: &[Vec<Node>]) -> Result<(), WriteError> {
    if rows.is_empty() {
        return Err(TableStructureError("表格必须至少有一行").into());
    }
    // 更多验证...
    Ok(())
}
```

### 2. 代码错误

使用 `#[coded_error]` 属性宏定义带错误代码的错误：

```rust
use cmark_writer::coded_error;
use cmark_writer::WriteError;

// 定义带错误代码的代码错误
#[coded_error]
struct ValidationError(pub String, pub String);

// 使用该错误
fn validate_content(content: &str) -> Result<(), WriteError> {
    if content.contains("<script>") {
        return Err(ValidationError("内容包含不安全的脚本标签".to_string(), 
                                 "UNSAFE_CONTENT_ERROR".to_string()).into());
    }
    Ok(())
}
```

### 3. 自定义错误工厂

对于更复杂的错误，实现 `CustomErrorFactory` 特质：

```rust
use std::fmt;
use cmark_writer::error::{CustomErrorFactory, WriteError};

struct ComplexError {
    message: String,
    line: usize,
    column: usize,
}

impl fmt::Display for ComplexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "第 {} 行第 {} 列发生错误：{}", 
               self.line, self.column, self.message)
    }
}

impl fmt::Debug for ComplexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl CustomErrorFactory for ComplexError {
    fn message(&self) -> &str {
        &self.message
    }
    
    fn construct_error(self: Box<Self>) -> WriteError {
        WriteError::CustomError(self)
    }
}

// 使用该错误
fn process_at_position(content: &str, line: usize, column: usize) -> Result<(), WriteError> {
    if content.is_empty() {
        let error = ComplexError {
            message: "不允许空内容".to_string(),
            line,
            column,
        };
        return Err(WriteError::CustomError(Box::new(error)));
    }
    Ok(())
}
```

## 结果扩展

该库提供了 `WriteResultExt` 特质，其中包含用于处理结果的有用方法：

```rust
use cmark_writer::error::WriteResultExt;

fn process_content() -> Result<(), WriteError> {
    let result = possibly_failing_operation()
        .context("内容处理期间失败")?;
    
    Ok(())
}
```

## 错误转换

该库的错误类型实现了 `From` 以便于转换：

```rust
// 从 std::io::Error 转换
fn write_to_file(content: &str, path: &str) -> Result<(), WriteError> {
    let mut file = std::fs::File::create(path)
        .map_err(WriteError::from)?;
    
    // 更多操作...
    Ok(())
}
```

## 最佳实践

1. **具体明确**：对每种情况使用最具体的错误类型
2. **添加上下文**：在错误消息中包含有用的细节
3. **适当传播**：适当时使用 `?` 传播错误
4. **优雅处理**：尽可能提供有意义的恢复选项
5. **测试错误路径**：确保您的错误处理逻辑正常工作
