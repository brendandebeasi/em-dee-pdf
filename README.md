# em-dee-pdf

Rust CLI that converts Markdown files to beautiful PDFs using Typst as the rendering engine.

<p align="center">
  <a href="examples/screenshots/slate.png"><img src="examples/screenshots/slate.png" width="150"></a>
  <a href="examples/screenshots/coral.png"><img src="examples/screenshots/coral.png" width="150"></a>
  <a href="examples/screenshots/tech.png"><img src="examples/screenshots/tech.png" width="150"></a>
  <a href="examples/screenshots/book.png"><img src="examples/screenshots/book.png" width="150"></a>
  <a href="examples/screenshots/cards.png"><img src="examples/screenshots/cards.png" width="150"></a>
</p>

## What it does

em-dee-pdf transforms Markdown documents into styled PDFs. The conversion process follows this architecture:
Markdown -> comrak parser -> AST -> Typst transpiler -> Typst renderer -> PDF.

## Quickstart

### Installation

Install from source:
```bash
cargo install --path crates/em-dee-pdf-cli
```

Or build the binary:
```bash
cargo build --release
# Binary at target/release/em-dee-pdf
```

### Usage Examples

```bash
# Basic conversion
em-dee-pdf document.md

# Choose a theme
em-dee-pdf document.md --theme coral

# Multiple files
em-dee-pdf chapter1.md chapter2.md

# Pipe from stdin
cat document.md | em-dee-pdf - -o output.pdf

# See the generated Typst source
em-dee-pdf document.md --emit-typst

# Use a container theme with sections
em-dee-pdf document.md --theme cards --sections

# Sort a table before rendering
em-dee-pdf report.md --sort-table "0:2:desc:num"
```

## CLI Reference

```
em-dee-pdf [OPTIONS] <INPUT>...

Arguments:
  <INPUT>...   Input Markdown file(s). Use '-' for stdin.

Options:
  -o, --output <OUTPUT>        Output file path. Defaults to input filename with .pdf extension.
  -t, --theme <THEME>          Theme name or path to custom .typ file [default: slate]
      --toc                    Generate table of contents
      --sections               Wrap H2 sections in visual containers (for container themes)
      --page-size <PAGE_SIZE>  Page size [default: us-letter]
      --emit-typst             Output Typst source instead of PDF
  -c, --config <CONFIG>        Configuration file path (TOML)
  -v, --verbose                Verbose output
  -q, --quiet                  Quiet mode
      --sort-table <SORT_SPEC> Sort table by column. Format: "table_index:column_index:asc|desc[:num|str]"
      --list-tables            List tables in the markdown (useful for finding indices)
      --mermaid                Enable mermaid diagram rendering (requires mermaid-cli)
  -h, --help                   Print help
  -V, --version                Print version
```

## Themes

The project includes 18 built-in themes:

- **Special**: corporate, tech, book, coral
- **Container**: cards, panels, boxed (use with --sections)
- **Neutral**: slate (default), zinc, stone
- **Colors**: emerald, teal, sky, indigo, violet, rose, amber, orange

Container themes wrap H2 sections in styled boxes when the `--sections` flag is used.
For custom themes, pass a path to a `.typ` file via the `--theme` option.

### Theme Gallery

<table>
<tr>
<td align="center"><strong>Slate</strong> — Table of Contents<br><img src="examples/screenshots/slate.png" width="380"></td>
<td align="center"><strong>Coral</strong> — Typography & Lists<br><img src="examples/screenshots/coral.png" width="380"></td>
</tr>
<tr>
<td align="center"><strong>Tech</strong> — Code & Tables<br><img src="examples/screenshots/tech.png" width="380"></td>
<td align="center"><strong>Book</strong> — Alerts & Math<br><img src="examples/screenshots/book.png" width="380"></td>
</tr>
<tr>
<td align="center"><strong>Cards</strong> — Section Containers<br><img src="examples/screenshots/cards.png" width="380"></td>
<td align="center"><strong>Corporate</strong> — Code & Data<br><img src="examples/screenshots/corporate.png" width="380"></td>
</tr>
</table>

Generate these samples yourself:

```bash
em-dee-pdf examples/showcase.md --theme slate --toc -o examples/slate.pdf
em-dee-pdf examples/showcase.md --theme coral --toc -o examples/coral.pdf
em-dee-pdf examples/showcase.md --theme tech --toc -o examples/tech.pdf
em-dee-pdf examples/showcase.md --theme book --toc -o examples/book.pdf
em-dee-pdf examples/showcase.md --theme cards --toc --sections -o examples/cards.pdf
em-dee-pdf examples/showcase.md --theme corporate --toc -o examples/corporate.pdf
```

## Configuration

Configure default behavior using a TOML file.

```toml
[theme]
name = "coral"

[output]
page_size = "us-letter"
toc = false
toc_depth = 3
page_numbers = true
section_containers = false

[fonts]
search_paths = []
# default_family = "Arial"
# monospace_family = "Fira Code"

[extensions]
tables = true
task_lists = true
strikethrough = true
footnotes = true
math = true
autolinks = true
superscript = true
description_lists = true
front_matter = true
syntax_highlighting = true
mermaid = false
```

## Docker

Build the image:
```bash
./scripts/docker-build.sh
```

Run with Docker:
```bash
docker run --rm -v "$(pwd):/work" em-dee-pdf:latest input.md -o output.pdf
```

Or use the wrapper script:
```bash
./scripts/em-dee-pdf-docker input.md -o output.pdf
```

## Library Usage

The core conversion logic is available as a Rust library in the `em-dee-pdf-core` crate.

```rust
use em_dee_pdf_core::{Converter, Config};

let config = Config::default();
let converter = Converter::new(config)?;
let pdf_bytes = converter.convert("# Hello World")?;
std::fs::write("output.pdf", pdf_bytes)?;
```

## License

This project is licensed under either the MIT License or the Apache License, Version 2.0.
