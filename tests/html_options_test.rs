use cmark_writer::{
    CommonMarkWriter, HtmlAttribute, HtmlElement, HtmlWriter, HtmlWriterOptions, Node,
    WriterOptions,
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
        tag: "div".to_string(),
        attributes: vec![HtmlAttribute {
            name: "class".to_string(),
            value: "container".to_string(),
        }],
        children: vec![Node::Text("Content in HTML element".to_string())],
        self_closing: false,
    };

    // 使用 CommonMarkWriter 写入 HTML 元素
    cmark_writer
        .write(&Node::HtmlElement(html_element))
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
        tag: "invalid<tag>".to_string(),
        attributes: vec![],
        children: vec![Node::Text("Content".to_string())],
        self_closing: false,
    };

    // 严格模式下应该生成错误
    let mut strict_writer = HtmlWriter::with_options(options_strict);
    let strict_result = strict_writer.write_node(&Node::HtmlElement(invalid_tag_element.clone()));
    assert!(strict_result.is_err());

    // 非严格模式下应该进行文本化处理
    let mut lenient_writer = HtmlWriter::with_options(options_lenient);
    let lenient_result = lenient_writer.write_node(&Node::HtmlElement(invalid_tag_element));
    assert!(lenient_result.is_ok());

    let output = lenient_writer.into_string();
    // 验证输出进行了文本化处理
    assert!(output.contains("&lt;invalid&lt;tag&gt;&gt;"));
}

#[test]
fn test_code_block_language_class() {
    // 测试代码块语言类前缀选项
    let options_with_prefix = HtmlWriterOptions {
        code_block_language_class_prefix: Some("lang-".to_string()),
        ..Default::default()
    };

    let options_without_prefix = HtmlWriterOptions {
        code_block_language_class_prefix: None,
        ..Default::default()
    };

    // 创建代码块节点
    let code_block = Node::CodeBlock {
        language: Some("rust".to_string()),
        content: "fn main() {\n    println!(\"Hello\");\n}".to_string(),
        block_type: Default::default(),
    };

    // 使用前缀
    let mut writer_with_prefix = HtmlWriter::with_options(options_with_prefix);
    writer_with_prefix.write_node(&code_block).unwrap();
    let output_with_prefix = writer_with_prefix.into_string();

    // 验证输出包含预期的类前缀
    assert!(output_with_prefix.contains("class=\"lang-rust\""));

    // 不使用前缀
    let mut writer_without_prefix = HtmlWriter::with_options(options_without_prefix);
    writer_without_prefix.write_node(&code_block).unwrap();
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
        gfm_disallowed_html_tags: vec!["script".to_string()],
        ..Default::default()
    };

    // 创建一个 script 标签
    let script_element = HtmlElement {
        tag: "script".to_string(),
        attributes: vec![],
        children: vec![Node::Text("alert('test');".to_string())],
        self_closing: false,
    };

    let mut writer = HtmlWriter::with_options(options);
    writer
        .write_node(&Node::HtmlElement(script_element))
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
        tag: "div".to_string(),
        attributes: vec![HtmlAttribute {
            name: "class".to_string(),
            value: "outer".to_string(),
        }],
        children: vec![Node::HtmlElement(HtmlElement {
            tag: "div".to_string(),
            attributes: vec![HtmlAttribute {
                name: "class".to_string(),
                value: "inner".to_string(),
            }],
            children: vec![Node::Paragraph(vec![Node::Text(
                "Paragraph inside HTML".to_string(),
            )])],
            self_closing: false,
        })],
        self_closing: false,
    };

    writer
        .write_node(&Node::HtmlElement(nested_element))
        .unwrap();
    let output = writer.into_string();

    // 验证输出包含正确嵌套的 HTML 结构
    assert!(output.contains("<div class=\"outer\">"));
    assert!(output.contains("<div class=\"inner\">"));
    assert!(output.contains("<p>Paragraph inside HTML</p>"));
    assert!(output.contains("</div></div>"));
}
