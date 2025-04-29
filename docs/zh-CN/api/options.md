# WriterOptions API

`WriterOptions` 结构体控制 CommonMarkWriter 输出的格式化行为。通过这些选项，您可以自定义 Markdown 输出的各个方面。

## 结构定义

```rust
pub struct WriterOptions {
    // CommonMark 核心选项
    pub strict: bool,
    pub hard_break_spaces: bool,
    pub indent_spaces: u8,
    pub list_marker: char,
    pub thematic_break_char: char,
    
    // GFM 特定选项
    pub enable_gfm: bool,
    pub gfm_tables: bool,
    pub gfm_tasklists: bool,
    pub gfm_strikethrough: bool,
    pub gfm_autolinks: bool,
    pub gfm_disallowed_html_tags: Vec<String>,
    
    // 其他选项
    // ...
}
```

## 选项说明

### CommonMark 核心选项

| 选项 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `strict` | `bool` | `false` | 严格遵循 CommonMark 规范 |
| `hard_break_spaces` | `bool` | `true` | 使用空格表示硬换行（而非反斜杠） |
| `indent_spaces` | `u8` | `4` | 缩进的空格数量 |
| `list_marker` | `char` | `-` | 无序列表的标记字符 |
| `thematic_break_char` | `char` | `-` | 分隔线的字符 |

### GFM 选项

| 选项 | 类型 | 默认值 | 描述 |
|------|------|--------|------|
| `enable_gfm` | `bool` | `false` | 启用所有 GitHub Flavored Markdown 功能 |
| `gfm_tables` | `bool` | `false` | 启用 GFM 表格 |
| `gfm_tasklists` | `bool` | `false` | 启用 GFM 任务列表 |
| `gfm_strikethrough` | `bool` | `false` | 启用 GFM 删除线 |
| `gfm_autolinks` | `bool` | `false` | 启用 GFM 扩展自动链接 |
| `gfm_disallowed_html_tags` | `Vec<String>` | `[]` | 要过滤的 HTML 标签列表 |

## 使用 WriterOptions

### 直接创建

```rust
use cmark_writer::options::WriterOptions;
use cmark_writer::writer::CommonMarkWriter;

// 使用默认值创建，然后修改特定选项
let mut options = WriterOptions::default();
options.indent_spaces = 2;
options.list_marker = '*';

// 或者一次性设置所有选项
let options = WriterOptions {
    strict: true,
    indent_spaces: 2,
    list_marker: '*',
    // ... 其他选项使用默认值
    ..Default::default()
};

let writer = CommonMarkWriter::with_options(options);
```

### 使用构建器模式

`WriterOptionsBuilder` 提供了一个流式 API 来创建选项：

```rust
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;

let options = WriterOptionsBuilder::new()
    .strict(true)
    .indent_spaces(2)
    .list_marker('*')
    .thematic_break_char('_')
    .enable_gfm(true)
    .build();

let writer = CommonMarkWriter::with_options(options);
```

## 常见配置场景

### 标准 CommonMark 模式

```rust
let options = WriterOptionsBuilder::new()
    .strict(true)
    .build();
```

### 自定义格式化

```rust
let options = WriterOptionsBuilder::new()
    .indent_spaces(2)
    .list_marker('*')
    .thematic_break_char('_')
    .build();
```

### 启用所有 GFM 功能

```rust
let options = WriterOptionsBuilder::new()
    .enable_gfm(true)
    .build();
```

### 选择性启用 GFM 功能

```rust
let options = WriterOptionsBuilder::new()
    .gfm_tables(true)
    .gfm_strikethrough(true)
    .build(); // enable_gfm 会自动设置为 true
```

### HTML 安全过滤

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

## 选项对输出的影响

### 不同的 `list_marker` 设置

```markdown
# 使用默认值 '-'
- 项目 1
- 项目 2

# 使用 '*'
* 项目 1
* 项目 2

# 使用 '+'
+ 项目 1
+ 项目 2
```

### 不同的 `indent_spaces` 设置

```markdown
# 使用默认值 4
> 第一级引用
    > 第二级引用
        > 第三级引用

# 使用值 2
> 第一级引用
  > 第二级引用
    > 第三级引用
```

### 不同的 `thematic_break_char` 设置

```markdown
# 使用默认值 '-'
---

# 使用 '*'
***

# 使用 '_'
___
```
