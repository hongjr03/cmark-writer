use cmark_writer::ast::{Alignment, ListItem, Node};
use cmark_writer::writer::{CommonMarkWriter, WriterOptions};

#[test]
fn test_write_text() {
    let mut writer = CommonMarkWriter::new();
    let text = Node::Text("Hello, World!".to_string());
    writer.write(&text).unwrap();
    assert_eq!(writer.into_string(), "Hello, World!");
}

#[test]
fn test_write_escaped_text() {
    let mut writer = CommonMarkWriter::new();
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
    assert_eq!(writer.into_string(), "*emphasized*");
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
    };
    writer.write(&code_block).unwrap();
    assert_eq!(
        writer.into_string(),
        "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```"
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
    };
    writer.write(&heading).unwrap();
    assert_eq!(writer.into_string(), "## Section Title");
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
        "This is a **paragraph** with formatting."
    );
}

#[test]
fn test_write_unordered_list() {
    let mut writer = CommonMarkWriter::new();
    let list = Node::UnorderedList(vec![
        ListItem {
            content: vec![Node::Text("Item 1".to_string())],
            is_task: false,
            task_completed: false,
        },
        ListItem {
            content: vec![Node::Text("Item 2".to_string())],
            is_task: false,
            task_completed: false,
        },
    ]);
    writer.write(&list).unwrap();
    assert_eq!(writer.into_string(), "- Item 1\n- Item 2");
}

#[test]
fn test_write_task_list() {
    let mut writer = CommonMarkWriter::new();
    let list = Node::UnorderedList(vec![
        ListItem {
            content: vec![Node::Text("Task 1".to_string())],
            is_task: true,
            task_completed: true,
        },
        ListItem {
            content: vec![Node::Text("Task 2".to_string())],
            is_task: true,
            task_completed: false,
        },
    ]);
    writer.write(&list).unwrap();
    assert_eq!(writer.into_string(), "- [x] Task 1\n- [ ] Task 2");
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
        alt: "Alt text".to_string(),
    };
    writer.write(&image).unwrap();
    assert_eq!(writer.into_string(), "![Alt text](image.png \"An image\")");
}

#[test]
fn test_writer_options() {
    // Test custom hard break options
    let options = WriterOptions {
        strict: true,
        hard_break_spaces: false, // Use backslash for line breaks
        indent_spaces: 4,
    };

    let mut writer = CommonMarkWriter::with_options(options);
    writer.write(&Node::HardBreak).unwrap();
    assert_eq!(writer.into_string(), "\\\n");

    // Use default options (two spaces for line breaks)
    let mut writer = CommonMarkWriter::new();
    writer.write(&Node::HardBreak).unwrap();
    assert_eq!(writer.into_string(), "  \n");
}

#[test]
fn test_write_table() {
    let mut writer = CommonMarkWriter::new();
    let table = Node::Table {
        headers: vec![
            Node::Text("Name".to_string()),
            Node::Text("Age".to_string()),
        ],
        rows: vec![
            vec![
                Node::Text("Alice".to_string()),
                Node::Text("30".to_string()),
            ],
            vec![Node::Text("Bob".to_string()), Node::Text("25".to_string())],
        ],
        alignments: vec![Alignment::Left, Alignment::Right],
    };

    writer.write(&table).unwrap();
    let expected = "| Name | Age |\n| :--- | ---: |\n| Alice | 30 |\n| Bob | 25 |\n";
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
        ListItem {
            content: vec![Node::Text("Level 1 item 1".to_string())],
            is_task: false,
            task_completed: false,
        },
        // Second level 1 item (with ordered sublist)
        ListItem {
            content: vec![
                Node::Text("Level 1 item 2".to_string()),
                Node::OrderedList {
                    start: 1,
                    items: vec![
                        // First level 2 ordered item
                        ListItem {
                            content: vec![Node::Text("Level 2 ordered item 1".to_string())],
                            is_task: false,
                            task_completed: false,
                        },
                        // Second level 2 ordered item
                        ListItem {
                            content: vec![
                                Node::Text("Level 2 ordered item 2".to_string()),
                                // Level 3 unordered list
                                Node::UnorderedList(vec![ListItem {
                                    content: vec![Node::Text("Level 3 unordered item".to_string())],
                                    is_task: true,
                                    task_completed: true,
                                }]),
                            ],
                            is_task: false,
                            task_completed: false,
                        },
                    ],
                },
            ],
            is_task: false,
            task_completed: false,
        },
        // Third level 1 item (task item)
        ListItem {
            content: vec![Node::Text("Level 1 item 3".to_string())],
            is_task: true,
            task_completed: false,
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
        - [x] Level 3 unordered item
- [ ] Level 1 item 3"#;

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
    let expected = "This is **bold** and *emphasized* text with a [link](https://example.com \"Link title\") and `some code`.";
    assert_eq!(result, expected);

    // Test inline elements in list items
    let list = Node::UnorderedList(vec![
        ListItem {
            content: vec![
                Node::Text("Item with ".to_string()),
                Node::Strong(vec![Node::Text("bold".to_string())]),
                Node::Text(" and ".to_string()),
                Node::Emphasis(vec![Node::Text("emphasis".to_string())]),
            ],
            is_task: false,
            task_completed: false,
        },
        ListItem {
            content: vec![
                Node::Text("Item with ".to_string()),
                Node::InlineCode("code".to_string()),
                Node::Text(" and a ".to_string()),
                Node::Link {
                    url: "https://example.com".to_string(),
                    title: None,
                    content: vec![Node::Text("link".to_string())],
                },
            ],
            is_task: false,
            task_completed: false,
        },
    ]);

    let mut writer = CommonMarkWriter::new();
    writer.write(&list).unwrap();
    let result = writer.into_string();

    // Inline elements in list items should not have incorrect line breaks
    let expected =
        "- Item with **bold** and *emphasis*\n- Item with `code` and a [link](https://example.com)";
    assert_eq!(result, expected);
}
