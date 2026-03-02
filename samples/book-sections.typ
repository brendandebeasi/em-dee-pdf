// Book theme for em-dee-pdf
// Classic book/chapter style with drop caps aesthetic
// Use with: --sections flag for contained sections

// =============================================================================
// SECTION CONTAINER FUNCTION
// =============================================================================
#let md-section(content) = {
  block(
    stroke: (top: 0.5pt + luma(180), bottom: 0.5pt + luma(180)),
    inset: (x: 0pt, top: 1.5em, bottom: 1.5em),
    width: 100%,
    above: 1.5em,
    below: 1.5em,
    breakable: true,
    content
  )
}

#set page(
  paper: "us-letter",
  margin: (inside: 1.25in, outside: 1in, top: 1in, bottom: 1.25in),
  numbering: "1",
  number-align: center,
)

#set text(
  font: ("Baskerville", "Libre Baskerville", "Georgia", "serif"),
  size: 11pt,
  lang: "en",
)

#set par(
  leading: 0.65em,
  justify: true,
  first-line-indent: 1.5em,
)

// Chapter headings - elegant book style
#show heading.where(level: 1): it => {
  set text(size: 28pt, weight: "regular")
  set align(center)
  pagebreak(weak: true)
  v(2in)
  block(below: 2em, smallcaps(it))
  v(1em)
}

#show heading.where(level: 2): it => {
  set text(size: 16pt, weight: "bold")
  block(above: 1.8em, below: 0.8em, it)
}

#show heading.where(level: 3): it => {
  set text(size: 13pt, weight: "bold", style: "italic")
  block(above: 1.5em, below: 0.6em, it)
}

#show heading.where(level: 4): it => {
  set text(size: 11pt, style: "italic")
  block(above: 1.2em, below: 0.5em, it)
}

// Code - understated
#show raw.where(block: false): it => {
  set text(font: ("Menlo", "Consolas", "monospace"), size: 10pt)
  it
}

#show raw.where(block: true): it => {
  set text(font: ("Menlo", "Consolas", "monospace"), size: 9pt)
  block(
    inset: (left: 1.5em, y: 0.8em),
    above: 1em,
    below: 1em,
    it
  )
}

// Links - subtle
#show link: it => {
  set text(fill: rgb("#1a1a1a"))
  underline(stroke: 0.5pt, it)
}

// Quotes - elegant indentation
#show quote: it => {
  set text(style: "italic")
  block(
    inset: (left: 1.5em, right: 1.5em),
    above: 1em,
    below: 1em,
    it
  )
}

// Tables - minimal elegant with subtle stripes
#set table(
  stroke: none,
  inset: 8pt,
  fill: (x, y) => {
    if y == 0 { none }
    else if calc.even(y) { luma(252) }
    else { none }
  },
)

#show table: it => {
  block(
    above: 1em,
    below: 1.2em,
    stroke: (top: 1pt + black, bottom: 1pt + black),
    inset: (y: 4pt),
    it
  )
}

#show table.cell.where(y: 0): it => {
  set text(weight: "bold", size: 10pt, style: "italic")
  it
}

// Lists - multi-level markers
#set list(
  indent: 1.5em,
  body-indent: 0.5em,
  marker: ([-], [\*], [.]),
)
#set enum(indent: 1.5em, body-indent: 0.5em)

// Horizontal rules - ornamental
#show line: it => {
  set align(center)
  block(above: 2em, below: 2em, text(size: 14pt)[\~ \* \~])
}

// Figures
#show figure: it => block(above: 0.8em, below: 2em, breakable: false, it)
#show figure.caption: it => {
  set text(size: 9pt, style: "italic")
  set align(center)
  it
}

// Footnotes
#show footnote.entry: it => {
  set text(size: 9pt)
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
