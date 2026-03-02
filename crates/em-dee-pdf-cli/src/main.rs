//! em-dee-pdf CLI - High-quality Markdown to PDF converter

use std::io::{self, Read, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use em_dee_pdf_core::{extract_tables, replace_table, Config, Converter};
use tracing_subscriber::EnvFilter;

/// High-quality Markdown to PDF converter
#[derive(Parser, Debug)]
#[command(name = "em-dee-pdf")]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input Markdown file(s). Use '-' for stdin.
    #[arg(required = true)]
    input: Vec<PathBuf>,

    /// Output file path. Defaults to input filename with .pdf extension.
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Theme to use (built-in name or path to custom .typ file)
    #[arg(short, long, default_value = "slate")]
    theme: String,

    /// Generate table of contents
    #[arg(long)]
    toc: bool,

    /// Wrap H2 sections in visual containers (for container themes)
    #[arg(long)]
    sections: bool,

    /// Page size
    #[arg(long, default_value = "us-letter")]
    page_size: String,

    /// Output Typst source instead of PDF
    #[arg(long)]
    emit_typst: bool,

    /// Configuration file path
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Quiet mode (suppress non-error output)
    #[arg(short, long)]
    quiet: bool,

    /// Sort table(s) by column before rendering.
    /// Format: "table_index:column_index:asc|desc:num|str"
    /// Example: "0:1:desc:num" sorts first table by second column, descending, numeric
    /// Can be specified multiple times for multiple tables.
    #[arg(long = "sort-table", value_name = "SORT_SPEC")]
    sort_tables: Vec<String>,

    /// List tables in the markdown file (useful for finding table/column indices)
    #[arg(long)]
    list_tables: bool,

    /// Enable mermaid diagram rendering (requires mermaid-cli)
    #[arg(long)]
    mermaid: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up logging
    let filter = if cli.verbose {
        "em_dee_pdf=debug,em_dee_pdf_core=debug"
    } else if cli.quiet {
        "error"
    } else {
        "em_dee_pdf=info,em_dee_pdf_core=warn"
    };

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(filter))
        .with_writer(io::stderr)
        .init();

    // Load configuration
    let mut config = if let Some(ref config_path) = cli.config {
        Config::from_file(config_path)
            .with_context(|| format!("Failed to load config from {:?}", config_path))?
    } else {
        Config::default()
    };

    // Apply CLI overrides
    config.theme.name = cli.theme;
    config.output.toc = cli.toc;
    config.output.page_size = cli.page_size;
    config.output.section_containers = cli.sections;
    config.extensions.mermaid = cli.mermaid;

    // Create converter
    let converter = Converter::new(config).context("Failed to create converter")?;

    // Process input files
    for input_path in &cli.input {
        let mut markdown = if input_path.as_os_str() == "-" {
            // Read from stdin
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .context("Failed to read from stdin")?;
            buffer
        } else {
            // Read from file
            std::fs::read_to_string(input_path)
                .with_context(|| format!("Failed to read {:?}", input_path))?
        };

        // Handle --list-tables
        if cli.list_tables {
            let tables = extract_tables(&markdown);
            if tables.is_empty() {
                eprintln!("No tables found in {:?}", input_path);
            } else {
                eprintln!("Tables in {:?}:", input_path);
                for (i, table) in tables.iter().enumerate() {
                    eprintln!("\n  Table {} (line {}):", i, table.line);
                    eprintln!("    Columns: {}", table.headers.join(", "));
                    for (j, header) in table.headers.iter().enumerate() {
                        eprintln!("      [{}] {}", j, header);
                    }
                    eprintln!("    Rows: {}", table.rows.len());
                }
            }
            continue;
        }

        // Apply table sorting if specified
        for sort_spec in &cli.sort_tables {
            markdown = apply_table_sort(&markdown, sort_spec)?;
        }

        // Determine output path
        let output_path = if let Some(ref out) = cli.output {
            out.clone()
        } else if input_path.as_os_str() == "-" {
            PathBuf::from("output.pdf")
        } else {
            input_path.with_extension(if cli.emit_typst { "typ" } else { "pdf" })
        };

        if cli.emit_typst {
            // Output Typst source
            let typst_source = converter
                .to_typst(&markdown)
                .context("Failed to transpile to Typst")?;

            if output_path.as_os_str() == "-" {
                io::stdout()
                    .write_all(typst_source.as_bytes())
                    .context("Failed to write to stdout")?;
            } else {
                std::fs::write(&output_path, &typst_source)
                    .with_context(|| format!("Failed to write {:?}", output_path))?;

                if !cli.quiet {
                    eprintln!("Wrote Typst source to {:?}", output_path);
                }
            }
        } else {
            // Generate PDF
            let pdf_bytes = converter
                .convert(&markdown)
                .context("Failed to convert to PDF")?;

            std::fs::write(&output_path, &pdf_bytes)
                .with_context(|| format!("Failed to write {:?}", output_path))?;

            if !cli.quiet {
                eprintln!("Wrote PDF to {:?}", output_path);
            }
        }
    }

    Ok(())
}

/// Parse and apply a table sort specification.
/// Format: "table_index:column_index:asc|desc:num|str"
fn apply_table_sort(markdown: &str, spec: &str) -> Result<String> {
    let parts: Vec<&str> = spec.split(':').collect();
    if parts.len() < 3 {
        anyhow::bail!(
            "Invalid sort spec '{}'. Format: table_index:column_index:asc|desc[:num|str]",
            spec
        );
    }

    let table_index: usize = parts[0]
        .parse()
        .with_context(|| format!("Invalid table index: {}", parts[0]))?;

    let column_index: usize = parts[1]
        .parse()
        .with_context(|| format!("Invalid column index: {}", parts[1]))?;

    let ascending = match parts[2].to_lowercase().as_str() {
        "asc" | "ascending" => true,
        "desc" | "descending" => false,
        _ => anyhow::bail!("Invalid sort direction '{}'. Use 'asc' or 'desc'", parts[2]),
    };

    let numeric = parts
        .get(3)
        .map(|s| s.to_lowercase() == "num" || s.to_lowercase() == "numeric")
        .unwrap_or(false);

    let mut tables = extract_tables(markdown);
    if table_index >= tables.len() {
        anyhow::bail!(
            "Table index {} out of range. Only {} tables found.",
            table_index,
            tables.len()
        );
    }

    let table = &mut tables[table_index];
    if column_index >= table.headers.len() {
        anyhow::bail!(
            "Column index {} out of range. Table has {} columns.",
            column_index,
            table.headers.len()
        );
    }

    table.sort_by_column(column_index, ascending, numeric);

    Ok(replace_table(markdown, table_index, table))
}
