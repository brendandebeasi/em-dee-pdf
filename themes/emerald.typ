// Emerald theme for em-dee-pdf
// Fresh modern look with green accents
// Inspired by Tailwind CSS Emerald palette - Best for: environmental, health, growth content
// Use with: --sections flag for contained sections

// =============================================================================
// COLOR PALETTE - Tailwind Emerald
// =============================================================================
#let primary = rgb("#059669")         // Emerald 600 - Primary accent
#let primary-dark = rgb("#047857")    // Emerald 700
#let secondary = rgb("#10b981")       // Emerald 500
#let text-primary = rgb("#064e3b")    // Emerald 900 - Body text
#let text-secondary = rgb("#065f46")  // Emerald 800
#let text-muted = rgb("#6ee7b7")      // Emerald 300

#let background = rgb("#f0fdf4")      // Green 50 - Page background
#let surface = rgb("#ecfdf5")         // Emerald 50 - Cards
#let surface-alt = rgb("#d1fae5")     // Emerald 200
#let border = rgb("#a7f3d0")          // Emerald 200

#let code-bg = rgb("#064e3b")         // Emerald 900 - Dark code
#let code-text = rgb("#d1fae5")       // Emerald 200

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

#set par(
  leading: 0.75em,
  justify: true,
)
#show raw: set par(justify: false)

#show par: set block(spacing: 0.9em)

// =============================================================================
// HEADINGS
// =============================================================================
#show heading.where(level: 1): it => {
  set text(size: 30pt, weight: "bold", fill: primary-dark)
  block(
    above: 1.2em,
    below: 0.8em,
    stroke: (bottom: 3pt + secondary),
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
// CODE BLOCKS - Dark emerald theme
// =============================================================================
#show raw.where(block: false): box.with(
  fill: surface-alt,
  inset: (x: 4pt, y: 2pt),
  outset: (y: 2pt),
  radius: 3pt,
)

#show raw.where(block: true): it => {
  set text(
    font: ("JetBrains Mono", "Fira Code", "SF Mono", "Menlo", "Courier New", "monospace"),
    size: 9.5pt,
    fill: code-text,
  )
  set par(justify: false)
  block(
    fill: code-bg,
    stroke: (left: 3pt + secondary),
    inset: 16pt,
    radius: (right: 6pt),
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
  set text(fill: primary, weight: "medium")
  underline(stroke: 1pt + text-muted, offset: 2pt, it)
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
// TABLES - Green header with stripes
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
    text(fill: primary, weight: "bold")[•],
    text(fill: secondary)[–],
    text(fill: text-muted)[‣],
  ),
)

#set enum(indent: 1.25em, body-indent: 0.5em)

// =============================================================================
// HORIZONTAL RULES
// =============================================================================
#set line(length: 100%, stroke: 1.5pt + border)
#show line: it => block(above: 1.5em, below: 1.5em, it)

// =============================================================================
// FIGURES
// =============================================================================
#show figure: it => {
  set align(center)
  block(above: 1em, below: 1.5em, breakable: false, it)
}

#show figure.caption: it => {
  set text(size: 9pt, fill: text-secondary)
  it
}
