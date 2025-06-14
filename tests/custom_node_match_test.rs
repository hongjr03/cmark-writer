#[cfg(test)]
mod tests {
    use cmark_writer::{ast::Node, error::WriteResult, CommonMarkWriter};
    use cmark_writer_macros::custom_node;
    use ecow::EcoString;

    // 使用 block=false 指定为行内元素
    #[derive(Debug, Clone, PartialEq)]
    #[custom_node(block = false)]
    struct ColoredTextNode {
        content: EcoString,
        color: EcoString,
    }

    impl ColoredTextNode {
        fn write_custom(&self, writer: &mut CommonMarkWriter) -> WriteResult<()> {
            writer.write_str("<span style=\"color: ")?;
            writer.write_str(&self.color)?;
            writer.write_str("\">")?;
            writer.write_str(&self.content)?;
            writer.write_str("</span>")?;
            Ok(())
        }
    }

    // 使用 block=true 指定为块级元素
    #[derive(Debug, Clone, PartialEq)]
    #[custom_node(block = true)]
    struct AlertBoxNode {
        content: EcoString,
        level: AlertLevel,
    }

    #[derive(Debug, Clone, PartialEq)]
    #[allow(dead_code)]
    enum AlertLevel {
        Info,
        Warning,
        Error,
    }

    impl AlertBoxNode {
        fn write_custom(&self, writer: &mut CommonMarkWriter) -> WriteResult<()> {
            let class = match self.level {
                AlertLevel::Info => "info",
                AlertLevel::Warning => "warning",
                AlertLevel::Error => "error",
            };

            writer.write_str("<div class=\"alert alert-")?;
            writer.write_str(class)?;
            writer.write_str("\">")?;
            writer.write_str(&self.content)?;
            writer.write_str("</div>")?;
            Ok(())
        }
    }

    // 创建一个处理节点的函数
    fn process_node(node: &Node) -> EcoString {
        match node {
            Node::Document(children) => {
                let mut result = EcoString::new();
                for child in children {
                    result.push_str(&process_node(child));
                }
                result
            }
            Node::Paragraph(children) => {
                let mut result = EcoString::from("<p>");
                for child in children {
                    result.push_str(&process_node(child));
                }
                result.push_str("</p>");
                result
            }
            Node::Text(text) => text.clone(),
            node if node.is_custom_type::<ColoredTextNode>() => {
                let colored = node.as_custom_type::<ColoredTextNode>().unwrap();
                format!(
                    "<span style=\"color: {}\">{}</span>",
                    colored.color, colored.content
                )
                .into()
            }
            node if node.is_custom_type::<AlertBoxNode>() => {
                let alert = node.as_custom_type::<AlertBoxNode>().unwrap();
                let class = match alert.level {
                    AlertLevel::Info => "info",
                    AlertLevel::Warning => "warning",
                    AlertLevel::Error => "error",
                };
                format!(
                    "<div class=\"alert alert-{}\">{}</div>",
                    class, alert.content
                )
                .into()
            }
            _ => EcoString::from("[未处理的节点]"),
        }
    }

    // 另一种匹配方式，使用自定义节点类型的 matches 方法
    fn process_node_alt(node: &Node) -> EcoString {
        match node {
            Node::Document(children) => {
                let mut result = EcoString::new();
                for child in children {
                    result.push_str(&process_node_alt(child));
                }
                result
            }
            Node::Paragraph(children) => {
                let mut result = EcoString::from("<p>");
                for child in children {
                    result.push_str(&process_node_alt(child));
                }
                result.push_str("</p>");
                result
            }
            Node::Text(text) => text.clone(),
            Node::Custom(custom) => {
                if ColoredTextNode::matches(&**custom) {
                    if let Some(colored) = custom.as_any().downcast_ref::<ColoredTextNode>() {
                        format!(
                            "<span style=\"color: {}\">{}</span>",
                            colored.color, colored.content
                        )
                        .into()
                    } else {
                        EcoString::from("[类型转换失败]")
                    }
                } else if AlertBoxNode::matches(&**custom) {
                    if let Some(alert) = custom.as_any().downcast_ref::<AlertBoxNode>() {
                        let class = match alert.level {
                            AlertLevel::Info => "info",
                            AlertLevel::Warning => "warning",
                            AlertLevel::Error => "error",
                        };
                        format!(
                            "<div class=\"alert alert-{}\">{}</div>",
                            class, alert.content
                        )
                        .into()
                    } else {
                        EcoString::from("[类型转换失败]")
                    }
                } else {
                    EcoString::from("[未知的自定义节点]")
                }
            }
            _ => EcoString::from("[未处理的节点]"),
        }
    }

    #[test]
    fn test_custom_node_matching() {
        // 创建一个包含自定义节点的文档
        let nodes = vec![
            // 添加一个普通段落
            Node::Paragraph(vec![
                Node::Text("这是普通文本，".into()),
                Node::Custom(Box::new(ColoredTextNode {
                    content: "这是彩色文本".into(),
                    color: "red".into(),
                })),
                Node::Text("。".into()),
            ]),
            // 添加一个警告框
            Node::Custom(Box::new(AlertBoxNode {
                content: "这是一个警告信息！".into(),
                level: AlertLevel::Warning,
            })),
        ];

        let doc = Node::Document(nodes);

        // 使用第一种匹配方式
        let result1 = process_node(&doc);
        assert!(result1.contains("<span style=\"color: red\">这是彩色文本</span>"));
        assert!(result1.contains("<div class=\"alert alert-warning\">这是一个警告信息！</div>"));

        // 使用第二种匹配方式
        let result2 = process_node_alt(&doc);
        assert!(result2.contains("<span style=\"color: red\">这是彩色文本</span>"));
        assert!(result2.contains("<div class=\"alert alert-warning\">这是一个警告信息！</div>"));
    }
}
