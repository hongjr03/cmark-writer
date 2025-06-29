#[cfg(feature = "gfm")]
use cmark_writer::ast::TableAlignment;
use cmark_writer::ast::{HeadingType, HtmlAttribute, HtmlElement, ListItem, Node};
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;
use cmark_writer::{CodeBlockType, WriteError, WriterOptions};

#[test]
fn test_write_text() {
    let mut writer = CommonMarkWriter::new();
    let text = Node::Text("Hello, World!".into());
    writer.write(&text).unwrap();
    assert_eq!(writer.into_string(), "Hello, World!");
}

#[test]
fn test_write_escaped_text() {
    let mut writer = CommonMarkWriter::with_options(
        WriterOptionsBuilder::new()
            .strict(true)
            .escape_special_chars(true)
            .build(),
    );
    let text = Node::Text("Special chars: * _ [ ] < > ` \\".into());
    writer.write(&text).unwrap();
    assert_eq!(
        writer.into_string(),
        "Special chars: \\* \\_ \\[ \\] \\< \\> \\` \\\\"
    );
}

#[test]
fn test_write_emphasis() {
    let mut writer = CommonMarkWriter::new();
    let emphasis = Node::Emphasis(vec![Node::Text("emphasized".into())]);
    writer.write(&emphasis).unwrap();
    assert_eq!(writer.into_string(), "_emphasized_");
}

#[test]
fn test_write_strong() {
    let mut writer = CommonMarkWriter::new();
    let strong = Node::Strong(vec![Node::Text("bold".into())]);
    writer.write(&strong).unwrap();
    assert_eq!(writer.into_string(), "**bold**");
}

#[test]
fn test_write_code_block() {
    let mut writer = CommonMarkWriter::new();
    let code_block = Node::CodeBlock {
        language: Some("rust".into()),
        content: "fn main() {\n    println!(\"Hello\");\n}".into(),
        block_type: cmark_writer::ast::CodeBlockType::Fenced,
    };
    writer.write(&code_block).unwrap();
    assert_eq!(
        writer.into_string(),
        "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```\n"
    );
}

#[test]
fn test_write_indented_code_block() {
    let mut writer = CommonMarkWriter::new();
    let code_block = Node::CodeBlock {
        language: None,
        content: "fn main() {\n    println!(\"Hello\");\n}".into(),
        block_type: CodeBlockType::Indented,
    };
    writer.write(&code_block).unwrap();
    assert_eq!(
        writer.into_string(),
        "    fn main() {\n        println!(\"Hello\");\n    }\n"
    );
}

#[test]
fn test_write_inline_code() {
    let mut writer = CommonMarkWriter::new();
    let inline_code = Node::InlineCode("let x = 42;".into());
    writer.write(&inline_code).unwrap();
    assert_eq!(writer.into_string(), "`let x = 42;`");
}

#[test]
fn test_write_heading() {
    let mut writer = CommonMarkWriter::new();
    let heading = Node::Heading {
        level: 2,
        content: vec![Node::Text("Section Title".into())],
        heading_type: HeadingType::Atx, // 添加默认的 ATX 标题类型
    };
    writer.write(&heading).unwrap();
    assert_eq!(writer.into_string(), "## Section Title\n");
}

#[test]
fn test_write_paragraph() {
    let mut writer = CommonMarkWriter::new();
    let paragraph = Node::Paragraph(vec![
        Node::Text("This is a ".into()),
        Node::Strong(vec![Node::Text("paragraph".into())]),
        Node::Text(" with formatting.".into()),
    ]);
    writer.write(&paragraph).unwrap();
    assert_eq!(
        writer.into_string(),
        "This is a **paragraph** with formatting.\n"
    );
}

#[test]
fn test_write_unordered_list() {
    let mut writer = CommonMarkWriter::new();
    let list = Node::UnorderedList(vec![
        ListItem::Unordered {
            content: vec![Node::Paragraph(vec![Node::Text("Item 1".into())])],
        },
        ListItem::Unordered {
            content: vec![Node::Paragraph(vec![Node::Text("Item 2".into())])],
        },
    ]);
    writer.write(&list).unwrap();
    assert_eq!(writer.into_string(), "- Item 1\n- Item 2\n");
}

