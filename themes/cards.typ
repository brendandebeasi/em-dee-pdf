// Cards theme for em-dee-pdf
// Each H2 section appears as a distinct card on a subtle background
// Best for: dashboards, reports, modular content
// Use with: --sections flag to wrap content in cards

// =============================================================================
// COLOR PALETTE
// =============================================================================
#let primary = rgb("#3b82f6")         // Blue 500
#let primary-dark = rgb("#1d4ed8")    // Blue 700
#let secondary = rgb("#60a5fa")       // Blue 400
#let text-primary = rgb("#1e293b")    // Slate 800
#let text-secondary = rgb("#475569")  // Slate 600
#let text-muted = rgb("#94a3b8")      // Slate 400

#let background = rgb("#f1f5f9")      // Slate 100 - Page bg
#let surface = rgb("#ffffff")         // White - Card bg
#let surface-alt = rgb("#f8fafc")     // Slate 50
#let border = rgb("#e2e8f0")          // Slate 200

#let code-bg = rgb("#1e293b")         // Slate 800
#let code-text = rgb("#e2e8f0")       // Slate 200

// =============================================================================
// SECTION CONTAINER FUNCTION
// =============================================================================
#let md-section(content) = {
  block(
    fill: surface,
    stroke: 1pt + border,
    radius: 10pt,
    inset: (x: 24pt, top: 20pt, bottom: 24pt),
    width: 100%,
    above: 0.8em,
    below: 0.8em,
    breakable: true,
    content
  )
}

// =============================================================================
// PAGE SETUP
// =============================================================================
#set page(
  paper: "us-letter",
  margin: (x: 0.9in, y: 1in),
  numbering: "1",
  number-align: center,
  fill: background,
  footer: context {
    set align(center)
    set text(size: 8pt, fill: text-muted)
    counter(page).display()
  },
)

// =============================================================================
// TYPOGRAPHY
// =============================================================================
#set text(
  font: ("Inter", "SF Pro Text", "Helvetica Neue", "sans-serif"),
  size: 10.5pt,
  fill: text-primary,
  lang: "en",
)

#set par(leading: 0.7em, justify: true)

// =============================================================================
// HEADINGS
// =============================================================================
#show heading.where(level: 1): it => {
  set text(size: 28pt, weight: "bold", fill: primary-dark)
  block(above: 0.5em, below: 1em, it)
}

#show heading.where(level: 2): it => {
  set text(size: 18pt, weight: "semibold", fill: primary-dark)
  block(
    stroke: (bottom: 2pt + secondary),
    inset: (bottom: 8pt),
    width: 100%,
    above: 0em,
    below: 0.8em,
    it
  )
}

#show heading.where(level: 3): it => {
  set text(size: 14pt, weight: "semibold", fill: primary)
  block(above: 1.2em, below: 0.5em, it)
}

#show heading.where(level: 4): it => {
  set text(size: 12pt, weight: "medium", fill: text-secondary)
  block(above: 1em, below: 0.4em, it)
}

// =============================================================================
// CODE BLOCKS
// =============================================================================
#show raw.where(block: false): box.with(
  fill: surface-alt,
  stroke: 1pt + border,
  inset: (x: 4pt, y: 2pt),
  outset: (y: 2pt),
  radius: 4pt
)

#show raw.where(block: true): it => {
  set text(font: ("JetBrains Mono", "Fira Code", "SF Mono", "monospace"), size: 9pt, fill: code-text)
  block(
    fill: code-bg,
    stroke: 1pt + rgb("#334155"),
    inset: 14pt,
    radius: 6pt,
    width: 100%,
    above: 0.8em,
    below: 0.8em,
    it
  )
}

// =============================================================================
// LINKS
// =============================================================================
#show link: it => {
  set text(fill: primary, weight: "medium")
  underline(stroke: 1pt + secondary, offset: 2pt, it)
}

// =============================================================================
// BLOCKQUOTES
// =============================================================================
#show quote: it => {
  set text(fill: text-secondary, style: "italic")
  block(
    fill: surface-alt,
    stroke: (left: 3pt + primary, rest: 1pt + border),
    inset: (left: 14pt, right: 14pt, y: 10pt),
    radius: (right: 6pt),
    above: 0.8em,
    below: 0.8em,
    it
  )
}

// =============================================================================
// TABLES
// =============================================================================
#set table(
  stroke: 1pt + border,
  inset: 8pt,
  fill: (x, y) => {
    if y == 0 { primary }
    else if calc.even(y) { surface-alt }
    else { surface }
  }
)
#show table: it => block(above: 0.8em, below: 1em, breakable: true, it)
#show table.cell.where(y: 0): set text(weight: "semibold", fill: white, size: 9.5pt)

// =============================================================================
// LISTS
// =============================================================================
#set list(indent: 1em, body-indent: 0.5em, marker: (text(fill: primary, weight: "bold")[•], text(fill: secondary)[–], text(fill: text-muted)[‣]))
#set enum(indent: 1em, body-indent: 0.5em)

// =============================================================================
// OTHER ELEMENTS
// =============================================================================
#set line(length: 100%, stroke: 1pt + border)
#show line: it => block(above: 1.2em, below: 1.2em, it)
#show figure: it => { set align(center); block(above: 0.8em, below: 1em, breakable: false, it) }
#show figure.caption: it => { set text(size: 9pt, fill: text-secondary); it }
