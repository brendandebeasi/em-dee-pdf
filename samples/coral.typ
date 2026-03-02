// Coral theme for em-dee-pdf
// Warm, modern design with coral/salmon accents

#set page(
  paper: "us-letter",
  margin: (x: 1in, y: 1in),
  numbering: "1",
  number-align: center,
  header: context {
    if counter(page).get().first() > 1 {
      set text(size: 9pt, fill: rgb("#E94845"))
      line(length: 100%, stroke: 0.5pt + rgb("#E94845"))
    }
  },
)

#set text(
  font: ("Helvetica Neue", "Arial", "sans-serif"),
  size: 10.5pt,
  fill: rgb("#2d2d2d"),
)

#set par(
  leading: 0.75em,
  justify: true,
)

// Headings - coral accented
#show heading.where(level: 1): it => {
  set text(size: 28pt, weight: "bold", fill: rgb("#E94845"))
  block(above: 1.5em, below: 0.8em, it)
}

#show heading.where(level: 2): it => {
  set text(size: 18pt, weight: "bold", fill: rgb("#c93c3a"))
  block(
    above: 1.8em,
    below: 0.6em,
    stroke: (bottom: 2pt + rgb("#E94845")),
    inset: (bottom: 8pt),
    it
  )
}

#show heading.where(level: 3): it => {
  set text(size: 14pt, weight: "semibold", fill: rgb("#E94845"))
  block(above: 1.5em, below: 0.5em, it)
}

#show heading.where(level: 4): it => {
  set text(size: 12pt, weight: "medium", fill: rgb("#666"))
  block(above: 1.2em, below: 0.4em, it)
}

// Code - warm gray
#show raw.where(block: false): box.with(
  fill: rgb("#fff5f5"),
  inset: (x: 5pt, y: 2pt),
  radius: 3pt,
)

#show raw.where(block: true): it => {
  set text(font: ("SF Mono", "Consolas", "monospace"), size: 9pt)
  block(
    fill: rgb("#2d2d2d"),
    stroke: none,
    inset: 16pt,
    radius: 8pt,
    width: 100%,
    above: 1em,
    below: 1em,
    {
      set text(fill: rgb("#f8f8f2"))
      it
    }
  )
}

// Links - coral
#show link: it => {
  set text(fill: rgb("#E94845"))
  underline(stroke: 1pt + rgb("#E94845"), offset: 2pt, it)
}

// Quotes - coral left border
#show quote: it => {
  block(
    fill: rgb("#fff5f5"),
    stroke: (left: 4pt + rgb("#E94845")),
    inset: (left: 16pt, right: 16pt, y: 12pt),
    radius: (right: 4pt),
    above: 1em,
    below: 1em,
    it
  )
}

// Tables - coral header with stripes
#set table(
  stroke: 1pt + rgb("#eee"),
  inset: 10pt,
  fill: (x, y) => {
    if y == 0 { rgb("#E94845") }
    else if calc.even(y) { rgb("#fff8f8") }
    else { none }
  },
)

#show table: it => block(above: 1em, below: 2em, breakable: false, it)

#show table.cell.where(y: 0): set text(weight: "bold", fill: white)

// Lists - coral bullets multi-level
#set list(
  indent: 1.5em,
  body-indent: 0.5em,
  marker: (
    text(fill: rgb("#E94845"))[-->],
    text(fill: rgb("#f87171"))[--],
    text(fill: rgb("#fca5a5"))[-],
  ),
)
#set enum(indent: 1.5em, body-indent: 0.5em)

// Horizontal rules
#set line(length: 100%, stroke: 2pt + rgb("#E94845"))
#show line: it => block(above: 1.5em, below: 1.5em, it)

// Figures
#show figure: it => block(above: 1.2em, below: 1.2em, breakable: false, it)
#show figure.caption: set text(size: 9pt, fill: rgb("#666"))


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

