//! Transpiler from Markdown AST to Typst source.

use std::path::Path;

use comrak::nodes::{AstNode, ListType, NodeValue, TableAlignment};
use comrak::{parse_document, Arena};
use crate::config::Config;
use crate::error::Result;
use crate::mermaid;
use crate::parser::{collect_text, ParsedDocument};
use crate::theme::Theme;

/// Result of transpilation, including the Typst source and any temp files
/// that must be kept alive until rendering completes.
pub struct TranspileResult {
    /// The Typst source code.
    pub source: String,
    /// Temp files that must be kept alive (e.g., mermaid diagram PNGs).
    pub temp_files: Vec<tempfile::NamedTempFile>,
}

/// Per-call context threaded through the recursive visit methods.
struct TranspileCtx<'a> {
    /// Base directory for resolving relative image paths.
    base_path: Option<&'a Path>,
    /// Temp files that must be kept alive (e.g., mermaid diagram PNGs).
    temp_files: Vec<tempfile::NamedTempFile>,
}

/// Transpiles Markdown AST to Typst source code.
pub struct Transpiler {
    theme: Theme,
    generate_toc: bool,
    toc_depth: u8,
    section_containers: bool,
    mermaid_enabled: bool,
}

impl Transpiler {
    /// Create a new transpiler.
    pub fn new(theme: Theme, config: &Config) -> Self {
        Self {
            theme,
            generate_toc: config.output.toc,
            toc_depth: config.output.toc_depth,
            section_containers: config.output.section_containers,
            mermaid_enabled: config.extensions.mermaid,
        }
    }

    /// Transpile a parsed document to Typst source.
    /// Returns just the source string (temp files are not tracked).
    pub fn transpile(&self, doc: &ParsedDocument, base_path: Option<&Path>) -> Result<String> {
        let result = self.transpile_with_resources(doc, base_path)?;
        // Note: temp_files will be dropped here, so mermaid diagrams
        // won't work with this method. Use transpile_with_resources instead.
        Ok(result.source)
    }

    /// Transpile a parsed document to Typst source with resource tracking.
    /// Returns the source and temp files that must be kept alive during rendering.
    pub fn transpile_with_resources(&self, doc: &ParsedDocument, base_path: Option<&Path>) -> Result<TranspileResult> {
        let mut output = String::new();
        let mut ctx = TranspileCtx {
            base_path,
            temp_files: Vec::new(),
        };

        // Emit theme preamble
        output.push_str(self.theme.preamble());
        output.push_str("\n\n");

        // Emit document metadata if available
        if let Some(ref fm) = doc.front_matter {
            if let Some(ref title) = fm.title {
                output.push_str(&format!("#set document(title: \"{}\")\n", escape_string(title)));
            }
            if let Some(ref author) = fm.author {
                output.push_str(&format!("#set document(author: \"{}\")\n", escape_string(author)));
            }
            output.push('\n');
        }

        // Emit table of contents if requested
        let toc_enabled = self.generate_toc
            || doc.front_matter.as_ref().and_then(|fm| fm.toc).unwrap_or(false);

        if toc_enabled {
            // Remove dot leaders since entries use underline styling.
            // fill moved from outline() to outline.entry in Typst 0.13+.
            output.push_str("#set outline.entry(fill: none)\n");
            // Make outline entries clickable PDF links with visible link styling.
            // context + link() is required in Typst 0.12+ for in-document navigation.
            output.push_str(
                "#show outline.entry: it => context link(it.element.location())[#text(fill: rgb(\"#0969da\"), weight: \"medium\")[#underline(offset: 2pt, stroke: 0.5pt + rgb(\"#0969da\"))[#it]]]\n"
            );
            output.push_str(&format!(
                "#outline(title: \"Contents\", indent: 1em, depth: {})\n",
                self.toc_depth
            ));
            output.push_str("#pagebreak()\n\n");
        }

        // Parse and transpile document content
        let arena = Arena::new();
        let root = parse_document(&arena, &doc.source, &doc.options);

        if self.section_containers {
            self.visit_with_sections(root, &mut output, &mut ctx)?;
        } else {
            self.visit_children(root, &mut output, &mut ctx)?;
        }

        Ok(TranspileResult {
            source: output,
            temp_files: ctx.temp_files,
        })
    }

