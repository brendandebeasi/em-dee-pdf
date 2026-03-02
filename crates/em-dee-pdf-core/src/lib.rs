//! # em-dee-pdf-core
//!
//! Core library for converting Markdown to PDF via Typst.
//!
//! ## Architecture
//!
//! ```text
//! Markdown -> Parser (comrak) -> AST -> Transpiler -> Typst -> Renderer -> PDF
//! ```
//!
//! ## Example
//!
//! ```rust,ignore
//! use em_dee_pdf_core::{Converter, Config};
//!
//! let config = Config::default();
//! let converter = Converter::new(config)?;
//! let pdf_bytes = converter.convert("# Hello World", None)?;
//! std::fs::write("output.pdf", pdf_bytes)?;
//! ```

pub mod config;
pub mod error;
pub mod mermaid;
pub mod parser;
pub mod tables;
pub mod theme;
pub mod transpiler;
pub mod renderer;

pub use config::Config;
pub use error::{Error, Result};
pub use parser::Parser;
pub use tables::{extract_tables, ExtractedTable, Alignment, replace_table};
pub use theme::Theme;
pub use transpiler::{TranspileResult, Transpiler};
pub use renderer::Renderer;

/// High-level converter that orchestrates the full pipeline.
pub struct Converter {
    #[allow(dead_code)]
    config: Config,
    parser: Parser,
    transpiler: Transpiler,
    renderer: Renderer,
}

impl Converter {
    /// Create a new converter with the given configuration.
    pub fn new(config: Config) -> Result<Self> {
        let parser = Parser::new(&config);
        let theme = Theme::load(&config.theme)?;
        let transpiler = Transpiler::new(theme, &config);
        let renderer = Renderer::new(&config)?;

        Ok(Self {
            config,
            parser,
            transpiler,
            renderer,
        })
    }

    /// Convert Markdown to PDF bytes.
    ///
    /// `base_path` is the directory containing the input Markdown file,
    /// used to resolve relative image paths. Pass `None` for string-only
    /// input where relative paths are not expected.
    pub fn convert(&self, markdown: &str, base_path: Option<&std::path::Path>) -> Result<Vec<u8>> {
        // Parse markdown to AST
        let document = self.parser.parse(markdown)?;

        // Transpile AST to Typst source (with temp file tracking for mermaid)
        let transpile_result = self.transpiler.transpile_with_resources(&document, base_path)?;

        // Render Typst to PDF (temp_files stay alive until this completes)
        let pdf_bytes = self.renderer.render(&transpile_result.source)?;

        // temp_files are dropped here, after rendering is complete
        drop(transpile_result.temp_files);

        Ok(pdf_bytes)
    }

    /// Convert Markdown to Typst source (useful for debugging).
    ///
    /// `base_path` is the directory containing the input Markdown file,
    /// used to resolve relative image paths.
    /// Note: Mermaid diagrams will have temp file paths that may not exist
    /// after this function returns. Use convert() for full PDF generation.
    pub fn to_typst(&self, markdown: &str, base_path: Option<&std::path::Path>) -> Result<String> {
        let document = self.parser.parse(markdown)?;
        self.transpiler.transpile(&document, base_path)
    }
}
