use cmark_writer::ast::{Alignment, BlockNode, InlineNode, ListItem, Node};
use cmark_writer::writer::{CommonMarkWriter, WriterOptions};

#[test]
fn test_write_text() {
    let mut writer = CommonMarkWriter::new();
    let text = Node::Inline(InlineNode::Text("Hello, World!".to_string()));
    writer.write(&text).unwrap();
    assert_eq!(writer.into_string(), "Hello, World!");
}

#[test]
fn test_write_escaped_text() {
    let mut writer = CommonMarkWriter::new();
    let text = Node::Inline(InlineNode::Text(
        "Special chars: * _ [ ] < > ` \\".to_string(),
    ));
    writer.write(&text).unwrap();
    assert_eq!(
        writer.into_string(),
        "Special chars: \\* \\_ \\[ \\] \\< \\> \\` \\\\"
    );
}

#[test]
fn test_write_emphasis() {
    let mut writer = CommonMarkWriter::new();
    let emphasis = Node::Inline(InlineNode::Emphasis(vec![InlineNode::Text(
        "emphasized".to_string(),
    )]));
    writer.write(&emphasis).unwrap();
    assert_eq!(writer.into_string(), "*emphasized*");
}

#[test]
fn test_write_strong() {
    let mut writer = CommonMarkWriter::new();
    let strong = Node::Inline(InlineNode::Strong(vec![InlineNode::Text(
        "bold".to_string(),
    )]));
    writer.write(&strong).unwrap();
    assert_eq!(writer.into_string(), "**bold**");
}

#[test]
fn test_write_code_block() {
    let mut writer = CommonMarkWriter::new();
    let code_block = Node::Block(BlockNode::CodeBlock {
        language: Some("rust".to_string()),
        content: "fn main() {\n    println!(\"Hello\");\n}".to_string(),
    });
    writer.write(&code_block).unwrap();
    assert_eq!(
        writer.into_string(),
        "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```"
    );
}

#[test]
fn test_write_inline_code() {
    let mut writer = CommonMarkWriter::new();
    let inline_code = Node::Inline(InlineNode::InlineCode("let x = 42;".to_string()));
    writer.write(&inline_code).unwrap();
    assert_eq!(writer.into_string(), "`let x = 42;`");
}

#[test]
fn test_write_heading() {
    let mut writer = CommonMarkWriter::new();
    let heading = Node::Block(BlockNode::Heading {
        level: 2,
        content: vec![InlineNode::Text("Section Title".to_string())],
    });
    writer.write(&heading).unwrap();
    assert_eq!(writer.into_string(), "## Section Title");
}

#[test]
fn test_write_paragraph() {
    let mut writer = CommonMarkWriter::new();
    let paragraph = Node::Block(BlockNode::Paragraph(vec![
        InlineNode::Text("This is a ".to_string()),
        InlineNode::Strong(vec![InlineNode::Text("paragraph".to_string())]),
        InlineNode::Text(" with formatting.".to_string()),
    ]));
    writer.write(&paragraph).unwrap();
    assert_eq!(
        writer.into_string(),
        "This is a **paragraph** with formatting."
    );
}

#[test]
fn test_write_unordered_list() {
    let mut writer = CommonMarkWriter::new();
    let list = Node::Block(BlockNode::UnorderedList(vec![
        ListItem::Regular {
            content: vec![BlockNode::Paragraph(vec![InlineNode::Text(
                "Item 1".to_string(),
            )])],
        },
        ListItem::Regular {
            content: vec![BlockNode::Paragraph(vec![InlineNode::Text(
                "Item 2".to_string(),
            )])],
        },
    ]));
    writer.write(&list).unwrap();
    assert_eq!(writer.into_string(), "- Item 1\n- Item 2");
}

#[test]
fn test_write_task_list() {
    let mut writer = CommonMarkWriter::new();
    let list = Node::Block(BlockNode::UnorderedList(vec![
        ListItem::Task {
            completed: true,
            content: vec![BlockNode::Paragraph(vec![InlineNode::Text(
                "Task 1".to_string(),
            )])],
        },
        ListItem::Task {
            completed: false,
            content: vec![BlockNode::Paragraph(vec![InlineNode::Text(
                "Task 2".to_string(),
            )])],
        },
    ]));
    writer.write(&list).unwrap();
    assert_eq!(writer.into_string(), "- [x] Task 1\n- [ ] Task 2");
}

