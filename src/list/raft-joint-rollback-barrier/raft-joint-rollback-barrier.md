## Raft: (Variant): Joint Consensus Needs No Log Barrier, but Joint Rollback Does

**Standard joint consensus needs no log barrier. But if a joint config may roll back to the old config, both ways out of the joint config need one.**

A log barrier requires a new leader to commit an entry of its own term before it appends a config entry.
Raft's single-server membership change needs it, and [Ongaro's safety argument](https://gist.github.com/ongardie/a11f32b70581e20d6bcd) for that change depends on it.

### Standard Joint Consensus Is Safe without a Barrier

[Raft](https://raft.github.io/raft.pdf) (§6) changes `C0` to `C1` in two steps: it commits the joint config `C01 = [C0, C1]`, then appends `C1`.
Under `C01`, an election or a commit needs both a majority of `C0` and a majority of `C1`.

- Step 1: from the same `C0`, two leaders of different terms may propose `C01 = [C0, C1]` and `C02 = [C0, C2]`.
  Every quorum of either config contains a majority of `C0`, so every quorum of `C01` intersects every quorum of `C02`.
- Step 2: once `C01` commits, `C1` is the only allowed next config.
  Every leader that completes the change uses the quorums of `C1`, and any two of them intersect.

A new leader that inherits an uncommitted `C01` must still commit it before it appends `C1`.
Raft's commit rule (§5.4.2) may then require it to commit an entry of its own term first.

### Rollback Lets Two Leaders Commit Different Configs at the Same Index

Suppose a leader may also roll `C01` back to `C0`, for example to give up a change after the nodes of `C1` fail.
Then the config after `C01` is either `C0` or `C1`.
With `C0 = {a,b,c}` and `C1 = {x,y,z}`, the quorum `{a,b}` of `C0` shares no node with the quorum `{x,y}` of `C1`.
So, starting from a committed `C01`, two leaders can commit `C0` and `C1` at the same index:

1. Term 1: leader `a` commits a noop on `{a,b,x,y}`, a quorum of `C01`.
   Then `a` appends the rollback `C0` at index `i`. Only `a` stores it, and `a` crashes.
2. Term 2: `y` wins the votes of `{b,c,x,y}`, a quorum of `C01`.
   It appends `C1` at index `i` at once, and commits it on `{y,z}`, a majority of `C1`.
   `b` receives no entry of term 2.
3. Term 3: `a` restarts.
   A node uses the last config in its log, even an uncommitted one, so `a` runs its election under `C0`.
   The last entry on `b` is the term-1 noop at index `i-1`, and the last entry on `a` is `C0` at index `i`, also of term 1.
   So `b` votes for `a`, and `a` wins with `{a,b}`, a majority of `C0`.
   Then `a` commits a term-3 noop on `{a,b}`, which also commits `C0` at index `i`.

Now both `C1` and `C0` are committed at index `i`.

### Both Ways out of the Joint Config Need the Barrier

Before a leader appends `C0` or `C1` after `C01`, it must commit an entry of its own term under `C01`.

This barrier is enough.
The barrier entry is committed on a quorum of `C01`, so a majority of `C0` and a majority of `C1` both hold it.
Suppose a leader of an older term appended the other config.
No leader of the barrier's term or later has that config in its log, so the config stays only in logs that end in an older term.
To win an election or to commit under that config, a leader needs a majority of `C0` or of `C1`.
That majority includes a node that holds the barrier entry.
This node refuses to vote for a log that ends in an older term, and it rejects AppendEntries from a leader of an older term.

In the run above, `y` must commit an entry of term 2 under `C01` before it appends `C1`.
That commit needs two nodes of `{a,b,c}`, and `a` is down, so both `b` and `c` store an entry of term 2.
Neither of them then votes for `a`, whose log ends in term 1, so `a` cannot win under `C0`.

A check on rollback alone does not stop this run: `a` committed its noop before the rollback.
The barrier that was skipped is the one before `y` completes the change to `C1`.

[OpenRaft](https://github.com/databendlabs/openraft) allows this rollback and checks the barrier.
Its `Raft::change_membership()` can move a committed `[C0, C1]` straight back to `C0`, as [`Membership::next_coherent()`](https://github.com/databendlabs/openraft/blob/2302753830e8a7e7dc2379fab0af2c38bd60a1e4/openraft/src/membership/membership.rs#L409-L436) shows.
Before every config change, [`ensure_leader_log_committed()`](https://github.com/databendlabs/openraft/blob/2302753830e8a7e7dc2379fab0af2c38bd60a1e4/openraft/src/core/raft_core.rs#L577-L609) requires the noop of the leader's own term to be committed.

**References**:

- [In Search of an Understandable Consensus Algorithm](https://raft.github.io/raft.pdf), §6: joint consensus
- Log barrier for single-server changes: [Safety of Raft single-server membership changes](https://gist.github.com/ongardie/a11f32b70581e20d6bcd)
- [Raft Membership Changes: When a Leader Must Commit a Current-Term Entry First](https://blog.openacid.com/distributed/raft-membership-log-barrier/)
- OpenRaft rollback from a joint config: [`Membership::next_coherent()`](https://github.com/databendlabs/openraft/blob/2302753830e8a7e7dc2379fab0af2c38bd60a1e4/openraft/src/membership/membership.rs#L409-L436)
- OpenRaft barrier check: [`ensure_leader_log_committed()`](https://github.com/databendlabs/openraft/blob/2302753830e8a7e7dc2379fab0af2c38bd60a1e4/openraft/src/core/raft_core.rs#L577-L609)
