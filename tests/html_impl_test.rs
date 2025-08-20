use cmark_writer::{HtmlWriter, HtmlWriterOptions, Node};

#[test]
fn test_html_writer_options() {
    let options = HtmlWriterOptions {
        strict: true,
        code_block_language_class_prefix: Some("language-".into()),
        #[cfg(feature = "gfm")]
        enable_gfm: true,
        #[cfg(feature = "gfm")]
        gfm_disallowed_html_tags: vec!["script".into()],
    };

    let mut writer = HtmlWriter::with_options(options);
    let code_block = Node::CodeBlock {
        language: Some("rust".into()),
        content: "fn main() {}".into(),
        block_type: Default::default(),
    };
    writer.write_node(&code_block).unwrap();
    let output = writer.into_string();
    assert!(output.contains("class=\"language-rust\""));
}

#[test]
fn test_ensure_tag_closed() {
    let mut writer = HtmlWriter::new();
    writer.start_tag("div").unwrap();
    let output = writer.into_string();
    assert_eq!(output, "<div>");
}
