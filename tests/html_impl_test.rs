use cmark_writer::{
    custom_node, CommonMarkWriter, HtmlWriteResult, HtmlWriter, HtmlWriterOptions, Node,
    WriteResult,
};
use ecow::EcoString;

// 1. 自定义节点，明确指定 html_impl=true，使用自定义 HTML 实现
#[derive(Debug, PartialEq, Clone)]
#[custom_node(block = false, html_impl = true)]
struct ColoredTextWithHtmlImpl {
    text: EcoString,
    color: EcoString,
}

impl ColoredTextWithHtmlImpl {
    // CommonMark 实现
    fn write_custom(&self, writer: &mut CommonMarkWriter) -> WriteResult<()> {
        writer.write_str("<span style=\"color: ")?;
        writer.write_str(&self.color)?;
        writer.write_str("\">")?;
        writer.write_str(&self.text)?;
        writer.write_str("</span>")?;
        Ok(())
    }

    // HTML 实现 - 由于使用了 html_impl = true，宏会生成代码调用这个方法
    fn write_html_custom(&self, writer: &mut HtmlWriter) -> HtmlWriteResult<()> {
        writer.start_tag("span")?;
        writer.attribute("style", &format!("color: {}", self.color))?;
        writer.finish_tag()?;
        writer.text(&self.text)?;
        writer.end_tag("span")?;
        Ok(())
    }
}

// 2. 自定义节点，不指定 html_impl，使用默认 HTML 实现（注释）
#[derive(Debug, PartialEq, Clone)]
#[custom_node(block = false)]
struct ColoredTextWithoutHtmlImpl {
    text: EcoString,
    color: EcoString,
}

impl ColoredTextWithoutHtmlImpl {
    // 只提供 CommonMark 实现
    fn write_custom(&self, writer: &mut CommonMarkWriter) -> WriteResult<()> {
        writer.write_str("<span style=\"color: ")?;
        writer.write_str(&self.color)?;
        writer.write_str("\">")?;
        writer.write_str(&self.text)?;
        writer.write_str("</span>")?;
        Ok(())
    }

    // 不提供 write_html_custom 方法
}

#[test]
fn test_html_impl_parameter() {
    // 1. 测试使用 html_impl = true 的节点
    let colored_with_impl = ColoredTextWithHtmlImpl {
        text: "Hello, custom HTML!".into(),
        color: "#ff0000".into(),
    };

    let mut html_writer = HtmlWriter::new();
    html_writer
        .write_node(&Node::Custom(Box::new(colored_with_impl)))
        .unwrap();
    let custom_html_output = html_writer.into_string();

    // 验证使用了自定义 HTML 实现（应该包含正确格式的 HTML 标签）
    assert!(custom_html_output.contains("<span style=\"color: #ff0000\">"));
    assert!(custom_html_output.contains("Hello, custom HTML!"));
    assert!(custom_html_output.contains("</span>"));

    // 2. 测试不使用 html_impl 的节点
    let colored_without_impl = ColoredTextWithoutHtmlImpl {
        text: "Hello, default HTML!".into(),
        color: "#00ff00".into(),
    };

    let mut html_writer = HtmlWriter::new();
    html_writer
        .write_node(&Node::Custom(Box::new(colored_without_impl)))
        .unwrap();
    let default_html_output = html_writer.into_string();

    // 打印实际输出进行调试
    println!("Actual output: {}", default_html_output);

    // 验证使用了默认 HTML 实现（注释）
    assert!(default_html_output.contains(
        "<!-- HTML rendering not implemented for Custom Node: html_impl_test::ColoredTextWithoutHtmlImpl -->"
    ));
}

#[test]
fn test_html_writer_options() {
    // 测试配置选项影响 HTML 输出
    let options = HtmlWriterOptions {
        strict: true,
        code_block_language_class_prefix: Some("language-".into()),
        #[cfg(feature = "gfm")]
        enable_gfm: true,
        #[cfg(feature = "gfm")]
        gfm_disallowed_html_tags: vec!["script".to_string()],
    };

    let mut writer = HtmlWriter::with_options(options);

    // 测试代码块渲染
    let code_block = Node::CodeBlock {
        language: Some("rust".into()),
        content: "fn main() {\n    println!(\"Hello\");\n}".into(),
        block_type: Default::default(),
    };

    writer.write_node(&code_block).unwrap();
    let output = writer.into_string();

    // 验证语言类前缀正确应用
    assert!(output.contains("class=\"language-rust\""));
}

#[test]
fn test_ensure_tag_closed() {
    // 测试 ensure_tag_closed 方法和 into_string 中的自动闭合标签
    let mut writer = HtmlWriter::new();

    // 开始一个标签但不完成它
    writer.start_tag("div").unwrap();

    // 不显式调用 finish_tag

    // into_string 应该自动关闭标签
    let output = writer.into_string();

    // 验证输出包含闭合的标签
    assert_eq!(output, "<div>");
}
