use cmark_writer::{
    CommonMarkWriter, HtmlAttribute, HtmlElement, HtmlWriter, HtmlWriterOptions, Node,
    ToCommonMark, ToHtml, WriterOptions,
};

#[test]
fn test_derived_html_options() {
    // 测试从 CommonMarkWriter 选项派生 HTML 选项
    let cmark_options = WriterOptions {
        strict: false,
        indent_spaces: 2,
        ..Default::default()
    };

    let mut cmark_writer = CommonMarkWriter::with_options(cmark_options);

    // 创建一个包含 HTML 元素的节点
    let html_element = HtmlElement {
        tag: "div".into(),
        attributes: vec![HtmlAttribute {
            name: "class".into(),
            value: "container".into(),
        }],
        children: vec![Node::Text("Content in HTML element".into())],
        self_closing: false,
    };

    // 使用 CommonMarkWriter 写入 HTML 元素
    Node::HtmlElement(html_element)
        .to_commonmark(&mut cmark_writer)
        .unwrap();
    let output = cmark_writer.into_string();

    // 验证输出包含预期的 HTML，且选项正确应用（strict=false 允许直接输出 HTML）
    assert!(output.contains("<div class=\"container\">Content in HTML element</div>"));
}

#[test]
fn test_html_options_strict_mode() {
    // 测试严格模式下 HTML 验证
    let options_strict = HtmlWriterOptions {
        strict: true,
        ..Default::default()
    };

    let options_lenient = HtmlWriterOptions {
        strict: false,
        ..Default::default()
    };

    // 创建一个包含无效标签名的 HTML 元素
    let invalid_tag_element = HtmlElement {
        tag: "invalid<tag>".into(),
        attributes: vec![],
        children: vec![Node::Text("Content".into())],
        self_closing: false,
    };

    // 严格模式下应该生成错误
    let mut strict_writer = HtmlWriter::with_options(options_strict);
    let strict_result = Node::HtmlElement(invalid_tag_element.clone()).to_html(&mut strict_writer);
    assert!(strict_result.is_err());

    // 非严格模式下应该进行文本化处理
    let mut lenient_writer = HtmlWriter::with_options(options_lenient);
    let lenient_result = Node::HtmlElement(invalid_tag_element).to_html(&mut lenient_writer);
    assert!(lenient_result.is_ok());

    let output = lenient_writer.into_string();
    // 验证输出进行了文本化处理
    assert!(output.contains("&lt;invalid&lt;tag&gt;&gt;"));
}

#[test]
fn test_code_block_language_class() {
    // 测试代码块语言类前缀选项
    let options_with_prefix = HtmlWriterOptions {
        code_block_language_class_prefix: Some("lang-".into()),
        ..Default::default()
    };

    let options_without_prefix = HtmlWriterOptions {
        code_block_language_class_prefix: None,
        ..Default::default()
    };

    // 创建代码块节点
    let code_block = Node::CodeBlock {
        language: Some("rust".into()),
        content: "fn main() {\n    println!(\"Hello\");\n}".into(),
        block_type: Default::default(),
    };

    // 使用前缀
    let mut writer_with_prefix = HtmlWriter::with_options(options_with_prefix);
    code_block.to_html(&mut writer_with_prefix).unwrap();
    let output_with_prefix = writer_with_prefix.into_string();

    // 验证输出包含预期的类前缀
    assert!(output_with_prefix.contains("class=\"lang-rust\""));

    // 不使用前缀
    let mut writer_without_prefix = HtmlWriter::with_options(options_without_prefix);
    code_block.to_html(&mut writer_without_prefix).unwrap();
    let output_without_prefix = writer_without_prefix.into_string();

    // 验证输出不包含类属性
    assert!(!output_without_prefix.contains("class="));
}

#[cfg(feature = "gfm")]
#[test]
fn test_gfm_html_filtering() {
    // 测试 GFM HTML 过滤功能
    let options = HtmlWriterOptions {
        enable_gfm: true,
        gfm_disallowed_html_tags: vec!["script".into()],
        ..Default::default()
    };

    // 创建一个 script 标签
    let script_element = HtmlElement {
        tag: "script".into(),
        attributes: vec![],
        children: vec![Node::Text("alert('test');".into())],
        self_closing: false,
    };

    let mut writer = HtmlWriter::with_options(options);
    Node::HtmlElement(script_element)
        .to_html(&mut writer)
        .unwrap();
    let output = writer.into_string();

    // 验证 script 标签被文本化处理
    assert!(output.contains("&lt;script&gt;"));
    println!("Output: {}", output);
    assert!(output.contains("alert('test');"));
}

#[test]
fn test_nested_html_structures() {
    // 测试复杂嵌套 HTML 结构
    let mut writer = HtmlWriter::new();

    // 创建嵌套结构
    let nested_element = HtmlElement {
        tag: "div".into(),
        attributes: vec![HtmlAttribute {
            name: "class".into(),
            value: "outer".into(),
        }],
        children: vec![Node::HtmlElement(HtmlElement {
            tag: "div".into(),
            attributes: vec![HtmlAttribute {
                name: "class".into(),
                value: "inner".into(),
            }],
            children: vec![Node::Paragraph(vec![Node::Text(
                "Paragraph inside HTML".into(),
            )])],
            self_closing: false,
        })],
        self_closing: false,
    };

    Node::HtmlElement(nested_element)
        .to_html(&mut writer)
        .unwrap();
    let output = writer.into_string();

    // 验证输出包含正确嵌套的 HTML 结构
    assert!(output.contains("<div class=\"outer\">"));
    assert!(output.contains("<div class=\"inner\">"));
    assert!(output.contains("<p>Paragraph inside HTML</p>"));
    assert!(output.contains("</div></div>"));
}
