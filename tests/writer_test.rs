#[cfg(feature = "gfm")]
use cmark_writer::ast::TableAlignment;
use cmark_writer::ast::{HeadingType, HtmlAttribute, HtmlElement, ListItem, Node};
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;
use cmark_writer::{CodeBlockType, WriteError};

#[test]
fn test_write_text() {
    let mut writer = CommonMarkWriter::new();
    let text = Node::Text("Hello, World!".to_string());
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
    let text = Node::Text("Special chars: * _ [ ] < > ` \\".to_string());
    writer.write(&text).unwrap();
    assert_eq!(
        writer.into_string(),
        "Special chars: \\* \\_ \\[ \\] \\< \\> \\` \\\\"
    );
}

#[test]
fn test_write_emphasis() {
    let mut writer = CommonMarkWriter::new();
    let emphasis = Node::Emphasis(vec![Node::Text("emphasized".to_string())]);
    writer.write(&emphasis).unwrap();
    assert_eq!(writer.into_string(), "_emphasized_");
}

#[test]
fn test_write_strong() {
    let mut writer = CommonMarkWriter::new();
    let strong = Node::Strong(vec![Node::Text("bold".to_string())]);
    writer.write(&strong).unwrap();
    assert_eq!(writer.into_string(), "**bold**");
}

#[test]
fn test_write_code_block() {
    let mut writer = CommonMarkWriter::new();
    let code_block = Node::CodeBlock {
        language: Some("rust".to_string()),
        content: "fn main() {\n    println!(\"Hello\");\n}".to_string(),
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
        content: "fn main() {\n    println!(\"Hello\");\n}".to_string(),
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
    let inline_code = Node::InlineCode("let x = 42;".to_string());
    writer.write(&inline_code).unwrap();
    assert_eq!(writer.into_string(), "`let x = 42;`");
}

#[test]
fn test_write_heading() {
    let mut writer = CommonMarkWriter::new();
    let heading = Node::Heading {
        level: 2,
        content: vec![Node::Text("Section Title".to_string())],
        heading_type: HeadingType::Atx, // 添加默认的 ATX 标题类型
    };
    writer.write(&heading).unwrap();
    assert_eq!(writer.into_string(), "## Section Title\n");
}

#[test]
fn test_write_paragraph() {
    let mut writer = CommonMarkWriter::new();
    let paragraph = Node::Paragraph(vec![
        Node::Text("This is a ".to_string()),
        Node::Strong(vec![Node::Text("paragraph".to_string())]),
        Node::Text(" with formatting.".to_string()),
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
            content: vec![Node::Paragraph(vec![Node::Text("Item 1".to_string())])],
        },
        ListItem::Unordered {
            content: vec![Node::Paragraph(vec![Node::Text("Item 2".to_string())])],
        },
    ]);
    writer.write(&list).unwrap();
    assert_eq!(writer.into_string(), "- Item 1\n- Item 2\n");
}

#[test]
fn test_write_link() {
    let mut writer = CommonMarkWriter::new();
    let link = Node::Link {
        url: "https://www.rust-lang.org".to_string(),
        title: Some("Rust Website".to_string()),
        content: vec![Node::Text("Rust".to_string())],
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
        url: "image.png".to_string(),
        title: Some("An image".to_string()),
        alt: vec![Node::Text("Alt text".to_string())],
    };
    writer.write(&image).unwrap();
    assert_eq!(writer.into_string(), "![Alt text](image.png \"An image\")");
}

