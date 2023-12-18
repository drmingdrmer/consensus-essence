## Paxos/Raft: (Generalize): Round Number Reversion

### The capability in Paxos to revert `last_rnd`

In Paxos, the `last_rnd` variable is designed to be monotonically increasing. When an acceptor receives a proposal with a round number (`rnd`) higher than its current `last_rnd` during either `phase-1` or `phase-2`, it updates its `last_rnd` accordingly. Despite this, **`last_rnd` can be reverted while still maintaining correctness**: If during `phase-1`, a proposer increases an acceptor's `last_rnd` from 1 to 2, a reversion can be initiated before `phase-2` commences. By sending a `phase-1-revert` message, the proposer can request the acceptor to rollback `last_rnd` from 2 to 1, provided the acceptor's `last_rnd` remains at 2.

### Correctness of reversions

The logic behind revert operations is straightforward. A reversion can be equated to an artificially induced **message loss** event. Since Paxos is designed to handle message loss without compromising consistency, the correctness of the protocol is not affected by such reversions.

### Reversion example

Consider a scenario where three proposers, P1, P2, and P3, have carried out `phase-1` with round numbers 1, 2, and 3, respectively. The permissible revert operations in this context would be:

- A1 ✅ Allows P3 to revert from 3 to 1
- A2 ✅ Allows P3 to revert from 3 to 2, ✅ then permits P2 to revert from 2 to 1; ❌ however, a direct reversion from 3 to 1 is not permissible.

![](paxos-revert-rnd-margin.jpeg)

Reversion is applicable to both Paxos (by reverting the acceptor's `last_rnd`) and Raft (by reverting `(term, voted_for)`).
It's worth noting that Paxos's `last_rnd` corresponds to Raft's `(term, voted_for)`. These elements represent the virtual **time** in these systems, while Paxos's `phase-2` and Raft's log append operations are analogous to creating an **event** at a specific **time point**.

### Applications

The main objective of implementing reversions is to handle conflicts that emerge due to [partial order relations](https://zh.wikipedia.org/wiki/偏序关系) in consensus protocols.
For example, consider a Raft cluster where Follower N3, unaware of any logs from term=2, initiates an election for term=3.
N1 and N2 would deny N3's request for votes because N3's log is outdated.
Although N1 retains its leader status, it cannot replicate logs to N3, as N3 is at a higher term.
Consequently, N1 is compelled to transition from Leader to Candidate and initiate a new election with a higher term number (at least 3), leading to a temporary service interruption.

Using the reversion strategy, N3 could smoothly revert its term after an unsuccessful election attempt, thus avoiding any disruption to the cluster's leadership.

![](paxos-revert-rnd-raft-margin.jpeg)