#[test]
fn test_write_link() {
    let mut writer = CommonMarkWriter::new();
    let link = Node::Link {
        url: "https://www.rust-lang.org".into(),
        title: Some("Rust Website".into()),
        content: vec![Node::Text("Rust".into())],
    };
    writer.write(&link).unwrap();
    assert_eq!(
        writer.into_string(),
        "[Rust](https://www.rust-lang.org \"Rust Website\")"
    );
}

#[test]
fn test_write_image() {
    let mut writer = CommonMarkWriter::new();
    let image = Node::Image {
        url: "image.png".into(),
        title: Some("An image".into()),
        alt: vec![Node::Text("Alt text".into())],
    };
    writer.write(&image).unwrap();
    assert_eq!(writer.into_string(), "![Alt text](image.png \"An image\")");
}

#[test]
fn test_write_image_with_formatted_alt() {
    let mut writer = CommonMarkWriter::new();
    let image = Node::Image {
        url: "image.png".into(),
        title: Some("An image with formatted alt text".into()),
        alt: vec![
            Node::Text("Image with ".into()),
            Node::Strong(vec![Node::Text("bold".into())]),
            Node::Text(" and ".into()),
            Node::Emphasis(vec![Node::Text("italic".into())]),
            Node::Text(" text".into()),
        ],
    };
    writer.write(&image).unwrap();
    assert_eq!(
        writer.into_string(),
        "![Image with **bold** and _italic_ text](image.png \"An image with formatted alt text\")"
    );
}

#[test]
fn test_writer_options() {
    let options = WriterOptionsBuilder::new()
        .strict(true)
        .hard_break_spaces(true) // Use spaces for line breaks
        .indent_spaces(4)
        .build();

    let mut writer = CommonMarkWriter::with_options(options);
    writer.write(&Node::HardBreak).unwrap();
    assert_eq!(writer.into_string(), "  \n");

    // Use default options (two spaces for line breaks)
    let mut writer = CommonMarkWriter::new();
    writer.write(&Node::HardBreak).unwrap();
    assert_eq!(writer.into_string(), "\\\n");
}

#[test]
fn test_write_table() {
    let mut writer = CommonMarkWriter::new();
    let table = Node::Table {
        headers: vec![Node::Text("Name".into()), Node::Text("Age".into())],
        #[cfg(feature = "gfm")]
        alignments: vec![TableAlignment::Left, TableAlignment::Left],
        rows: vec![
            vec![Node::Text("Alice".into()), Node::Text("30".into())],
            vec![Node::Text("Bob".into()), Node::Text("25".into())],
        ],
    };

    writer.write(&table).unwrap();
    let expected = "| Name | Age |\n| --- | --- |\n| Alice | 30 |\n| Bob | 25 |\n";
    assert_eq!(writer.into_string(), expected);
}

#[test]
fn test_display_trait() {
    let node = Node::Strong(vec![Node::Text("important".into())]);
    assert_eq!(format!("{}", node), "**important**");
}

#[test]
fn test_table_with_block_elements_strict_mode() {
    let mut writer =
        CommonMarkWriter::with_options(WriterOptionsBuilder::new().strict(true).build());

    // Create a table with a code block in a cell (block-level element)
    let table = Node::Table {
        headers: vec![Node::Text("Header 1".into()), Node::Text("Header 2".into())],
        #[cfg(feature = "gfm")]
        alignments: vec![],
        rows: vec![vec![
            Node::Text("Regular text".into()),
            Node::CodeBlock {
                language: Some("rust".into()),
                content: "fn main() {\n    println!(\"Hello\");\n}".into(),
                block_type: CodeBlockType::Fenced,
            },
        ]],
    };

    // In strict mode, this should fail because code blocks are block-level elements
    let result = writer.write(&table);
    assert!(result.is_err());
    if let Err(WriteError::InvalidStructure(msg)) = result {
        assert!(msg.contains("block-level elements"));
    } else {
        panic!("Expected InvalidStructure error, got: {:?}", result);
    }
}

