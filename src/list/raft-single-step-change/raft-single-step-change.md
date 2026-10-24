## Raft: (Generalize): Single-Step Change Needs Only Quorum Overlap

**A Raft config change from `C0` to `C1` is safe as one config entry when every quorum of `C0` intersects every quorum of `C1`. For majority quorums, the test is `maj(C0) + maj(C1) > |C0 ∪ C1|`, where `maj(C)` is the majority size of `C`.**

The [dissertation](https://web.stanford.edu/~ouster/cgi-bin/papers/OngaroPhD.pdf) (§4.1) allows a one-entry change only when it adds or removes one node, the single-server change.
That change is a special case of this condition.
The other two rules of the single-server change stay.
A leader appends a config entry only after the previous config entry has committed.
A new leader also commits an entry of its own term before it appends a config entry.
This is the log barrier of [Single-Server Change Loses a Committed Config](../raft-single-server-change/raft-single-server-change.md).

### Replacing One Node of `{a,b,c,d}` Keeps Quorum Overlap

Take `C0 = {a,b,c,d}` and `C1 = {a,b,c,e}`.
This change removes `d` and adds `e`, so the dissertation's rule needs two config entries for it.
Every quorum of either config has 3 nodes, and at most one of them is outside `{a,b,c}`.
So a quorum of `C0` and a quorum of `C1` each hold at least 2 of `a`, `b` and `c`, and they share at least one of them.

### Quorum Overlap Is Enough for Safety

The one-node rule matters in only two cases of [Ongaro's safety argument](https://gist.github.com/ongardie/a11f32b70581e20d6bcd) for the single-server change.
In both cases, an entry commits under one config, and a later leader wins under another.
One of the two configs is the parent of the other.
There the argument needs only that every quorum of one config intersects every quorum of the other.
The other cases need no overlap between two different configs.
So the argument holds for any change with quorum overlap.

### Majority Sizes Give an Exact Test

With majority quorums, `maj(C) = ⌊|C|/2⌋ + 1`.
Every quorum of `C0` intersects every quorum of `C1` exactly when `maj(C0) + maj(C1) > |C0 ∪ C1|`:

- Two quorums that share no node hold `maj(C0) + maj(C1)` distinct nodes of `C0 ∪ C1`.
  So when the test passes, no such pair exists.
- Conversely, suppose the test fails. Build a quorum `Q0` of `C0` from nodes outside `C1` first.
  If `Q0` needs no node of `C1`, any quorum of `C1` misses it.
  Otherwise `Q0` holds every node of `C0` outside `C1`, so `|C0 ∪ C1| - maj(C0) ≥ maj(C1)` nodes of `C1` stay outside `Q0`.
  They form a quorum of `C1` that misses `Q0`.

### The Test Adds Three Changes, All between Even-Sized Configs

Let `changed = |C0 ∪ C1| - |C0 ∩ C1|` be the number of nodes that the change adds or removes.
The test is `maj(C0) + maj(C1) ≥ |C0 ∪ C1| + 1`.
With `maj(C) ≤ |C|/2 + 1` and `|C0| + |C1| = |C0 ∪ C1| + |C0 ∩ C1|`, it implies `changed ≤ 2`.

- With `changed = 1`, the change adds or removes one node.
  It always passes, because then `maj(C0) + maj(C1) = |C0 ∪ C1| + 1`.
- With `changed = 2`, the change passes exactly when `maj(C) = |C|/2 + 1` for both configs, that is, when `|C0|` and `|C1|` are both even.

So, besides adding or removing one node, one config entry can also replace one node, add two nodes, or remove two nodes, if both configs have an even size.

**References**:

- [Consensus: Bridging Theory and Practice](https://web.stanford.edu/~ouster/cgi-bin/papers/OngaroPhD.pdf), §4.1: single-server membership changes
- The safety argument with the log barrier: [Safety of Raft single-server membership changes](https://gist.github.com/ongardie/a11f32b70581e20d6bcd)
- The same condition, named `QuorumsOverlap`: [On Raft's Reconfiguration Bug](https://will62794.github.io/distributed-systems/2024/02/09/note-on-raft-reconfig-bug.html), by William Schultz
- [Adaptive Membership Changes in OpenRaft: Skip Joint Consensus When Quorums Overlap](https://blog.openacid.com/distributed/raft-adaptive-config-change/), §2
