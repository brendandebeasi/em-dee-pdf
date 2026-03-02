---
title: em-dee-pdf Feature Showcase
author: em-dee-pdf
toc: true
---

# Feature Showcase

This document demonstrates the full range of formatting capabilities available in em-dee-pdf.

## Typography & Inline Formatting

Regular text flows naturally with **bold emphasis**, *italic styling*, and ***bold italic*** combined. You can also use `inline code` for technical terms and ~~strikethrough~~ for corrections.

Links work seamlessly: visit [the Typst project](https://typst.app) or reference an [internal section](#tables-data).

## Lists & Task Lists

Bullet lists with proper nesting:

- First-level item with a bullet
- Another item at this level
  - Second-level nested item
  - More detail here
    - Third-level deep nesting
- Back to the top level

Numbered lists maintain their order:

1. Parse the Markdown source
2. Transform to Typst AST
3. Render the final PDF
4. Write to output file

Task lists track completion:

- [x] Markdown parsing
- [x] Typst transpilation
- [ ] Custom theme support
- [ ] Plugin architecture

## Code Blocks

Syntax-highlighted code blocks with language detection:

```rust
use em_dee_pdf_core::{Converter, Config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();
    let converter = Converter::new(config)?;

    let markdown = std::fs::read_to_string("input.md")?;
    let pdf = converter.convert(&markdown, None)?;

    std::fs::write("output.pdf", pdf)?;
    Ok(())
}
```

```python
import subprocess

def convert_markdown(input_path: str, output_path: str, theme: str = "slate"):
    """Convert a Markdown file to PDF using em-dee-pdf."""
    subprocess.run([
        "em-dee-pdf", input_path,
        "--theme", theme,
        "-o", output_path,
    ], check=True)
```

## Tables & Data

| Metric | Q1 2025 | Q2 2025 | Change |
|--------|---------|---------|--------|
| Revenue | $2.4M | $3.1M | +29% |
| Users | 14,200 | 21,800 | +54% |
| Uptime | 99.92% | 99.97% | +0.05% |
| Response Time | 142ms | 98ms | -31% |

## Block Quotes

> The best way to predict the future is to invent it.
> — Alan Kay

## Alerts & Admonitions

> [!NOTE]
> em-dee-pdf uses Typst as its rendering engine, producing high-quality typographic output with full Unicode support.

> [!TIP]
> Use `--emit-typst` to inspect the intermediate Typst source for debugging or manual adjustments.

> [!WARNING]
> Large documents with many mermaid diagrams may take longer to render due to external CLI invocations.

## Math Support

Inline math works with LaTeX syntax: the quadratic formula is $x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}$.

Display math gets its own block:

$$E = mc^2$$

$$\int_0^\infty e^{-x^2} dx = \frac{\sqrt{\pi}}{2}$$

---

*Generated with em-dee-pdf*