#[test]
fn test_table_with_block_elements_soft_mode_fallback() {
    let mut writer = CommonMarkWriter::with_options(
        WriterOptionsBuilder::new()
            .strict(false) // Enable soft mode
            .build(),
    );

    // Create a table with a code block in a cell (block-level element)
    let table = Node::Table {
        headers: vec![Node::Text("Header 1".into()), Node::Text("Header 2".into())],
        #[cfg(feature = "gfm")]
        alignments: vec![],
        rows: vec![vec![
            Node::Text("Regular text".into()),
            Node::CodeBlock {
                language: Some("rust".into()),
                content: "fn main() {\n    println!(\"Hello\");\n}".into(),
                block_type: CodeBlockType::Fenced,
            },
        ]],
    };

    // In soft mode, this should fallback to HTML output
    writer.write(&table).unwrap();
    let output = writer.into_string();
    println!("{}", output);

    // Should generate HTML table instead of markdown
    assert!(output.contains("<table>"));
    assert!(output.contains("<thead>"));
    assert!(output.contains("<tbody>"));
    assert!(output.contains("<th>Header 1</th>"));
    assert!(output.contains("<th>Header 2</th>"));
    assert!(output.contains("<td>Regular text</td>"));
    assert!(output.contains("<pre><code class=\"language-rust\">"));
    assert!(output.contains("fn main()"));
}

#[test]
fn test_table_with_paragraph_in_cell_soft_mode() {
    let mut writer =
        CommonMarkWriter::with_options(WriterOptionsBuilder::new().strict(false).build());

    // Create a table with a paragraph in a cell (block-level element)
    let table = Node::Table {
        headers: vec![Node::Text("Column 1".into()), Node::Text("Column 2".into())],
        #[cfg(feature = "gfm")]
        alignments: vec![],
        rows: vec![vec![
            Node::Paragraph(vec![Node::Text(
                "This is a paragraph in a table cell".into(),
            )]),
            Node::Text("Simple text".into()),
        ]],
    };

    // Should fallback to HTML in soft mode
    writer.write(&table).unwrap();
    let output = writer.into_string();

    assert!(output.contains("<table>"));
    assert!(output.contains("<p>This is a paragraph in a table cell</p>"));
}

#[test]
fn test_table_with_only_inline_elements_no_fallback() {
    let mut writer =
        CommonMarkWriter::with_options(WriterOptionsBuilder::new().strict(false).build());

    // Create a table with only inline elements
    let table = Node::Table {
        headers: vec![Node::Text("Name".into()), Node::Text("Age".into())],
        #[cfg(feature = "gfm")]
        alignments: vec![],
        rows: vec![vec![
            Node::Strong(vec![Node::Text("Alice".into())]),
            Node::Emphasis(vec![Node::Text("30".into())]),
        ]],
    };

    // Should use regular markdown table syntax (no fallback needed)
    writer.write(&table).unwrap();
    let output = writer.into_string();

    // Should generate markdown table, not HTML
    assert!(output.contains("| Name | Age |"));
    assert!(output.contains("| --- | --- |"));
    assert!(output.contains("| **Alice** | _30_ |"));
    assert!(!output.contains("<table>"));
}

#[test]
fn test_write_mixed_nested_lists() {
    let mut writer = CommonMarkWriter::new();

    // Create mixed multi-level list (combination of ordered and unordered lists)
    let mixed_list = Node::UnorderedList(vec![
        // First level 1 item
        ListItem::Unordered {
            content: vec![Node::Paragraph(vec![Node::Text("Level 1 item 1".into())])],
        },
        // Second level 1 item (with ordered sublist)
        ListItem::Unordered {
            content: vec![
                Node::Paragraph(vec![Node::Text("Level 1 item 2".into())]),
                Node::OrderedList {
                    start: 1,
                    items: vec![
                        // First level 2 ordered item
                        ListItem::Ordered {
                            number: None,
                            content: vec![Node::Paragraph(vec![Node::Text(
                                "Level 2 ordered item 1".into(),
                            )])],
                        },
                        // Second level 2 ordered item
                        ListItem::Ordered {
                            number: None,
                            content: vec![
                                Node::Paragraph(vec![Node::Text("Level 2 ordered item 2".into())]),
                                // Level 3 unordered list
                                Node::UnorderedList(vec![ListItem::Unordered {
                                    content: vec![Node::Paragraph(vec![Node::Text(
                                        "Level 3 unordered item".into(),
                                    )])],
                                }]),
                            ],
                        },
                    ],
                },
            ],
        },
        // Third level 1 item
        ListItem::Unordered {
            content: vec![Node::Paragraph(vec![Node::Text("Level 1 item 3".into())])],
        },
    ]);

    writer.write(&mixed_list).unwrap();
    let result = writer.into_string();

    // Using explicit escape characters for newlines and spaces to ensure correct
    // preservation of indentation
    let expected = r#"- Level 1 item 1
- Level 1 item 2
  
  1. Level 2 ordered item 1
  2. Level 2 ordered item 2
     
     - Level 3 unordered item
- Level 1 item 3
"#;

    assert_eq!(result, expected);
}

