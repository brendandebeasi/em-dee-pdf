// Panels theme for em-dee-pdf
// Full-width colored panel headers with contained content
// Best for: presentations, executive summaries, structured reports
// Use with: --sections flag to wrap content in panels

// =============================================================================
// COLOR PALETTE - Deep professional blues
// =============================================================================
#let primary = rgb("#1e40af")         // Blue 800
#let primary-dark = rgb("#1e3a8a")    // Blue 900
#let secondary = rgb("#3b82f6")       // Blue 500
#let accent = rgb("#06b6d4")          // Cyan 500
#let text-primary = rgb("#0f172a")    // Slate 900
#let text-secondary = rgb("#334155")  // Slate 700
#let text-muted = rgb("#64748b")      // Slate 500

#let background = rgb("#ffffff")      // White
#let surface = rgb("#f8fafc")         // Slate 50
#let surface-alt = rgb("#f1f5f9")     // Slate 100
#let border = rgb("#cbd5e1")          // Slate 300

#let code-bg = rgb("#0f172a")         // Slate 900
#let code-text = rgb("#e2e8f0")       // Slate 200

// =============================================================================
// SECTION CONTAINER FUNCTION
// =============================================================================
#let md-section(content) = {
  block(
    stroke: 1pt + border,
    radius: 8pt,
    width: 100%,
    above: 0.8em,
    below: 0.8em,
    clip: true,
    breakable: true,
    {
      block(
        fill: surface,
        inset: (x: 20pt, top: 16pt, bottom: 20pt),
        width: 100%,
        content
      )
    }
  )
}

// =============================================================================
// PAGE SETUP
// =============================================================================
#set page(
  paper: "us-letter",
  margin: (x: 0.85in, y: 1in),
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
#show raw: set par(justify: false)

// =============================================================================
// HEADINGS
// =============================================================================
#show heading.where(level: 1): it => {
  set text(size: 32pt, weight: "bold", fill: primary-dark)
  block(above: 0.5em, below: 0.8em, it)
  v(0.3em)
  line(length: 100%, stroke: 2pt + secondary)
  v(0.6em)
}

// H2 gets gradient header bar inside sections
#show heading.where(level: 2): it => {
  // Pull the heading outside the content padding to touch edges
  block(
    fill: gradient.linear(primary, primary-dark, angle: 90deg),
    inset: (x: 20pt, y: 12pt),
    outset: (x: 20pt, top: 16pt),
    width: 100% + 40pt,
    above: 0em,
    below: 1em,
    {
      set text(size: 16pt, weight: "bold", fill: white)
      it
    }
  )
}

#show heading.where(level: 3): it => {
  set text(size: 14pt, weight: "semibold", fill: primary)
  block(
    stroke: (left: 3pt + secondary),
    inset: (left: 12pt),
    above: 1.4em,
    below: 0.5em,
    it
  )
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
  radius: 3pt
)

#show raw.where(block: true): it => {
  set text(font: ("JetBrains Mono", "Fira Code", "SF Mono", "Menlo", "Courier New", "monospace"), size: 9pt, fill: code-text)
  set par(justify: false)
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
  set text(fill: secondary, weight: "medium")
  underline(stroke: 1pt + accent, offset: 2pt, it)
}

// =============================================================================
// BLOCKQUOTES
// =============================================================================
#show quote: it => {
  set text(fill: text-secondary, style: "italic")
  block(
    fill: surface-alt,
    stroke: (left: 4pt + accent),
    inset: (left: 16pt, right: 16pt, y: 12pt),
    radius: (right: 4pt),
    above: 1em,
    below: 1em,
    it
  )
}

// =============================================================================
// TABLES
// =============================================================================
#set table(
  stroke: 1pt + border,
  inset: 10pt,
  fill: (x, y) => {
    if y == 0 { primary }
    else if calc.even(y) { surface-alt }
    else { white }
  }
)
#show table: it => block(above: 1em, below: 1.2em, breakable: true, it)
#show table.cell.where(y: 0): set text(weight: "bold", fill: white)

// =============================================================================
// LISTS
// =============================================================================
#set list(indent: 1.25em, body-indent: 0.5em, marker: (text(fill: secondary, weight: "bold")[•], text(fill: accent)[–], text(fill: text-muted)[‣]))
#set enum(indent: 1.25em, body-indent: 0.5em)

// =============================================================================
// OTHER ELEMENTS
// =============================================================================
#set line(length: 100%, stroke: 1.5pt + border)
#show line: it => block(above: 1.5em, below: 1.5em, it)
#show figure: it => { set align(center); block(above: 1em, below: 1.2em, breakable: false, it) }
#show figure.caption: it => { set text(size: 9pt, fill: text-secondary); it }
