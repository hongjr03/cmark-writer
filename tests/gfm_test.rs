//! Tests for GitHub Flavored Markdown (GFM) features
//!
//! These tests verify that GFM features work correctly when the "gfm" feature is enabled.
//! This includes:
//! - Tables with alignment
//! - Strikethrough
//! - Task lists
//! - Extended autolinks
//! - HTML tag filtering

#[cfg(feature = "gfm")]
mod gfm_tests {
    use cmark_writer::ast::{HtmlAttribute, HtmlElement};
    use cmark_writer::ast::{ListItem, Node, TableAlignment, TaskListStatus};
    use cmark_writer::options::WriterOptionsBuilder;
    use cmark_writer::writer::CommonMarkWriter;

    /// Helper function to create a writer with GFM features enabled
    fn create_gfm_writer() -> CommonMarkWriter {
        let options = WriterOptionsBuilder::new().enable_gfm().build();
        CommonMarkWriter::with_options(options)
    }

    #[test]
    fn test_strikethrough() {
        // Create a paragraph with strikethrough text
        let node = Node::Paragraph(vec![
            Node::Text("Normal text ".to_string()),
            Node::Strikethrough(vec![Node::Text("struck through text".to_string())]),
            Node::Text(" normal text again".to_string()),
        ]);

        // Write with GFM enabled
        let mut writer = create_gfm_writer();
        writer.write(&node).expect("Failed to write node");
        let result = writer.into_string();

        // Verify result includes strikethrough markers
        let expected = "Normal text ~~struck through text~~ normal text again\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_task_list() {
        // Create task lists with checked and unchecked items
        let node = Node::Document(vec![
            Node::UnorderedList(vec![
                ListItem::Task {
                    status: TaskListStatus::Unchecked,
                    content: vec![Node::Paragraph(vec![Node::Text(
                        "Unchecked task".to_string(),
                    )])],
                },
                ListItem::Task {
                    status: TaskListStatus::Checked,
                    content: vec![Node::Paragraph(vec![Node::Text(
                        "Completed task".to_string(),
                    )])],
                },
            ]),
            // Test with ordered lists too
            Node::OrderedList {
                start: 1,
                items: vec![
                    ListItem::Task {
                        status: TaskListStatus::Unchecked,
                        content: vec![Node::Paragraph(vec![Node::Text(
                            "Unchecked ordered task".to_string(),
                        )])],
                    },
                    ListItem::Task {
                        status: TaskListStatus::Checked,
                        content: vec![Node::Paragraph(vec![Node::Text(
                            "Completed ordered task".to_string(),
                        )])],
                    },
                ],
            },
        ]);

        // Write with GFM enabled
        let mut writer = create_gfm_writer();
        writer.write(&node).expect("Failed to write node");
        let result = writer.into_string();

        // Verify result includes checkbox syntax
        let expected = "- [ ] Unchecked task\n- [x] Completed task\n\n1. [ ] Unchecked ordered task\n2. [x] Completed ordered task\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_table_alignment() {
        // Create a table with different column alignments
        let node = Node::Table {
            headers: vec![
                Node::Text("Left".to_string()),
                Node::Text("Center".to_string()),
                Node::Text("Right".to_string()),
                Node::Text("Default".to_string()),
            ],
            alignments: vec![
                TableAlignment::Left,
                TableAlignment::Center,
                TableAlignment::Right,
                TableAlignment::None,
            ],
            rows: vec![
                vec![
                    Node::Text("L1".to_string()),
                    Node::Text("C1".to_string()),
                    Node::Text("R1".to_string()),
                    Node::Text("D1".to_string()),
                ],
                vec![
                    Node::Text("L2".to_string()),
                    Node::Text("C2".to_string()),
                    Node::Text("R2".to_string()),
                    Node::Text("D2".to_string()),
                ],
            ],
        };

        // Write with GFM enabled
        let mut writer = create_gfm_writer();
        writer.write(&node).expect("Failed to write node");
        let result = writer.into_string();

        // Verify table has correct alignment markers
        let expected = "| Left | Center | Right | Default |\n| :--- | :---: | ---: | --- |\n| L1 | C1 | R1 | D1 |\n| L2 | C2 | R2 | D2 |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extended_autolink() {
        // Test autolinks without angle brackets
        let node = Node::Paragraph(vec![
            Node::Text("Check this link: ".to_string()),
            Node::ExtendedAutolink("https://example.com".to_string()),
            Node::Text(" and continue reading.".to_string()),
        ]);

        // Write with GFM enabled
        let mut writer = create_gfm_writer();
        writer.write(&node).expect("Failed to write node");
        let result = writer.into_string();

        // The extended autolink should be preserved without angle brackets
        let expected = "Check this link: https://example.com and continue reading.\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_html_filtering() {
        // Test with disallowed HTML tags (script)
        let script_element = HtmlElement {
            tag: "script".to_string(),
            attributes: vec![HtmlAttribute {
                name: "type".to_string(),
                value: "text/javascript".to_string(),
            }],
            children: vec![Node::Text("alert('test');".to_string())],
            self_closing: false,
        };

        let node = Node::Paragraph(vec![
            Node::Text("Before script ".to_string()),
            Node::HtmlElement(script_element),
            Node::Text(" after script.".to_string()),
        ]);

        // Write with GFM enabled
        let mut writer = create_gfm_writer();
        writer.write(&node).expect("Failed to write node");
        let result = writer.into_string();

        // The script tag should be escaped to prevent execution
        let expected = "Before script &lt;script type=\"text/javascript\"&gt;alert('test');&lt;/script&gt; after script.\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_allowed_html() {
        // Test with allowed HTML tags (div)
        let div_element = HtmlElement {
            tag: "div".to_string(),
            attributes: vec![HtmlAttribute {
                name: "class".to_string(),
                value: "container".to_string(),
            }],
            children: vec![Node::Text("Content in div".to_string())],
            self_closing: false,
        };

        let node = Node::Paragraph(vec![
            Node::Text("Before div ".to_string()),
            Node::HtmlElement(div_element),
            Node::Text(" after div.".to_string()),
        ]);

        // Write with GFM enabled
        let mut writer = create_gfm_writer();
        writer.write(&node).expect("Failed to write node");
        let result = writer.into_string();

        // The div tag should not be escaped since it's allowed
        let expected = "Before div <div class=\"container\">Content in div</div> after div.\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_factory_methods() {
        // Test factory methods for GFM elements
        let task = Node::task_list_item(
            TaskListStatus::Checked,
            vec![Node::Paragraph(vec![Node::Text(
                "A completed task".to_string(),
            )])],
        );

        let table = Node::table_with_alignment(
            vec![Node::Text("Header".to_string())],
            vec![TableAlignment::Center],
            vec![vec![Node::Text("Cell".to_string())]],
        );

        let document = Node::Document(vec![task, table]);

        let mut writer = create_gfm_writer();
        writer.write(&document).expect("Failed to write document");
        let result = writer.into_string();

        // Expected output with task list and table
        let expected = "- [x] A completed task\n\n| Header |\n| :---: |\n| Cell |\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_gfm_disabled_features() {
        // Create nodes with GFM features
        let node = Node::Document(vec![
            // Strikethrough
            Node::Paragraph(vec![Node::Strikethrough(vec![Node::Text(
                "This should not have tildes when GFM is disabled".to_string(),
            )])]),
            // Task list
            Node::UnorderedList(vec![ListItem::Task {
                status: TaskListStatus::Checked,
                content: vec![Node::Paragraph(vec![Node::Text(
                    "No checkbox when disabled".to_string(),
                )])],
            }]),
        ]);

        // Create options with GFM disabled
        let options = WriterOptionsBuilder::new().build(); // GFM disabled by default

        let mut writer = CommonMarkWriter::with_options(options);
        writer.write(&node).expect("Failed to write node");
        let result = writer.into_string();

        // GFM syntax should not be used when disabled
        let expected =
            "This should not have tildes when GFM is disabled\n\n- No checkbox when disabled\n";
        assert_eq!(result, expected);
    }
}