#[test]
fn test_inline_elements_line_breaks() {
    let mut writer = CommonMarkWriter::new();

    // Test inline elements in a paragraph
    let paragraph = Node::Paragraph(vec![
        Node::Text("This is ".into()),
        Node::Strong(vec![Node::Text("bold".into())]),
        Node::Text(" and ".into()),
        Node::Emphasis(vec![Node::Text("emphasized".into())]),
        Node::Text(" text with a ".into()),
        Node::Link {
            url: "https://example.com".into(),
            title: Some("Link title".into()),
            content: vec![Node::Text("link".into())],
        },
        Node::Text(" and ".into()),
        Node::InlineCode("some code".into()),
        Node::Text(".".into()),
    ]);

    writer.write(&paragraph).unwrap();
    let result = writer.into_string();

    // All inline elements should be on the same line without incorrect line breaks
    let expected = "This is **bold** and _emphasized_ text with a [link](https://example.com \"Link title\") and `some code`.\n";
    assert_eq!(result, expected);

    // Test inline elements in list items
    let list = Node::UnorderedList(vec![
        ListItem::Unordered {
            content: vec![Node::Paragraph(vec![
                Node::Text("Item with ".into()),
                Node::Strong(vec![Node::Text("bold".into())]),
                Node::Text(" and ".into()),
                Node::Emphasis(vec![Node::Text("emphasis".into())]),
            ])],
        },
        ListItem::Unordered {
            content: vec![Node::Paragraph(vec![
                Node::Text("Item with ".into()),
                Node::InlineCode("code".into()),
                Node::Text(" and a ".into()),
                Node::Link {
                    url: "https://example.com".into(),
                    title: None,
                    content: vec![Node::Text("link".into())],
                },
            ])],
        },
    ]);

    let mut writer = CommonMarkWriter::new();
    writer.write(&list).unwrap();
    let result = writer.into_string();

    // Inline elements in list items should not have incorrect line breaks
    let expected =
        "- Item with **bold** and _emphasis_\n- Item with `code` and a [link](https://example.com)\n";
    assert_eq!(result, expected);
}

#[test]
fn test_write_text_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let text = Node::Text("Hello\nWorld".into());
    assert!(writer.write(&text).is_err());
}

#[test]
fn test_write_inline_code_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let code = Node::InlineCode("let x = 1;\nlet y = 2;".into());
    assert!(writer.write(&code).is_err());
}

#[test]
fn test_write_emphasis_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let emph = Node::Emphasis(vec![Node::Text("foo\nbar".into())]);
    assert!(writer.write(&emph).is_err());
}

#[test]
fn test_write_strong_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let strong = Node::Strong(vec![Node::Text("foo\nbar".into())]);
    assert!(writer.write(&strong).is_err());
}

#[test]
fn test_write_link_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let link = Node::Link {
        url: "https://example.com".into(),
        title: None,
        content: vec![Node::Text("foo\nbar".into())],
    };
    assert!(writer.write(&link).is_err());
}

#[test]
fn test_write_image_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let image = Node::Image {
        url: "img.png".into(),
        title: None,
        alt: vec![Node::Text("foo\nbar".into())],
    };
    assert!(writer.write(&image).is_err());
}

#[test]
fn test_write_table_cell_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let table = Node::Table {
        headers: vec![Node::Text("header".into())],
        #[cfg(feature = "gfm")]
        alignments: vec![TableAlignment::Left],
        rows: vec![vec![Node::Text("foo\nbar".into())]],
    };
    assert!(writer.write(&table).is_err());
}

// #[test]
// fn test_write_strike() {
//     let mut writer = CommonMarkWriter::new();
//     let strike = Node::Emphasis(vec![Node::Text("emphasis".into())]);
//     writer.write(&strike).unwrap();
//     assert_eq!(writer.into_string(), "~~emphasis~~");
// }

// #[test]
// fn test_write_strike_with_newline_should_fail() {
//     let mut writer = CommonMarkWriter::new();
//     let strike = Node::Emphasis(vec![Node::Text("foo\nbar".into())]);
//     assert!(writer.write(&strike).is_err());
// }

