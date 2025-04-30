# 格式化选项

cmark-writer 通过 `WriterOptions` 结构体提供了各种选项来自定义输出格式。本页解释了可用选项及其使用方法。

## 可用选项

以下是可用于自定义 CommonMark 输出的关键选项：

| 选项 | 类型 | 默认值 | 描述 |
|--------|------|---------|-------------|
| `strict` | `bool` | `false` | 严格遵循 CommonMark 规范 |
| `hard_break_spaces` | `bool` | `true` | 使用空格表示硬换行（而非反斜杠） |
| `indent_spaces` | `u8` | `4` | 缩进的空格数量 |
| `list_marker` | `char` | `-` | 无序列表的标记字符 |
| `thematic_break_char` | `char` | `-` | 分隔线的字符 |
| `escape_special_chars` | `bool` | `false` | 转义文本中的特殊字符 |
| `enable_gfm` | `bool` | `false` | 启用 GitHub Flavored Markdown |
| `gfm_tables` | `bool` | `false` | 启用 GFM 表格 |
| `gfm_tasklists` | `bool` | `false` | 启用 GFM 任务列表 |
| `gfm_strikethrough` | `bool` | `false` | 启用 GFM 删除线 |
| `gfm_autolinks` | `bool` | `false` | 启用 GFM 扩展自动链接 |
| `gfm_disallowed_html_tags` | `Vec<String>` | 空 | 要过滤的 HTML 标签 |

## 使用选项

有两种方式配置选项：

### 直接结构体初始化

通过直接初始化结构体创建选项：

```rust
use cmark_writer::options::WriterOptions;
use cmark_writer::writer::CommonMarkWriter;

let options = WriterOptions {
    strict: true,
    hard_break_spaces: false,
    indent_spaces: 2,
    list_marker: '*',
    ..Default::default()
};

let writer = CommonMarkWriter::with_options(options);
```

### 构建器模式

或使用更便捷的构建器模式：

```rust
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;

let options = WriterOptionsBuilder::new()
    .strict(true)
    .hard_break_spaces(false)
    .indent_spaces(2)
    .list_marker('*')
    .build();

let writer = CommonMarkWriter::with_options(options);
```

## 选项示例

### 标准 CommonMark 模式

严格模式严格遵循 CommonMark 规范：

```rust
let options = WriterOptionsBuilder::new()
    .strict(true)
    .build();
```

### 自定义格式化

更改输出的视觉样式：

```rust
let options = WriterOptionsBuilder::new()
    .indent_spaces(2)           // 使用 2 个空格缩进
    .list_marker('*')           // 使用 * 作为项目符号
    .thematic_break_char('_')   // 使用 ___ 作为水平分割线
    .build();
```

### 启用 GFM 功能

启用 GitHub Flavored Markdown 扩展：

```rust
let options = WriterOptionsBuilder::new()
    .gfm_tables(true)
    .gfm_strikethrough(true)
    .gfm_tasklists(true)
    .build();  // enable_gfm 自动设置为 true
```

或一次性启用所有 GFM 功能：

```rust
let options = WriterOptionsBuilder::new()
    .enable_gfm(true)  // 启用所有 GFM 功能
    .build();
```

### HTML 安全

使用 GFM 时，您可以过滤掉潜在不安全的 HTML：

```rust
let options = WriterOptionsBuilder::new()
    .enable_gfm(true)
    .gfm_disallowed_html_tags(vec![
        "script".to_string(),
        "iframe".to_string(),
        "object".to_string(),
    ])
    .build();
```

## 选项效果

以下示例展示了不同选项如何影响输出：

### 硬换行空格

```rust
// 使用 hard_break_spaces = true（默认）
// 行尾的两个空格成为硬换行
"带有硬换行的行  \n下一行"

// 使用 hard_break_spaces = false
// 使用反斜杠表示硬换行
"带有硬换行的行\\\n下一行"
```

### 缩进

```rust
// 使用 indent_spaces = 4（默认）
">     缩进的引用内容"

// 使用 indent_spaces = 2
">   缩进的引用内容"
```

### 列表标记

```rust
// 使用 list_marker = '-'（默认）
"- 项目 1
- 项目 2"

// 使用 list_marker = '*'
"* 项目 1
* 项目 2"
```
