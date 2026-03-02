// Boxed theme for em-dee-pdf
// Clean bordered containers with labeled section headers
// Best for: technical documentation, manuals, structured guides
// Use with: --sections flag to wrap content in boxes

// =============================================================================
// COLOR PALETTE - Neutral with green accent
// =============================================================================
#let primary = rgb("#059669")         // Emerald 600
#let primary-dark = rgb("#047857")    // Emerald 700
#let secondary = rgb("#10b981")       // Emerald 500
#let text-primary = rgb("#111827")    // Gray 900
#let text-secondary = rgb("#374151")  // Gray 700
#let text-muted = rgb("#6b7280")      // Gray 500

#let background = rgb("#ffffff")      // White
#let surface = rgb("#f9fafb")         // Gray 50
#let surface-alt = rgb("#f3f4f6")     // Gray 100
#let border = rgb("#d1d5db")          // Gray 300
#let border-dark = rgb("#9ca3af")     // Gray 400

#let code-bg = rgb("#111827")         // Gray 900
#let code-text = rgb("#f3f4f6")       // Gray 100

// =============================================================================
// SECTION CONTAINER FUNCTION
// =============================================================================
#let md-section(content) = {
  block(
    stroke: 2pt + border-dark,
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
  margin: (x: 1in, y: 1in),
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
  set text(size: 28pt, weight: "bold", fill: text-primary)
  block(above: 0.5em, below: 0.6em, it)
  block(
    stroke: (bottom: 3pt + primary),
    width: 100%,
    below: 1em,
    []
  )
}

// H2 creates header bar
#show heading.where(level: 2): it => {
  block(
    fill: surface-alt,
    stroke: (bottom: 2pt + primary),
    inset: (x: 20pt, y: 12pt),
    outset: (x: 20pt, top: 16pt),
    width: 100% + 40pt,
    above: 0em,
    below: 0.8em,
    {
      set text(size: 16pt, weight: "bold", fill: primary-dark)
      it
    }
  )
}

#show heading.where(level: 3): it => {
  set text(size: 13pt, weight: "semibold", fill: primary)
  block(
    fill: white,
    stroke: (left: 3pt + secondary),
    inset: (left: 12pt, y: 6pt),
    radius: (right: 4pt),
    above: 1.2em,
    below: 0.5em,
    it
  )
}

#show heading.where(level: 4): it => {
  set text(size: 11pt, weight: "semibold", fill: text-secondary)
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
  set text(font: ("JetBrains Mono", "Fira Code", "SF Mono", "monospace"), size: 9pt, fill: code-text)
  block(
    fill: code-bg,
    stroke: 2pt + rgb("#374151"),
    inset: 14pt,
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
  set text(fill: primary, weight: "medium")
  underline(stroke: 1pt + secondary, offset: 2pt, it)
}

// =============================================================================
// BLOCKQUOTES
// =============================================================================
#show quote: it => {
  set text(fill: text-secondary, style: "italic")
  block(
    fill: white,
    stroke: (left: 4pt + primary, rest: 1pt + border),
    inset: (left: 14pt, right: 14pt, y: 10pt),
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
  stroke: 1.5pt + border,
  inset: 10pt,
  fill: (x, y) => {
    if y == 0 { primary }
    else if calc.even(y) { surface }
    else { white }
  }
)
#show table: it => block(
  stroke: 2pt + border-dark,
  radius: 6pt,
  clip: true,
  above: 1em,
  below: 1.2em,
  breakable: true,
  it
)
#show table.cell.where(y: 0): set text(weight: "bold", fill: white)

// =============================================================================
// LISTS
// =============================================================================
#set list(indent: 1.25em, body-indent: 0.5em, marker: (text(fill: primary, weight: "bold")[•], text(fill: secondary)[–], text(fill: text-muted)[‣]))
#set enum(indent: 1.25em, body-indent: 0.5em)

// =============================================================================
// OTHER ELEMENTS
// =============================================================================
#set line(length: 100%, stroke: 1.5pt + border)
#show line: it => block(above: 1.5em, below: 1.5em, it)
#show figure: it => { set align(center); block(above: 1em, below: 1.2em, breakable: false, it) }
#show figure.caption: it => { set text(size: 9pt, fill: text-secondary); it }