#[test]
fn test_write_mixed_formatting() {
    let mut writer = CommonMarkWriter::new();
    let paragraph = Node::Paragraph(vec![
        Node::Text("This is ".into()),
        Node::Strong(vec![Node::Text("bold".into())]),
        Node::Text(" and ".into()),
        Node::Emphasis(vec![Node::Text("emphasized".into())]),
        Node::Text(" and ".into()),
        Node::Emphasis(vec![Node::Text("emphasis".into())]),
        Node::Text(" text.".into()),
    ]);

    writer.write(&paragraph).unwrap();
    let result = writer.into_string();

    let expected = "This is **bold** and _emphasized_ and _emphasis_ text.\n";
    assert_eq!(result, expected);
}

#[test]
fn test_write_nested_formatting_with_strike() {
    let mut writer = CommonMarkWriter::new();
    let paragraph = Node::Paragraph(vec![
        Node::Text("This contains ".into()),
        Node::Emphasis(vec![
            Node::Text("emphasis with ".into()),
            Node::Strong(vec![Node::Text("bold".into())]),
            Node::Text(" inside".into()),
        ]),
        Node::Text(".".into()),
    ]);

    writer.write(&paragraph).unwrap();
    let result = writer.into_string();

    let expected = "This contains _emphasis with **bold** inside_.\n";
    assert_eq!(result, expected);
}

#[test]
fn test_write_html_element() {
    let mut writer = CommonMarkWriter::new();
    let html_element = Node::HtmlElement(HtmlElement {
        tag: "div".into(),
        attributes: vec![
            HtmlAttribute {
                name: "class".into(),
                value: "container".into(),
            },
            HtmlAttribute {
                name: "id".into(),
                value: "main".into(),
            },
        ],
        children: vec![Node::Text("内容".into())],
        self_closing: false,
    });

    writer.write(&html_element).unwrap();
    assert_eq!(
        writer.into_string(),
        "<div class=\"container\" id=\"main\">内容</div>"
    );
}

#[test]
fn test_write_self_closing_html_element() {
    let mut writer = CommonMarkWriter::new();
    let img = Node::HtmlElement(HtmlElement {
        tag: "img".into(),
        attributes: vec![
            HtmlAttribute {
                name: "src".into(),
                value: "image.jpg".into(),
            },
            HtmlAttribute {
                name: "alt".into(),
                value: "图片描述".into(),
            },
        ],
        children: vec![],
        self_closing: true,
    });

    writer.write(&img).unwrap();
    assert_eq!(
        writer.into_string(),
        "<img src=\"image.jpg\" alt=\"图片描述\" />"
    );
}

#[test]
fn test_nested_html_elements() {
    let mut writer = CommonMarkWriter::new();
    let nested_element = Node::HtmlElement(HtmlElement {
        tag: "div".into(),
        attributes: vec![HtmlAttribute {
            name: "class".into(),
            value: "outer".into(),
        }],
        children: vec![
            Node::Text("开始 ".into()),
            Node::HtmlElement(HtmlElement {
                tag: "span".into(),
                attributes: vec![HtmlAttribute {
                    name: "class".into(),
                    value: "inner".into(),
                }],
                children: vec![Node::Text("嵌套内容".into())],
                self_closing: false,
            }),
            Node::Text(" 结束".into()),
        ],
        self_closing: false,
    });

    writer.write(&nested_element).unwrap();
    assert_eq!(
        writer.into_string(),
        "<div class=\"outer\">开始 <span class=\"inner\">嵌套内容</span> 结束</div>"
    );
}

#[test]
fn test_html_element_with_unsafe_tag() {
    let mut writer = CommonMarkWriter::new();
    let html_element = Node::HtmlElement(HtmlElement {
        tag: "script<dangerous>".into(), // 不安全的标签名
        attributes: vec![],
        children: vec![Node::Text("alert('危险代码')".into())],
        self_closing: false,
    });

    let result = writer.write(&html_element);
    assert!(result.is_err());
    if let Err(WriteError::InvalidHtmlTag(tag)) = result {
        assert_eq!(tag, "script<dangerous>");
    } else {
        panic!("Expected InvalidHtmlTag error");
    }
}

