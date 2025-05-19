#[cfg(test)]
mod tests {
    use cmark_writer::ast::{HtmlElement, ListItem, Node};
    #[cfg(feature = "gfm")]
    use cmark_writer::ast::{TableAlignment, TaskListStatus};
    use cmark_writer::writer::{HtmlRenderOptions, HtmlWriteResult, HtmlWriter};
    use log::{LevelFilter, Log};
    use std::io::Cursor;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn setup_logger() {
        INIT.call_once(|| {
            struct TestLogger;
            impl Log for TestLogger {
                fn enabled(&self, metadata: &log::Metadata) -> bool {
                    metadata.level() <= LevelFilter::Warn
                }

                fn log(&self, record: &log::Record) {
                    if self.enabled(record.metadata()) {
                        let color_code = match record.level() {
                            log::Level::Error => "\x1b[31m", // 红色
                            log::Level::Warn => "\x1b[33m",  // 黄色
                            log::Level::Info => "\x1b[32m",  // 绿色
                            log::Level::Debug => "\x1b[34m", // 蓝色
                            log::Level::Trace => "\x1b[90m", // 灰色
                        };
                        let reset = "\x1b[0m";
                        println!(
                            "{}[{}]{} {}: {}",
                            color_code,
                            record.level(),
                            reset,
                            record.target(),
                            record.args()
                        );
                    }
                }

                fn flush(&self) {}
            }

            log::set_boxed_logger(Box::new(TestLogger))
                .map(|()| log::set_max_level(LevelFilter::Warn))
                .expect("Failed to initialize logger");
        });
    }

    // Helper function to render a node to string with given options
    fn render_node_to_html(node: &Node, options: &HtmlRenderOptions) -> HtmlWriteResult<String> {
        let mut buffer = Cursor::new(Vec::new());
        let mut html_writer = HtmlWriter::new(&mut buffer);
        html_writer.write_node(node, options)?;
        html_writer.flush()?;
        Ok(String::from_utf8(buffer.into_inner()).unwrap())
    }

    // Helper function to render a node to string with default options
    fn render_node_to_html_default(node: &Node) -> HtmlWriteResult<String> {
        render_node_to_html(node, &HtmlRenderOptions::default())
    }