#[test]
fn test_write_image_with_formatted_alt() {
    let mut writer = CommonMarkWriter::new();
    let image = Node::Image {
        url: "image.png".to_string(),
        title: Some("An image with formatted alt text".to_string()),
        alt: vec![
            Node::Text("Image with ".to_string()),
            Node::Strong(vec![Node::Text("bold".to_string())]),
            Node::Text(" and ".to_string()),
            Node::Emphasis(vec![Node::Text("italic".to_string())]),
            Node::Text(" text".to_string()),
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
        headers: vec![
            Node::Text("Name".to_string()),
            Node::Text("Age".to_string()),
        ],
        #[cfg(feature = "gfm")]
        alignments: vec![TableAlignment::Left, TableAlignment::Left],
        rows: vec![
            vec![
                Node::Text("Alice".to_string()),
                Node::Text("30".to_string()),
            ],
            vec![Node::Text("Bob".to_string()), Node::Text("25".to_string())],
        ],
    };

    writer.write(&table).unwrap();
    let expected = "| Name | Age |\n| --- | --- |\n| Alice | 30 |\n| Bob | 25 |\n";
    assert_eq!(writer.into_string(), expected);
}

#[test]
fn test_display_trait() {
    let node = Node::Strong(vec![Node::Text("important".to_string())]);
    assert_eq!(format!("{}", node), "**important**");
}

#[test]
fn test_write_mixed_nested_lists() {
    let mut writer = CommonMarkWriter::new();

    // Create mixed multi-level list (combination of ordered and unordered lists)
    let mixed_list = Node::UnorderedList(vec![
        // First level 1 item
        ListItem::Unordered {
            content: vec![Node::Paragraph(vec![Node::Text(
                "Level 1 item 1".to_string(),
            )])],
        },
        // Second level 1 item (with ordered sublist)
        ListItem::Unordered {
            content: vec![
                Node::Paragraph(vec![Node::Text("Level 1 item 2".to_string())]),
                Node::OrderedList {
                    start: 1,
                    items: vec![
                        // First level 2 ordered item
                        ListItem::Ordered {
                            number: None,
                            content: vec![Node::Paragraph(vec![Node::Text(
                                "Level 2 ordered item 1".to_string(),
                            )])],
                        },
                        // Second level 2 ordered item
                        ListItem::Ordered {
                            number: None,
                            content: vec![
                                Node::Paragraph(vec![Node::Text(
                                    "Level 2 ordered item 2".to_string(),
                                )]),
                                // Level 3 unordered list
                                Node::UnorderedList(vec![ListItem::Unordered {
                                    content: vec![Node::Paragraph(vec![Node::Text(
                                        "Level 3 unordered item".to_string(),
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
            content: vec![Node::Paragraph(vec![Node::Text(
                "Level 1 item 3".to_string(),
            )])],
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
        Node::Text("This is ".to_string()),
        Node::Strong(vec![Node::Text("bold".to_string())]),
        Node::Text(" and ".to_string()),
        Node::Emphasis(vec![Node::Text("emphasized".to_string())]),
        Node::Text(" text with a ".to_string()),
        Node::Link {
            url: "https://example.com".to_string(),
            title: Some("Link title".to_string()),
            content: vec![Node::Text("link".to_string())],
        },
        Node::Text(" and ".to_string()),
        Node::InlineCode("some code".to_string()),
        Node::Text(".".to_string()),
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
                Node::Text("Item with ".to_string()),
                Node::Strong(vec![Node::Text("bold".to_string())]),
                Node::Text(" and ".to_string()),
                Node::Emphasis(vec![Node::Text("emphasis".to_string())]),
            ])],
        },
        ListItem::Unordered {
            content: vec![Node::Paragraph(vec![
                Node::Text("Item with ".to_string()),
                Node::InlineCode("code".to_string()),
                Node::Text(" and a ".to_string()),
                Node::Link {
                    url: "https://example.com".to_string(),
                    title: None,
                    content: vec![Node::Text("link".to_string())],
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
    let text = Node::Text("Hello\nWorld".to_string());
    assert!(writer.write(&text).is_err());
}

#[test]
fn test_write_inline_code_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let code = Node::InlineCode("let x = 1;\nlet y = 2;".to_string());
    assert!(writer.write(&code).is_err());
}

#[test]
fn test_write_emphasis_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let emph = Node::Emphasis(vec![Node::Text("foo\nbar".to_string())]);
    assert!(writer.write(&emph).is_err());
}

#[test]
fn test_write_strong_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let strong = Node::Strong(vec![Node::Text("foo\nbar".to_string())]);
    assert!(writer.write(&strong).is_err());
}

#[test]
fn test_write_link_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let link = Node::Link {
        url: "https://example.com".to_string(),
        title: None,
        content: vec![Node::Text("foo\nbar".to_string())],
    };
    assert!(writer.write(&link).is_err());
}

#[test]
fn test_write_image_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let image = Node::Image {
        url: "img.png".to_string(),
        title: None,
        alt: vec![Node::Text("foo\nbar".to_string())],
    };
    assert!(writer.write(&image).is_err());
}

#[test]
fn test_write_table_cell_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let table = Node::Table {
        headers: vec![Node::Text("header".to_string())],
        #[cfg(feature = "gfm")]
        alignments: vec![TableAlignment::Left],
        rows: vec![vec![Node::Text("foo\nbar".to_string())]],
    };
    assert!(writer.write(&table).is_err());
}

// #[test]
// fn test_write_strike() {
//     let mut writer = CommonMarkWriter::new();
//     let strike = Node::Emphasis(vec![Node::Text("emphasis".to_string())]);
//     writer.write(&strike).unwrap();
//     assert_eq!(writer.into_string(), "~~emphasis~~");
// }

// #[test]
// fn test_write_strike_with_newline_should_fail() {
//     let mut writer = CommonMarkWriter::new();
//     let strike = Node::Emphasis(vec![Node::Text("foo\nbar".to_string())]);
//     assert!(writer.write(&strike).is_err());
// }

#[test]
fn test_write_mixed_formatting() {
    let mut writer = CommonMarkWriter::new();
    let paragraph = Node::Paragraph(vec![
        Node::Text("This is ".to_string()),
        Node::Strong(vec![Node::Text("bold".to_string())]),
        Node::Text(" and ".to_string()),
        Node::Emphasis(vec![Node::Text("emphasized".to_string())]),
        Node::Text(" and ".to_string()),
        Node::Emphasis(vec![Node::Text("emphasis".to_string())]),
        Node::Text(" text.".to_string()),
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
        Node::Text("This contains ".to_string()),
        Node::Emphasis(vec![
            Node::Text("emphasis with ".to_string()),
            Node::Strong(vec![Node::Text("bold".to_string())]),
            Node::Text(" inside".to_string()),
        ]),
        Node::Text(".".to_string()),
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
        children: vec![Node::Text("内容".to_string())],
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
        tag: "div".to_string(),
        attributes: vec![HtmlAttribute {
            name: "class".to_string(),
            value: "outer".to_string(),
        }],
        children: vec![
            Node::Text("开始 ".to_string()),
            Node::HtmlElement(HtmlElement {
                tag: "span".to_string(),
                attributes: vec![HtmlAttribute {
                    name: "class".to_string(),
                    value: "inner".to_string(),
                }],
                children: vec![Node::Text("嵌套内容".to_string())],
                self_closing: false,
            }),
            Node::Text(" 结束".to_string()),
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
        tag: "script<dangerous>".to_string(), // 不安全的标签名
        attributes: vec![],
        children: vec![Node::Text("alert('危险代码')".to_string())],
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
    let mut writer = CommonMarkWriter::new();
    let html_element = Node::HtmlElement(HtmlElement {
        tag: "div".to_string(),
        attributes: vec![HtmlAttribute {
            name: "on<click>".to_string(), // 不安全的属性名
            value: "alert('危险')".to_string(),
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
        tag: "div".to_string(),
        attributes: vec![HtmlAttribute {
            name: "data-text".to_string(),
            value: "引号\"和<标签>以及&符号".to_string(),
        }],
        children: vec![Node::Text("内容".to_string())],
        self_closing: false,
    });

    writer.write(&html_element).unwrap();
    assert_eq!(
        writer.into_string(),
        "<div data-text=\"引号&quot;和&lt;标签&gt;以及&amp;符号\">内容</div>"
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
                content: vec![Node::Paragraph(vec![Node::Text("第一项".to_string())])],
            },
            ListItem::Ordered {
                number: None,
                content: vec![Node::Paragraph(vec![Node::Text("第二项".to_string())])],
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
                content: vec![Node::Paragraph(vec![Node::Text("第一项".to_string())])],
            },
            ListItem::Ordered {
                number: Some(5), // Use custom number
                content: vec![Node::Paragraph(vec![Node::Text(
                    "从 5 开始的项".to_string(),
                )])],
            },
            ListItem::Ordered {
                number: None, // Continue incrementing from previous number
                content: vec![Node::Paragraph(vec![Node::Text("自动递增项".to_string())])],
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
                content: vec![Node::Paragraph(vec![Node::Text(
                    "从 10 开始的项".to_string(),
                )])],
            },
            ListItem::Unordered {
                content: vec![Node::Paragraph(vec![Node::Text("无序列表项".to_string())])],
            },
            ListItem::Ordered {
                number: Some(20), // Custom number jumps to 20
                content: vec![Node::Paragraph(vec![Node::Text("跳跃到 20".to_string())])],
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
        url: "https://www.example.com".to_string(),
        is_email: false,
    };
    writer.write(&autolink).unwrap();
    assert_eq!(writer.into_string(), "<https://www.example.com>");
}

#[test]
fn test_write_uri_autolink_without_scheme() {
    let mut writer = CommonMarkWriter::new();
    let autolink = Node::Autolink {
        url: "www.example.com".to_string(),
        is_email: false,
    };
    writer.write(&autolink).unwrap();
    assert_eq!(writer.into_string(), "<https://www.example.com>");
}

#[test]
fn test_write_email_autolink() {
    let mut writer = CommonMarkWriter::new();
    let autolink = Node::Autolink {
        url: "user@example.com".to_string(),
        is_email: true,
    };
    writer.write(&autolink).unwrap();
    assert_eq!(writer.into_string(), "<user@example.com>");
}

#[test]
fn test_autolink_with_newline_should_fail() {
    let mut writer = CommonMarkWriter::new();
    let autolink = Node::Autolink {
        url: "https://example.com\nwith-newline".to_string(),
        is_email: false,
    };
    assert!(writer.write(&autolink).is_err());
}

#[test]
fn test_autolink_in_paragraph() {
    let mut writer = CommonMarkWriter::new();
    let paragraph = Node::Paragraph(vec![
        Node::Text("Visit ".to_string()),
        Node::Autolink {
            url: "https://www.example.com".to_string(),
            is_email: false,
        },
        Node::Text(" or contact ".to_string()),
        Node::Autolink {
            url: "user@example.com".to_string(),
            is_email: true,
        },
        Node::Text(" for more information.".to_string()),
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
        label: "foo".to_string(),
        destination: "/url".to_string(),
        title: Some("title".to_string()),
    };
    writer.write(&link_ref_def).unwrap();
    assert_eq!(writer.into_string(), "[foo]: /url \"title\"\n");
}

#[test]
fn test_write_link_reference_definition_no_title() {
    let mut writer = CommonMarkWriter::new();
    let link_ref_def = Node::LinkReferenceDefinition {
        label: "bar".to_string(),
        destination: "https://example.com".to_string(),
        title: None,
    };
    writer.write(&link_ref_def).unwrap();
    assert_eq!(writer.into_string(), "[bar]: https://example.com\n");
}

#[test]
fn test_write_reference_link() {
    let mut writer = CommonMarkWriter::new();
    let ref_link = Node::ReferenceLink {
        label: "foo".to_string(),
        content: vec![Node::Text("Link text".to_string())],
    };
    writer.write(&ref_link).unwrap();
    assert_eq!(writer.into_string(), "[Link text][foo]");
}

#[test]
fn test_write_shortcut_reference_link() {
    let mut writer = CommonMarkWriter::new();
    // When content is the same as label, it's a shortcut reference
    let ref_link = Node::ReferenceLink {
        label: "foo".to_string(),
        content: vec![Node::Text("foo".to_string())],
    };
    writer.write(&ref_link).unwrap();
    assert_eq!(writer.into_string(), "[foo]");

    // Empty content also produces a shortcut reference
    let mut writer = CommonMarkWriter::new();
    let ref_link = Node::ReferenceLink {
        label: "bar".to_string(),
        content: vec![],
    };
    writer.write(&ref_link).unwrap();
    assert_eq!(writer.into_string(), "[bar]");
}

#[test]
fn test_reference_link_in_paragraph() {
    let mut writer = CommonMarkWriter::new();
    let paragraph = Node::Paragraph(vec![
        Node::Text("See ".to_string()),
        Node::ReferenceLink {
            label: "example".to_string(),
            content: vec![Node::Text("this example".to_string())],
        },
        Node::Text(" for more information.".to_string()),
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
            label: "example".to_string(),
            destination: "/example".to_string(),
            title: Some("Example Page".to_string()),
        },
        Node::Paragraph(vec![
            Node::Text("See ".to_string()),
            Node::ReferenceLink {
                label: "example".to_string(),
                content: vec![Node::Text("this example".to_string())],
            },
            Node::Text(".".to_string()),
        ]),
        Node::Paragraph(vec![
            Node::Text("Or just click ".to_string()),
            Node::ReferenceLink {
                label: "example".to_string(),
                content: vec![Node::Text("example".to_string())],
            },
            Node::Text(".".to_string()),
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
            content: vec![Node::Paragraph(vec![Node::Text("普通段落".to_string())])],
        },
        ListItem::Unordered {
            content: vec![Node::Heading {
                level: 3,
                content: vec![Node::Text("列表中的标题".to_string())],
                heading_type: HeadingType::Atx, // 添加默认的 ATX 标题类型
            }],
        },
        ListItem::Unordered {
            content: vec![Node::CodeBlock {
                language: None,
                content: "function test() {\n  console.log('Hello');\n}".to_string(),
                block_type: cmark_writer::ast::CodeBlockType::Indented,
            }],
        },
        ListItem::Unordered {
            content: vec![Node::CodeBlock {
                language: Some("rust".to_string()),
                content: "fn main() {\n    println!(\"Hello\");\n}".to_string(),
                block_type: cmark_writer::ast::CodeBlockType::Fenced,
            }],
        },
        ListItem::Unordered {
            content: vec![Node::ThematicBreak],
        },
        ListItem::Unordered {
            content: vec![Node::HtmlBlock(
                "<div>\n  <p>HTML 内容</p>\n</div>".to_string(),
            )],
        },
        ListItem::Unordered {
            content: vec![Node::LinkReferenceDefinition {
                label: "link".to_string(),
                destination: "https://example.com".to_string(),
                title: Some("示例链接".to_string()),
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
        Node::Paragraph(vec![Node::Text("外部引用第一段落".to_string())]),
        Node::BlockQuote(vec![
            Node::Paragraph(vec![Node::Text("内部引用段落".to_string())]),
            Node::CodeBlock {
                language: Some("js".to_string()),
                content: "function nested() {\n  console.log('嵌套代码');\n}".to_string(),
                block_type: cmark_writer::ast::CodeBlockType::Fenced,
            },
        ]),
        Node::Paragraph(vec![Node::Text("外部引用第二段落".to_string())]),
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
        Node::Paragraph(vec![Node::Text("引用块中的段落".to_string())]),
        Node::UnorderedList(vec![
            ListItem::Unordered {
                content: vec![
                    Node::Paragraph(vec![Node::Text("列表项 1".to_string())]),
                    Node::BlockQuote(vec![Node::Paragraph(vec![Node::Text(
                        "列表项中的引用块".to_string(),
                    )])]),
                ],
            },
            ListItem::Unordered {
                content: vec![
                    Node::Paragraph(vec![Node::Text("列表项 2".to_string())]),
                    Node::CodeBlock {
                        language: None,
                        content: "code in list item".to_string(),
                        block_type: CodeBlockType::Indented,
                    },
                ],
            },
        ]),
        Node::Paragraph(vec![Node::Text("引用块的最后一段".to_string())]),
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
