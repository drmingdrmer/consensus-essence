#import "style.typ": *

#rw-figure(
  "Asymmetric Paxos 5-4: a quorum is any 4 of 5 acceptors",
  "P1 writes x=3, y=4, z=5 to A1–A4, one coded value each. Later, P2 reads A2–A5.",
  ("x=3", "y=4", "z=5", "x+y+z=12", "x+2y+4z=31"),
  4,
  1,
  [*P2 rebuilds* \ x = 3, y = 4, z = 5],
  [*A2, A3 and A4 are in both quorums.* \ Any two 4-of-5 quorums share 3 acceptors, enough to rebuild x, y and z.],
  card-w: 130,
)
