//! Tests for GFM task list utilities

#![cfg(feature = "gfm")]

use cmark_writer::ast::{ListItem, Node, TaskListStatus};
use cmark_writer::gfm::tasks::*;

#[test]
fn test_checked_task() {
    let content = vec![Node::Text("Complete this task".into())];
    let task = checked_task(content.clone());

    match task {
        Node::UnorderedList(items) if items.len() == 1 => match &items[0] {
            ListItem::Task {
                status: TaskListStatus::Checked,
                content: task_content,
            } => {
                assert_eq!(task_content, &content);
            }
            _ => panic!("Expected checked task list item"),
        },
        _ => panic!("Expected unordered list with one task item"),
    }
}

#[test]
fn test_unchecked_task() {
    let content = vec![Node::Text("Todo item".into())];
    let task = unchecked_task(content.clone());

    match task {
        Node::UnorderedList(items) if items.len() == 1 => match &items[0] {
            ListItem::Task {
                status: TaskListStatus::Unchecked,
                content: task_content,
            } => {
                assert_eq!(task_content, &content);
            }
            _ => panic!("Expected unchecked task list item"),
        },
        _ => panic!("Expected unordered list with one task item"),
    }
}

#[test]
fn test_task_list() {
    let items = vec![
        (
            TaskListStatus::Checked,
            vec![Node::Text("Done task".into())],
        ),
        (
            TaskListStatus::Unchecked,
            vec![Node::Text("Todo task".into())],
        ),
        (
            TaskListStatus::Checked,
            vec![Node::Text("Another done task".into())],
        ),
    ];

    let task_list_node = task_list(items.clone());

    match task_list_node {
        Node::Document(children) => {
            assert_eq!(children.len(), 3);

            // Check first item (checked)
            match &children[0] {
                Node::UnorderedList(list_items) if list_items.len() == 1 => match &list_items[0] {
                    ListItem::Task {
                        status: TaskListStatus::Checked,
                        content,
                    } => {
                        assert_eq!(content, &items[0].1);
                    }
                    _ => panic!("Expected checked task list item at position 0"),
                },
                _ => panic!("Expected unordered list at position 0"),
            }

            // Check second item (unchecked)
            match &children[1] {
                Node::UnorderedList(list_items) if list_items.len() == 1 => match &list_items[0] {
                    ListItem::Task {
                        status: TaskListStatus::Unchecked,
                        content,
                    } => {
                        assert_eq!(content, &items[1].1);
                    }
                    _ => panic!("Expected unchecked task list item at position 1"),
                },
                _ => panic!("Expected unordered list at position 1"),
            }

            // Check third item (checked)
            match &children[2] {
                Node::UnorderedList(list_items) if list_items.len() == 1 => match &list_items[0] {
                    ListItem::Task {
                        status: TaskListStatus::Checked,
                        content,
                    } => {
                        assert_eq!(content, &items[2].1);
                    }
                    _ => panic!("Expected checked task list item at position 2"),
                },
                _ => panic!("Expected unordered list at position 2"),
            }
        }
        _ => panic!("Expected Document node"),
    }
}

#[test]
fn test_empty_task_list() {
    let empty_items = vec![];
    let task_list_node = task_list(empty_items);

    match task_list_node {
        Node::Document(children) => {
            assert!(children.is_empty());
        }
        _ => panic!("Expected Document node"),
    }
}

#[test]
fn test_single_task_list() {
    let items = vec![(
        TaskListStatus::Unchecked,
        vec![Node::Text("Single task".into())],
    )];

    let task_list_node = task_list(items.clone());

    match task_list_node {
        Node::Document(children) => {
            assert_eq!(children.len(), 1);

            match &children[0] {
                Node::UnorderedList(list_items) if list_items.len() == 1 => match &list_items[0] {
                    ListItem::Task {
                        status: TaskListStatus::Unchecked,
                        content,
                    } => {
                        assert_eq!(content, &items[0].1);
                    }
                    _ => panic!("Expected unchecked task list item"),
                },
                _ => panic!("Expected unordered list"),
            }
        }
        _ => panic!("Expected Document node"),
    }
}
