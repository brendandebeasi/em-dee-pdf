// Orange theme for em-dee-pdf
// Vibrant warm orange - Best for: energetic, bold, startup content
// Inspired by Tailwind CSS Orange palette

// =============================================================================
// COLOR PALETTE - Tailwind Orange
// =============================================================================
#let primary = rgb("#ea580c")         // Orange 600
#let primary-dark = rgb("#c2410c")    // Orange 700
#let secondary = rgb("#f97316")       // Orange 500
#let text-primary = rgb("#431407")    // Orange 950
#let text-secondary = rgb("#7c2d12")  // Orange 900
#let text-muted = rgb("#fdba74")      // Orange 300

#let background = rgb("#fff7ed")      // Orange 50
#let surface = rgb("#ffedd5")         // Orange 100
#let surface-alt = rgb("#fed7aa")     // Orange 200
#let border = rgb("#fdba74")          // Orange 300

#let code-bg = rgb("#431407")         // Orange 950 - Dark code
#let code-text = rgb("#ffedd5")       // Orange 100

// =============================================================================
// PAGE SETUP
// =============================================================================
#set page(
  paper: "us-letter",
  margin: (x: 1.25in, y: 1in),
  numbering: "1",
  number-align: center,
  fill: background,
  footer: context {
    set align(center)
    set text(size: 8pt, fill: primary)
    counter(page).display()
  },
)

// =============================================================================
// TYPOGRAPHY
// =============================================================================
#set text(
  font: ("Inter", "SF Pro Text", "Helvetica Neue", "sans-serif"),
  size: 11pt,
  fill: text-primary,
  lang: "en",
)

#set par(leading: 0.75em, justify: true)

// =============================================================================
// HEADINGS
// =============================================================================
#show heading.where(level: 1): it => {
  set text(size: 30pt, weight: "bold", fill: primary-dark)
  block(above: 1.2em, below: 0.8em, stroke: (bottom: 3pt + secondary), inset: (bottom: 10pt), width: 100%, it)
}

#show heading.where(level: 2): it => {
  set text(size: 22pt, weight: "semibold", fill: primary)
  block(above: 1.6em, below: 0.6em, it)
}

#show heading.where(level: 3): it => {
  set text(size: 16pt, weight: "semibold", fill: secondary)
  block(above: 1.4em, below: 0.5em, it)
}

#show heading.where(level: 4): it => {
  set text(size: 13pt, weight: "medium", fill: text-secondary)
  block(above: 1.2em, below: 0.4em, it)
}

// =============================================================================
// CODE BLOCKS
// =============================================================================
#show raw.where(block: false): box.with(fill: surface, inset: (x: 4pt, y: 2pt), outset: (y: 2pt), radius: 3pt)

#show raw.where(block: true): it => {
  set text(font: ("JetBrains Mono", "Fira Code", "SF Mono", "monospace"), size: 9.5pt, fill: code-text)
  block(fill: code-bg, stroke: (left: 3pt + secondary), inset: 16pt, radius: (right: 6pt), width: 100%, above: 1em, below: 1em, it)
}

// =============================================================================
// LINKS
// =============================================================================
#show link: it => {
  set text(fill: primary, weight: "medium")
  underline(stroke: 1pt + text-muted, offset: 2pt, it)
}

// =============================================================================
// BLOCKQUOTES
// =============================================================================
#show quote: it => {
  set text(fill: text-secondary, style: "italic")
  block(fill: surface, stroke: (left: 4pt + primary), inset: (left: 16pt, right: 16pt, y: 12pt), radius: (right: 4pt), above: 1em, below: 1em, it)
}

// =============================================================================
// TABLES
// =============================================================================
#set table(stroke: 1pt + border, inset: 10pt, fill: (x, y) => {
  if y == 0 { primary } else if calc.even(y) { surface } else { white }
})
#show table: it => block(above: 1em, below: 1.5em, breakable: false, it)
#show table.cell.where(y: 0): set text(weight: "semibold", fill: white)

// =============================================================================
// LISTS
// =============================================================================
#set list(indent: 1.25em, body-indent: 0.5em, marker: (text(fill: primary, weight: "bold")[\*], text(fill: secondary)[-], text(fill: text-muted)[>]))
#set enum(indent: 1.25em, body-indent: 0.5em)

// =============================================================================
// OTHER ELEMENTS
// =============================================================================
#set line(length: 100%, stroke: 1.5pt + border)
#show line: it => block(above: 1.5em, below: 1.5em, it)
#show figure: it => { set align(center); block(above: 1em, below: 1.5em, breakable: false, it) }
#show figure.caption: it => { set text(size: 9pt, fill: text-secondary); it }


= Project Overview

This document demonstrates the visual styling of various themes.

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


== Code Example

```rust
fn main() {
    let message = "Hello, world!";
    println!("{}", message);
}
```

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

== Quote

#quote(block: true)[
Design is not just what it looks like and feels like. Design is how it works.
– Steve Jobs

]

=== Additional Notes

This is a sample document with *bold text*, _italic text_, and `inline code` to showcase the theme’s typography.

#line(length: 100%)

_End of demonstration_

