#[cfg(test)]
mod tests {
    use crate::support::{html as support_html, logger};
    use cmark_writer::ast::{HtmlElement, ListItem, Node};
    #[cfg(feature = "gfm")]
    use cmark_writer::ast::{TableAlignment, TaskListStatus};
    use cmark_writer::writer::HtmlWriterOptions;
    use ecow::EcoString;
    use log::LevelFilter;

    fn setup_logger() {
        logger::init(LevelFilter::Warn);
    }

    // Helper function wrappers to shared support helpers
    fn render_node_to_html(
        node: &Node,
        options: &HtmlWriterOptions,
    ) -> cmark_writer::writer::HtmlWriteResult<EcoString> {
        support_html::render_node(node, options)
    }
    fn render_node_to_html_default(
        node: &Node,
    ) -> cmark_writer::writer::HtmlWriteResult<EcoString> {
        support_html::render_node_default(node)
    }

    #[test]
    fn test_paragraph_and_text() {
        let node = Node::Paragraph(vec![Node::Text("Hello HTML world!".into())]);
        let expected_html = "<p>Hello HTML world!</p>\n";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_text_escaping() {
        let node = Node::Paragraph(vec![Node::Text("Hello < & > \" ' world!".into())]);
        let expected_html = "<p>Hello &lt; &amp; &gt; \" ' world!</p>\n";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_heading() {
        let node = Node::Heading {
            level: 1,
            content: vec![Node::Text("Title".into())],
            heading_type: Default::default(),
        };
        let expected_html = "<h1>Title</h1>\n";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_emphasis_and_strong() {
        let node = Node::Paragraph(vec![
            Node::Text("This is ".into()),
            Node::Emphasis(vec![Node::Text("emphasized".into())]),
            Node::Text(" and this is ".into()),
            Node::Strong(vec![Node::Text("strong".into())]),
            Node::Text("!".into()),
        ]);
        let expected_html =
            "<p>This is <em>emphasized</em> and this is <strong>strong</strong>!</p>\n";
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
        let node = Node::InlineCode("let x = 1;".into());
        let expected_html = "<code>let x = 1;</code>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_code_block_default_options() {
        let node = Node::CodeBlock {
            language: Some("rust".into()),
            content: "fn main() {\n    println!(\"Hello\");\n}".into(),
            block_type: Default::default(),
        };
        // Default prefix is "language-"
        let expected_html = "<pre><code class=\"language-rust\">fn main() {\n    println!(\"Hello\");\n}</code></pre>\n";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_code_block_custom_options() {
        let node = Node::CodeBlock {
            language: Some("python".into()),
            content: "print(\"Hello\")".into(),
            block_type: Default::default(),
        };
        #[cfg(feature = "gfm")]
        let options = HtmlWriterOptions {
            code_block_language_class_prefix: Some("lang-".into()),
            strict: false,
            ..Default::default()
        };
        #[cfg(not(feature = "gfm"))]
        let options = HtmlWriterOptions {
            code_block_language_class_prefix: Some("lang-".into()),
            strict: false,
        };
        let expected_html = "<pre><code class=\"lang-python\">print(\"Hello\")</code></pre>\n";
        assert_eq!(render_node_to_html(&node, &options).unwrap(), expected_html);
    }

    #[test]
    fn test_code_block_no_prefix_option() {
        let node = Node::CodeBlock {
            language: Some("rust".into()),
            content: "let _ = 1;".into(),
            block_type: Default::default(),
        };
        #[cfg(feature = "gfm")]
        let options = HtmlWriterOptions {
            code_block_language_class_prefix: None,
            strict: false,
            ..Default::default()
        };
        #[cfg(not(feature = "gfm"))]
        let options = HtmlWriterOptions {
            code_block_language_class_prefix: None,
            strict: false,
        };
        // No class attribute should be present if prefix is None
        let expected_html = "<pre><code>let _ = 1;</code></pre>\n";
        assert_eq!(render_node_to_html(&node, &options).unwrap(), expected_html);
    }

    #[test]
    fn test_code_block_no_language() {
        let node = Node::CodeBlock {
            language: None,
            content: "plain text".into(),
            block_type: Default::default(),
        };
        let expected_html = "<pre><code>plain text</code></pre>\n";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_link() {
        let node = Node::Link {
            url: "https://example.com".into(),
            title: Some("Example Domain".into()),
            content: vec![Node::Text("Visit Example".into())],
        };
        let expected_html =
            "<a href=\"https://example.com\" title=\"Example Domain\">Visit Example</a>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_image() {
        let node = Node::Image {
            url: "/logo.png".into(),
            title: Some("Logo".into()),
            alt: vec![Node::Text("Site Logo".into())],
        };
        let expected_html = "<img src=\"/logo.png\" alt=\"Site Logo\" title=\"Logo\" />";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_unordered_list() {
        let node = Node::UnorderedList(vec![
            ListItem::Unordered {
                content: vec![Node::Paragraph(vec![Node::Text("Item 1".into())])],
            },
            ListItem::Unordered {
                content: vec![Node::Paragraph(vec![Node::Text("Item 2".into())])],
            },
        ]);
        let expected_html = "<ul>\n<li><p>Item 1</p>\n</li>\n<li><p>Item 2</p>\n</li>\n</ul>\n";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_ordered_list() {
        let node = Node::OrderedList {
            start: 3,
            items: vec![
                ListItem::Ordered {
                    number: None,
                    content: vec![Node::Paragraph(vec![Node::Text("Item A".into())])],
                },
                ListItem::Ordered {
                    number: Some(5),
                    content: vec![Node::Paragraph(vec![Node::Text("Item B".into())])],
                },
            ],
        };
        let expected_html =
            "<ol start=\"3\">\n<li><p>Item A</p>\n</li>\n<li><p>Item B</p>\n</li>\n</ol>\n";
        // Note: Our current ListItem::to_html doesn't use the inner `number` for <li value="...">.
        // CommonMark to HTML spec usually just outputs <li> and relies on <ol start="...">.
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_html_block() {
        let node = Node::HtmlBlock("<div class=\"foo\">Bar</div>".into());
        let expected_html = "<div class=\"foo\">Bar</div>\n";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_html_element() {
        let element = HtmlElement {
            tag: "my-custom-tag".into(),
            attributes: vec![cmark_writer::ast::HtmlAttribute {
                name: "data-val".into(),
                value: "xyz".into(),
            }],
            children: vec![Node::Text("Content".into())],
            self_closing: false,
        };
        let node = Node::HtmlElement(element);
        let expected_html = "<my-custom-tag data-val=\"xyz\">Content</my-custom-tag>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_self_closing_html_element() {
        let element = HtmlElement {
            tag: "hr".into(),
            attributes: vec![cmark_writer::ast::HtmlAttribute {
                name: "class".into(),
                value: "fancy".into(),
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
        let node = Node::Strikethrough(vec![Node::Text("deleted".into())]);
        let expected_html = "<del>deleted</del>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[cfg(feature = "gfm")]
    #[test]
    fn test_task_list_item_gfm() {
        let unchecked_item = ListItem::Task {
            status: TaskListStatus::Unchecked,
            content: vec![Node::Text("To do".into())],
        };
        let checked_item = ListItem::Task {
            status: TaskListStatus::Checked,
            content: vec![Node::Text("Done".into())],
        };
        let node = Node::UnorderedList(vec![unchecked_item, checked_item]);
        let options = HtmlWriterOptions {
            enable_gfm: true,
            ..HtmlWriterOptions::default()
        };
        let expected_html = "<ul>\n<li class=\"task-list-item\"><input type=\"checkbox\" disabled=\"\" /> To do</li>\n<li class=\"task-list-item task-list-item-checked\"><input type=\"checkbox\" disabled=\"\" checked=\"\" /> Done</li>\n</ul>\n";
        assert_eq!(render_node_to_html(&node, &options).unwrap(), expected_html);
    }

    #[test]
    fn test_blockquote() {
        let node = Node::BlockQuote(vec![
            Node::Paragraph(vec![Node::Text("This is a quote.".into())]),
            Node::Paragraph(vec![Node::Text("Another paragraph in quote.".into())]),
        ]);
        let expected_html =
            "<blockquote>\n<p>This is a quote.</p>\n<p>Another paragraph in quote.</p>\n</blockquote>\n";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_autolink_uri() {
        let node = Node::Autolink {
            url: "https://example.com".into(),
            is_email: false,
        };
        let expected_html = "<a href=\"https://example.com\">https://example.com</a>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_autolink_email() {
        let node = Node::Autolink {
            url: "test@example.com".into(),
            is_email: true,
        };
        let expected_html = "<a href=\"mailto:test@example.com\">test@example.com</a>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    #[cfg(feature = "gfm")]
    fn test_extended_autolink() {
        // GFM, but our Node::ExtendedAutolink is not conditional
        let node = Node::ExtendedAutolink("www.example.com/path".into());
        let expected_html = "<a href=\"www.example.com/path\">www.example.com/path</a>";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_reference_link_full() {
        // Assuming ReferenceLink implies it was not resolved, so renders as text.
        let node = Node::ReferenceLink {
            label: "lbl".into(),
            content: vec![Node::Text("link text".into())],
        };
        let options = HtmlWriterOptions {
            strict: false,
            ..HtmlWriterOptions::default()
        };
        let expected_html = "[link text][lbl]";
        assert_eq!(render_node_to_html(&node, &options).unwrap(), expected_html);
    }

    #[test]
    fn test_reference_link_shortcut() {
        let node = Node::ReferenceLink {
            label: "shortcut".into(),
            content: vec![], // Empty content means use label as text
        };
        let options = HtmlWriterOptions {
            strict: false,
            ..HtmlWriterOptions::default()
        };
        let expected_html = "[shortcut]";
        assert_eq!(render_node_to_html(&node, &options).unwrap(), expected_html);
    }

    #[test]
    fn test_table_basic() {
        let node = Node::Table {
            headers: vec![Node::Text("Header 1".into()), Node::Text("Header 2".into())],
            #[cfg(feature = "gfm")]
            alignments: vec![], // No specific GFM alignment for this basic test
            rows: vec![
                vec![Node::Text("Cell 1.1".into()), Node::Text("Cell 1.2".into())],
                vec![Node::Text("Cell 2.1".into()), Node::Text("Cell 2.2".into())],
            ],
        };
        let expected_html = "<table>\n<thead>\n<tr>\n<th>Header 1</th>\n<th>Header 2</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td>Cell 1.1</td>\n<td>Cell 1.2</td>\n</tr>\n<tr>\n<td>Cell 2.1</td>\n<td>Cell 2.2</td>\n</tr>\n</tbody>\n</table>\n";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[cfg(feature = "gfm")]
    #[test]
    fn test_table_with_gfm_alignment() {
        let node = Node::Table {
            headers: vec![
                Node::Text("H1".into()),
                Node::Text("H2".into()),
                Node::Text("H3".into()),
            ],
            alignments: vec![
                TableAlignment::Left,
                TableAlignment::Center,
                TableAlignment::Right,
            ],
            rows: vec![vec![
                Node::Text("L".into()),
                Node::Text("C".into()),
                Node::Text("R".into()),
            ]],
        };
        let expected_html = "<table>\n<thead>\n<tr>\n<th style=\"text-align: left;\">H1</th>\n<th style=\"text-align: center;\">H2</th>\n<th style=\"text-align: right;\">H3</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td style=\"text-align: left;\">L</td>\n<td style=\"text-align: center;\">C</td>\n<td style=\"text-align: right;\">R</td>\n</tr>\n</tbody>\n</table>\n";
        assert_eq!(render_node_to_html_default(&node).unwrap(), expected_html);
    }

    #[test]
    fn test_warning_output() {
        setup_logger();

        // 测试无效的 HTML 标签
        let element = HtmlElement {
            tag: "invalid<tag>".into(),
            attributes: vec![],
            children: vec![Node::Text("Content".into())],
            self_closing: false,
        };
        let node = Node::HtmlElement(element);
        let options = HtmlWriterOptions {
            strict: false,
            ..HtmlWriterOptions::default()
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
            tag: "div".into(),
            attributes: vec![cmark_writer::ast::HtmlAttribute {
                name: "invalid<attr>".into(),
                value: "value".into(),
            }],
            children: vec![Node::Text("Content".into())],
            self_closing: false,
        };
        let node = Node::HtmlElement(element);
        let options = HtmlWriterOptions {
            strict: false,
            ..HtmlWriterOptions::default()
        };
        let expected_html = "<div invalid<attr>=\"value\">Content</div>";
        assert_eq!(render_node_to_html(&node, &options).unwrap(), expected_html);
    }

    #[cfg(feature = "gfm")]
    #[test]
    fn test_disallowed_html_tag_gfm() {
        let element = HtmlElement {
            tag: "script".into(),
            attributes: vec![],
            children: vec![Node::Text("alert('test')".into())],
            self_closing: false,
        };
        let node = Node::HtmlElement(element);
        let options = HtmlWriterOptions {
            enable_gfm: true,
            ..HtmlWriterOptions::default()
        };
        let expected_html = "<script>alert('test')</script>";
        assert_eq!(render_node_to_html(&node, &options).unwrap(), expected_html);
    }

    #[test]
    fn test_reference_link_warning() {
        let node = Node::ReferenceLink {
            label: "unresolved".into(),
            content: vec![Node::Text("Unresolved Link".into())],
        };
        let options = HtmlWriterOptions {
            strict: false,
            ..HtmlWriterOptions::default()
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
            tag: "script".into(),
            attributes: vec![],
            children: vec![Node::Text("alert('test')".into())],
            self_closing: false,
        };
        let node = Node::HtmlElement(element);
        let options = HtmlWriterOptions {
            enable_gfm: true,
            gfm_disallowed_html_tags: vec!["script".into()],
            ..HtmlWriterOptions::default()
        };

        // HTML 输出应该不受警告影响
        let expected_html = "&lt;script&gt;alert('test')&lt;/script&gt;";
        let actual_html = render_node_to_html(&node, &options).unwrap();
        assert_eq!(actual_html, expected_html);

        // 警告信息应该只输出到控制台，不影响 HTML 输出
        // 注意：这个测试主要验证 HTML 输出是否正确，警告信息会在控制台看到
    }
}
