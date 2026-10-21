#import "style.typ": *

#rw-figure(
  "Classic Paxos: a quorum is any 2 of 3 acceptors",
  "P1 writes x=3 to A1 and A2. Later, P2 reads A2 and A3.",
  ("x=3", "x=3", "x=3"),
  2,
  1,
  [*P2 rebuilds* \ x = 3],
  [*A2 is in both quorums.* \ Any two 2-of-3 quorums share 1 acceptor, so a read always finds x.],
)