#[test]
fn test_write_link() {
    let mut writer = CommonMarkWriter::new();
    let link = Node::Inline(InlineNode::Link {
        url: "https://www.rust-lang.org".to_string(),
        title: Some("Rust Website".to_string()),
        content: vec![InlineNode::Text("Rust".to_string())],
    });
    writer.write(&link).unwrap();
    assert_eq!(
        writer.into_string(),
        "[Rust](https://www.rust-lang.org \"Rust Website\")"
    );
}

#[test]
fn test_write_image() {
    let mut writer = CommonMarkWriter::new();
    let image = Node::Inline(InlineNode::Image {
        url: "image.png".to_string(),
        title: Some("An image".to_string()),
        alt: "Alt text".to_string(),
    });
    writer.write(&image).unwrap();
    assert_eq!(writer.into_string(), "![Alt text](image.png \"An image\")");
}

#[test]
fn test_writer_options() {
    // Test custom hard break options
    let options = WriterOptions {
        strict: true,
        hard_break_spaces: true, // Use backslash for line breaks
        indent_spaces: 4,
    };

    let mut writer = CommonMarkWriter::with_options(options);
    writer.write(&Node::Inline(InlineNode::HardBreak)).unwrap();
    assert_eq!(writer.into_string(), "  \n");

    // Use default options (two spaces for line breaks)
    let mut writer = CommonMarkWriter::new();
    writer.write(&Node::Inline(InlineNode::HardBreak)).unwrap();
    assert_eq!(writer.into_string(), "\\\n");
}

#[test]
fn test_write_table() {
    let mut writer = CommonMarkWriter::new();
    let table = Node::Block(BlockNode::Table {
        headers: vec![
            InlineNode::Text("Name".to_string()),
            InlineNode::Text("Age".to_string()),
        ],
        rows: vec![
            vec![
                InlineNode::Text("Alice".to_string()),
                InlineNode::Text("30".to_string()),
            ],
            vec![
                InlineNode::Text("Bob".to_string()),
                InlineNode::Text("25".to_string()),
            ],
        ],
        alignments: vec![Alignment::Left, Alignment::Right],
    });

    writer.write(&table).unwrap();
    let expected = "| Name | Age |\n| :--- | ---: |\n| Alice | 30 |\n| Bob | 25 |\n";
    assert_eq!(writer.into_string(), expected);
}

#[test]
fn test_display_trait() {
    let node = Node::Inline(InlineNode::Strong(vec![InlineNode::Text(
        "important".to_string(),
    )]));
    assert_eq!(format!("{}", node), "**important**");
}

#[test]
fn test_write_mixed_nested_lists() {
    let mut writer = CommonMarkWriter::new();

    // Create mixed multi-level list (combination of ordered and unordered lists)
    let mixed_list = Node::Block(BlockNode::UnorderedList(vec![
        // First level 1 item
        ListItem::Regular {
            content: vec![BlockNode::Paragraph(vec![InlineNode::Text(
                "Level 1 item 1".to_string(),
            )])],
        },
        // Second level 1 item (with ordered sublist)
        ListItem::Regular {
            content: vec![
                BlockNode::Paragraph(vec![InlineNode::Text("Level 1 item 2".to_string())]),
                BlockNode::OrderedList {
                    start: 1,
                    items: vec![
                        // First level 2 ordered item
                        ListItem::Regular {
                            content: vec![BlockNode::Paragraph(vec![InlineNode::Text(
                                "Level 2 ordered item 1".to_string(),
                            )])],
                        },
                        // Second level 2 ordered item
                        ListItem::Regular {
                            content: vec![
                                BlockNode::Paragraph(vec![InlineNode::Text(
                                    "Level 2 ordered item 2".to_string(),
                                )]),
                                // Level 3 unordered list
                                BlockNode::UnorderedList(vec![ListItem::Task {
                                    completed: true,
                                    content: vec![BlockNode::Paragraph(vec![InlineNode::Text(
                                        "Level 3 unordered item".to_string(),
                                    )])],
                                }]),
                            ],
                        },
                    ],
                },
            ],
        },
        // Third level 1 item (task item)
        ListItem::Task {
            completed: false,
            content: vec![BlockNode::Paragraph(vec![InlineNode::Text(
                "Level 1 item 3".to_string(),
            )])],
        },
    ]));

    writer.write(&mixed_list).unwrap();
    let result = writer.into_string();

    // Using explicit escape characters for newlines and spaces to ensure correct
    // preservation of indentation
    let expected = r#"- Level 1 item 1
- Level 1 item 2
    1. Level 2 ordered item 1
    2. Level 2 ordered item 2
        - [x] Level 3 unordered item
- [ ] Level 1 item 3"#;

    assert_eq!(result, expected);
}

