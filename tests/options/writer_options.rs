//! Tests for WriterOptions and WriterOptionsBuilder

use cmark_writer::options::{WriterOptions, WriterOptionsBuilder};

#[test]
fn test_writer_options_default() {
    let options = WriterOptions::default();

    assert!(options.strict);
    assert!(!options.hard_break_spaces);
    assert_eq!(options.indent_spaces, 4);
    assert_eq!(options.list_marker, '-');
    assert_eq!(options.thematic_break_char, '-');
    assert_eq!(options.emphasis_char, '_');
    assert_eq!(options.strong_char, '*');
    assert!(!options.escape_special_chars);
    assert!(options.trim_paragraph_trailing_hard_breaks);

    #[cfg(feature = "gfm")]
    {
        assert!(!options.enable_gfm);
        assert!(!options.gfm_strikethrough);
        assert!(!options.gfm_tasklists);
        assert!(!options.gfm_tables);
        assert!(!options.gfm_autolinks);
        assert!(!options.gfm_disallowed_html_tags.is_empty());
    }

    assert!(options.html_writer_options.is_none());
}

#[test]
fn test_writer_options_builder_new() {
    let builder = WriterOptionsBuilder::new();
    let options = builder.build();

    // Should match default options
    let default_options = WriterOptions::default();
    assert_eq!(options.strict, default_options.strict);
    assert_eq!(options.indent_spaces, default_options.indent_spaces);
}

#[test]
fn test_writer_options_builder_strict() {
    let options = WriterOptionsBuilder::new().strict(false).build();

    assert!(!options.strict);
}

#[test]
fn test_writer_options_builder_hard_break_spaces() {
    let options = WriterOptionsBuilder::new().hard_break_spaces(true).build();

    assert!(options.hard_break_spaces);
}

#[test]
fn test_writer_options_builder_indent_spaces() {
    let options = WriterOptionsBuilder::new().indent_spaces(8).build();

    assert_eq!(options.indent_spaces, 8);
}

#[test]
fn test_writer_options_builder_list_marker() {
    // Test valid markers
    let options_dash = WriterOptionsBuilder::new().list_marker('-').build();
    assert_eq!(options_dash.list_marker, '-');

    let options_plus = WriterOptionsBuilder::new().list_marker('+').build();
    assert_eq!(options_plus.list_marker, '+');

    let options_star = WriterOptionsBuilder::new().list_marker('*').build();
    assert_eq!(options_star.list_marker, '*');

    // Test invalid marker (should be ignored)
    let options_invalid = WriterOptionsBuilder::new().list_marker('x').build();
    assert_eq!(options_invalid.list_marker, '-'); // Should remain default
}

#[test]
fn test_writer_options_builder_escape_special_chars() {
    let options = WriterOptionsBuilder::new()
        .escape_special_chars(true)
        .build();

    assert!(options.escape_special_chars);
}

#[test]
fn test_writer_options_builder_trim_paragraph_trailing_hard_breaks() {
    let options = WriterOptionsBuilder::new()
        .trim_paragraph_trailing_hard_breaks(false)
        .build();

    assert!(!options.trim_paragraph_trailing_hard_breaks);
}

#[test]
fn test_writer_options_builder_thematic_break_char() {
    // Test valid characters
    let options_dash = WriterOptionsBuilder::new().thematic_break_char('-').build();
    assert_eq!(options_dash.thematic_break_char, '-');

    let options_star = WriterOptionsBuilder::new().thematic_break_char('*').build();
    assert_eq!(options_star.thematic_break_char, '*');

    let options_underscore = WriterOptionsBuilder::new().thematic_break_char('_').build();
    assert_eq!(options_underscore.thematic_break_char, '_');

    // Test invalid character (should be ignored)
    let options_invalid = WriterOptionsBuilder::new().thematic_break_char('x').build();
    assert_eq!(options_invalid.thematic_break_char, '-'); // Should remain default
}

