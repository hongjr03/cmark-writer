//! Tests for GFM table utilities

#![cfg(feature = "gfm")]

use cmark_writer::ast::{Node, TableAlignment};
use cmark_writer::gfm::tables::*;

#[test]
fn test_right_aligned_table() {
    let headers = vec![Node::Text("Name".into()), Node::Text("Age".into())];

    let rows = vec![vec![Node::Text("John".into()), Node::Text("25".into())]];

    let table = right_aligned_table(headers.clone(), rows.clone());

    match table {
        Node::Table {
            headers: table_headers,
            alignments,
            rows: table_rows,
        } => {
            assert_eq!(table_headers, headers);
            assert_eq!(table_rows, rows);
            assert_eq!(alignments.len(), 2);
            assert_eq!(alignments[0], TableAlignment::Right);
            assert_eq!(alignments[1], TableAlignment::Right);
        }
        _ => panic!("Expected Table node"),
    }
}

#[test]
fn test_alternating_table() {
    let headers = vec![
        Node::Text("A".into()),
        Node::Text("B".into()),
        Node::Text("C".into()),
        Node::Text("D".into()),
        Node::Text("E".into()),
    ];

    let rows = vec![vec![
        Node::Text("1".into()),
        Node::Text("2".into()),
        Node::Text("3".into()),
        Node::Text("4".into()),
        Node::Text("5".into()),
    ]];

    let table = alternating_table(headers.clone(), rows.clone());

    match table {
        Node::Table {
            headers: table_headers,
            alignments,
            rows: table_rows,
        } => {
            assert_eq!(table_headers, headers);
            assert_eq!(table_rows, rows);
            assert_eq!(alignments.len(), 5);
            // Pattern: Left(0), Center(1), Right(2), Left(3), Center(4)
            assert_eq!(alignments[0], TableAlignment::Left);
            assert_eq!(alignments[1], TableAlignment::Center);
            assert_eq!(alignments[2], TableAlignment::Right);
            assert_eq!(alignments[3], TableAlignment::Left);
            assert_eq!(alignments[4], TableAlignment::Center);
        }
        _ => panic!("Expected Table node"),
    }
}

#[test]
fn test_alternating_table_pattern() {
    let headers = vec![
        Node::Text("Col1".into()),
        Node::Text("Col2".into()),
        Node::Text("Col3".into()),
        Node::Text("Col4".into()),
        Node::Text("Col5".into()),
        Node::Text("Col6".into()),
    ];

    let table = alternating_table(headers, vec![]);

    match table {
        Node::Table { alignments, .. } => {
            assert_eq!(alignments.len(), 6);
            // Verify the full pattern repeats correctly
            assert_eq!(alignments[0], TableAlignment::Left); // 0 % 3 = 0
            assert_eq!(alignments[1], TableAlignment::Center); // 1 % 3 = 1
            assert_eq!(alignments[2], TableAlignment::Right); // 2 % 3 = 2
            assert_eq!(alignments[3], TableAlignment::Left); // 3 % 3 = 0
            assert_eq!(alignments[4], TableAlignment::Center); // 4 % 3 = 1
            assert_eq!(alignments[5], TableAlignment::Right); // 5 % 3 = 2
        }
        _ => panic!("Expected Table node"),
    }
}

#[test]
fn test_simple_table_re_export() {
    let headers = vec![Node::Text("Header".into())];
    let rows = vec![vec![Node::Text("Cell".into())]];

    let table = simple_table(headers.clone(), rows.clone());

    match table {
        Node::Table {
            headers: table_headers,
            rows: table_rows,
            ..
        } => {
            assert_eq!(table_headers, headers);
            assert_eq!(table_rows, rows);
        }
        _ => panic!("Expected Table node"),
    }
}

#[test]
fn test_centered_table_re_export() {
    let headers = vec![Node::Text("Header".into())];
    let rows = vec![vec![Node::Text("Cell".into())]];

    let table = centered_table(headers.clone(), rows.clone());

    match table {
        Node::Table {
            headers: table_headers,
            alignments,
            rows: table_rows,
        } => {
            assert_eq!(table_headers, headers);
            assert_eq!(table_rows, rows);
            assert_eq!(alignments.len(), 1);
            assert_eq!(alignments[0], TableAlignment::Center);
        }
        _ => panic!("Expected Table node"),
    }
}
