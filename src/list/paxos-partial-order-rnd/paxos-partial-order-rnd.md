## Paxos: (Generalize): Partial Order Round Number = Paxos + 2PC

[Paxos](https://en.wikipedia.org/wiki/Paxos_(computer_science)) is a protocol where, during phase-1, a Proposer must generate an integer value `n` as the `rnd` number. However, this definition can be expanded beyond integers. By using any value from a [partially ordered set](https://en.wikipedia.org/wiki/Partially_ordered_set), the essential ordering property of `rnd` in Paxos is still preserved, which is critical for the protocol's correctness.

In a generalized Paxos, the `rnd` can employ a partial order, allowing for a choice between **mandatory** conflict exclusion, akin to the [Two-Phase Commit Protocol (2PC)](https://en.wikipedia.org/wiki/Two-phase_commit_protocol), and **non-mandatory** conflict exclusion, reminiscent of the potential for livelock in Paxos, to uphold the safety of the consensus process.

Consider the **divisibility** as a partial ordering criterion in Paxos, where `rnd` is a positive integer, and the order is determined by divisibility: **if a divides b, then a is less than b**. For example, we have `1 < 2 < 6`, and `1 < 3 < 6`, but `2` is not comparable to `3`. In a scenario where Proposer P2 has finished phase-1, P3 cannot proceed because `3 ≯ 2` on Acceptor A2. Therefore, P3 quits and P6 can initiate phase-1 and proceed to phase-2, ultimately commit a value.

![](paxos-partial-order-rnd.jpeg)

In practical applications, the concept of a partial order `rnd` greatly expands the possibilities for consistency algorithms like Paxos by introducing multi-dimensional ordering, which can be thought of as an analogy to multi-dimensional time.

For instance, in a distributed storage system, one could establish two separate `rnd` groups:

- One group of Proposers might use powers of 2 for `rnd`, targeting transaction A;
- The other group may use powers of 3 for `rnd`, targeting transaction B.

These two groups operate independently, ensuring that only one transaction can be committed at a time, thus avoiding Paxos’s livelock issues between the two groups. Additionally, multiple Proposers within the same group can provide redundancy, without the single point of failure risk inherent in 2PC's Coordinator.

However, the two groups can block each other. In the figure above, after A1 has promised 3 and A2 has promised 2, suppose A3 goes down. A1 rejects every power of 2 and A2 rejects every power of 3, so neither group can make progress, even though A1 and A2 form a majority.

Any Proposer can then recover the way P6 does in the figure. It picks a `rnd` of the form 2ⁿ·3ᵐ that is a multiple of every promise in its quorum: 6 here, or 36 if the promises have grown to 4 and 9. The recovering Proposer runs plain Paxos: if phase-1 returns accepted values, it adopts the one with the greatest `rnd`, so it may commit the other group's transaction. This greatest `rnd` always exists. A value is accepted only at a `rnd` that has finished phase-1, and the quorums of two such `rnd`s share an Acceptor whose promises only grow, so all accepted `rnd`s are comparable.

A recovery `rnd` can be greater than the `rnd`s of both groups, so recovery falls back to the **non-mandatory** conflict exclusion of Paxos: concurrent recovering Proposers may preempt each other, and recovery is only guaranteed to finish once a single recovering Proposer runs alone long enough. High availability therefore depends on this recovery, and recovery has the same liveness as classic Paxos.

In conclusion, **partial order Paxos cleverly combines the transaction exclusivity of 2PC with the robustness of Paxos. This fusion simplifies the complex two-layer framework found in distributed databases, such as Spanner, into a more streamlined single-layer architecture**.
