// Shared style and helpers for the asymmetric-paxos figures.
//
// One canvas unit is one SVG pixel. CeTZ's y axis points up, so a point
// that sits `y` pixels below the top edge is written as `(x, -y)`.

#import "@preview/cetz:0.5.2"
#import cetz.draw: *

#let px = 0.75pt

// Color usage:
// ink: titles, main labels, acceptors and shared-acceptor rings.
// slate: neutral facts, secondary labels and axis ticks.
// panel-*: neutral scene backgrounds; lifeline-ink: timelines and guides.
// read-* (blue): read quorums and the values they return.
// write-* (green): write quorums and the values they store.
// ok-*: completed operations and established guarantees.
#let ink = rgb("#172b45")
#let slate = rgb("#52657a")
#let panel-fill = rgb("#f8fafc")
#let panel-line = rgb("#dbe3ec")
#let read-ink = rgb("#3068cf")
#let read-text = rgb("#2459b5")
#let write-ink = rgb("#238a55")
#let write-text = rgb("#176b40")
#let ok-text = rgb("#1b6a43")
#let lifeline-ink = rgb("#c6d0db")

// Text styles. Plain text is 20px Arial in `ink`.
#let sans = ("Arial", "Helvetica Neue")
#let small(fill: ink, it) = text(size: 17 * px, fill: fill, it)
#let subtitle(it) = text(size: 18 * px, fill: slate, it)
#let mono(fill: ink, it) = text(font: "Menlo", size: 17 * px, fill: fill, it)

// A stroke `width` pixels wide.
#let pen(paint, width, dash: none) = (
  paint: paint,
  thickness: width * px,
  dash: dash,
  cap: "round",
  join: "round",
)

#let quorum-dash = (8 * px, 5 * px)
#let guide-dash = (5 * px, 5 * px)

// A rectangle with its top-left corner at `pos`. A trailing content block
// is drawn at the rectangle's center; descenders do not shift it.
#let card(pos, w, h, radius: 14, fill: none, stroke: none, name: none, ..body) = {
  rect(pos, (rel: (w, -h)), radius: radius, fill: fill, stroke: stroke, name: name)
  for it in body.pos() {
    let label = text(top-edge: "cap-height", bottom-edge: "baseline", it)
    content((rel: (w / 2, -h / 2), to: pos), label)
  }
}

#let panel(pos, w, h, ..args) = card(pos, w, h, fill: panel-fill, stroke: pen(panel-line, 1.5), ..args)

#let note-styles = (
  info: (symbol: "ℹ️", paint: slate),
  shared: (symbol: "🔗", paint: ink),
  ok: (symbol: "✅", paint: ok-text),
)

// An unframed annotation; wrapped lines align with the text, not the icon.
#let note(pos, w, body, kind: "shared") = {
  let style = note-styles.at(kind)
  let icon = text(font: "Apple Color Emoji", size: 18 * px, style.symbol)
  let marker = std.move(dy: -4 * px, icon)
  let label = block({
    set text(size: 18 * px, fill: style.paint)
    set par(leading: 6 * px, spacing: 0pt)
    body
  })
  let layout = block(width: w * px, std.grid(
    columns: (20 * px, 1fr), column-gutter: 8 * px,
    align: top, marker, label,
  ))
  content(pos, layout, anchor: "north-west")
}

// The outline of a read or write quorum.
#let quorum-pen(kind, dashed: false) = {
  let paint = if kind == "read" { read-ink } else { write-ink }
  let dash = if dashed { quorum-dash } else { none }
  pen(paint.transparentize(40%), 2.5, dash: dash)
}

#let quorum(pos, w, h, kind: "read", dashed: false, ..args) = card(pos, w, h, stroke: quorum-pen(kind, dashed: dashed), ..args)

// An acceptor: a circle with its name at the center.
#let node(pos, label, name: none, r: 24) = {
  circle(pos, radius: r, fill: white, stroke: pen(slate, 2), name: name)
  content(pos, text(size: 20 * px, weight: "bold", fill: ink, label))
}

// A ring that marks an acceptor shared by two quorums.
#let ring(node-name, r: 33, name: none) = circle(node-name, radius: r, stroke: pen(ink, 3), name: name)

// One figure: page setup, white background, title and subtitle.
#let diagram(w, h, title, sub, body) = {
  set page(width: auto, height: auto, margin: 0pt)
  set text(font: sans, size: 20 * px, fill: ink)
  cetz.canvas(length: px, {
    rect((0, 0), (w, -h), fill: white, stroke: none)
    content((40, -45), text(weight: "semibold", title), anchor: "base-west")
    content((40, -77), subtitle(sub), anchor: "base-west")
    body
  })
}
