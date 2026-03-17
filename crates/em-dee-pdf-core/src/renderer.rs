//! PDF renderer using Typst.

use std::path::PathBuf;

use typst_as_lib::TypstEngine;

use crate::config::Config;
use crate::error::{Error, Result};

/// Renders Typst source to PDF.
pub struct Renderer {
    /// Font data loaded from system paths.
    fonts: Vec<Vec<u8>>,
}

impl Renderer {
    /// Create a new renderer.
    pub fn new(config: &Config) -> Result<Self> {
        let mut font_paths = config.fonts.search_paths.clone();

        // Add system font directories
        #[cfg(target_os = "macos")]
        {
            font_paths.push("/System/Library/Fonts".into());
            font_paths.push("/Library/Fonts".into());
            if let Some(home) = dirs::home_dir() {
                font_paths.push(home.join("Library/Fonts"));
            }
        }

        #[cfg(target_os = "linux")]
        {
            font_paths.push("/usr/share/fonts".into());
            font_paths.push("/usr/local/share/fonts".into());
            if let Some(home) = dirs::home_dir() {
                font_paths.push(home.join(".fonts"));
                font_paths.push(home.join(".local/share/fonts"));
            }
        }

        #[cfg(target_os = "windows")]
        {
            if let Some(windir) = std::env::var_os("WINDIR") {
                font_paths.push(PathBuf::from(windir).join("Fonts"));
            }
        }

        // Load fonts from paths
        let fonts = Self::load_fonts_from_paths(&font_paths);

        Ok(Self { fonts })
    }

    /// Render Typst source to PDF bytes.
    pub fn render(&self, typst_source: &str, compress: bool) -> Result<Vec<u8>> {
        let font_refs: Vec<&[u8]> = self.fonts.iter().map(|f| f.as_slice()).collect();

        let engine = TypstEngine::builder()
            .main_file(typst_source)
            .fonts(font_refs)
            .with_file_system_resolver("/")
            .build();

        let result = engine.compile();

        let doc = result
            .output
            .map_err(|e| Error::Render(format!("Typst compilation failed: {:?}", e)))?;

        let pdf_options = typst_pdf::PdfOptions {
            tagged: !compress,
            ..typst_pdf::PdfOptions::default()
        };
        let pdf_bytes = typst_pdf::pdf(&doc, &pdf_options)
            .map_err(|e| Error::Render(format!("PDF export failed: {:?}", e)))?;

        if compress {
            Self::compress_pdf(pdf_bytes)
        } else {
            Ok(pdf_bytes)
        }
    }

    fn compress_pdf(pdf_bytes: Vec<u8>) -> Result<Vec<u8>> {
        let mut doc = lopdf::Document::load_mem(&pdf_bytes)
            .map_err(|e| Error::Render(format!("Failed to load PDF for compression: {}", e)))?;

        doc.compress();

        let mut out = Vec::new();
        doc.save_to(&mut out)
            .map_err(|e| Error::Render(format!("Failed to save compressed PDF: {}", e)))?;

        Ok(out)
    }

    /// Load fonts from multiple paths.
    fn load_fonts_from_paths(paths: &[PathBuf]) -> Vec<Vec<u8>> {
        let mut fonts = Vec::new();

        for path in paths {
            if !path.exists() {
                continue;
            }

            if path.is_file() {
                if let Some(data) = Self::load_font_file(path) {
                    fonts.push(data);
                }
            } else if path.is_dir() {
                Self::load_fonts_from_dir(path, &mut fonts);
            }
        }

        fonts
    }

    /// Load fonts recursively from a directory.
    fn load_fonts_from_dir(dir: &PathBuf, fonts: &mut Vec<Vec<u8>>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                Self::load_fonts_from_dir(&path, fonts);
            } else if let Some(data) = Self::load_font_file(&path) {
                fonts.push(data);
            }
        }
    }

    /// Load a single font file.
    fn load_font_file(path: &PathBuf) -> Option<Vec<u8>> {
        let ext = path.extension()?.to_str()?.to_lowercase();
        if !matches!(ext.as_str(), "ttf" | "otf" | "ttc" | "otc") {
            return None;
        }

        std::fs::read(path).ok()
    }
}