#[test]
fn test_html_element_with_unsafe_attribute() {
    let mut writer = CommonMarkWriter::with_options(WriterOptions {
        strict: true,
        ..Default::default()
    });
    let html_element = Node::HtmlElement(HtmlElement {
        tag: "div".into(),
        attributes: vec![HtmlAttribute {
            name: "on<click>".into(), // 不安全的属性名
            value: "alert('危险')".into(),
        }],
        children: vec![],
        self_closing: false,
    });

    // 应该返回错误
    let result = writer.write(&html_element);
    assert!(result.is_err());
    if let Err(WriteError::InvalidHtmlAttribute(attr)) = result {
        assert_eq!(attr, "on<click>");
    } else {
        panic!("Expected InvalidHtmlAttribute error");
    }
}

#[test]
fn test_html_attribute_value_escaping() {
    let mut writer = CommonMarkWriter::new();
    let html_element = Node::HtmlElement(HtmlElement {
        tag: "div".into(),
        attributes: vec![HtmlAttribute {
            name: "data-text".into(),
            value: "引号\"和<标签>以及&符号".into(),
        }],
        children: vec![Node::Text("内容".into())],
        self_closing: false,
    });

    writer.write(&html_element).unwrap();
    assert_eq!(
        writer.into_string(),
        "<div data-text=\"引号\"和&lt;标签&gt;以及&amp;符号\">内容</div>"
    );
}

#[test]
fn test_write_ordered_list() {
    let mut writer = CommonMarkWriter::new();
    let list = Node::OrderedList {
        start: 1,
        items: vec![
            ListItem::Ordered {
                number: None,
                content: vec![Node::Paragraph(vec![Node::Text("第一项".into())])],
            },
            ListItem::Ordered {
                number: None,
                content: vec![Node::Paragraph(vec![Node::Text("第二项".into())])],
            },
        ],
    };
    writer.write(&list).unwrap();
    assert_eq!(writer.into_string(), "1. 第一项\n2. 第二项\n");
}

#[test]
fn test_write_ordered_list_with_custom_number() {
    let mut writer = CommonMarkWriter::new();
    let list = Node::OrderedList {
        start: 1,
        items: vec![
            ListItem::Ordered {
                number: None,
                content: vec![Node::Paragraph(vec![Node::Text("第一项".into())])],
            },
            ListItem::Ordered {
                number: Some(5), // Use custom number
                content: vec![Node::Paragraph(vec![Node::Text("从 5 开始的项".into())])],
            },
            ListItem::Ordered {
                number: None, // Continue incrementing from previous number
                content: vec![Node::Paragraph(vec![Node::Text("自动递增项".into())])],
            },
        ],
    };
    writer.write(&list).unwrap();
    assert_eq!(
        writer.into_string(),
        "1. 第一项\n5. 从 5 开始的项\n6. 自动递增项\n"
    );
}

#[test]
fn test_mixed_ordered_and_unordered_items() {
    let mut writer = CommonMarkWriter::new();
    let list = Node::OrderedList {
        start: 10, // Start from 10
        items: vec![
            ListItem::Ordered {
                number: None,
                content: vec![Node::Paragraph(vec![Node::Text("从 10 开始的项".into())])],
            },
            ListItem::Unordered {
                content: vec![Node::Paragraph(vec![Node::Text("无序列表项".into())])],
            },
            ListItem::Ordered {
                number: Some(20), // Custom number jumps to 20
                content: vec![Node::Paragraph(vec![Node::Text("跳跃到 20".into())])],
            },
        ],
    };
    writer.write(&list).unwrap();
    assert_eq!(
        writer.into_string(),
        "10. 从 10 开始的项\n11. 无序列表项\n20. 跳跃到 20\n"
    );
}

#[test]
fn test_write_uri_autolink() {
    let mut writer = CommonMarkWriter::new();
    let autolink = Node::Autolink {
        url: "https://www.example.com".into(),
        is_email: false,
    };
    writer.write(&autolink).unwrap();
    assert_eq!(writer.into_string(), "<https://www.example.com>");
}

#[test]
fn test_write_uri_autolink_without_scheme() {
    let mut writer = CommonMarkWriter::new();
    let autolink = Node::Autolink {
        url: "www.example.com".into(),
        is_email: false,
    };
    writer.write(&autolink).unwrap();
    assert_eq!(writer.into_string(), "<https://www.example.com>");
}

