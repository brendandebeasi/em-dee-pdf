//! Table extraction and sorting utilities.
//!
//! This module provides utilities for extracting tables from markdown,
//! sorting them programmatically, and re-rendering them. This is primarily
//! useful for native apps that want to offer interactive table sorting.

use comrak::nodes::{AstNode, NodeValue};
use comrak::{parse_document, Arena, Options};

/// A table extracted from markdown.
#[derive(Debug, Clone)]
pub struct ExtractedTable {
    /// Header row (column names).
    pub headers: Vec<String>,
    /// Data rows.
    pub rows: Vec<Vec<String>>,
    /// Column alignments (left, center, right, or none).
    pub alignments: Vec<Alignment>,
    /// Original position in the markdown (line number).
    pub line: usize,
}

/// Column alignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
    None,
}

impl ExtractedTable {
    /// Sort the table by a column.
    ///
    /// # Arguments
    /// * `column` - Zero-based column index
    /// * `ascending` - Sort direction
    /// * `numeric` - If true, parse values as numbers for sorting
    pub fn sort_by_column(&mut self, column: usize, ascending: bool, numeric: bool) {
        if column >= self.headers.len() {
            return;
        }

        self.rows.sort_by(|a, b| {
            let val_a = a.get(column).map(|s| s.as_str()).unwrap_or("");
            let val_b = b.get(column).map(|s| s.as_str()).unwrap_or("");

            let cmp = if numeric {
                // Try to parse as numbers
                let num_a: f64 = val_a.trim().parse().unwrap_or(f64::NAN);
                let num_b: f64 = val_b.trim().parse().unwrap_or(f64::NAN);
                num_a
                    .partial_cmp(&num_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            } else {
                val_a.cmp(val_b)
            };

            if ascending {
                cmp
            } else {
                cmp.reverse()
            }
        });
    }

    /// Convert the table back to markdown.
    pub fn to_markdown(&self) -> String {
        let mut output = String::new();

        // Header row
        output.push('|');
        for header in &self.headers {
            output.push(' ');
            output.push_str(header);
            output.push_str(" |");
        }
        output.push('\n');

        // Separator row with alignments
        output.push('|');
        for alignment in &self.alignments {
            match alignment {
                Alignment::Left => output.push_str(":---|"),
                Alignment::Center => output.push_str(":--:|"),
                Alignment::Right => output.push_str("---:|"),
                Alignment::None => output.push_str("----|"),
            }
        }
        output.push('\n');

        // Data rows
        for row in &self.rows {
            output.push('|');
            for cell in row {
                output.push(' ');
                output.push_str(cell);
                output.push_str(" |");
            }
            output.push('\n');
        }

        output
    }
}

/// Extract all tables from markdown text.
pub fn extract_tables(markdown: &str) -> Vec<ExtractedTable> {
    let arena = Arena::new();
    let mut options = Options::default();
    options.extension.table = true;

    let root = parse_document(&arena, markdown, &options);

    let mut tables = Vec::new();
    extract_tables_recursive(root, &mut tables);
    tables
}

fn extract_tables_recursive<'a>(node: &'a AstNode<'a>, tables: &mut Vec<ExtractedTable>) {
    if let NodeValue::Table(table) = &node.data.borrow().value {
        let mut headers = Vec::new();
        let mut rows = Vec::new();
        let alignments: Vec<Alignment> = table
            .alignments
            .iter()
            .map(|a| match a {
                comrak::nodes::TableAlignment::Left => Alignment::Left,
                comrak::nodes::TableAlignment::Center => Alignment::Center,
                comrak::nodes::TableAlignment::Right => Alignment::Right,
                comrak::nodes::TableAlignment::None => Alignment::None,
            })
            .collect();

        for child in node.children() {
            if let NodeValue::TableRow(is_header) = &child.data.borrow().value {
                let cells: Vec<String> = child
                    .children()
                    .map(|cell| collect_cell_text(cell))
                    .collect();

                if *is_header {
                    headers = cells;
                } else {
                    rows.push(cells);
                }
            }
        }

        let line = node.data.borrow().sourcepos.start.line;

        tables.push(ExtractedTable {
            headers,
            rows,
            alignments,
            line,
        });
    }

    for child in node.children() {
        extract_tables_recursive(child, tables);
    }
}

/// Collect text content from a table cell.
fn collect_cell_text<'a>(node: &'a AstNode<'a>) -> String {
    let mut text = String::new();
    collect_text_recursive(node, &mut text);
    text.trim().to_string()
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

/// Replace a table in markdown with a sorted version.
///
/// # Arguments
/// * `markdown` - Original markdown text
/// * `table_index` - Index of the table to replace (0-based)
/// * `sorted_table` - The sorted table to insert
pub fn replace_table(markdown: &str, table_index: usize, sorted_table: &ExtractedTable) -> String {
    let tables = extract_tables(markdown);

    if table_index >= tables.len() {
        return markdown.to_string();
    }

    let target = &tables[table_index];
    let lines: Vec<&str> = markdown.lines().collect();

    // Find the start and end lines of the table
    let start_line = target.line.saturating_sub(1);
    let mut end_line = start_line;

    // Find where the table ends (first line that's not a table row)
    for (i, line) in lines.iter().enumerate().skip(start_line) {
        let trimmed = line.trim();
        if trimmed.starts_with('|') || trimmed.starts_with("|-") || trimmed.starts_with("|:") {
            end_line = i;
        } else if !trimmed.is_empty() && i > start_line {
            break;
        }
    }

    // Reconstruct the markdown
    let mut result = String::new();

    // Lines before the table
    for line in lines.iter().take(start_line) {
        result.push_str(line);
        result.push('\n');
    }

    // Insert sorted table
    result.push_str(&sorted_table.to_markdown());

    // Lines after the table
    for line in lines.iter().skip(end_line + 1) {
        result.push_str(line);
        result.push('\n');
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_table() {
        let md = r#"
| Name | Age |
|------|-----|
| Alice | 30 |
| Bob | 25 |
"#;

        let tables = extract_tables(md);
        assert_eq!(tables.len(), 1);

        let table = &tables[0];
        assert_eq!(table.headers, vec!["Name", "Age"]);
        assert_eq!(table.rows.len(), 2);
        assert_eq!(table.rows[0], vec!["Alice", "30"]);
        assert_eq!(table.rows[1], vec!["Bob", "25"]);
    }

    #[test]
    fn test_sort_table() {
        let mut table = ExtractedTable {
            headers: vec!["Name".to_string(), "Age".to_string()],
            rows: vec![
                vec!["Alice".to_string(), "30".to_string()],
                vec!["Bob".to_string(), "25".to_string()],
                vec!["Carol".to_string(), "35".to_string()],
            ],
            alignments: vec![Alignment::Left, Alignment::Right],
            line: 1,
        };

        // Sort by Age numerically, ascending
        table.sort_by_column(1, true, true);

        assert_eq!(table.rows[0][0], "Bob");
        assert_eq!(table.rows[1][0], "Alice");
        assert_eq!(table.rows[2][0], "Carol");
    }

    #[test]
    fn test_to_markdown() {
        let table = ExtractedTable {
            headers: vec!["Name".to_string(), "Age".to_string()],
            rows: vec![
                vec!["Alice".to_string(), "30".to_string()],
                vec!["Bob".to_string(), "25".to_string()],
            ],
            alignments: vec![Alignment::Left, Alignment::Right],
            line: 1,
        };

        let md = table.to_markdown();
        assert!(md.contains("| Name | Age |"));
        assert!(md.contains("| Alice | 30 |"));
    }
}