#[test]
fn test_inline_elements_line_breaks() {
    let mut writer = CommonMarkWriter::new();

    // Test inline elements in a paragraph
    let paragraph = Node::Block(BlockNode::Paragraph(vec![
        InlineNode::Text("This is ".to_string()),
        InlineNode::Strong(vec![InlineNode::Text("bold".to_string())]),
        InlineNode::Text(" and ".to_string()),
        InlineNode::Emphasis(vec![InlineNode::Text("emphasized".to_string())]),
        InlineNode::Text(" text with a ".to_string()),
        InlineNode::Link {
            url: "https://example.com".to_string(),
            title: Some("Link title".to_string()),
            content: vec![InlineNode::Text("link".to_string())],
        },
        InlineNode::Text(" and ".to_string()),
        InlineNode::InlineCode("some code".to_string()),
        InlineNode::Text(".".to_string()),
    ]));

    writer.write(&paragraph).unwrap();
    let result = writer.into_string();

    // All inline elements should be on the same line without incorrect line breaks
    let expected = "This is **bold** and *emphasized* text with a [link](https://example.com \"Link title\") and `some code`.";
    assert_eq!(result, expected);

    // Test inline elements in list items
    let list = Node::Block(BlockNode::UnorderedList(vec![
        ListItem::Regular {
            content: vec![BlockNode::Paragraph(vec![
                InlineNode::Text("Item with ".to_string()),
                InlineNode::Strong(vec![InlineNode::Text("bold".to_string())]),
                InlineNode::Text(" and ".to_string()),
                InlineNode::Emphasis(vec![InlineNode::Text("emphasis".to_string())]),
            ])],
        },
        ListItem::Regular {
            content: vec![BlockNode::Paragraph(vec![
                InlineNode::Text("Item with ".to_string()),
                InlineNode::InlineCode("code".to_string()),
                InlineNode::Text(" and a ".to_string()),
                InlineNode::Link {
                    url: "https://example.com".to_string(),
                    title: None,
                    content: vec![InlineNode::Text("link".to_string())],
                },
            ])],
        },
    ]));

    let mut writer = CommonMarkWriter::new();
    writer.write(&list).unwrap();
    let result = writer.into_string();

    // Inline elements in list items should not have incorrect line breaks
    let expected =
        "- Item with **bold** and *emphasis*\n- Item with `code` and a [link](https://example.com)";
    assert_eq!(result, expected);
}

#[test]
fn test_write_text_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let text = Node::Inline(InlineNode::Text("Hello\nWorld".to_string()));
    assert!(writer.write(&text).is_err());
}

#[test]
fn test_write_inline_code_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let code = Node::Inline(InlineNode::InlineCode("let x = 1;\nlet y = 2;".to_string()));
    assert!(writer.write(&code).is_err());
}

#[test]
fn test_write_emphasis_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let emph = Node::Inline(InlineNode::Emphasis(vec![InlineNode::Text(
        "foo\nbar".to_string(),
    )]));
    assert!(writer.write(&emph).is_err());
}

#[test]
fn test_write_strong_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let strong = Node::Inline(InlineNode::Strong(vec![InlineNode::Text(
        "foo\nbar".to_string(),
    )]));
    assert!(writer.write(&strong).is_err());
}

#[test]
fn test_write_link_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let link = Node::Inline(InlineNode::Link {
        url: "https://example.com".to_string(),
        title: None,
        content: vec![InlineNode::Text("foo\nbar".to_string())],
    });
    assert!(writer.write(&link).is_err());
}

#[test]
fn test_write_image_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let image = Node::Inline(InlineNode::Image {
        url: "img.png".to_string(),
        title: None,
        alt: "foo\nbar".to_string(),
    });
    assert!(writer.write(&image).is_err());
}

#[test]
fn test_write_table_cell_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let table = Node::Block(BlockNode::Table {
        headers: vec![InlineNode::Text("header".to_string())],
        rows: vec![vec![InlineNode::Text("foo\nbar".to_string())]],
        alignments: vec![Alignment::Left],
    });
    assert!(writer.write(&table).is_err());
}

