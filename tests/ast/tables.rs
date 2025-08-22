//! Tests for table AST structures and builders

use cmark_writer::ast::{tables::*, Node};

#[cfg(feature = "gfm")]
use cmark_writer::ast::TableAlignment;

#[test]
fn test_table_builder_new() {
    let builder = TableBuilder::new();
    let table = builder.build();

    match table {
        #[cfg(feature = "gfm")]
        Node::Table {
            headers,
            alignments,
            rows,
        } => {
            assert!(headers.is_empty());
            assert!(alignments.is_empty());
            assert!(rows.is_empty());
        }
        #[cfg(not(feature = "gfm"))]
        Node::Table { headers, rows } => {
            assert!(headers.is_empty());
            assert!(rows.is_empty());
        }
        _ => panic!("Expected Table node"),
    }
}

#[test]
fn test_table_builder_default() {
    let builder = TableBuilder::default();
    let table = builder.build();

    match table {
        #[cfg(feature = "gfm")]
        Node::Table {
            headers,
            alignments,
            rows,
        } => {
            assert!(headers.is_empty());
            assert!(alignments.is_empty());
            assert!(rows.is_empty());
        }
        #[cfg(not(feature = "gfm"))]
        Node::Table { headers, rows } => {
            assert!(headers.is_empty());
            assert!(rows.is_empty());
        }
        _ => panic!("Expected Table node"),
    }
}

#[test]
fn test_table_builder_headers() {
    let headers = vec![
        Node::Text("Name".into()),
        Node::Text("Age".into()),
        Node::Text("City".into()),
    ];

    let table = TableBuilder::new().headers(headers.clone()).build();

    match table {
        #[cfg(feature = "gfm")]
        Node::Table {
            headers: table_headers,
            ..
        } => {
            assert_eq!(table_headers, headers);
        }
        #[cfg(not(feature = "gfm"))]
        Node::Table {
            headers: table_headers,
            ..
        } => {
            assert_eq!(table_headers, headers);
        }
        _ => panic!("Expected Table node"),
    }
}

#[test]
fn test_table_builder_add_row() {
    let row = vec![
        Node::Text("John".into()),
        Node::Text("25".into()),
        Node::Text("NYC".into()),
    ];

    let table = TableBuilder::new().add_row(row.clone()).build();

    match table {
        #[cfg(feature = "gfm")]
        Node::Table { rows, .. } => {
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0], row);
        }
        #[cfg(not(feature = "gfm"))]
        Node::Table { rows, .. } => {
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0], row);
        }
        _ => panic!("Expected Table node"),
    }
}

#[test]
fn test_table_builder_add_rows() {
    let rows = vec![
        vec![Node::Text("John".into()), Node::Text("25".into())],
        vec![Node::Text("Jane".into()), Node::Text("30".into())],
    ];

    let table = TableBuilder::new().add_rows(rows.clone()).build();

    match table {
        #[cfg(feature = "gfm")]
        Node::Table {
            rows: table_rows, ..
        } => {
            assert_eq!(table_rows, rows);
        }
        #[cfg(not(feature = "gfm"))]
        Node::Table {
            rows: table_rows, ..
        } => {
            assert_eq!(table_rows, rows);
        }
        _ => panic!("Expected Table node"),
    }
}

#[test]
fn test_table_builder_fluent_api() {
    let headers = vec![Node::Text("Name".into()), Node::Text("Age".into())];
    let row1 = vec![Node::Text("John".into()), Node::Text("25".into())];
    let row2 = vec![Node::Text("Jane".into()), Node::Text("30".into())];

    let table = TableBuilder::new()
        .headers(headers.clone())
        .add_row(row1.clone())
        .add_row(row2.clone())
        .build();

    match table {
        #[cfg(feature = "gfm")]
        Node::Table {
            headers: table_headers,
            rows,
            ..
        } => {
            assert_eq!(table_headers, headers);
            assert_eq!(rows.len(), 2);
            assert_eq!(rows[0], row1);
            assert_eq!(rows[1], row2);
        }
        #[cfg(not(feature = "gfm"))]
        Node::Table {
            headers: table_headers,
            rows,
        } => {
            assert_eq!(table_headers, headers);
            assert_eq!(rows.len(), 2);
            assert_eq!(rows[0], row1);
            assert_eq!(rows[1], row2);
        }
        _ => panic!("Expected Table node"),
    }
}

#[test]
#[cfg(feature = "gfm")]
fn test_table_builder_align_all() {
    let headers = vec![Node::Text("A".into()), Node::Text("B".into())];

    let table = TableBuilder::new()
        .headers(headers.clone())
        .align_all(TableAlignment::Center)
        .build();

    match table {
        Node::Table { alignments, .. } => {
            assert_eq!(alignments.len(), 2);
            assert_eq!(alignments[0], TableAlignment::Center);
            assert_eq!(alignments[1], TableAlignment::Center);
        }
        _ => panic!("Expected Table node"),
    }
}

#[test]
#[cfg(feature = "gfm")]
fn test_table_builder_align_column() {
    let table = TableBuilder::new()
        .align_column(0, TableAlignment::Left)
        .align_column(2, TableAlignment::Right)
        .build();

    match table {
        Node::Table { alignments, .. } => {
            assert_eq!(alignments.len(), 3);
            assert_eq!(alignments[0], TableAlignment::Left);
            assert_eq!(alignments[1], TableAlignment::default());
            assert_eq!(alignments[2], TableAlignment::Right);
        }
        _ => panic!("Expected Table node"),
    }
}

#[test]
#[cfg(feature = "gfm")]
fn test_table_builder_alignments() {
    let alignments = vec![
        TableAlignment::Left,
        TableAlignment::Center,
        TableAlignment::Right,
    ];

    let table = TableBuilder::new().alignments(alignments.clone()).build();

    match table {
        Node::Table {
            alignments: table_alignments,
            ..
        } => {
            assert_eq!(table_alignments, alignments);
        }
        _ => panic!("Expected Table node"),
    }
}

#[test]
fn test_simple_table() {
    let headers = vec![Node::Text("Name".into()), Node::Text("Age".into())];
    let rows = vec![
        vec![Node::Text("John".into()), Node::Text("25".into())],
        vec![Node::Text("Jane".into()), Node::Text("30".into())],
    ];

    let table = simple_table(headers.clone(), rows.clone());

    match table {
        #[cfg(feature = "gfm")]
        Node::Table {
            headers: table_headers,
            rows: table_rows,
            ..
        } => {
            assert_eq!(table_headers, headers);
            assert_eq!(table_rows, rows);
        }
        #[cfg(not(feature = "gfm"))]
        Node::Table {
            headers: table_headers,
            rows: table_rows,
        } => {
            assert_eq!(table_headers, headers);
            assert_eq!(table_rows, rows);
        }
        _ => panic!("Expected Table node"),
    }
}

#[test]
#[cfg(feature = "gfm")]
fn test_centered_table() {
    let headers = vec![Node::Text("A".into()), Node::Text("B".into())];
    let rows = vec![vec![Node::Text("1".into()), Node::Text("2".into())]];

    let table = centered_table(headers.clone(), rows.clone());

    match table {
        Node::Table {
            headers: table_headers,
            alignments,
            rows: table_rows,
        } => {
            assert_eq!(table_headers, headers);
            assert_eq!(table_rows, rows);
            assert_eq!(alignments.len(), 2);
            assert_eq!(alignments[0], TableAlignment::Center);
            assert_eq!(alignments[1], TableAlignment::Center);
        }
        _ => panic!("Expected Table node"),
    }
}
