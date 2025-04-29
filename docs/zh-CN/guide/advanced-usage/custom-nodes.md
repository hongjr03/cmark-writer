# 自定义节点

cmark-writer 允许您通过创建自定义节点类型来扩展其功能。当您需要表示不属于标准 CommonMark 规范的文档元素时，此功能非常有用。

## 创建自定义节点

要创建自定义节点，您需要：

1. 为您的自定义节点定义结构体或枚举
2. 为您的类型实现 `CustomNode` 特质
3. 将 `#[custom_node]` 属性应用到您的类型
4. 创建包装在 `Node::Custom` 中的自定义节点实例

### 基本示例

这是创建自定义高亮节点的简单示例：

```rust
use cmark_writer::ast::{CustomNodeWriter, Node};
use cmark_writer::error::WriteResult;
use cmark_writer::custom_node;

// 定义自定义高亮节点
#[derive(Debug, Clone, PartialEq)]
#[custom_node]
struct HighlightNode {
    content: String,
    color: String,
}

// 实现所需的方法
impl HighlightNode {
    // 自定义节点写入逻辑
    fn write_custom(&self, writer: &mut dyn CustomNodeWriter) -> WriteResult<()> {
        writer.write_str("<span style=\"background-color: ")?;
        writer.write_str(&self.color)?;
        writer.write_str("\">")?;
        writer.write_str(&self.content)?;
        writer.write_str("</span>")?;
        Ok(())
    }
    
    // 确定是块级元素还是行内元素
    fn is_block_custom(&self) -> bool {
        false // 这是一个行内元素
    }
}
```

### 使用自定义节点

一旦定义，您可以在文档中使用您的自定义节点：

```rust
use cmark_writer::writer::CommonMarkWriter;

// 创建包含自定义节点的文档
let document = Node::Document(vec![
    Node::Paragraph(vec![
        Node::Text("此文本包含一个 ".to_string()),
        Node::Custom(Box::new(HighlightNode {
            content: "高亮部分".to_string(),
            color: "yellow".to_string(),
        })),
        Node::Text("。".to_string()),
    ]),
]);

// 写入文档
let mut writer = CommonMarkWriter::new();
writer.write(&document).expect("写入文档失败");
let markdown = writer.into_string();
```

## 自定义节点接口

`CustomNode` 特质需要实现几个方法：

```rust
pub trait CustomNode: Debug + Send + Sync {
    // 由 #[custom_node] 宏要求：
    fn write(&self, writer: &mut dyn CustomNodeWriter) -> WriteResult<()>;
    fn is_block(&self) -> bool;
    fn clone_custom(&self) -> Box<dyn CustomNode>;
    fn eq_custom(&self, other: &dyn CustomNode) -> bool;
}
```

`#[custom_node]` 属性通过委托给以下方法自动实现这些方法：

- `write_custom(&self, writer: &mut dyn CustomNodeWriter) -> WriteResult<()>`
- `is_block_custom(&self) -> bool`

您只需要实现这两个方法。

## CustomNodeWriter 接口

`CustomNodeWriter` 特质提供了写入内容的方法：

```rust
pub trait CustomNodeWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result;
    fn write_char(&mut self, c: char) -> fmt::Result;
}
```

在您的 `write_custom` 实现中使用这些方法来生成输出。

## 更复杂的示例

这是创建一个彩色框节点的更复杂示例：

```rust
#[derive(Debug, Clone, PartialEq)]
#[custom_node]
struct ColorBoxNode {
    content: Vec<Node>,
    background_color: String,
    border_color: Option<String>,
}

impl ColorBoxNode {
    fn write_custom(&self, writer: &mut dyn CustomNodeWriter) -> WriteResult<()> {
        // 彩色框的 HTML 开始
        writer.write_str("<div style=\"background-color: ")?;
        writer.write_str(&self.background_color)?;
        
        if let Some(border) = &self.border_color {
            writer.write_str("; border: 1px solid ")?;
            writer.write_str(border)?;
        }
        
        writer.write_str("; padding: 10px;\">\n")?;
        
        // 对于包含其他节点的复杂节点，
        // 您通常会将此编写器转换为 CommonMarkWriter
        // 并使用它来写入子节点。这需要更高级的
        // 实现，超出了这个简单示例的范围。
        
        writer.write_str("</div>")?;
        
        Ok(())
    }
    
    fn is_block_custom(&self) -> bool {
        true // 这是一个块级元素
    }
}
```

## 最佳实践

创建自定义节点时：

1. **明确职责**：每个自定义节点应该有一个明确定义的目的
2. **正确嵌套**：嵌套自定义节点时尊重块级/行内区分
3. **错误处理**：在您的 `write_custom` 方法中使用适当的错误处理
4. **文档**：为用户彻底记录您的自定义节点
5. **测试**：编写测试以确保您的自定义节点正确渲染