#[test]
fn test_write_email_autolink() {
    let mut writer = CommonMarkWriter::new();
    let autolink = Node::Autolink {
        url: "user@example.com".into(),
        is_email: true,
    };
    writer.write(&autolink).unwrap();
    assert_eq!(writer.into_string(), "<user@example.com>");
}

#[test]
fn test_autolink_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let autolink = Node::Autolink {
        url: "https://example.com\nwith-newline".into(),
        is_email: false,
    };
    assert!(writer.write(&autolink).is_err());
}

#[test]
fn test_autolink_in_paragraph() {
    let mut writer = CommonMarkWriter::new();
    let paragraph = Node::Paragraph(vec![
        Node::Text("Visit ".into()),
        Node::Autolink {
            url: "https://www.example.com".into(),
            is_email: false,
        },
        Node::Text(" or contact ".into()),
        Node::Autolink {
            url: "user@example.com".into(),
            is_email: true,
        },
        Node::Text(" for more information.".into()),
    ]);

    writer.write(&paragraph).unwrap();
    assert_eq!(
        writer.into_string(),
        "Visit <https://www.example.com> or contact <user@example.com> for more information.\n"
    );
}

#[test]
fn test_write_link_reference_definition() {
    let mut writer = CommonMarkWriter::new();
    let link_ref_def = Node::LinkReferenceDefinition {
        label: "foo".into(),
        destination: "/url".into(),
        title: Some("title".into()),
    };
    writer.write(&link_ref_def).unwrap();
    assert_eq!(writer.into_string(), "[foo]: /url \"title\"\n");
}

#[test]
fn test_write_link_reference_definition_no_title() {
    let mut writer = CommonMarkWriter::new();
    let link_ref_def = Node::LinkReferenceDefinition {
        label: "bar".into(),
        destination: "https://example.com".into(),
        title: None,
    };
    writer.write(&link_ref_def).unwrap();
    assert_eq!(writer.into_string(), "[bar]: https://example.com\n");
}

#[test]
fn test_write_reference_link() {
    let mut writer = CommonMarkWriter::new();
    let ref_link = Node::ReferenceLink {
        label: "foo".into(),
        content: vec![Node::Text("Link text".into())],
    };
    writer.write(&ref_link).unwrap();
    assert_eq!(writer.into_string(), "[Link text][foo]");
}

#[test]
fn test_write_shortcut_reference_link() {
    let mut writer = CommonMarkWriter::new();
    // When content is the same as label, it's a shortcut reference
    let ref_link = Node::ReferenceLink {
        label: "foo".into(),
        content: vec![Node::Text("foo".into())],
    };
    writer.write(&ref_link).unwrap();
    assert_eq!(writer.into_string(), "[foo]");

    // Empty content also produces a shortcut reference
    let mut writer = CommonMarkWriter::new();
    let ref_link = Node::ReferenceLink {
        label: "bar".into(),
        content: vec![],
    };
    writer.write(&ref_link).unwrap();
    assert_eq!(writer.into_string(), "[bar]");
}

#[test]
fn test_reference_link_in_paragraph() {
    let mut writer = CommonMarkWriter::new();
    let paragraph = Node::Paragraph(vec![
        Node::Text("See ".into()),
        Node::ReferenceLink {
            label: "example".into(),
            content: vec![Node::Text("this example".into())],
        },
        Node::Text(" for more information.".into()),
    ]);

    writer.write(&paragraph).unwrap();
    assert_eq!(
        writer.into_string(),
        "See [this example][example] for more information.\n"
    );
}

#[test]
fn test_document_with_reference_links() {
    let mut writer = CommonMarkWriter::new();
    let doc = Node::Document(vec![
        Node::LinkReferenceDefinition {
            label: "example".into(),
            destination: "/example".into(),
            title: Some("Example Page".into()),
        },
        Node::Paragraph(vec![
            Node::Text("See ".into()),
            Node::ReferenceLink {
                label: "example".into(),
                content: vec![Node::Text("this example".into())],
            },
            Node::Text(".".into()),
        ]),
        Node::Paragraph(vec![
            Node::Text("Or just click ".into()),
            Node::ReferenceLink {
                label: "example".into(),
                content: vec![Node::Text("example".into())],
            },
            Node::Text(".".into()),
        ]),
    ]);

    writer.write(&doc).unwrap();
    assert_eq!(
        writer.into_string(),
        "[example]: /example \"Example Page\"

See [this example][example].

Or just click [example].
"
    );
}