#[test]
fn test_write_strike() {
    let mut writer = CommonMarkWriter::new();
    let strike = Node::Inline(InlineNode::Strike(vec![InlineNode::Text(
        "strikethrough".to_string(),
    )]));
    writer.write(&strike).unwrap();
    assert_eq!(writer.into_string(), "~~strikethrough~~");
}

#[test]
fn test_write_strike_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let strike = Node::Inline(InlineNode::Strike(vec![InlineNode::Text(
        "foo\nbar".to_string(),
    )]));
    assert!(writer.write(&strike).is_err());
}

#[test]
fn test_write_mixed_formatting() {
    let mut writer = CommonMarkWriter::new();
    let paragraph = Node::Block(BlockNode::Paragraph(vec![
        InlineNode::Text("This is ".to_string()),
        InlineNode::Strong(vec![InlineNode::Text("bold".to_string())]),
        InlineNode::Text(" and ".to_string()),
        InlineNode::Emphasis(vec![InlineNode::Text("emphasized".to_string())]),
        InlineNode::Text(" and ".to_string()),
        InlineNode::Strike(vec![InlineNode::Text("strikethrough".to_string())]),
        InlineNode::Text(" text.".to_string()),
    ]));

    writer.write(&paragraph).unwrap();
    let result = writer.into_string();

    let expected = "This is **bold** and *emphasized* and ~~strikethrough~~ text.";
    assert_eq!(result, expected);
}

#[test]
fn test_write_nested_formatting_with_strike() {
    let mut writer = CommonMarkWriter::new();
    let paragraph = Node::Block(BlockNode::Paragraph(vec![
        InlineNode::Text("This contains ".to_string()),
        InlineNode::Strike(vec![
            InlineNode::Text("strikethrough with ".to_string()),
            InlineNode::Strong(vec![InlineNode::Text("bold".to_string())]),
            InlineNode::Text(" inside".to_string()),
        ]),
        InlineNode::Text(".".to_string()),
    ]));

    writer.write(&paragraph).unwrap();
    let result = writer.into_string();

    let expected = "This contains ~~strikethrough with **bold** inside~~.";
    assert_eq!(result, expected);
}

#[test]
fn test_write_html_element() {
    use cmark_writer::ast::{HtmlAttribute, HtmlElement};

    let mut writer = CommonMarkWriter::new();
    let html_element = Node::Inline(InlineNode::HtmlElement(HtmlElement {
        tag: "div".to_string(),
        attributes: vec![
            HtmlAttribute {
                name: "class".to_string(),
                value: "container".to_string(),
            },
            HtmlAttribute {
                name: "id".to_string(),
                value: "main".to_string(),
            },
        ],
        children: vec![InlineNode::Text("内容".to_string())],
        self_closing: false,
    }));

    writer.write(&html_element).unwrap();
    assert_eq!(
        writer.into_string(),
        "<div class=\"container\" id=\"main\">内容</div>"
    );
}

#[test]
fn test_write_self_closing_html_element() {
    use cmark_writer::ast::{HtmlAttribute, HtmlElement};

    let mut writer = CommonMarkWriter::new();
    let img = Node::Inline(InlineNode::HtmlElement(HtmlElement {
        tag: "img".to_string(),
        attributes: vec![
            HtmlAttribute {
                name: "src".to_string(),
                value: "image.jpg".to_string(),
            },
            HtmlAttribute {
                name: "alt".to_string(),
                value: "图片描述".to_string(),
            },
        ],
        children: vec![],
        self_closing: true,
    }));

    writer.write(&img).unwrap();
    assert_eq!(
        writer.into_string(),
        "<img src=\"image.jpg\" alt=\"图片描述\" />"
    );
}

#[test]
fn test_nested_html_elements() {
    use cmark_writer::ast::{HtmlAttribute, HtmlElement};

    let mut writer = CommonMarkWriter::new();
    let nested_element = Node::Inline(InlineNode::HtmlElement(HtmlElement {
        tag: "div".to_string(),
        attributes: vec![HtmlAttribute {
            name: "class".to_string(),
            value: "outer".to_string(),
        }],
        children: vec![
            InlineNode::Text("开始 ".to_string()),
            InlineNode::HtmlElement(HtmlElement {
                tag: "span".to_string(),
                attributes: vec![HtmlAttribute {
                    name: "class".to_string(),
                    value: "inner".to_string(),
                }],
                children: vec![InlineNode::Text("嵌套内容".to_string())],
                self_closing: false,
            }),
            InlineNode::Text(" 结束".to_string()),
        ],
        self_closing: false,
    }));

    writer.write(&nested_element).unwrap();
    assert_eq!(
        writer.into_string(),
        "<div class=\"outer\">开始 <span class=\"inner\">嵌套内容</span> 结束</div>"
    );
}

