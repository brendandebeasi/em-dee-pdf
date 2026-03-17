//! Configuration for the Markdown to PDF converter.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration for the converter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Theme to use (built-in name or path to custom theme).
    pub theme: ThemeConfig,

    /// Output configuration.
    pub output: OutputConfig,

    /// Font configuration.
    pub fonts: FontConfig,

    /// Parser extensions configuration.
    pub extensions: ExtensionConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: ThemeConfig::default(),
            output: OutputConfig::default(),
            fonts: FontConfig::default(),
            extensions: ExtensionConfig::default(),
        }
    }
}

/// Theme configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ThemeConfig {
    /// Theme name (built-in) or path to custom theme file.
    pub name: String,

    /// Optional overrides to apply on top of the theme.
    pub overrides: Option<ThemeOverrides>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: "slate".to_string(),
            overrides: None,
        }
    }
}

/// Overrides that can be applied to any theme.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeOverrides {
    pub font_size: Option<String>,
    pub page_size: Option<String>,
    pub margin: Option<String>,
    pub font_family: Option<String>,
    pub line_height: Option<f64>,
}

/// Output configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct OutputConfig {
    /// Page size (e.g., "a4", "us-letter").
    pub page_size: String,

    /// Whether to generate a table of contents.
    pub toc: bool,

    /// Maximum depth for table of contents.
    pub toc_depth: u8,

    /// Whether to include page numbers.
    pub page_numbers: bool,

    /// Page number position.
    pub page_number_position: PageNumberPosition,

    /// Whether to wrap H2 sections in containers.
    /// When enabled, content between H2 headings is wrapped in #md-section[] blocks.
    pub section_containers: bool,

    /// Remove background fills for print-friendly output.
    /// Strips page, code block, table, and container backgrounds.
    pub no_background: bool,

    /// Compress the resulting PDF and embedded resources.
    /// Disables PDF tagging and applies deflate compression to PDF streams.
    pub compress: bool,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            page_size: "us-letter".to_string(),
            toc: false,
            toc_depth: 3,
            page_numbers: true,
            page_number_position: PageNumberPosition::BottomCenter,
            section_containers: false,
            no_background: false,
            compress: false,
        }
    }
}

/// Position for page numbers.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum PageNumberPosition {
    #[default]
    BottomCenter,
    BottomRight,
    BottomLeft,
    TopCenter,
    TopRight,
    TopLeft,
}

/// Font configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FontConfig {
    /// Additional directories to search for fonts.
    pub search_paths: Vec<PathBuf>,

    /// Default font family override.
    pub default_family: Option<String>,

    /// Default monospace font override.
    pub monospace_family: Option<String>,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            search_paths: Vec::new(),
            default_family: None,
            monospace_family: None,
        }
    }
}

/// Parser extension configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ExtensionConfig {
    /// Enable GitHub Flavored Markdown tables.
    pub tables: bool,

    /// Enable task lists (checkboxes).
    pub task_lists: bool,

    /// Enable strikethrough.
    pub strikethrough: bool,

    /// Enable footnotes.
    pub footnotes: bool,

    /// Enable LaTeX math ($...$ and $$...$$).
    pub math: bool,

    /// Enable autolinks.
    pub autolinks: bool,

    /// Enable superscript (^text^).
    pub superscript: bool,

    /// Enable description lists.
    pub description_lists: bool,

    /// Enable front matter parsing.
    pub front_matter: bool,

    /// Enable syntax highlighting for code blocks.
    pub syntax_highlighting: bool,

    /// Enable mermaid diagram rendering.
    pub mermaid: bool,
}

impl Default for ExtensionConfig {
    fn default() -> Self {
        Self {
            tables: true,
            task_lists: true,
            strikethrough: true,
            footnotes: true,
            math: true,
            autolinks: true,
            superscript: true,
            description_lists: true,
            front_matter: true,
            syntax_highlighting: true,
            mermaid: false, // Requires external dependency
        }
    }
}

impl Config {
    /// Load configuration from a TOML file.
    pub fn from_file(path: &std::path::Path) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        toml::from_str(&content).map_err(|e| crate::Error::Config(e.to_string()))
    }

    /// Load configuration from a TOML string.
    pub fn from_str(s: &str) -> crate::Result<Self> {
        toml::from_str(s).map_err(|e| crate::Error::Config(e.to_string()))
    }
}