    /// Visit document with H2 section containers.
    fn visit_with_sections<'a>(
        &self,
        node: &'a AstNode<'a>,
        output: &mut String,
        ctx: &mut TranspileCtx<'_>,
    ) -> Result<()> {
        let mut in_section = false;
        let mut preamble_content = String::new();

        for child in node.children() {
            let is_h2 = matches!(
                &child.data.borrow().value,
                NodeValue::Heading(h) if h.level == 2
            );

            if is_h2 {
                // Close previous section if open
                if in_section {
                    output.push_str("]\n\n");
                }

                // Emit any preamble content (before first H2)
                if !preamble_content.is_empty() {
                    output.push_str(&preamble_content);
                    preamble_content.clear();
                }

                // Start new section with heading
                output.push_str("#md-section[\n");
                self.visit_node(child, output, ctx)?;
                in_section = true;
            } else if in_section {
                // Inside a section, emit content normally
                self.visit_node(child, output, ctx)?;
            } else {
                // Before first H2, collect preamble (like H1)
                self.visit_node(child, &mut preamble_content, ctx)?;
            }
        }

        // Close final section
        if in_section {
            output.push_str("]\n");
        }

        // If no sections were found, emit preamble as regular content
        if !in_section && !preamble_content.is_empty() {
            output.push_str(&preamble_content);
        }

        Ok(())
    }

    /// Visit all children of a node.
    fn visit_children<'a>(
        &self,
        node: &'a AstNode<'a>,
        output: &mut String,
        ctx: &mut TranspileCtx<'_>,
    ) -> Result<()> {
        for child in node.children() {
            self.visit_node(child, output, ctx)?;
        }
        Ok(())
    }

    /// Visit a single AST node.
    fn visit_node<'a>(
        &self,
        node: &'a AstNode<'a>,
        output: &mut String,
        ctx: &mut TranspileCtx<'_>,
    ) -> Result<()> {
        let value = &node.data.borrow().value;

        match value {
            NodeValue::Document => {
                self.visit_children(node, output, ctx)?;
            }

            NodeValue::FrontMatter(_) => {
                // Already handled in transpile()
            }

            NodeValue::Heading(heading) => {
                // Typst uses = for h1, == for h2, etc.
                let prefix = "=".repeat(heading.level as usize);
                let text = collect_text(node);
                let slug = heading_slug(&text);
                output.push_str(&prefix);
                output.push(' ');
                self.visit_children(node, output, ctx)?;
                output.push_str(&format!(" <{}>", slug));
                output.push_str("\n\n");
            }

            NodeValue::Paragraph => {
                self.visit_children(node, output, ctx)?;
                output.push_str("\n\n");
            }

            NodeValue::Text(text) => {
                output.push_str(&escape_typst(text));
            }

            NodeValue::SoftBreak => {
                output.push('\n');
            }

            NodeValue::LineBreak => {
                output.push_str(" \\\n");
            }

            NodeValue::Strong => {
                output.push('*');
                self.visit_children(node, output, ctx)?;
                output.push('*');
            }

            NodeValue::Emph => {
                output.push('_');
                self.visit_children(node, output, ctx)?;
                output.push('_');
            }

            NodeValue::Strikethrough => {
                output.push_str("#strike[");
                self.visit_children(node, output, ctx)?;
                output.push(']');
            }

            NodeValue::Code(code) => {
                // Escape backticks in code
                let literal = &code.literal;
                if literal.contains('`') {
                    output.push_str(&format!("#raw(\"{}\")", escape_string(literal)));
                } else {
                    output.push('`');
                    output.push_str(literal);
                    output.push('`');
                }
            }

            NodeValue::CodeBlock(block) => {
                let lang = if block.info.is_empty() {
                    None
                } else {
                    block.info.split_whitespace().next()
                };

                // Check for mermaid code blocks
                if lang == Some("mermaid") && self.mermaid_enabled {
                    output.push_str(&self.render_mermaid(&block.literal, ctx));
                } else {
                    output.push_str("```");
                    if let Some(lang) = lang {
                        output.push_str(lang);
                    }
                    output.push('\n');
                    output.push_str(&block.literal);
                    if !block.literal.ends_with('\n') {
                        output.push('\n');
                    }
                    output.push_str("```\n\n");
                }
            }

            NodeValue::Link(link) => {
                if link.url.starts_with('#') {
                    // Internal anchor link — convert to Typst label reference
                    let slug = heading_slug(&link.url[1..]);
                    output.push_str(&format!("#link(<{}>)[", slug));
                } else {
                    output.push_str(&format!("#link(\"{}\")[", escape_string(&link.url)));
                }
                self.visit_children(node, output, ctx)?;
                output.push(']');
            }

            NodeValue::Image(link) => {
                let alt_text = collect_text(node);
                let resolved_url = resolve_image_path(&link.url, ctx.base_path);
                output.push_str(&format!(
                    "#figure(\n  image(\"{}\"),\n  caption: [{}]\n)\n\n",
                    escape_string(&resolved_url),
                    escape_typst(&alt_text)
                ));
            }

            NodeValue::List(list) => {
                self.emit_list(list.list_type, list.start, node, output, ctx)?;
            }

            NodeValue::Item(_) => {
                // Handled by emit_list
            }

            NodeValue::BlockQuote => {
                // Check for GitHub-style alerts: > [!NOTE], > [!WARNING], etc.
                if let Some(alert_type) = self.detect_github_alert(node) {
                    output.push_str(&self.emit_admonition(&alert_type, node, ctx)?);
                } else {
                    output.push_str("#quote(block: true)[\n");
                    self.visit_children(node, output, ctx)?;
                    output.push_str("]\n\n");
                }
            }

            NodeValue::ThematicBreak => {
                output.push_str("#line(length: 100%)\n\n");
            }

            NodeValue::Table(table) => {
                self.emit_table(&table.alignments, node, output, ctx)?;
            }

            NodeValue::TableRow(_) | NodeValue::TableCell => {
                // Handled by emit_table
            }

            NodeValue::TaskItem(symbol) => {
                // symbol is Option<char> where 'x' or 'X' means checked
                let is_checked = symbol.map(|c| c == 'x' || c == 'X').unwrap_or(false);
                let checkbox = if is_checked { "[x]" } else { "[ ]" };
                output.push_str(&format!("{} ", checkbox));
                self.visit_children(node, output, ctx)?;
            }

            NodeValue::FootnoteDefinition(footnote) => {
                output.push_str("#footnote[");
                self.visit_children(node, output, ctx)?;
                output.push_str(&format!("] <fn-{}>", escape_label(&footnote.name)));
                output.push('\n');
            }

            NodeValue::FootnoteReference(footnote) => {
                output.push_str(&format!("#footnote[@fn-{}]", escape_label(&footnote.name)));
            }

            NodeValue::Math(math) => {
                let converted = latex_to_typst_math(&math.literal);
                if math.display_math {
                    output.push_str("$ ");
                    output.push_str(&converted);
                    output.push_str(" $\n\n");
                } else {
                    output.push('$');
                    output.push_str(&converted);
                    output.push('$');
                }
            }

            NodeValue::HtmlInline(html) => {
                // Pass through as raw content with a comment
                output.push_str(&format!("/* HTML: {} */", html.trim()));
            }

            NodeValue::HtmlBlock(html_block) => {
                // Pass through as raw content with a comment
                output.push_str(&format!("/* HTML: {} */", html_block.literal.trim()));
            }

            NodeValue::Superscript => {
                output.push_str("#super[");
                self.visit_children(node, output, ctx)?;
                output.push(']');
            }

            NodeValue::DescriptionList => {
                output.push_str("#terms(\n");
                self.visit_children(node, output, ctx)?;
                output.push_str(")\n\n");
            }

            NodeValue::DescriptionItem(_) => {
                self.visit_children(node, output, ctx)?;
            }

            NodeValue::DescriptionTerm => {
                output.push_str("  [");
                self.visit_children(node, output, ctx)?;
                output.push_str("]: ");
            }

            NodeValue::DescriptionDetails => {
                output.push('[');
                self.visit_children(node, output, ctx)?;
                output.push_str("],\n");
            }

            // Catch-all for unhandled nodes
            _ => {
                tracing::warn!("unhandled node type: {:?}", value);
                self.visit_children(node, output, ctx)?;
            }
        }

        Ok(())
    }

    fn emit_list<'a>(
        &self,
        list_type: ListType,
        start: usize,
        node: &'a AstNode<'a>,
        output: &mut String,
        ctx: &mut TranspileCtx<'_>,
    ) -> Result<()> {
        let mut item_num = start;

        for child in node.children() {
            if let NodeValue::Item(_) = &child.data.borrow().value {
                let prefix = match list_type {
                    ListType::Bullet => "- ".to_string(),
                    ListType::Ordered => {
                        let p = format!("{}. ", item_num);
                        item_num += 1;
                        p
                    }
                };

                output.push_str(&prefix);

                // Visit item children
                let mut first = true;
                for item_child in child.children() {
                    if !first {
                        output.push_str("  ");
                    }
                    first = false;
                    self.visit_node(item_child, output, ctx)?;
                }
            }
        }

        output.push('\n');
        Ok(())
    }

    fn emit_table<'a>(
        &self,
        alignments: &[TableAlignment],
        node: &'a AstNode<'a>,
        output: &mut String,
        ctx: &mut TranspileCtx<'_>,
    ) -> Result<()> {
        let rows: Vec<_> = node.children().collect();
        if rows.is_empty() {
            return Ok(());
        }

        // Convert alignments to Typst
        let align_str: Vec<&str> = alignments
            .iter()
            .map(|a| match a {
                TableAlignment::Left => "left",
                TableAlignment::Center => "center",
                TableAlignment::Right => "right",
                TableAlignment::None => "auto",
            })
            .collect();

        let cols = alignments.len();
        output.push_str(&format!(
            "#table(\n  columns: {},\n  align: ({}),\n",
            cols,
            align_str.join(", ")
        ));

        // Emit rows, wrapping header in table.header for repeat across page breaks
        for row in rows.iter() {
            if let NodeValue::TableRow(is_header) = &row.data.borrow().value {
                let cells: Vec<_> = row.children().collect();

                if *is_header {
                    output.push_str("  table.header(");
                }

                for cell in cells {
                    output.push_str("  [");

                    // Bold header cells
                    if *is_header {
                        output.push('*');
                    }

                    self.visit_children(cell, output, ctx)?;

                    if *is_header {
                        output.push('*');
                    }

                    output.push_str("],\n");
                }

                if *is_header {
                    output.push_str("  ),\n");
                }
            }
        }

        output.push_str(")\n\n");
        Ok(())
    }

    /// Render a mermaid diagram to Typst code.
    fn render_mermaid(
        &self,
        mermaid_source: &str,
        ctx: &mut TranspileCtx<'_>,
    ) -> String {
        // Check if mmdc is available
        if !mermaid::is_mmdc_available() {
            tracing::warn!("mermaid-cli (mmdc) not available, using placeholder");
            return mermaid::generate_placeholder(mermaid_source);
        }

        // Try to render to PNG and save to a temp file
        match mermaid::render_and_embed(mermaid_source) {
            Ok((typst_code, temp_file)) => {
                tracing::debug!("Successfully rendered mermaid diagram to PNG");
                ctx.temp_files.push(temp_file);
                typst_code
            }
            Err(e) => {
                tracing::error!("Failed to render mermaid diagram: {}", e);
                // Return a placeholder with error info
                format!(
                    r##"#block(
  fill: rgb("#ffebee"),
  stroke: 1pt + rgb("#ef5350"),
  radius: 4pt,
  inset: 16pt,
  width: 100%,
)[
  #text(fill: rgb("#c62828"), weight: "medium")[Mermaid Error]

  #v(8pt)

  #text(size: 9pt, fill: rgb("#666666"))[{}]
]

"##,
                    escape_string(&e.to_string())
                )
            }
        }
    }

    /// Detect GitHub-style alert syntax in a blockquote.
    /// Returns the alert type if found (NOTE, WARNING, TIP, IMPORTANT, CAUTION).
    fn detect_github_alert<'a>(&self, node: &'a AstNode<'a>) -> Option<String> {
        // Get the first paragraph child
        for child in node.children() {
            if let NodeValue::Paragraph = &child.data.borrow().value {
                // Check first text node for [!TYPE] pattern
                for text_node in child.children() {
                    if let NodeValue::Text(text) = &text_node.data.borrow().value {
                        let trimmed = text.trim();
                        if trimmed.starts_with("[!") && trimmed.contains(']') {
                            // Extract the alert type
                            if let Some(end) = trimmed.find(']') {
                                let alert_type = &trimmed[2..end];
                                let valid_types = ["NOTE", "TIP", "IMPORTANT", "WARNING", "CAUTION"];
                                if valid_types.contains(&alert_type.to_uppercase().as_str()) {
                                    return Some(alert_type.to_uppercase());
                                }
                            }
                        }
                        break; // Only check first text node
                    }
                }
                break; // Only check first paragraph
            }
        }
        None
    }

    /// Emit a styled admonition block.
    fn emit_admonition<'a>(
        &self,
        alert_type: &str,
        node: &'a AstNode<'a>,
        ctx: &mut TranspileCtx<'_>,
    ) -> Result<String> {
        // Icons use Unicode symbols to avoid escaping issues
        let (bg_color, border_color, icon, title) = match alert_type {
            "NOTE" => ("#e7f3ff", "#0969da", "\u{2139}", "Note"),        // ℹ
            "TIP" => ("#d4edda", "#1a7f37", "\u{2713}", "Tip"),          // ✓
            "IMPORTANT" => ("#f3e8ff", "#8250df", "\u{2757}", "Important"), // ❗
            "WARNING" => ("#fff8e6", "#9a6700", "\u{26A0}", "Warning"),  // ⚠
            "CAUTION" => ("#ffebe9", "#cf222e", "\u{2718}", "Caution"),  // ✘
            _ => ("#f6f8fa", "#656d76", "\u{2139}", "Note"),
        };

        let mut content = String::new();

        // Process children, skipping the [!TYPE] marker
        let mut skip_marker = true;
        for child in node.children() {
            if let NodeValue::Paragraph = &child.data.borrow().value {
                if skip_marker {
                    // Process this paragraph but skip the [!TYPE] marker
                    let mut para_content = String::new();
                    let mut first_text = true;
                    for text_node in child.children() {
                        if first_text {
                            if let NodeValue::Text(text) = &text_node.data.borrow().value {
                                // Skip the [!TYPE] part
                                let trimmed = text.trim();
                                if let Some(end) = trimmed.find(']') {
                                    let remainder = trimmed[end + 1..].trim();
                                    if !remainder.is_empty() {
                                        para_content.push_str(&escape_typst(remainder));
                                    }
                                }
                                first_text = false;
                                continue;
                            }
                        }
                        // Visit non-first nodes normally
                        self.visit_node(text_node, &mut para_content, ctx)?;
                    }
                    if !para_content.trim().is_empty() {
                        content.push_str(&para_content);
                        content.push_str("\n\n");
                    }
                    skip_marker = false;
                    continue;
                }
            }
            self.visit_node(child, &mut content, ctx)?;
        }

        Ok(format!(
            r##"#block(
  fill: rgb("{}"),
  stroke: (left: 3pt + rgb("{}")),
  radius: 4pt,
  inset: (x: 14pt, y: 10pt),
  width: 100%,
  above: 1em,
  below: 1em,
)[
  #text(fill: rgb("{}"), weight: "semibold")[#box(baseline: 1pt)[{}] {}]
  #v(4pt)
  {}
]

"##,
            bg_color, border_color, border_color, icon, title,
            content.trim()
        ))
    }
}

