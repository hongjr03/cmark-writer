# GFM Task Lists Example

This example demonstrates how to create GitHub Flavored Markdown task lists, which are essentially checkboxes that can be either checked or unchecked.

## Basic Task List Example

```rust
#[cfg(feature = "gfm")]
use cmark_writer::ast::{Node, ListItem, TaskListStatus};
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;

#[cfg(feature = "gfm")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a document with task lists
    let document = Node::Document(vec![
        Node::heading(1, vec![Node::Text("Project Tasks".to_string())]),
        
        Node::Paragraph(vec![
            Node::Text("The following tasks need to be completed:".to_string())
        ]),
        
        // Unordered list with task items
        Node::UnorderedList(vec![
            // Unchecked task
            ListItem::Task {
                status: TaskListStatus::Unchecked,
                content: vec![Node::Paragraph(vec![
                    Node::Text("Implement feature X".to_string())
                ])],
            },
            
            // Checked task
            ListItem::Task {
                status: TaskListStatus::Checked,
                content: vec![Node::Paragraph(vec![
                    Node::Text("Write documentation".to_string())
                ])],
            },
            
            // Another unchecked task
            ListItem::Task {
                status: TaskListStatus::Unchecked,
                content: vec![Node::Paragraph(vec![
                    Node::Text("Test on various platforms".to_string())
                ])],
            },
            
            // Nested tasks
            ListItem::Task {
                status: TaskListStatus::Unchecked,
                content: vec![
                    Node::Paragraph(vec![Node::Text("Deploy to production".to_string())]),
                    // Nested task list
                    Node::UnorderedList(vec![
                        ListItem::Task {
                            status: TaskListStatus::Checked,
                            content: vec![Node::Paragraph(vec![
                                Node::Text("Prepare staging environment".to_string())
                            ])],
                        },
                        ListItem::Task {
                            status: TaskListStatus::Unchecked,
                            content: vec![Node::Paragraph(vec![
                                Node::Text("Configure CI/CD pipeline".to_string())
                            ])],
                        },
                    ]),
                ],
            },
        ]),
    ]);
    
    // Configure writer with GFM task lists enabled
    let options = WriterOptionsBuilder::new()
        .gfm_tasklists(true)
        .build();
    
    let mut writer = CommonMarkWriter::with_options(options);
    writer.write(&document)?;
    let markdown = writer.into_string();
    
    // Print the generated markdown
    println!("{}", markdown);
    
    Ok(())
}

#[cfg(not(feature = "gfm"))]
fn main() {
    println!("This example requires the 'gfm' feature to be enabled");
}
```

With the `gfm` feature enabled, this produces:

```markdown
# Project Tasks

The following tasks need to be completed:

- [ ] Implement feature X
- [x] Write documentation
- [ ] Test on various platforms
- [ ] Deploy to production
  - [x] Prepare staging environment
  - [ ] Configure CI/CD pipeline
```

## Mixed List Types Example

You can mix task list items with regular list items:

```rust
#[cfg(feature = "gfm")]
use cmark_writer::ast::{Node, ListItem, TaskListStatus};
use cmark_writer::options::WriterOptionsBuilder;
use cmark_writer::writer::CommonMarkWriter;

#[cfg(feature = "gfm")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a document with mixed list types
    let document = Node::Document(vec![
        Node::heading(1, vec![Node::Text("Mixed List Example".to_string())]),
        
        // Unordered list with both task and regular items
        Node::UnorderedList(vec![
            // Regular list item
            ListItem::Unordered {
                content: vec![Node::Paragraph(vec![
                    Node::Text("This is a regular list item".to_string())
                ])],
            },
            
            // Task list item
            ListItem::Task {
                status: TaskListStatus::Unchecked,
                content: vec![Node::Paragraph(vec![
                    Node::Text("This is a task list item".to_string())
                ])],
            },
            
            // Another regular list item
            ListItem::Unordered {
                content: vec![Node::Paragraph(vec![
                    Node::Text("Another regular item".to_string())
                ])],
            },
            
            // Completed task
            ListItem::Task {
                status: TaskListStatus::Checked,
                content: vec![Node::Paragraph(vec![
                    Node::Text("A completed task".to_string())
                ])],
            },
        ]),
        
        // Also works with ordered lists
        Node::heading(2, vec![Node::Text("With Ordered Lists".to_string())]),
        
        Node::OrderedList {
            start: 1,
            items: vec![
                // Regular ordered item
                ListItem::Ordered {
                    number: None,
                    content: vec![Node::Paragraph(vec![
                        Node::Text("First ordered item".to_string())
                    ])],
                },
                
                // Task item in ordered list
                ListItem::Task {
                    status: TaskListStatus::Unchecked,
                    content: vec![Node::Paragraph(vec![
                        Node::Text("Task in ordered list".to_string())
                    ])],
                },
                
                // Another regular ordered item
                ListItem::Ordered {
                    number: None,
                    content: vec![Node::Paragraph(vec![
                        Node::Text("Another ordered item".to_string())
                    ])],
                },
            ],
        },
    ]);
    
    // Configure writer with GFM task lists enabled
    let options = WriterOptionsBuilder::new()
        .gfm_tasklists(true)
        .build();
    
    let mut writer = CommonMarkWriter::with_options(options);
    writer.write(&document)?;
    let markdown = writer.into_string();
    
    // Print the generated markdown
    println!("{}", markdown);
    
    Ok(())
}
```

With GFM enabled, this produces:

```markdown
# Mixed List Example

- This is a regular list item
- [ ] This is a task list item
- Another regular item
- [x] A completed task

## With Ordered Lists

1. First ordered item
2. [ ] Task in ordered list
3. Another ordered item
```

## Task List Best Practices

1. **Use task lists for actionable items**: Task lists are best for tracking to-do items, rather than general information
2. **Keep task descriptions concise**: Brief, clear descriptions work best in task lists
3. **Use nesting for hierarchical tasks**: Group related sub-tasks under parent tasks
4. **Combine with other Markdown elements**: Task list descriptions can include other formatting like emphasis or links
5. **Consider state representation**: Checked items typically represent completed tasks, use them consistently

## Implementation Notes

When implementing task lists:

- Remember to enable the `gfm` feature in your `Cargo.toml`
- Use the `WriterOptionsBuilder` to enable GFM task lists
- Task lists can be nested within other lists
- Task items work in both ordered and unordered lists