#[test]
fn test_html_element_with_formatted_content() {
    use cmark_writer::ast::{HtmlAttribute, HtmlElement};

    let mut writer = CommonMarkWriter::new();
    let element = Node::Inline(InlineNode::HtmlElement(HtmlElement {
        tag: "p".to_string(),
        attributes: vec![HtmlAttribute {
            name: "class".to_string(),
            value: "text".to_string(),
        }],
        children: vec![
            InlineNode::Text("普通文本 ".to_string()),
            InlineNode::Strong(vec![InlineNode::Text("粗体文本".to_string())]),
            InlineNode::Text(" 和 ".to_string()),
            InlineNode::Emphasis(vec![InlineNode::Text("斜体文本".to_string())]),
        ],
        self_closing: false,
    }));

    writer.write(&element).unwrap();
    assert_eq!(
        writer.into_string(),
        "<p class=\"text\">普通文本 **粗体文本** 和 *斜体文本*</p>"
    );
}

#[test]
fn test_html_attribute_with_quotes() {
    use cmark_writer::ast::{HtmlAttribute, HtmlElement};

    let mut writer = CommonMarkWriter::new();
    let element = Node::Inline(InlineNode::HtmlElement(HtmlElement {
        tag: "div".to_string(),
        attributes: vec![HtmlAttribute {
            name: "data-text".to_string(),
            value: "含有\"引号\"的属性值".to_string(),
        }],
        children: vec![InlineNode::Text("内容".to_string())],
        self_closing: false,
    }));

    writer.write(&element).unwrap();
    assert_eq!(
        writer.into_string(),
        "<div data-text=\"含有&quot;引号&quot;的属性值\">内容</div>"
    );
}

#[test]
fn test_html_element_in_paragraph() {
    use cmark_writer::ast::HtmlElement;

    let mut writer = CommonMarkWriter::new();
    let paragraph = Node::Block(BlockNode::Paragraph(vec![
        InlineNode::Text("文本开始 ".to_string()),
        InlineNode::HtmlElement(HtmlElement {
            tag: "code".to_string(),
            attributes: vec![],
            children: vec![InlineNode::Text("代码片段".to_string())],
            self_closing: false,
        }),
        InlineNode::Text(" 文本结束".to_string()),
    ]));

    writer.write(&paragraph).unwrap();
    assert_eq!(
        writer.into_string(),
        "文本开始 <code>代码片段</code> 文本结束"
    );
}

#[test]
fn test_write_html_block() {
    let mut writer = CommonMarkWriter::new();
    let html_block = Node::Block(BlockNode::HtmlBlock(
        "<div class=\"container\">\n  <h1>标题</h1>\n  <p>段落</p>\n</div>".to_string(),
    ));
    writer.write(&html_block).unwrap();
    assert_eq!(
        writer.into_string(),
        "<div class=\"container\">\n  <h1>标题</h1>\n  <p>段落</p>\n</div>"
    );
}

#[test]
fn test_hard_break_with_chinese_text() {
    let mut writer = CommonMarkWriter::new();
    let paragraph = Node::Block(BlockNode::Paragraph(vec![
        InlineNode::Text("换行测试：".to_string()),
        InlineNode::HardBreak,
        InlineNode::Text("这行文字应该在上一行的下方紧跟着。".to_string()),
    ]));

    writer.write(&paragraph).unwrap();
    let result = writer.into_string();

    // 默认使用反斜杠换行
    let expected = "换行测试：\\\n这行文字应该在上一行的下方紧跟着。";
    assert_eq!(result, expected);

    // 测试使用空格换行的选项
    let options = WriterOptions {
        strict: true,
        hard_break_spaces: true,  // 使用两个空格加换行
        indent_spaces: 4,
    };
    let mut writer = CommonMarkWriter::with_options(options);
    writer.write(&paragraph).unwrap();
    let result = writer.into_string();
    
    let expected_spaces = "换行测试：  \n这行文字应该在上一行的下方紧跟着。";
    assert_eq!(result, expected_spaces);
}