/// Generate a URL-safe slug from heading text, matching GFM anchor generation.
fn heading_slug(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter_map(|c| {
            if c.is_alphanumeric() || c == '-' || c == ' ' {
                Some(c)
            } else {
                None
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}

/// Resolve an image path relative to the base directory of the input file.
/// Returns the original URL unchanged for absolute paths, URLs, and data URIs.
/// For relative paths, joins with base_path and canonicalizes.
fn resolve_image_path(url: &str, base_path: Option<&Path>) -> String {
    // Don't touch URLs, data URIs, or absolute paths
    if url.starts_with("http://")
        || url.starts_with("https://")
        || url.starts_with("data:")
        || Path::new(url).is_absolute()
    {
        return url.to_string();
    }

    // Resolve relative path against the base directory
    if let Some(base) = base_path {
        let resolved = base.join(url);
        // Try to canonicalize, fall back to the joined path
        resolved
            .canonicalize()
            .unwrap_or(resolved)
            .to_string_lossy()
            .into_owned()
    } else {
        url.to_string()
    }
}

/// Escape special Typst characters.
fn escape_typst(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('*', "\\*")
        .replace('_', "\\_")
        .replace('#', "\\#")
        .replace('@', "\\@")
        .replace('$', "\\$")
        .replace('<', "\\<")
        .replace('>', "\\>")
}

/// Escape string for Typst string literals.
fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

/// Escape string for use as a Typst label.
fn escape_label(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' { c } else { '_' })
        .collect()
}

/// Convert LaTeX math to Typst math syntax.
/// Handles common LaTeX constructs and adds spacing between consecutive letters.
fn latex_to_typst_math(latex: &str) -> String {
    let mut result = latex.to_string();

    // Replace common LaTeX commands with Typst equivalents
    let replacements = [
        // Greek letters (already work in Typst with same names)
        // Operators
        (r"\cdot", " dot "),
        (r"\times", " times "),
        (r"\div", " div "),
        (r"\pm", " plus.minus "),
        (r"\mp", " minus.plus "),
        (r"\leq", " <= "),
        (r"\geq", " >= "),
        (r"\neq", " != "),
        (r"\approx", " approx "),
        (r"\equiv", " equiv "),
        (r"\sim", " tilde.op "),
        (r"\propto", " prop "),
        // Calculus
        (r"\int", " integral "),
        (r"\iint", " integral.double "),
        (r"\iiint", " integral.triple "),
        (r"\oint", " integral.cont "),
        (r"\sum", " sum "),
        (r"\prod", " product "),
        (r"\lim", " lim "),
        (r"\infty", " infinity "),
        (r"\partial", " diff "),
        (r"\nabla", " gradient "),
        // Functions
        (r"\sin", " sin "),
        (r"\cos", " cos "),
        (r"\tan", " tan "),
        (r"\log", " log "),
        (r"\ln", " ln "),
        (r"\exp", " exp "),
        (r"\sqrt", " sqrt"),
        // Arrows
        (r"\to", " -> "),
        (r"\rightarrow", " -> "),
        (r"\leftarrow", " <- "),
        (r"\Rightarrow", " => "),
        (r"\Leftarrow", " arrow.l.double "),
        (r"\leftrightarrow", " <-> "),
        // Sets
        (r"\in", " in "),
        (r"\notin", " in.not "),
        (r"\subset", " subset "),
        (r"\subseteq", " subset.eq "),
        (r"\supset", " supset "),
        (r"\supseteq", " supset.eq "),
        (r"\cup", " union "),
        (r"\cap", " sect "),
        (r"\emptyset", " nothing "),
        (r"\forall", " forall "),
        (r"\exists", " exists "),
        // Dots
        (r"\ldots", " dots.h "),
        (r"\cdots", " dots.c "),
        (r"\vdots", " dots.v "),
        (r"\ddots", " dots.down "),
        // Spacing
        (r"\quad", "  "),
        (r"\qquad", "    "),
        (r"\,", " "),
        (r"\ ", " "),
        // Braces
        (r"\{", " { "),
        (r"\}", " } "),
        (r"\left(", "("),
        (r"\right)", ")"),
        (r"\left[", "["),
        (r"\right]", "]"),
        (r"\left\{", "{"),
        (r"\right\}", "}"),
        // Text
        (r"\text{", "\""),
    ];

    for (from, to) in replacements {
        result = result.replace(from, to);
    }

    // Handle \frac{a}{b} -> (a)/(b)
    result = convert_latex_frac(&result);

    // Add spaces between consecutive letters to separate variables
    // e.g., "mc" -> "m c" but preserve things like "sin", "cos"
    result = add_spaces_between_letters(&result);

    result
}

/// Convert LaTeX \frac{num}{den} to Typst fraction syntax.
fn convert_latex_frac(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            // Check for \frac
            let mut cmd = String::new();
            while let Some(&next) = chars.peek() {
                if next.is_alphabetic() {
                    cmd.push(chars.next().unwrap());
                } else {
                    break;
                }
            }

            if cmd == "frac" {
                // Parse {numerator}{denominator}
                let num = extract_brace_content(&mut chars);
                let den = extract_brace_content(&mut chars);
                result.push_str(&format!("({})/({}) ", num, den));
            } else {
                // Put back the command
                result.push('\\');
                result.push_str(&cmd);
            }
        } else {
            result.push(c);
        }
    }

    result
}

/// Extract content within braces.
fn extract_brace_content<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>) -> String {
    // Skip whitespace
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }

    // Check for opening brace
    if chars.peek() != Some(&'{') {
        // Single character without braces
        return chars.next().map(|c| c.to_string()).unwrap_or_default();
    }

    chars.next(); // consume '{'

    let mut content = String::new();
    let mut depth = 1;

    while let Some(c) = chars.next() {
        match c {
            '{' => {
                depth += 1;
                content.push(c);
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    break;
                }
                content.push(c);
            }
            _ => content.push(c),
        }
    }

    content
}

/// Add spaces between consecutive lowercase letters.
/// This helps Typst interpret them as separate variables.
fn add_spaces_between_letters(s: &str) -> String {
    let mut result = String::new();
    let mut prev_was_letter = false;

    for c in s.chars() {
        if c.is_ascii_lowercase() {
            if prev_was_letter {
                result.push(' ');
            }
            result.push(c);
            prev_was_letter = true;
        } else {
            result.push(c);
            prev_was_letter = false;
        }
    }

    result
}
