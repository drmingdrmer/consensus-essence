#import "style.typ": *

// Plot coordinates: redundancy in percent maps to x, unavailability in
// units of p^2 maps to y.
#let origin = (150, -500)
#let x-of(redundancy) = origin.at(0) + (redundancy - 100) * 3
#let y-of(k) = origin.at(1) + k * 30

#let p2(k) = [#k;p#super[2]]

// (redundancy, unavailability in p^2, label, label anchor)
#let setups = (
  (300, 3, "classic paxos, 3 nodes", "south"),
  (200, 6, "asymmetric paxos, 4 nodes", "west"),
  (500 / 3, 10, "asymmetric paxos, 5 nodes", "west"),
)

#diagram(
  960,
  650,
  "Asymmetric Paxos stores less but is less available",
  "Each setup tolerates 1 failed acceptor. p: the probability that one acceptor is down, independently.",
  {
    panel((40, -110), 880, 450)

    let x-end = 800
    let y-end = -160
    let axis-mark = (end: ">", fill: ink, length: 12, width: 10)
    line(origin, (x-end, origin.at(1)), stroke: pen(ink, 2.5), mark: axis-mark)
    line(origin, (origin.at(0), y-end), stroke: pen(ink, 2.5), mark: axis-mark)
    content((origin.at(0), y-end + 16), small(fill: slate)[Unavailability], anchor: "base")
    content((x-end + 12, origin.at(1) - 6), small(fill: slate)[Redundancy], anchor: "base-west")
    content((origin.at(0) - 14, origin.at(1) - 6), small(fill: slate)[0], anchor: "base-east")
    content((origin.at(0), origin.at(1) - 30), small(fill: slate)[100%], anchor: "base")

    for (redundancy, k, label, anchor) in setups {
      let pos = (x-of(redundancy), y-of(k))
      let guide = pen(lifeline-ink, 2, dash: guide-dash)
      line((origin.at(0), pos.at(1)), pos, stroke: guide)
      line((pos.at(0), origin.at(1)), pos, stroke: guide)
      circle(pos, radius: 7, fill: ink, stroke: none)

      let tick = calc.round(redundancy, digits: 1)
      content((pos.at(0), origin.at(1) - 30), small(fill: slate)[#tick%], anchor: "base")
      content((origin.at(0) - 14, pos.at(1) - 6), small(fill: slate, p2(k)), anchor: "base-east")

      let offset = if anchor == "south" { (0, 14) } else { (16, 0) }
      content((rel: offset, to: pos), label, anchor: anchor)
    }

    note((40, -588), 880, kind: "info")[
      Each setup needs all but 1 of its n acceptors, so any 2 failures stop it: unavailability ≈ C(n, 2) · p#super[2].
    ]
  },
)
