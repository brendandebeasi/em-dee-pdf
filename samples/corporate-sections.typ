// Corporate theme for md-pdf
// Premium business documents - inspired by McKinsey, BCG, Big 4 reports
// Authoritative, clean, professional
// Use with: --sections flag for contained sections

// =============================================================================
// COLOR PALETTE - Navy blue corporate identity
// =============================================================================
#let navy = rgb("#0c1f3f")            // Deep navy - primary
#let navy-light = rgb("#1e3a5f")      // Lighter navy
#let navy-muted = rgb("#4a6fa5")      // Muted navy for accents

#let text-primary = rgb("#1a1a2e")    // Near black
#let text-secondary = rgb("#4a5568")  // Slate gray
#let text-muted = rgb("#718096")      // Light gray

#let surface = rgb("#f7fafc")         // Very light gray
#let surface-alt = rgb("#edf2f7")     // Alternate surface
#let border = rgb("#e2e8f0")          // Light border
#let gold = rgb("#c9a227")            // Gold accent for premium feel

#let code-bg = rgb("#1e3a5f")         // Navy code background
#let code-text = rgb("#e2e8f0")       // Light code text

// =============================================================================
// SECTION CONTAINER FUNCTION
// =============================================================================
#let md-section(content) = {
  block(
    fill: white,
    stroke: (left: 3pt + gold, rest: 1pt + border),
    radius: (right: 6pt),
    inset: (x: 20pt, top: 16pt, bottom: 20pt),
    width: 100%,
    above: 1.5em,
    below: 1.5em,
    breakable: true,
    content
  )
}

// =============================================================================
// PAGE SETUP - Professional with page numbers
// =============================================================================
#set page(
  paper: "us-letter",
  margin: (x: 1in, y: 1in),
  numbering: "1 / 1",
  number-align: right,
  header: context {
    if counter(page).get().first() > 1 {
      set text(size: 8pt, fill: text-muted, tracking: 0.05em)
      upper[Confidential]
      h(1fr)
      line(length: 100%, stroke: 0.5pt + border)
    }
  },
  footer: context {
    line(length: 100%, stroke: 0.5pt + border)
    v(4pt)
    set text(size: 8pt, fill: text-muted)
    h(1fr)
    counter(page).display("1 / 1", both: true)
  },
)

// =============================================================================
// TYPOGRAPHY - Professional serif/sans combination
// =============================================================================
#set text(
  font: ("Source Sans Pro", "Calibri", "Helvetica Neue", "sans-serif"),
  size: 10.5pt,
  fill: text-primary,
  lang: "en",
)

#set par(
  leading: 0.75em,
  justify: true,
)

#show par: set block(spacing: 0.85em)

// =============================================================================
// HEADINGS - Authoritative hierarchy
// =============================================================================
#show heading.where(level: 1): it => {
  set text(
    font: ("Georgia", "Times New Roman", "serif"),
    size: 26pt,
    weight: "bold",
    fill: navy,
  )
  block(
    above: 1em,
    below: 0.8em,
    stroke: (bottom: 3pt + gold),
    inset: (bottom: 12pt),
    width: 100%,
    it
  )
}

#show heading.where(level: 2): it => {
  set text(
    font: ("Georgia", "Times New Roman", "serif"),
    size: 18pt,
    weight: "semibold",
    fill: navy,
  )
  block(above: 1.6em, below: 0.6em, it)
}

#show heading.where(level: 3): it => {
  set text(size: 14pt, weight: "semibold", fill: navy-light)
  block(above: 1.4em, below: 0.5em, it)
}

#show heading.where(level: 4): it => {
  set text(size: 12pt, weight: "semibold", fill: text-secondary)
  block(above: 1.2em, below: 0.4em, it)
}

// =============================================================================
// CODE BLOCKS - Professional dark style
// =============================================================================
#show raw.where(block: false): box.with(
  fill: surface-alt,
  inset: (x: 4pt, y: 2pt),
  radius: 2pt,
)

#show raw.where(block: true): it => {
  set text(
    font: ("Source Code Pro", "SF Mono", "Consolas", "monospace"),
    size: 9pt,
    fill: code-text,
  )
  block(
    fill: code-bg,
    inset: 14pt,
    radius: 4pt,
    width: 100%,
    above: 1em,
    below: 1em,
    it
  )
}

// =============================================================================
// LINKS
// =============================================================================
#show link: it => {
  set text(fill: navy-muted, weight: "medium")
  underline(stroke: 0.5pt + navy-muted, offset: 2pt, it)
}

// =============================================================================
// BLOCKQUOTES - Executive callout style
// =============================================================================
#show quote: it => {
  set text(style: "italic", fill: text-secondary)
  block(
    fill: surface,
    stroke: (left: 4pt + gold),
    inset: (left: 16pt, right: 16pt, y: 12pt),
    radius: (right: 3pt),
    above: 1em,
    below: 1em,
    it
  )
}

// =============================================================================
// TABLES - Professional with navy header
// =============================================================================
#set table(
  stroke: 1pt + border,
  inset: 10pt,
  fill: (x, y) => {
    if y == 0 { navy }
    else if calc.even(y) { surface }
    else { white }
  },
)

#show table: it => block(above: 1em, below: 1.5em, breakable: false, it)

#show table.cell.where(y: 0): set text(weight: "semibold", fill: white)

// =============================================================================
// LISTS - Professional markers
// =============================================================================
#set list(
  indent: 1.25em,
  body-indent: 0.5em,
  marker: (
    text(fill: navy, weight: "bold")[\*],
    text(fill: navy-muted)[-],
    text(fill: text-muted)[>],
  ),
)

#set enum(indent: 1.25em, body-indent: 0.5em)

// =============================================================================
// HORIZONTAL RULES
// =============================================================================
#set line(length: 100%, stroke: 1pt + border)
#show line: it => block(above: 1.5em, below: 1.5em, it)

// =============================================================================
// FIGURES
// =============================================================================
#show figure: it => {
  set align(center)
  block(
    stroke: 1pt + border,
    radius: 4pt,
    inset: 12pt,
    above: 1em,
    below: 1em,
    breakable: false,
    it
  )
}

#show figure.caption: it => {
  set text(size: 9pt, fill: text-secondary)
  it
}


= Project Overview

This document demonstrates the visual styling of various themes.

#md-section[
== Key Features

Here’s what makes this project stand out:

- Fast markdown to PDF conversion

- Multiple built-in themes

  - Professional designs

- Creative options


- Customizable styling


=== Technical Specifications

The system uses Typst for rendering, providing:

1. High-quality typography

2. Consistent page layouts

3. Beautiful code blocks


]

#md-section[
== Code Example

```rust
fn main() {
    let message = "Hello, world!";
    println!("{}", message);
}
```

]

#md-section[
== Data Summary

#table(
  columns: 3,
  align: (auto, auto, auto),
  [*Feature*],
  [*Status*],
  [*Priority*],
  [Themes],
  [Complete],
  [High],
  [Tables],
  [Complete],
  [Medium],
  [Images],
  [Planned],
  [Low],
)

]

#md-section[
== Quote

#quote(block: true)[
Design is not just what it looks like and feels like. Design is how it works.
– Steve Jobs

]

=== Additional Notes

This is a sample document with *bold text*, _italic text_, and `inline code` to showcase the theme’s typography.

#line(length: 100%)

_End of demonstration_

]
