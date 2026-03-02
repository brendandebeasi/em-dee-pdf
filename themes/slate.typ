// Slate theme for em-dee-pdf
// Clean professional look with neutral grays and subtle blue undertone
// Inspired by Tailwind CSS Slate palette - Best for: technical docs, business reports
// Use with: --sections flag for contained sections

// =============================================================================
// COLOR PALETTE - Tailwind Slate
// =============================================================================
#let primary = rgb("#334155")         // Slate 700 - Primary accent
#let primary-dark = rgb("#1e293b")    // Slate 800
#let secondary = rgb("#64748b")       // Slate 500
#let text-primary = rgb("#0f172a")    // Slate 950 - Body text
#let text-secondary = rgb("#475569")  // Slate 600
#let text-muted = rgb("#94a3b8")      // Slate 400

#let background = rgb("#f8fafc")      // Slate 50 - Page background
#let surface = rgb("#f1f5f9")         // Slate 100 - Cards
#let surface-alt = rgb("#e2e8f0")     // Slate 200
#let border = rgb("#cbd5e1")          // Slate 300

#let code-bg = rgb("#1e293b")         // Slate 800 - Dark code
#let code-text = rgb("#e2e8f0")       // Slate 200

#let link-color = rgb("#2563eb")      // Blue 600

// =============================================================================
// SECTION CONTAINER FUNCTION
// =============================================================================
#let md-section(content) = {
  block(
    fill: white,
    stroke: (left: 3pt + primary, rest: 1pt + border),
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
    set text(size: 8pt, fill: text-muted)
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

#set par(
  leading: 0.75em,
  justify: true,
)

#show par: set block(spacing: 0.9em)

// =============================================================================
// HEADINGS
// =============================================================================
#show heading.where(level: 1): it => {
  set text(size: 30pt, weight: "bold", fill: primary-dark)
  block(
    above: 1.2em,
    below: 0.8em,
    stroke: (bottom: 3pt + primary),
    inset: (bottom: 10pt),
    width: 100%,
    it
  )
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
// CODE BLOCKS - Dark theme
// =============================================================================
#show raw.where(block: false): box.with(
  fill: surface,
  inset: (x: 4pt, y: 2pt),
  outset: (y: 2pt),
  radius: 3pt,
)

#show raw.where(block: true): it => {
  set text(
    font: ("JetBrains Mono", "Fira Code", "SF Mono", "monospace"),
    size: 9.5pt,
    fill: code-text,
  )
  block(
    fill: code-bg,
    inset: 16pt,
    radius: 6pt,
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
  set text(fill: link-color, weight: "medium")
  underline(stroke: 1pt + rgb("#93c5fd"), offset: 2pt, it)
}

// =============================================================================
// BLOCKQUOTES
// =============================================================================
#show quote: it => {
  set text(fill: text-secondary, style: "italic")
  block(
    fill: surface,
    stroke: (left: 4pt + primary),
    inset: (left: 16pt, right: 16pt, y: 12pt),
    radius: (right: 4pt),
    above: 1em,
    below: 1em,
    it
  )
}

// =============================================================================
// TABLES - Colored header with stripes
// =============================================================================
#set table(
  stroke: 1pt + border,
  inset: 10pt,
  fill: (x, y) => {
    if y == 0 { primary }
    else if calc.even(y) { surface }
    else { white }
  },
)

#show table: it => block(above: 1em, below: 1.5em, breakable: true, it)

#show table.cell.where(y: 0): set text(weight: "semibold", fill: white)

// =============================================================================
// LISTS
// =============================================================================
#set list(
  indent: 1.25em,
  body-indent: 0.5em,
  marker: (
    text(fill: primary, weight: "bold")[\*],
    text(fill: secondary)[-],
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
  block(above: 1em, below: 1.5em, breakable: false, it)
}

#show figure.caption: it => {
  set text(size: 9pt, fill: text-muted)
  it
}
