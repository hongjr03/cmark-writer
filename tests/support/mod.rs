// Shared test utilities for integration and unit test suites

pub mod logger {
    use log::{LevelFilter, Log};
    use std::sync::Once;

    static INIT: Once = Once::new();

    /// Initialize a simple stdout logger once for tests.
    /// Safe to call multiple times; only first call takes effect.
    pub fn init(level: LevelFilter) {
        INIT.call_once(|| {
            struct TestLogger;
            impl Log for TestLogger {
                fn enabled(&self, metadata: &log::Metadata) -> bool {
                    metadata.level() <= LevelFilter::Warn
                }

                fn log(&self, record: &log::Record) {
                    if self.enabled(record.metadata()) {
                        let color_code = match record.level() {
                            log::Level::Error => "\x1b[31m",
                            log::Level::Warn => "\x1b[33m",
                            log::Level::Info => "\x1b[32m",
                            log::Level::Debug => "\x1b[34m",
                            log::Level::Trace => "\x1b[90m",
                        };
                        let reset = "\x1b[0m";
                        println!(
                            "{}[{}]{} {}: {}",
                            color_code,
                            record.level(),
                            reset,
                            record.target(),
                            record.args()
                        );
                    }
                }

                fn flush(&self) {}
            }

            let _ = log::set_boxed_logger(Box::new(TestLogger))
                .map(|()| log::set_max_level(level));
        });
    }
}

pub mod cmark {
    use cmark_writer::options::WriterOptionsBuilder;
    use cmark_writer::writer::CommonMarkWriter;

    /// Create a CommonMark writer with GFM features enabled.
    #[cfg(feature = "gfm")]
    pub fn writer_with_gfm() -> CommonMarkWriter {
        let options = WriterOptionsBuilder::new().enable_gfm().build();
        CommonMarkWriter::with_options(options)
    }
}

pub mod html {
    use cmark_writer::ast::Node;
    use cmark_writer::writer::{HtmlWriteResult, HtmlWriter, HtmlWriterOptions};
    use cmark_writer::ToHtml;
    use ecow::EcoString;

    /// Render a node to HTML using provided options.
    pub fn render_node(node: &Node, options: &HtmlWriterOptions) -> HtmlWriteResult<EcoString> {
        let mut html_writer = HtmlWriter::with_options(options.clone());
        match node.to_html(&mut html_writer) {
            Ok(()) => {}
            Err(e) => return Err(cmark_writer::HtmlWriteError::CustomNodeError(e.to_string())),
        }
        Ok(html_writer.into_string())
    }

    /// Render a node to HTML using default options.
    pub fn render_node_default(node: &Node) -> HtmlWriteResult<EcoString> {
        render_node(
            node,
            #[cfg(feature = "gfm")]
            &HtmlWriterOptions::default().with_gfm_enabled(true),
            #[cfg(not(feature = "gfm"))]
            &HtmlWriterOptions::default(),
        )
    }
}
