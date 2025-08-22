//! Tests for GitHub Flavored Markdown (GFM) features
//!
//! These tests verify that GFM features work correctly when the "gfm" feature is enabled.
//! This includes:
//! - Tables with alignment
//! - Strikethrough
//! - Task lists
//! - Extended autolinks
//! - HTML tag filtering

mod gfm_tests {
    use cmark_writer::ast::{HtmlAttribute, HtmlElement};
    use cmark_writer::ast::{ListItem, Node, TableAlignment, TaskListStatus};
    use cmark_writer::options::WriterOptionsBuilder;
    use cmark_writer::writer::CommonMarkWriter;
    use cmark_writer::ToCommonMark;

    /// Helper function to create a writer with GFM features enabled
    fn create_gfm_writer() -> CommonMarkWriter {
        let options = WriterOptionsBuilder::new().enable_gfm().build();
        CommonMarkWriter::with_options(options)
    }

    #[test]
    fn test_strikethrough() {
        // Create a paragraph with strikethrough text
        let node = Node::Paragraph(vec![
            Node::Text("Normal text ".into()),
            Node::Strikethrough(vec![Node::Text("struck through text".into())]),
            Node::Text(" normal text again".into()),
        ]);

        // Write with GFM enabled
        let mut writer = create_gfm_writer();
        node.to_commonmark(&mut writer)
            .expect("Failed to write node");
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
                    content: vec![Node::Paragraph(vec![Node::Text("Unchecked task".into())])],
                },
                ListItem::Task {
                    status: TaskListStatus::Checked,
                    content: vec![Node::Paragraph(vec![Node::Text("Completed task".into())])],
                },
            ]),
            // Test with ordered lists too
            Node::OrderedList {
                start: 1,
                items: vec![
                    ListItem::Task {
                        status: TaskListStatus::Unchecked,
                        content: vec![Node::Paragraph(vec![Node::Text(
                            "Unchecked ordered task".into(),
                        )])],
                    },
                    ListItem::Task {
                        status: TaskListStatus::Checked,
                        content: vec![Node::Paragraph(vec![Node::Text(
                            "Completed ordered task".into(),
                        )])],
                    },
                ],
            },
        ]);

        // Write with GFM enabled
        let mut writer = create_gfm_writer();
        node.to_commonmark(&mut writer)
            .expect("Failed to write node");
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
                Node::Text("Left".into()),
                Node::Text("Center".into()),
                Node::Text("Right".into()),
                Node::Text("Default".into()),
            ],
            alignments: vec![
                TableAlignment::Left,
                TableAlignment::Center,
                TableAlignment::Right,
                TableAlignment::None,
            ],
            rows: vec![
                vec![
                    Node::Text("L1".into()),
                    Node::Text("C1".into()),
                    Node::Text("R1".into()),
                    Node::Text("D1".into()),
                ],
                vec![
                    Node::Text("L2".into()),
                    Node::Text("C2".into()),
                    Node::Text("R2".into()),
                    Node::Text("D2".into()),
                ],
            ],
        };

        // Write with GFM enabled
        let mut writer = create_gfm_writer();
        node.to_commonmark(&mut writer)
            .expect("Failed to write node");
        let result = writer.into_string();

        // Verify table has correct alignment markers
        let expected = "| Left | Center | Right | Default |\n| :--- | :---: | ---: | --- |\n| L1 | C1 | R1 | D1 |\n| L2 | C2 | R2 | D2 |\n\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extended_autolink() {
        // Test autolinks without angle brackets
        let node = Node::Paragraph(vec![
            Node::Text("Check this link: ".into()),
            Node::ExtendedAutolink("https://example.com".into()),
            Node::Text(" and continue reading.".into()),
        ]);

        // Write with GFM enabled
        let mut writer = create_gfm_writer();
        node.to_commonmark(&mut writer)
            .expect("Failed to write node");
        let result = writer.into_string();

        // The extended autolink should be preserved without angle brackets
        let expected = "Check this link: https://example.com and continue reading.\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_html_filtering() {
        // Test with disallowed HTML tags (script)
        let script_element = HtmlElement {
            tag: "script".into(),
            attributes: vec![HtmlAttribute {
                name: "type".into(),
                value: "text/javascript".into(),
            }],
            children: vec![Node::Text("alert('test');".into())],
            self_closing: false,
        };

        let node = Node::Paragraph(vec![
            Node::Text("Before script ".into()),
            Node::HtmlElement(script_element),
            Node::Text(" after script.".into()),
        ]);

        // Write with GFM enabled
        let mut writer = create_gfm_writer();
        node.to_commonmark(&mut writer)
            .expect("Failed to write node");
        let result = writer.into_string();

        // The script tag should be escaped to prevent execution
        let expected = "Before script &lt;script type=\"text/javascript\"&gt;alert('test');&lt;/script&gt; after script.\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_allowed_html() {
        // Test with allowed HTML tags (div)
        let div_element = HtmlElement {
            tag: "div".into(),
            attributes: vec![HtmlAttribute {
                name: "class".into(),
                value: "container".into(),
            }],
            children: vec![Node::Text("Content in div".into())],
            self_closing: false,
        };

        let node = Node::Paragraph(vec![
            Node::Text("Before div ".into()),
            Node::HtmlElement(div_element),
            Node::Text(" after div.".into()),
        ]);

        // Write with GFM enabled
        let mut writer = create_gfm_writer();
        node.to_commonmark(&mut writer)
            .expect("Failed to write node");
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
            vec![Node::Paragraph(vec![Node::Text("A completed task".into())])],
        );

        let table = Node::table_with_alignment(
            vec![Node::Text("Header".into())],
            vec![TableAlignment::Center],
            vec![vec![Node::Text("Cell".into())]],
        );

        let document = Node::Document(vec![task, table]);

        let mut writer = create_gfm_writer();
        document
            .to_commonmark(&mut writer)
            .expect("Failed to write document");
        let result = writer.into_string();

        // Expected output with task list and table
        let expected = "- [x] A completed task\n\n| Header |\n| :---: |\n| Cell |\n\n";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_gfm_disabled_features() {
        // Create nodes with GFM features
        let node = Node::Document(vec![
            // Strikethrough
            Node::Paragraph(vec![Node::Strikethrough(vec![Node::Text(
                "This should not have tildes when GFM is disabled".into(),
            )])]),
            // Task list
            Node::UnorderedList(vec![ListItem::Task {
                status: TaskListStatus::Checked,
                content: vec![Node::Paragraph(vec![Node::Text(
                    "No checkbox when disabled".into(),
                )])],
            }]),
        ]);

        // Create options with GFM disabled
        let options = WriterOptionsBuilder::new().build(); // GFM disabled by default

        let mut writer = CommonMarkWriter::with_options(options);
        node.to_commonmark(&mut writer)
            .expect("Failed to write node");
        let result = writer.into_string();

        // GFM syntax should not be used when disabled
        let expected =
            "This should not have tildes when GFM is disabled\n\n- No checkbox when disabled\n";
        assert_eq!(result, expected);
    }
}