#[test]
fn test_writer_options_builder_emphasis_char() {
    // Test valid characters
    let options_underscore = WriterOptionsBuilder::new().emphasis_char('_').build();
    assert_eq!(options_underscore.emphasis_char, '_');

    let options_star = WriterOptionsBuilder::new().emphasis_char('*').build();
    assert_eq!(options_star.emphasis_char, '*');

    // Test invalid character (should be ignored)
    let options_invalid = WriterOptionsBuilder::new().emphasis_char('x').build();
    assert_eq!(options_invalid.emphasis_char, '_'); // Should remain default
}

#[test]
fn test_writer_options_builder_strong_char() {
    // Test valid characters
    let options_underscore = WriterOptionsBuilder::new().strong_char('_').build();
    assert_eq!(options_underscore.strong_char, '_');

    let options_star = WriterOptionsBuilder::new().strong_char('*').build();
    assert_eq!(options_star.strong_char, '*');

    // Test invalid character (should be ignored)
    let options_invalid = WriterOptionsBuilder::new().strong_char('x').build();
    assert_eq!(options_invalid.strong_char, '*'); // Should remain default
}

#[cfg(feature = "gfm")]
#[test]
fn test_writer_options_builder_enable_gfm() {
    let options = WriterOptionsBuilder::new().enable_gfm().build();

    assert!(options.enable_gfm);
    assert!(options.gfm_strikethrough);
    assert!(options.gfm_tasklists);
    assert!(options.gfm_tables);
    assert!(options.gfm_autolinks);
}

#[cfg(feature = "gfm")]
#[test]
fn test_writer_options_builder_gfm_individual_features() {
    let options = WriterOptionsBuilder::new()
        .gfm_strikethrough(true)
        .gfm_tasklists(true)
        .gfm_tables(false)
        .gfm_autolinks(true)
        .build();

    assert!(options.gfm_strikethrough);
    assert!(options.gfm_tasklists);
    assert!(!options.gfm_tables);
    assert!(options.gfm_autolinks);
    // Individual features should enable GFM automatically
    assert!(options.enable_gfm);
}

#[cfg(feature = "gfm")]
#[test]
fn test_writer_options_builder_gfm_disallowed_tags() {
    let custom_tags = vec!["script".into(), "iframe".into()];
    let options = WriterOptionsBuilder::new()
        .gfm_disallowed_html_tags(custom_tags.clone())
        .build();

    assert_eq!(options.gfm_disallowed_html_tags, custom_tags);
}

#[test]
fn test_writer_options_builder_html_writer_options() {
    use cmark_writer::writer::html::options::HtmlWriterOptions;

    let html_options = HtmlWriterOptions::default();
    let options = WriterOptionsBuilder::new()
        .html_writer_options(Some(html_options))
        .build();

    assert!(options.html_writer_options.is_some());
}

#[test]
fn test_writer_options_builder_default() {
    let builder1 = WriterOptionsBuilder::new();
    let builder2 = WriterOptionsBuilder::default();

    let options1 = builder1.build();
    let options2 = builder2.build();

    assert_eq!(options1.strict, options2.strict);
    assert_eq!(options1.indent_spaces, options2.indent_spaces);
}

#[test]
fn test_writer_options_builder_chaining() {
    let options = WriterOptionsBuilder::new()
        .strict(false)
        .indent_spaces(2)
        .list_marker('+')
        .escape_special_chars(true)
        .build();

    assert!(!options.strict);
    assert_eq!(options.indent_spaces, 2);
    assert_eq!(options.list_marker, '+');
    assert!(options.escape_special_chars);
}

#[test]
fn test_writer_options_html_writer_options() {
    use cmark_writer::writer::html::options::HtmlWriterOptions;

    let html_options = HtmlWriterOptions::default();
    let options = WriterOptions::default().html_writer_options(Some(html_options));

    assert!(options.html_writer_options.is_some());
}

#[test]
fn test_writer_options_default_gfm_disallowed_tags() {
    #[cfg(feature = "gfm")]
    {
        let options = WriterOptions::default();
        let disallowed_tags = &options.gfm_disallowed_html_tags;

        assert!(disallowed_tags.contains(&"script".into()));
        assert!(disallowed_tags.contains(&"iframe".into()));
        assert!(disallowed_tags.contains(&"style".into()));
        assert!(disallowed_tags.contains(&"title".into()));
        assert!(disallowed_tags.len() > 5); // Should have multiple disallowed tags
    }
}
