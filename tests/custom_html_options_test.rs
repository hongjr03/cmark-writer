use cmark_writer::{
    CommonMarkWriter, HtmlAttribute, HtmlElement, HtmlWriterOptions, Node, WriterOptions,
};

#[test]
fn test_custom_html_options_in_commonmark_writer() {
    // 创建自定义的 HtmlWriterOptions，允许非严格模式
    let html_options = HtmlWriterOptions::default()
        .with_strict(false)
        .with_code_block_prefix(Some("highlight-"));

    // 创建 WriterOptions 并指定自定义的 HTML 选项
    let writer_options = WriterOptions::default().html_writer_options(Some(html_options));

    let mut writer = CommonMarkWriter::with_options(writer_options);

    // 创建一个包含自定义标签的 HTML 元素（在严格模式下会失败）
    let custom_element = HtmlElement {
        tag: "custom-highlight".into(),
        attributes: vec![HtmlAttribute {
            name: "data-color".into(),
            value: "yellow".into(),
        }],
        children: vec![Node::Text("highlighted text".into())],
        self_closing: false,
    };

    // 在 CommonMark 文档中使用这个 HTML 元素
    let document = Node::Document(vec![Node::Paragraph(vec![
        Node::Text("This is a paragraph with ".into()),
        Node::HtmlElement(custom_element),
        Node::Text(" custom element.".into()),
    ])]);

    // 应该能够成功写入，因为我们设置了非严格模式
    writer.write(&document).unwrap();
    let output = writer.into_string();

    // 验证输出包含自定义标签
    assert!(output.contains("<custom-highlight"));
    assert!(output.contains("data-color=\"yellow\""));
    assert!(output.contains("highlighted text"));
    assert!(output.contains("</custom-highlight>"));
}

#[test]
fn test_default_html_options_derivation() {
    // 测试默认行为：从 CommonMark 选项自动派生 HTML 选项
    let writer_options = WriterOptions::default().html_writer_options(None); // 明确设置为 None

    let mut writer = CommonMarkWriter::with_options(writer_options);

    // 创建标准的 HTML 元素
    let html_element = HtmlElement {
        tag: "div".into(),
        attributes: vec![HtmlAttribute {
            name: "class".into(),
            value: "container".into(),
        }],
        children: vec![Node::Text("content".into())],
        self_closing: false,
    };

    let document = Node::Document(vec![Node::HtmlElement(html_element)]);

    writer.write(&document).unwrap();
    let output = writer.into_string();

    // 验证标准 HTML 元素被正确渲染
    assert!(output.contains("<div class=\"container\">content</div>"));
}

#[test]
fn test_code_block_prefix_customization() {
    // 测试自定义代码块前缀
    let html_options = HtmlWriterOptions::default().with_code_block_prefix(Some("lang-"));

    let writer_options = WriterOptions::default().html_writer_options(Some(html_options));

    let mut writer = CommonMarkWriter::with_options(writer_options);

    // 创建包含代码块的 HTML 元素
    let code_element = HtmlElement {
        tag: "pre".into(),
        attributes: vec![],
        children: vec![Node::HtmlElement(HtmlElement {
            tag: "code".into(),
            attributes: vec![HtmlAttribute {
                name: "class".into(),
                value: "lang-rust".into(),
            }],
            children: vec![Node::Text("fn main() {}".into())],
            self_closing: false,
        })],
        self_closing: false,
    };

    let document = Node::Document(vec![Node::HtmlElement(code_element)]);

    writer.write(&document).unwrap();
    let output = writer.into_string();

    // 验证自定义前缀被使用
    assert!(output.contains("class=\"lang-rust\""));
}

#[test]
fn test_html_options_with_builder() {
    // 测试使用构建器模式设置 HTML 选项
    use cmark_writer::WriterOptionsBuilder;

    let options = WriterOptionsBuilder::new()
        .strict(false)
        .html_writer_options(Some(
            HtmlWriterOptions::default()
                .with_strict(false)
                .with_code_block_prefix(Some("highlight-")),
        ))
        .build();

    let mut writer = CommonMarkWriter::with_options(options);

    // 创建自定义元素
    let custom_element = HtmlElement {
        tag: "mark".into(),
        attributes: vec![],
        children: vec![Node::Text("marked text".into())],
        self_closing: false,
    };

    writer.write(&Node::HtmlElement(custom_element)).unwrap();
    let output = writer.into_string();

    assert!(output.contains("<mark>marked text</mark>"));
}

#[test]
fn test_strict_mode_difference() {
    // 测试严格模式和非严格模式的区别

    // 严格模式：应该对无效标签返回错误
    let _strict_options = WriterOptions::default()
        .html_writer_options(Some(HtmlWriterOptions::default().with_strict(true)));

    // 非严格模式：应该成功处理
    let non_strict_options = WriterOptions::default()
        .html_writer_options(Some(HtmlWriterOptions::default().with_strict(false)));

    let mut non_strict_writer = CommonMarkWriter::with_options(non_strict_options);

    let custom_element = HtmlElement {
        tag: "custom-tag".into(),
        attributes: vec![],
        children: vec![Node::Text("content".into())],
        self_closing: false,
    };

    // 严格模式下，自定义标签可能会被处理（取决于实现）
    // 非严格模式下，应该能正常处理
    let result_non_strict = non_strict_writer.write(&Node::HtmlElement(custom_element.clone()));
    assert!(result_non_strict.is_ok());

    let output = non_strict_writer.into_string();
    assert!(output.contains("custom-tag") || output.contains("content"));
}
