#import "style.typ": *

#rw-figure(
  "Asymmetric Paxos: a quorum is any 3 of 4 acceptors",
  "P1 writes x=3, y=4 to A1–A3, one coded value each. Later, P2 reads A2–A4.",
  ("x=3", "y=4", "x+y=7", "x−y=−1"),
  3,
  1,
  [*P2 rebuilds* \ x = 3, y = 4],
  [*A2 and A3 are in both quorums.* \ Any two 3-of-4 quorums share 2 acceptors, enough to rebuild x and y.],
  card-w: 100,
)
