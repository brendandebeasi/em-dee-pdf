//! Markdown parser using comrak.

use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena, Options};

use crate::config::Config;
use crate::error::Result;

/// Front matter metadata parsed from YAML.
#[derive(Debug, Clone, Default)]
pub struct FrontMatter {
    pub title: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
    pub theme: Option<String>,
    pub toc: Option<bool>,
    pub page_size: Option<String>,
    /// Raw YAML content for custom fields.
    pub raw: String,
}

/// Markdown parser.
pub struct Parser {
    options: Options,
}

impl Parser {
    /// Create a new parser with the given configuration.
    pub fn new(config: &Config) -> Self {
        let mut options = Options::default();

        // Extension options based on config
        let ext = &config.extensions;
        options.extension.strikethrough = ext.strikethrough;
        options.extension.table = ext.tables;
        options.extension.autolink = ext.autolinks;
        options.extension.tasklist = ext.task_lists;
        options.extension.superscript = ext.superscript;
        options.extension.footnotes = ext.footnotes;
        options.extension.description_lists = ext.description_lists;
        options.extension.front_matter_delimiter = if ext.front_matter {
            Some("---".to_owned())
        } else {
            None
        };
        options.extension.math_dollars = ext.math;
        options.extension.math_code = ext.math;

        // Parse options
        options.parse.smart = true;
        options.parse.relaxed_tasklist_matching = true;
        options.parse.relaxed_autolinks = true;

        // Render options
        options.render.github_pre_lang = true;
        options.render.full_info_string = true;
        options.render.unsafe_ = true;

        Self { options }
    }

    /// Parse Markdown text and return extracted metadata and the markdown for transpilation.
    pub fn parse(&self, markdown: &str) -> Result<ParsedDocument> {
        let arena = Arena::new();
        let root = parse_document(&arena, markdown, &self.options);

        // Extract front matter if present
        let front_matter = self.extract_front_matter(root);

        // Extract title from front matter or first heading
        let title = front_matter
            .as_ref()
            .and_then(|fm| fm.title.clone())
            .or_else(|| self.extract_first_heading(root));

        // We need to re-parse for the transpiler since the arena goes out of scope
        // The transpiler will do its own parsing
        Ok(ParsedDocument {
            title,
            front_matter,
            source: markdown.to_string(),
            options: self.options.clone(),
        })
    }

    /// Extract front matter from the document.
    fn extract_front_matter<'a>(&self, root: &'a AstNode<'a>) -> Option<FrontMatter> {
        for node in root.children() {
            if let NodeValue::FrontMatter(ref yaml) = node.data.borrow().value {
                return Some(self.parse_front_matter(yaml));
            }
        }
        None
    }

    /// Parse YAML front matter into structured data.
    fn parse_front_matter(&self, yaml: &str) -> FrontMatter {
        let mut fm = FrontMatter {
            raw: yaml.to_string(),
            ..Default::default()
        };

        for line in yaml.lines() {
            let line = line.trim();
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim().to_lowercase();
                let value = value
                    .trim()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string();

                match key.as_str() {
                    "title" => fm.title = Some(value),
                    "author" => fm.author = Some(value),
                    "date" => fm.date = Some(value),
                    "theme" => fm.theme = Some(value),
                    "toc" => fm.toc = value.parse().ok(),
                    "page-size" | "pagesize" => fm.page_size = Some(value),
                    _ => {}
                }
            }
        }

        fm
    }

    /// Extract the first heading as potential title.
    fn extract_first_heading<'a>(&self, root: &'a AstNode<'a>) -> Option<String> {
        for node in root.children() {
            if let NodeValue::Heading(ref heading) = node.data.borrow().value {
                if heading.level == 1 {
                    return Some(collect_text(node));
                }
            }
        }
        None
    }
}

/// A parsed Markdown document.
pub struct ParsedDocument {
    /// Document title.
    pub title: Option<String>,

    /// Parsed front matter.
    pub front_matter: Option<FrontMatter>,

    /// Original markdown source.
    pub(crate) source: String,

    /// Parser options for re-parsing.
    pub(crate) options: Options,
}

impl ParsedDocument {
    /// Get the original markdown source.
    pub fn source(&self) -> &str {
        &self.source
    }
}

/// Collect text content from a node and its children.
pub(crate) fn collect_text<'a>(node: &'a AstNode<'a>) -> String {
    let mut text = String::new();
    collect_text_recursive(node, &mut text);
    text
}

fn collect_text_recursive<'a>(node: &'a AstNode<'a>, text: &mut String) {
    match &node.data.borrow().value {
        NodeValue::Text(t) => text.push_str(t),
        NodeValue::Code(c) => text.push_str(&c.literal),
        NodeValue::SoftBreak | NodeValue::LineBreak => text.push(' '),
        _ => {
            for child in node.children() {
                collect_text_recursive(child, text);
            }
        }
    }
}
