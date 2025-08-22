//! Tests for CommonMarkWriter HTML fallback behavior

use cmark_writer::ast::{CodeBlockType, Node, TableAlignment};
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;
use cmark_writer::ToCommonMark;

#[test]
fn table_with_block_elements_and_alignments_soft_mode_falls_back_to_html() {
    // Soft mode to allow fallback
    let mut writer = CommonMarkWriter::with_options(
        WriterOptionsBuilder::new()
            .strict(false)
            .enable_gfm()
            .build(),
    );

    // Table with GFM alignments and a block-level element in a cell
    let table = Node::Table {
        headers: vec![
            Node::Text("Left".into()),
            Node::Text("Center".into()),
            Node::Text("Right".into()),
        ],
        alignments: vec![
            TableAlignment::Left,
            TableAlignment::Center,
            TableAlignment::Right,
        ],
        rows: vec![vec![
            // left cell: inline text
            Node::Text("L1".into()),
            // center cell: block-level code block triggers fallback
            Node::CodeBlock {
                language: Some("rust".into()),
                content: "fn main() { println!(\"hi\"); }".into(),
                block_type: CodeBlockType::Fenced,
            },
            // right cell: inline text
            Node::Text("R1".into()),
        ]],
    };

    table
        .to_commonmark(&mut writer)
        .expect("fallback should succeed");
    let output = writer.into_string();

    eprintln!("{}", output);

    // Should be HTML table, not markdown pipe table
    assert!(output.contains("<table>"));
    assert!(output.contains("<thead>"));
    assert!(output.contains("<tbody>"));
    // Header cells should include text with alignment styles when GFM is enabled
    assert!(output.contains("<th style=\"text-align: left;\">Left</th>"));
    assert!(output.contains("<th style=\"text-align: center;\">Center</th>"));
    assert!(output.contains("<th style=\"text-align: right;\">Right</th>"));

    // Alignment should be emitted as inline style on th/td
    assert!(output.contains("<th style=\"text-align: left;\">"));
    assert!(output.contains("<th style=\"text-align: center;\">"));
    assert!(output.contains("<th style=\"text-align: right;\">"));

    // Body row with alignment and HTML-rendered code block
    assert!(output.contains("<td style=\"text-align: left;\">L1</td>"));
    assert!(output.contains("<td style=\"text-align: center;\">"));
    assert!(output.contains("<pre><code class=\"language-rust\">"));
    assert!(output.contains("println!(\"hi\");"));
}

#[test]
fn table_with_block_elements_strict_mode_errors() {
    // Strict mode should error out instead of falling back
    let mut writer =
        CommonMarkWriter::with_options(WriterOptionsBuilder::new().strict(true).build());

    let table = Node::Table {
        headers: vec![Node::Text("H1".into())],
        alignments: vec![TableAlignment::Left],
        rows: vec![vec![Node::Paragraph(vec![Node::Text("Para".into())])]],
    };

    let res = table.to_commonmark(&mut writer);
    assert!(res.is_err());
}
