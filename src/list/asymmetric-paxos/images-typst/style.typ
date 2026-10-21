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

// A value card `w` pixels wide, centered on a timeline at (x, -y).
// kind: "write" (stored), "read" (returned to the reader), "missed" (the
// write never arrived) or "empty" (the reader found nothing).
#let value(x, y, w, kind, label) = {
  let pos = (x - w / 2, -y + 20)
  let faded = slate.transparentize(45%)
  if kind == "write" {
    card(pos, w, 40, radius: 10, fill: white, stroke: pen(write-ink, 2.5), mono(fill: write-text, label))
  } else if kind == "read" {
    card(pos, w, 40, radius: 10, fill: white, stroke: pen(read-ink, 2.5), mono(fill: read-text, label))
  } else if kind == "missed" {
    card(pos, w, 40, radius: 10, fill: panel-fill, stroke: pen(faded, 2, dash: quorum-dash), mono(fill: faded, label))
  } else {
    card(pos, w, 40, radius: 10, fill: white, stroke: pen(read-ink, 2, dash: quorum-dash), small(fill: slate, label))
  }
}

// P1 writes, then P2 reads, on one timeline per acceptor. Acceptor i stores
// `stored.at(i)`. P1 writes to the first `written` acceptors; P2 reads every
// acceptor from index `read-from` on. A ring marks each acceptor in both
// quorums. `result` is what P2 rebuilds; `shared` explains the overlap.
#let rw-figure(title, sub, stored, written, read-from, result, shared, card-w: 84) = {
  let rows = range(stored.len()).map(i => 210 + 80 * i)
  let first = rows.first()
  let last = rows.last()
  let read-first = rows.at(read-from)
  let write-last = rows.at(written - 1)
  let write-x = 240 + card-w / 2
  let read-x = write-x + card-w + 100
  let line-end = read-x + card-w / 2 + 60
  let note-y = last + 114

  diagram(960, note-y + 66, title, sub, {
    panel((40, -110), 880, last - 30)
    content((write-x, -152), small(fill: write-text)[P1 writes], anchor: "base")
    content((read-x, -152), small(fill: read-text)[P2 reads], anchor: "base")

    for (i, y) in rows.enumerate() {
      let name = "a" + str(i + 1)
      let mark = (end: ">", fill: lifeline-ink, length: 10, width: 9)
      line((136, -y), (line-end, -y), stroke: pen(lifeline-ink, 2.5), mark: mark)
      node((100, -y), "A" + str(i + 1), name: name)
      if read-from <= i and i < written {
        ring(name)
      }
    }

    let quorum-w = card-w + 36
    quorum((write-x - quorum-w / 2, -first + 38), quorum-w, write-last - first + 76, kind: "write")
    quorum((read-x - quorum-w / 2, -read-first + 38), quorum-w, last - read-first + 76)

    for (i, y) in rows.enumerate() {
      let wrote = i < written
      value(write-x, y, card-w, if wrote { "write" } else { "missed" }, stored.at(i))
      if i < read-from {
        continue
      }
      if wrote {
        value(read-x, y, card-w, "read", stored.at(i))
      } else {
        value(read-x, y, card-w, "empty", [empty])
      }
    }

    let result-x = line-end + 36
    let result-y = (read-first + last) / 2 - 26
    note((result-x, -result-y), 904 - result-x, kind: "ok", result)
    note((40, -note-y), 880, shared)
  })
}
