//! Theme system for PDF styling.

use std::path::Path;

use crate::config::ThemeConfig;
use crate::error::{Error, Result};

/// A theme defines the visual styling of the PDF output.
#[derive(Debug, Clone)]
pub struct Theme {
    /// Theme name.
    pub name: String,

    /// Typst preamble (imports, set rules, show rules).
    pub preamble: String,

    /// Whether this is a built-in theme.
    pub builtin: bool,
}

impl Theme {
    /// Load a theme by name or path.
    pub fn load(config: &ThemeConfig) -> Result<Self> {
        let name = &config.name;

        // Check if it's a path to a custom theme
        if name.ends_with(".typ") || name.contains('/') || name.contains('\\') {
            return Self::load_custom(Path::new(name));
        }

        // Load built-in theme
        Self::load_builtin(name)
    }

    /// Load a built-in theme.
    fn load_builtin(name: &str) -> Result<Self> {
        let preamble = match name {
            // Special themes
            "corporate" => include_str!("../../../themes/corporate.typ"),
            "tech" => include_str!("../../../themes/tech.typ"),
            "book" => include_str!("../../../themes/book.typ"),
            "coral" => include_str!("../../../themes/coral.typ"),

            // Container themes
            "cards" => include_str!("../../../themes/cards.typ"),
            "panels" => include_str!("../../../themes/panels.typ"),
            "boxed" => include_str!("../../../themes/boxed.typ"),

            // Tailwind color themes
            "slate" => include_str!("../../../themes/slate.typ"),
            "zinc" => include_str!("../../../themes/zinc.typ"),
            "stone" => include_str!("../../../themes/stone.typ"),
            "emerald" => include_str!("../../../themes/emerald.typ"),
            "teal" => include_str!("../../../themes/teal.typ"),
            "sky" => include_str!("../../../themes/sky.typ"),
            "indigo" => include_str!("../../../themes/indigo.typ"),
            "violet" => include_str!("../../../themes/violet.typ"),
            "rose" => include_str!("../../../themes/rose.typ"),
            "amber" => include_str!("../../../themes/amber.typ"),
            "orange" => include_str!("../../../themes/orange.typ"),

            _ => {
                return Err(Error::Theme {
                    name: name.to_string(),
                    reason: format!(
                        "unknown built-in theme. Available themes:\n\
                         \n  Special:    corporate, tech, book, coral\
                         \n  Container:  cards, panels, boxed\
                         \n  Neutral:    slate, zinc, stone\
                         \n  Colors:     emerald, teal, sky, indigo, violet, rose, amber, orange"
                    ),
                });
            }
        };

        Ok(Self {
            name: name.to_string(),
            preamble: preamble.to_string(),
            builtin: true,
        })
    }

    /// Load a custom theme from file.
    fn load_custom(path: &Path) -> Result<Self> {
        let preamble = std::fs::read_to_string(path).map_err(|e| Error::Theme {
            name: path.display().to_string(),
            reason: e.to_string(),
        })?;

        Ok(Self {
            name: path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("custom")
                .to_string(),
            preamble,
            builtin: false,
        })
    }

    /// Get the Typst preamble for this theme.
    pub fn preamble(&self) -> &str {
        &self.preamble
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::load_builtin("slate").expect("slate theme should always exist")
    }
}
