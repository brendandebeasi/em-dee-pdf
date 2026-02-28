//! Error types for md-pdf-core.

use thiserror::Error;

/// Result type alias for md-pdf-core operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during Markdown to PDF conversion.
#[derive(Error, Debug)]
pub enum Error {
    /// Error parsing Markdown input.
    #[error("failed to parse markdown: {0}")]
    Parse(String),

    /// Error parsing front matter.
    #[error("failed to parse front matter: {0}")]
    FrontMatter(String),

    /// Error transpiling to Typst.
    #[error("failed to transpile to typst: {0}")]
    Transpile(String),

    /// Error rendering PDF.
    #[error("failed to render PDF: {0}")]
    Render(String),

    /// Error loading theme.
    #[error("failed to load theme '{name}': {reason}")]
    Theme { name: String, reason: String },

    /// Error loading configuration.
    #[error("failed to load config: {0}")]
    Config(String),

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Error resolving image or resource.
    #[error("failed to resolve resource '{path}': {reason}")]
    Resource { path: String, reason: String },
}
