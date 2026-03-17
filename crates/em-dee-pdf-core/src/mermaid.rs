//! Mermaid diagram rendering support.
//!
//! This module provides utilities for rendering mermaid diagrams to SVG.
//! It supports multiple backends:
//! 1. Shell out to `mmdc` (mermaid-cli) if available
//! 2. Generate placeholder for unsupported diagrams

use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

use crate::error::{Error, Result};

/// Check if mermaid-cli (mmdc) is available on the system.
pub fn is_mmdc_available() -> bool {
    Command::new("mmdc")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Render a mermaid diagram to SVG using mmdc.
///
/// Returns the SVG content as a string, or an error if rendering fails.
pub fn render_to_svg(mermaid_source: &str) -> Result<String> {
    // Create temp files for input and output
    let mut input_file = NamedTempFile::new()
        .map_err(|e| Error::Render(format!("Failed to create temp file: {}", e)))?;

    input_file
        .write_all(mermaid_source.as_bytes())
        .map_err(|e| Error::Render(format!("Failed to write mermaid source: {}", e)))?;

    let output_file = NamedTempFile::new()
        .map_err(|e| Error::Render(format!("Failed to create temp file: {}", e)))?;

    // Run mmdc
    let output = Command::new("mmdc")
        .args([
            "-i",
            input_file.path().to_str().unwrap(),
            "-o",
            output_file.path().to_str().unwrap(),
            "-e",
            "svg",
            "--backgroundColor",
            "transparent",
        ])
        .output()
        .map_err(|e| Error::Render(format!("Failed to run mmdc: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::Render(format!("mmdc failed: {}", stderr)));
    }

    // Read the SVG output
    std::fs::read_to_string(output_file.path())
        .map_err(|e| Error::Render(format!("Failed to read SVG output: {}", e)))
}

/// Render a mermaid diagram to PNG using mmdc.
///
/// Returns the PNG bytes, or an error if rendering fails.
pub fn render_to_png(mermaid_source: &str) -> Result<Vec<u8>> {
    // Create temp files for input and output
    let mut input_file = NamedTempFile::new()
        .map_err(|e| Error::Render(format!("Failed to create temp file: {}", e)))?;

    input_file
        .write_all(mermaid_source.as_bytes())
        .map_err(|e| Error::Render(format!("Failed to write mermaid source: {}", e)))?;

    let output_file = NamedTempFile::with_suffix(".png")
        .map_err(|e| Error::Render(format!("Failed to create temp file: {}", e)))?;

    // Run mmdc
    let output = Command::new("mmdc")
        .args([
            "-i",
            input_file.path().to_str().unwrap(),
            "-o",
            output_file.path().to_str().unwrap(),
            "-e",
            "png",
            "--backgroundColor",
            "white",
            "-s",
            "2", // scale factor for better quality
        ])
        .output()
        .map_err(|e| Error::Render(format!("Failed to run mmdc: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::Render(format!("mmdc failed: {}", stderr)));
    }

    // Read the PNG output
    std::fs::read(output_file.path())
        .map_err(|e| Error::Render(format!("Failed to read PNG output: {}", e)))
}

/// Detect the type of mermaid diagram from its source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MermaidDiagramType {
    Flowchart,
    SequenceDiagram,
    ClassDiagram,
    StateDiagram,
    EntityRelationship,
    Gantt,
    PieChart,
    GitGraph,
    Mindmap,
    Timeline,
    Unknown,
}

impl MermaidDiagramType {
    /// Detect diagram type from mermaid source.
    pub fn detect(source: &str) -> Self {
        let first_line = source.lines().next().unwrap_or("").trim().to_lowercase();

        if first_line.starts_with("graph") || first_line.starts_with("flowchart") {
            MermaidDiagramType::Flowchart
        } else if first_line.starts_with("sequencediagram") {
            MermaidDiagramType::SequenceDiagram
        } else if first_line.starts_with("classdiagram") {
            MermaidDiagramType::ClassDiagram
        } else if first_line.starts_with("statediagram") {
            MermaidDiagramType::StateDiagram
        } else if first_line.starts_with("erdiagram") {
            MermaidDiagramType::EntityRelationship
        } else if first_line.starts_with("gantt") {
            MermaidDiagramType::Gantt
        } else if first_line.starts_with("pie") {
            MermaidDiagramType::PieChart
        } else if first_line.starts_with("gitgraph") {
            MermaidDiagramType::GitGraph
        } else if first_line.starts_with("mindmap") {
            MermaidDiagramType::Mindmap
        } else if first_line.starts_with("timeline") {
            MermaidDiagramType::Timeline
        } else {
            MermaidDiagramType::Unknown
        }
    }
}

/// Generate a Typst fallback/placeholder for when mmdc is not available.
pub fn generate_placeholder(mermaid_source: &str) -> String {
    let diagram_type = MermaidDiagramType::detect(mermaid_source);
    let type_str = match diagram_type {
        MermaidDiagramType::Flowchart => "Flowchart",
        MermaidDiagramType::SequenceDiagram => "Sequence Diagram",
        MermaidDiagramType::ClassDiagram => "Class Diagram",
        MermaidDiagramType::StateDiagram => "State Diagram",
        MermaidDiagramType::EntityRelationship => "ER Diagram",
        MermaidDiagramType::Gantt => "Gantt Chart",
        MermaidDiagramType::PieChart => "Pie Chart",
        MermaidDiagramType::GitGraph => "Git Graph",
        MermaidDiagramType::Mindmap => "Mindmap",
        MermaidDiagramType::Timeline => "Timeline",
        MermaidDiagramType::Unknown => "Mermaid Diagram",
    };

    format!(
        r##"#block(
  fill: rgb("#f0f0f0"),
  stroke: 1pt + rgb("#cccccc"),
  radius: 4pt,
  inset: 16pt,
  width: 100%,
)[
  #align(center)[
    #text(fill: rgb("#666666"), weight: "medium")[{} (mermaid-cli not available)]

    #v(8pt)

    #text(size: 9pt, fill: rgb("#888888"))[Install mermaid-cli: `npm install -g @mermaid-js/mermaid-cli`]
  ]
]

"##,
        type_str
    )
}

/// Save a PNG and return the path for Typst to reference.
/// Returns (path, file_handle) - keep the handle to prevent cleanup.
pub fn save_png_for_typst(png_bytes: &[u8]) -> Result<(String, NamedTempFile)> {
    let mut file = NamedTempFile::with_suffix(".png")
        .map_err(|e| Error::Render(format!("Failed to create temp file: {}", e)))?;

    file.write_all(png_bytes)
        .map_err(|e| Error::Render(format!("Failed to write PNG: {}", e)))?;

    let path = file.path().to_string_lossy().to_string();
    Ok((path, file))
}

/// Render a mermaid diagram and return Typst code to embed it.
/// Returns the Typst code and a temp file handle that must be kept alive
/// until the PDF is rendered.
pub fn render_and_embed(mermaid_source: &str) -> Result<(String, NamedTempFile)> {
    let png_bytes = render_to_png(mermaid_source)?;
    let (path, file) = save_png_for_typst(&png_bytes)?;

    // Escape backslashes for Windows paths
    let escaped_path = path.replace('\\', "/");

    let typst_code = format!("#figure(\n  image(\"{}\"),\n)\n\n", escaped_path);

    Ok((typst_code, file))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_diagram_type() {
        assert_eq!(
            MermaidDiagramType::detect("graph TD\nA-->B"),
            MermaidDiagramType::Flowchart
        );
        assert_eq!(
            MermaidDiagramType::detect("flowchart LR\nA-->B"),
            MermaidDiagramType::Flowchart
        );
        assert_eq!(
            MermaidDiagramType::detect("sequenceDiagram\nA->>B: Hello"),
            MermaidDiagramType::SequenceDiagram
        );
        assert_eq!(
            MermaidDiagramType::detect("pie\n\"A\": 30"),
            MermaidDiagramType::PieChart
        );
    }

    #[test]
    fn test_placeholder_generation() {
        let placeholder = generate_placeholder("graph TD\nA-->B");
        assert!(placeholder.contains("Flowchart"));
        assert!(placeholder.contains("mermaid-cli not available"));
    }
}
