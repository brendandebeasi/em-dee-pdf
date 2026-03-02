// Zinc theme for em-dee-pdf
// Pure neutral gray - Best for: minimal, modern, professional content
// Inspired by Tailwind CSS Zinc palette (shadcn/ui default)
// Use with: --sections flag for contained sections

// =============================================================================
// COLOR PALETTE - Tailwind Zinc
// =============================================================================
#let primary = rgb("#52525b")         // Zinc 600
#let primary-dark = rgb("#3f3f46")    // Zinc 700
#let secondary = rgb("#71717a")       // Zinc 500
#let text-primary = rgb("#09090b")    // Zinc 950
#let text-secondary = rgb("#18181b")  // Zinc 900
#let text-muted = rgb("#d4d4d8")      // Zinc 300

#let background = rgb("#fafafa")      // Zinc 50
#let surface = rgb("#f4f4f5")         // Zinc 100
#let surface-alt = rgb("#e4e4e7")     // Zinc 200
#let border = rgb("#d4d4d8")          // Zinc 300

#let code-bg = rgb("#18181b")         // Zinc 900 - Dark code
#let code-text = rgb("#f4f4f5")       // Zinc 100

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
    set text(size: 8pt, fill: secondary)
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
  set text(size: 30pt, weight: "bold", fill: text-primary)
  block(above: 1.2em, below: 0.8em, stroke: (bottom: 3pt + primary), inset: (bottom: 10pt), width: 100%, it)
}

#show heading.where(level: 2): it => {
  set text(size: 22pt, weight: "semibold", fill: primary-dark)
  block(above: 1.6em, below: 0.6em, it)
}

#show heading.where(level: 3): it => {
  set text(size: 16pt, weight: "semibold", fill: primary)
  block(above: 1.4em, below: 0.5em, it)
}

#show heading.where(level: 4): it => {
  set text(size: 13pt, weight: "medium", fill: secondary)
  block(above: 1.2em, below: 0.4em, it)
}

// =============================================================================
// CODE BLOCKS
// =============================================================================
#show raw.where(block: false): box.with(fill: surface-alt, inset: (x: 4pt, y: 2pt), outset: (y: 2pt), radius: 3pt)

#show raw.where(block: true): it => {
  set text(font: ("JetBrains Mono", "Fira Code", "SF Mono", "monospace"), size: 9.5pt, fill: code-text)
  block(fill: code-bg, inset: 16pt, radius: 6pt, width: 100%, above: 1em, below: 1em, it)
}

// =============================================================================
// LINKS
// =============================================================================
#show link: it => {
  set text(fill: primary, weight: "medium")
  underline(stroke: 1pt + border, offset: 2pt, it)
}

// =============================================================================
// BLOCKQUOTES
// =============================================================================
#show quote: it => {
  set text(fill: secondary, style: "italic")
  block(fill: surface, stroke: (left: 4pt + primary), inset: (left: 16pt, right: 16pt, y: 12pt), radius: (right: 4pt), above: 1em, below: 1em, it)
}

// =============================================================================
// TABLES
// =============================================================================
#set table(stroke: 1pt + border, inset: 10pt, fill: (x, y) => {
  if y == 0 { primary-dark } else if calc.even(y) { surface } else { white }
})
#show table: it => block(above: 1em, below: 1.5em, breakable: true, it)
#show table.cell.where(y: 0): set text(weight: "semibold", fill: white)

// =============================================================================
// LISTS
// =============================================================================
#set list(indent: 1.25em, body-indent: 0.5em, marker: (text(fill: primary, weight: "bold")[•], text(fill: secondary)[–], text(fill: text-muted)[‣]))
#set enum(indent: 1.25em, body-indent: 0.5em)

// =============================================================================
// OTHER ELEMENTS
// =============================================================================
#set line(length: 100%, stroke: 1pt + border)
#show line: it => block(above: 1.5em, below: 1.5em, it)
#show figure: it => { set align(center); block(above: 1em, below: 1.5em, breakable: false, it) }
#show figure.caption: it => { set text(size: 9pt, fill: secondary); it }
