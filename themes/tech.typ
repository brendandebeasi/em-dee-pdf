// Tech theme for em-dee-pdf
// Modern technical documentation - inspired by Stripe, Vercel, Linear
// Clean, precise, code-forward aesthetic
// Use with: --sections flag for contained sections

// =============================================================================
// COLOR PALETTE - Modern tech with cyan accent
// =============================================================================
#let accent = rgb("#06b6d4")          // Cyan 500 - primary accent
#let accent-dark = rgb("#0891b2")     // Cyan 600
#let accent-light = rgb("#67e8f9")    // Cyan 300
#let accent-bg = rgb("#ecfeff")       // Cyan 50

#let text-primary = rgb("#0f172a")    // Slate 900
#let text-secondary = rgb("#475569")  // Slate 600
#let text-muted = rgb("#94a3b8")      // Slate 400

#let surface = rgb("#f8fafc")         // Slate 50
#let surface-alt = rgb("#f1f5f9")     // Slate 100
#let border = rgb("#e2e8f0")          // Slate 200
#let border-strong = rgb("#cbd5e1")   // Slate 300

#let code-bg = rgb("#0f172a")         // Slate 900 - dark code blocks
#let code-text = rgb("#e2e8f0")       // Slate 200
#let code-accent = rgb("#38bdf8")     // Sky 400 - for code highlights

// =============================================================================
// SECTION CONTAINER FUNCTION
// =============================================================================
#let md-section(content) = {
  block(
    fill: surface,
    stroke: (left: 3pt + accent, rest: 1pt + border),
    radius: (right: 8pt),
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
  number-align: right,
  footer: context {
    set align(right)
    set text(size: 8pt, fill: text-muted, font: ("JetBrains Mono", "monospace"))
    [Page ] + counter(page).display()
  },
)

// =============================================================================
// TYPOGRAPHY - Modern sans-serif with monospace influence
// =============================================================================
#set text(
  font: ("Inter", "SF Pro Display", "Segoe UI", "sans-serif"),
  size: 10.5pt,
  fill: text-primary,
  lang: "en",
)

#set par(
  leading: 0.75em,
  justify: false,  // Tech docs usually don't justify
)

#show par: set block(spacing: 0.9em)

// =============================================================================
// HEADINGS - Clean hierarchy with subtle accents
// =============================================================================
#show heading.where(level: 1): it => {
  set text(size: 32pt, weight: "bold", fill: text-primary, tracking: -0.02em)
  block(
    above: 0.5em,
    below: 0.8em,
    it
  )
}

#show heading.where(level: 2): it => {
  set text(size: 22pt, weight: "semibold", fill: text-primary)
  block(
    above: 1.8em,
    below: 0.6em,
    stroke: (bottom: 2pt + accent),
    inset: (bottom: 10pt),
    it
  )
}

#show heading.where(level: 3): it => {
  set text(size: 16pt, weight: "semibold", fill: accent-dark)
  block(above: 1.5em, below: 0.5em, it)
}

#show heading.where(level: 4): it => {
  set text(size: 13pt, weight: "medium", fill: text-secondary)
  block(above: 1.2em, below: 0.4em, it)
}

// =============================================================================
// CODE BLOCKS - First-class treatment for technical docs
// =============================================================================
#show raw.where(block: false): it => {
  set text(font: ("JetBrains Mono", "Fira Code", "SF Mono", "monospace"), size: 9.5pt)
  box(
    fill: surface-alt,
    inset: (x: 5pt, y: 3pt),
    radius: 4pt,
    it
  )
}

#show raw.where(block: true): it => {
  set text(
    font: ("JetBrains Mono", "Fira Code", "SF Mono", "monospace"),
    size: 9.5pt,
    fill: code-text,
  )
  block(
    fill: code-bg,
    stroke: (left: 3pt + accent),
    inset: (left: 20pt, right: 16pt, y: 16pt),
    radius: (right: 6pt, left: 0pt),
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
  set text(fill: accent-dark, weight: "medium")
  underline(stroke: 1pt + accent-light, offset: 2pt, it)
}

// =============================================================================
// BLOCKQUOTES - Card-like callout style
// =============================================================================
#show quote: it => {
  block(
    fill: accent-bg,
    stroke: (left: 4pt + accent, rest: 1pt + border),
    inset: 16pt,
    radius: 6pt,
    above: 1em,
    below: 1em,
    {
      set text(fill: text-secondary)
      it
    }
  )
}

// =============================================================================
// TABLES - Clean modern style with colored header
// =============================================================================
#set table(
  stroke: 1pt + border,
  inset: 12pt,
  fill: (x, y) => {
    if y == 0 { accent }
    else if calc.even(y) { surface }
    else { white }
  },
)

#show table: it => block(above: 1em, below: 1.5em, breakable: true, it)

#show table.cell.where(y: 0): set text(weight: "semibold", fill: white)

// =============================================================================
// LISTS - Tech-style markers
// =============================================================================
#set list(
  indent: 1.25em,
  body-indent: 0.5em,
  marker: (
    text(fill: accent, weight: "bold")[>],
    text(fill: text-secondary)[-],
    text(fill: text-muted)[+],
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
#show figure: it => block(above: 1em, below: 1.5em, breakable: false, it)

#show figure.caption: it => {
  set text(size: 9pt, fill: text-secondary)
  it
}
