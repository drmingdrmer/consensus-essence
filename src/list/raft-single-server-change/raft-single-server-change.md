## Raft: (Bug): Single-Server Change Loses a Committed Config

**Raft's single-server membership change, as the [dissertation](https://web.stanford.edu/~ouster/cgi-bin/papers/OngaroPhD.pdf) (§4.1) describes it, can lose a committed config. A log barrier fixes it: a new leader commits an entry of its own term before it appends a config entry.**

The dissertation lets a leader start a change only after the previous config entry has committed.
But two leaders of different terms can both see the same committed `C0`, and each can append its own change from `C0`.
Every quorum of a new config intersects every quorum of `C0`. But a quorum of one new config may share no node with a quorum of the other.

### Two Changes from One `C0` Lose a Committed Config

Take `C0 = {a,b,c,d}`.
Leader `a` proposes `Cu = C0 + u`, and later leader `d` proposes `Cv = C0 + v`.
Each new config has 5 voters, so its majority is 3 nodes: `Cu` permits the quorum `{a,b,u}`, and `Cv` permits the quorum `{c,d,v}`.
These two quorums share no node.

All six nodes hold the same log up to index 5, of term 0. `u` and `v` are learners that have caught up.

1. Term 1: `a` wins the votes of `{a,b,c}` under `C0`, and appends `Cu` at index 6.
   Only `a` and `u` store `Cu`, and then `a` crashes.
2. Term 2: `b`, `c` and `d` have not received `Cu`, so they elect `d` under `C0`.
   `d` appends `Cv` at index 6, commits it on `{c,d,v}`, a majority of `Cv`, and then crashes.
   `b` votes for `d`, but receives no entry of term 2.
3. Term 3: `a` restarts and runs its election under its latest config, `Cu`.
   The log on `b` ends at index 5 in term 0, and the log on `a` ends at index 6 in term 1.
   So `b` votes for `a`, and `a` wins with `{a,b,u}`, a majority of `Cu`.
   `a` commits a term-3 noop on `{a,b,u}`, which also commits `Cu` at index 6.
   Then `a` replicates its log to `c`, and to `d` after `d` recovers. This overwrites the committed `Cv` on both.

### The Log Barrier Rules out the Older Change

A new leader must commit an entry of its own term before it appends a config entry.

In the run above, `d` must commit an entry of term 2 under `C0` before it appends `Cv`.
That commit needs three nodes of `{a,b,c,d}`, and `a` is down, so `b`, `c` and `d` all store an entry of term 2.
`Cu` adds only one node to `C0`, so every quorum of `Cu` includes at least one of them.
That node refuses to vote for `a`, whose log ends in term 1, so `a` cannot win under `Cu`.

The same holds for any older change.
The barrier entry is committed on a quorum of `C0`.
A config proposed from `C0` in an older term stays only in logs that end in an older term, because no leader of the barrier's term or later has it.
To win an election or to commit under that config, a leader needs a quorum of it, and that quorum intersects every quorum of `C0`.
So that quorum includes a node that holds the barrier entry.
This node refuses to vote for a log that ends in an older term, and it rejects AppendEntries from a leader of an older term.

Joint consensus is safe without this barrier: two joint configs proposed from `C0` both contain `C0`, so their quorums intersect.
See [Joint Consensus Needs No Log Barrier, but Joint Rollback Does](../raft-joint-rollback-barrier/raft-joint-rollback-barrier.md).

**References**:

- [Consensus: Bridging Theory and Practice](https://web.stanford.edu/~ouster/cgi-bin/papers/OngaroPhD.pdf), §4.1: single-server membership changes
- The bug report on raft-dev: [bug in single-server membership changes](https://groups.google.com/g/raft-dev/c/t4xj6dJTP6E)
- The safety argument with the log barrier: [Safety of Raft single-server membership changes](https://gist.github.com/ongardie/a11f32b70581e20d6bcd)
- [Raft Membership Changes: When a Leader Must Commit a Current-Term Entry First](https://blog.openacid.com/distributed/raft-membership-log-barrier/), §4
