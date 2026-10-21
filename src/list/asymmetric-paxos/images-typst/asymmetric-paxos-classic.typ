#import "style.typ": *

// Each acceptor's timeline runs left to right at its row.
#let rows = (210, 290, 370)
#let write-x = 300
#let read-x = 520

// A value an acceptor stores or returns, centered on its timeline.
#let value(x, y, kind, label) = {
  let paint = if kind == "read" { read-ink } else { write-ink }
  let fill = if kind == "read" { read-text } else { write-text }
  card((x - 42, -y + 20), 84, 40, radius: 10, fill: white, stroke: pen(paint, 2.5), mono(fill: fill, label))
}

#diagram(
  960,
  550,
  "Classic Paxos: a quorum is any 2 of 3 acceptors",
  "P1 writes x=3 to A1 and A2. Later, P2 reads A2 and A3.",
  {
    panel((40, -110), 880, 340)
    content((write-x, -152), small(fill: write-text)[P1 writes], anchor: "base")
    content((read-x, -152), small(fill: read-text)[P2 reads], anchor: "base")

    for (i, y) in rows.enumerate() {
      let name = "a" + str(i + 1)
      line((136, -y), (640, -y), stroke: pen(lifeline-ink, 2.5), mark: (end: ">", fill: lifeline-ink, length: 10, width: 9))
      node((100, -y), "A" + str(i + 1), name: name)
    }
    ring("a2")

    quorum((write-x - 60, -rows.at(0) + 38), 120, 156, kind: "write")
    value(write-x, rows.at(0), "write", "x=3")
    value(write-x, rows.at(1), "write", "x=3")

    quorum((read-x - 60, -rows.at(1) + 38), 120, 156)
    value(read-x, rows.at(1), "read", "x=3")
    card((read-x - 42, -rows.at(2) + 20), 84, 40, radius: 10, fill: white, stroke: pen(read-ink, 2, dash: quorum-dash), small(fill: slate)[empty])

    note((676, -284), 224, kind: "ok")[
      *P2 gets x = 3* \
      #small(fill: slate)[from A2.]
    ]

    note((40, -484), 880)[
      *A2 is in both quorums.* Any two 2-of-3 quorums share an acceptor, so a read always finds x.
    ]
  },
)