    #[test]
    fn test_paragraph_and_text() {
        let node = Node::Paragraph(vec![Node::Text("Hello HTML world!".to_string())]);
        let expected_html = "<p>Hello HTML world!</p>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_text_escaping() {
        let node = Node::Paragraph(vec![Node::Text("Hello < & > \" ' world!".to_string())]);
        let expected_html = "<p>Hello &lt; &amp; &gt; &quot; &#39; world!</p>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_heading() {
        let node = Node::Heading {
            level: 1,
            content: vec![Node::Text("Title".to_string())],
            heading_type: Default::default(),
        };
        let expected_html = "<h1>Title</h1>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_emphasis_and_strong() {
        let node = Node::Paragraph(vec![
            Node::Text("This is ".to_string()),
            Node::Emphasis(vec![Node::Text("emphasized".to_string())]),
            Node::Text(" and this is ".to_string()),
            Node::Strong(vec![Node::Text("strong".to_string())]),
            Node::Text("!".to_string()),
        ]);
        let expected_html =
            "<p>This is <em>emphasized</em> and this is <strong>strong</strong>!</p>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_thematic_break() {
        let node = Node::ThematicBreak;
        let expected_html = "<hr />\n";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_inline_code() {
        let node = Node::InlineCode("let x = 1;".to_string());
        let expected_html = "<code>let x = 1;</code>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_code_block_default_options() {
        let node = Node::CodeBlock {
            language: Some("rust".to_string()),
            content: "fn main() {\n    println!(\"Hello\");\n}".to_string(),
            block_type: Default::default(),
        };
        // Default prefix is "language-"
        let expected_html = "<pre class=\"language-rust\"><code>fn main() {\n    println!(&quot;Hello&quot;);\n}</code></pre>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_code_block_custom_options() {
        let node = Node::CodeBlock {
            language: Some("python".to_string()),
            content: "print(\"Hello\")".to_string(),
            block_type: Default::default(),
        };
        #[cfg(feature = "gfm")]
        let options = HtmlRenderOptions {
            code_block_language_class_prefix: Some("lang-".to_string()),
            strict: false,
            ..Default::default()
        };
        #[cfg(not(feature = "gfm"))]
        let options = HtmlRenderOptions {
            code_block_language_class_prefix: Some("lang-".to_string()),
            strict: false,
        };
        let expected_html =
            "<pre class=\"lang-python\"><code>print(&quot;Hello&quot;)</code></pre>";
        assert_eq!(render_node_to_html(&node, &options).unwrap(), expected_html);
    }

    #[test]
    fn test_code_block_no_prefix_option() {
        let node = Node::CodeBlock {
            language: Some("rust".to_string()),
            content: "let _ = 1;".to_string(),
            block_type: Default::default(),
        };
        #[cfg(feature = "gfm")]
        let options = HtmlRenderOptions {
            code_block_language_class_prefix: None,
            strict: false,
            ..Default::default()
        };
        #[cfg(not(feature = "gfm"))]
        let options = HtmlRenderOptions {
            code_block_language_class_prefix: None,
            strict: false,
        };
        // No class attribute should be present if prefix is None
        let expected_html = "<pre><code>let _ = 1;</code></pre>";
        assert_eq!(render_node_to_html(&node, &options).unwrap(), expected_html);
    }

    #[test]
    fn test_code_block_no_language() {
        let node = Node::CodeBlock {
            language: None,
            content: "plain text".to_string(),
            block_type: Default::default(),
        };
        let expected_html = "<pre><code>plain text</code></pre>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_link() {
        let node = Node::Link {
            url: "https://example.com".to_string(),
            title: Some("Example Domain".to_string()),
            content: vec![Node::Text("Visit Example".to_string())],
        };
        let expected_html =
            "<a href=\"https://example.com\" title=\"Example Domain\">Visit Example</a>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_image() {
        let node = Node::Image {
            url: "/logo.png".to_string(),
            title: Some("Logo".to_string()),
            alt: vec![Node::Text("Site Logo".to_string())],
        };
        let expected_html = "<img src=\"/logo.png\" alt=\"Site Logo\" title=\"Logo\" />";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_unordered_list() {
        let node = Node::UnorderedList(vec![
            ListItem::Unordered {
                content: vec![Node::Paragraph(vec![Node::Text("Item 1".to_string())])],
            },
            ListItem::Unordered {
                content: vec![Node::Paragraph(vec![Node::Text("Item 2".to_string())])],
            },
        ]);
        let expected_html = "<ul><li><p>Item 1</p></li><li><p>Item 2</p></li></ul>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_ordered_list() {
        let node = Node::OrderedList {
            start: 3,
            items: vec![
                ListItem::Ordered {
                    number: None,
                    content: vec![Node::Paragraph(vec![Node::Text("Item A".to_string())])],
                },
                ListItem::Ordered {
                    number: Some(5),
                    content: vec![Node::Paragraph(vec![Node::Text("Item B".to_string())])],
                },
            ],
        };
        let expected_html = "<ol start=\"3\"><li><p>Item A</p></li><li><p>Item B</p></li></ol>";
        // Note: Our current ListItem::to_html doesn't use the inner `number` for <li value="...">.
        // CommonMark to HTML spec usually just outputs <li> and relies on <ol start="...">.
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_html_block() {
        let node = Node::HtmlBlock("<div class=\"foo\">Bar</div>".to_string());
        let expected_html = "<div class=\"foo\">Bar</div>"; // raw_html is used
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_html_element() {
        let element = HtmlElement {
            tag: "my-custom-tag".to_string(),
            attributes: vec![cmark_writer::ast::HtmlAttribute {
                name: "data-val".to_string(),
                value: "xyz".to_string(),
            }],
            children: vec![Node::Text("Content".to_string())],
            self_closing: false,
        };
        let node = Node::HtmlElement(element);
        let expected_html = "<my-custom-tag data-val=\"xyz\">Content</my-custom-tag>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_self_closing_html_element() {
        let element = HtmlElement {
            tag: "hr".to_string(),
            attributes: vec![cmark_writer::ast::HtmlAttribute {
                name: "class".to_string(),
                value: "fancy".to_string(),
            }],
            children: vec![],
            self_closing: true,
        };
        let node = Node::HtmlElement(element);
        let expected_html = "<hr class=\"fancy\" />";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[cfg(feature = "gfm")]
    #[test]
    fn test_strikethrough_gfm() {
        let node = Node::Strikethrough(vec![Node::Text("deleted".to_string())]);
        let expected_html = "<del>deleted</del>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[cfg(feature = "gfm")]
    #[test]
    fn test_task_list_item_gfm() {
        let unchecked_item = ListItem::Task {
            status: TaskListStatus::Unchecked,
            content: vec![Node::Text("To do".to_string())],
        };
        let checked_item = ListItem::Task {
            status: TaskListStatus::Checked,
            content: vec![Node::Text("Done".to_string())],
        };
        let node = Node::UnorderedList(vec![unchecked_item, checked_item]);
        let options = HtmlRenderOptions {
            enable_gfm: true,
            ..HtmlRenderOptions::default()
        };
        let expected_html = "<ul><li class=\"task-list-item task-list-item-unchecked\"><input type=\"checkbox\" disabled=\"\" /> To do</li><li class=\"task-list-item task-list-item-checked\"><input type=\"checkbox\" disabled=\"\" checked=\"\" /> Done</li></ul>";
        assert_eq!(render_node_to_html(&node, &options).unwrap(), expected_html);
    }

    #[test]
    fn test_blockquote() {
        let node = Node::BlockQuote(vec![
            Node::Paragraph(vec![Node::Text("This is a quote.".to_string())]),
            Node::Paragraph(vec![Node::Text("Another paragraph in quote.".to_string())]),
        ]);
        let expected_html =
            "<blockquote><p>This is a quote.</p><p>Another paragraph in quote.</p></blockquote>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_autolink_uri() {
        let node = Node::Autolink {
            url: "https://example.com".to_string(),
            is_email: false,
        };
        let expected_html = "<a href=\"https://example.com\">https://example.com</a>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_autolink_email() {
        let node = Node::Autolink {
            url: "test@example.com".to_string(),
            is_email: true,
        };
        let expected_html = "<a href=\"mailto:test@example.com\">test@example.com</a>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_extended_autolink() {
        // GFM, but our Node::ExtendedAutolink is not conditional
        let node = Node::ExtendedAutolink("www.example.com/path".to_string());
        let expected_html = "<a href=\"www.example.com/path\">www.example.com/path</a>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_reference_link_full() {
        // Assuming ReferenceLink implies it was not resolved, so renders as text.
        let node = Node::ReferenceLink {
            label: "lbl".to_string(),
            content: vec![Node::Text("link text".to_string())],
        };
        let options = HtmlRenderOptions {
            strict: false,
            ..HtmlRenderOptions::default()
        };
        let expected_html = "[link text][lbl]";
        assert_eq!(render_node_to_html(&node, &options).unwrap(), expected_html);
    }

    #[test]
    fn test_reference_link_shortcut() {
        let node = Node::ReferenceLink {
            label: "shortcut".to_string(),
            content: vec![], // Empty content means use label as text
        };
        let options = HtmlRenderOptions {
            strict: false,
            ..HtmlRenderOptions::default()
        };
        let expected_html = "[shortcut][shortcut]";
        assert_eq!(render_node_to_html(&node, &options).unwrap(), expected_html);
    }

    #[test]
    fn test_table_basic() {
        let node = Node::Table {
            headers: vec![
                Node::Text("Header 1".to_string()),
                Node::Text("Header 2".to_string()),
            ],
            #[cfg(feature = "gfm")]
            alignments: vec![], // No specific GFM alignment for this basic test
            rows: vec![
                vec![
                    Node::Text("Cell 1.1".to_string()),
                    Node::Text("Cell 1.2".to_string()),
                ],
                vec![
                    Node::Text("Cell 2.1".to_string()),
                    Node::Text("Cell 2.2".to_string()),
                ],
            ],
        };
        let expected_html = "<table><thead><tr><th>Header 1</th><th>Header 2</th></tr></thead><tbody><tr><td>Cell 1.1</td><td>Cell 1.2</td></tr><tr><td>Cell 2.1</td><td>Cell 2.2</td></tr></tbody></table>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[cfg(feature = "gfm")]
    #[test]
    fn test_table_with_gfm_alignment() {
        let node = Node::Table {
            headers: vec![
                Node::Text("H1".to_string()),
                Node::Text("H2".to_string()),
                Node::Text("H3".to_string()),
            ],
            alignments: vec![
                TableAlignment::Left,
                TableAlignment::Center,
                TableAlignment::Right,
            ],
            rows: vec![vec![
                Node::Text("L".to_string()),
                Node::Text("C".to_string()),
                Node::Text("R".to_string()),
            ]],
        };
        let expected_html = "<table><thead><tr><th style=\"text-align: left;\">H1</th><th style=\"text-align: center;\">H2</th><th style=\"text-align: right;\">H3</th></tr></thead><tbody><tr><td style=\"text-align: left;\">L</td><td style=\"text-align: center;\">C</td><td style=\"text-align: right;\">R</td></tr></tbody></table>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_warning_output() {
        setup_logger();

        // 测试无效的 HTML 标签
        let element = HtmlElement {
            tag: "invalid<tag>".to_string(),
            attributes: vec![],
            children: vec![Node::Text("Content".to_string())],
            self_closing: false,
        };
        let node = Node::HtmlElement(element);
        let options = HtmlRenderOptions {
            strict: false,
            ..HtmlRenderOptions::default()
        };

        // HTML 输出应该不受警告影响
        let expected_html = "&lt;invalid&lt;tag&gt;&gt;Content&lt;/invalid&lt;tag&gt;&gt;";
        let actual_html = render_node_to_html(&node, &options).unwrap();
        assert_eq!(actual_html, expected_html);

        // 警告信息应该只输出到控制台，不影响 HTML 输出
        // 注意：这个测试主要验证 HTML 输出是否正确，警告信息会在控制台看到
    }

    #[test]
    fn test_invalid_html_attribute_non_strict() {
        let element = HtmlElement {
            tag: "div".to_string(),
            attributes: vec![cmark_writer::ast::HtmlAttribute {
                name: "invalid<attr>".to_string(),
                value: "value".to_string(),
            }],
            children: vec![Node::Text("Content".to_string())],
            self_closing: false,
        };
        let node = Node::HtmlElement(element);
        let options = HtmlRenderOptions {
            strict: false,
            ..HtmlRenderOptions::default()
        };
        let expected_html = "<div> invalid&lt;attr&gt;=&quot;value&quot;Content</div>";
        assert_eq!(render_node_to_html(&node, &options).unwrap(), expected_html);
    }

    #[cfg(feature = "gfm")]
    #[test]
    fn test_disallowed_html_tag_gfm() {
        let element = HtmlElement {
            tag: "script".to_string(),
            attributes: vec![],
            children: vec![Node::Text("alert('test')".to_string())],
            self_closing: false,
        };
        let node = Node::HtmlElement(element);
        let options = HtmlRenderOptions {
            enable_gfm: true,
            ..HtmlRenderOptions::default()
        };
        let expected_html = "<script>alert(&#39;test&#39;)</script>";
        assert_eq!(render_node_to_html(&node, &options).unwrap(), expected_html);
    }

    #[test]
    fn test_reference_link_warning() {
        let node = Node::ReferenceLink {
            label: "unresolved".to_string(),
            content: vec![Node::Text("Unresolved Link".to_string())],
        };
        let options = HtmlRenderOptions {
            strict: false,
            ..HtmlRenderOptions::default()
        };
        let expected_html = "[Unresolved Link][unresolved]";
        assert_eq!(render_node_to_html(&node, &options).unwrap(), expected_html);
    }

    #[cfg(feature = "gfm")]
    #[test]
    fn test_gfm_warning_output() {
        setup_logger();

        // 测试 GFM 模式下被禁用的 HTML 标签
        let element = HtmlElement {
            tag: "script".to_string(),
            attributes: vec![],
            children: vec![Node::Text("alert('test')".to_string())],
            self_closing: false,
        };
        let node = Node::HtmlElement(element);
        let options = HtmlRenderOptions {
            enable_gfm: true,
            gfm_disallowed_html_tags: vec!["script".to_string()],
            ..HtmlRenderOptions::default()
        };

        // HTML 输出应该不受警告影响
        let expected_html = "&lt;script&gt;alert(&#39;test&#39;)&lt;/script&gt;";
        let actual_html = render_node_to_html(&node, &options).unwrap();
        assert_eq!(actual_html, expected_html);

        // 警告信息应该只输出到控制台，不影响 HTML 输出
        // 注意：这个测试主要验证 HTML 输出是否正确，警告信息会在控制台看到
    }
}