#[test]
fn test_nested_leaf_blocks_with_indentation() {
    let mut writer = CommonMarkWriter::new();

    let list = Node::UnorderedList(vec![
        ListItem::Unordered {
            content: vec![Node::Paragraph(vec![Node::Text("普通段落".into())])],
        },
        ListItem::Unordered {
            content: vec![Node::Heading {
                level: 3,
                content: vec![Node::Text("列表中的标题".into())],
                heading_type: HeadingType::Atx, // 添加默认的 ATX 标题类型
            }],
        },
        ListItem::Unordered {
            content: vec![Node::CodeBlock {
                language: None,
                content: "function test() {\n  console.log('Hello');\n}".into(),
                block_type: cmark_writer::ast::CodeBlockType::Indented,
            }],
        },
        ListItem::Unordered {
            content: vec![Node::CodeBlock {
                language: Some("rust".into()),
                content: "fn main() {\n    println!(\"Hello\");\n}".into(),
                block_type: cmark_writer::ast::CodeBlockType::Fenced,
            }],
        },
        ListItem::Unordered {
            content: vec![Node::ThematicBreak],
        },
        ListItem::Unordered {
            content: vec![Node::HtmlBlock("<div>\n  <p>HTML 内容</p>\n</div>".into())],
        },
        ListItem::Unordered {
            content: vec![Node::LinkReferenceDefinition {
                label: "link".into(),
                destination: "https://example.com".into(),
                title: Some("示例链接".into()),
            }],
        },
    ]);

    writer.write(&list).unwrap();
    let result = writer.into_string();

    let expected = r#"- 普通段落
- ### 列表中的标题
-     function test() {
        console.log('Hello');
      }
- ```rust
  fn main() {
      println!("Hello");
  }
  ```
- ---
- <div>
    <p>HTML 内容</p>
  </div>
- [link]: https://example.com "示例链接"
"#;

    assert_eq!(result, expected);
}

#[test]
fn test_nested_blockquote_with_indentation() {
    let mut writer = CommonMarkWriter::new();

    let blockquote = Node::BlockQuote(vec![
        Node::Paragraph(vec![Node::Text("外部引用第一段落".into())]),
        Node::BlockQuote(vec![
            Node::Paragraph(vec![Node::Text("内部引用段落".into())]),
            Node::CodeBlock {
                language: Some("js".into()),
                content: "function nested() {\n  console.log('嵌套代码');\n}".into(),
                block_type: cmark_writer::ast::CodeBlockType::Fenced,
            },
        ]),
        Node::Paragraph(vec![Node::Text("外部引用第二段落".into())]),
    ]);

    writer.write(&blockquote).unwrap();
    let result = writer.into_string();

    let expected = "> 外部引用第一段落
> 
> > 内部引用段落
> > 
> > ```js
> > function nested() {
> >   console.log('嵌套代码');
> > }
> > ```
> 
> 外部引用第二段落
";

    assert_eq!(result, expected);
}

#[test]
fn test_nested_mixed_containers() {
    let mut writer = CommonMarkWriter::new();

    let mixed_containers = Node::BlockQuote(vec![
        Node::Paragraph(vec![Node::Text("引用块中的段落".into())]),
        Node::UnorderedList(vec![
            ListItem::Unordered {
                content: vec![
                    Node::Paragraph(vec![Node::Text("列表项 1".into())]),
                    Node::BlockQuote(vec![Node::Paragraph(vec![Node::Text(
                        "列表项中的引用块".into(),
                    )])]),
                ],
            },
            ListItem::Unordered {
                content: vec![
                    Node::Paragraph(vec![Node::Text("列表项 2".into())]),
                    Node::CodeBlock {
                        language: None,
                        content: "code in list item".into(),
                        block_type: CodeBlockType::Indented,
                    },
                ],
            },
        ]),
        Node::Paragraph(vec![Node::Text("引用块的最后一段".into())]),
    ]);

    writer.write(&mixed_containers).unwrap();
    let result = writer.into_string();

    let expected = "> 引用块中的段落
> 
> - 列表项 1
>   
>   > 列表项中的引用块
> - 列表项 2
>   
>       code in list item
> 
> 引用块的最后一段
";

    assert_eq!(result, expected);
}
