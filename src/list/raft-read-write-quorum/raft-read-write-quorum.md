## Raft: (Generalize): Read-Write Quorum

[Flexible Paxos](https://arxiv.org/abs/1608.06696) shows that Paxos is safe as long as every read quorum (phase-1) intersects every write quorum (phase-2). Two read quorums do not need to intersect, and neither do two write quorums.

Raft's election corresponds to phase-1, and `AppendEntries` corresponds to phase-2. But **applying this rule to Raft directly causes split brain**.

For example, take 4 nodes with read quorums `{1, 2}`, `{3, 4}` and write quorums `{1, 3}`, `{2, 4}`. This satisfies the Flexible Paxos rule.
If messages between N1 and N4 are delayed:

- N1 is elected with `{1, 2}`, and N4 is elected with `{3, 4}`, both in the same term `t`;
- N1 commits `a` at index `k` through `{1, 3}`, and N4 commits `b` at index `k` through `{2, 4}`.

N3 voted for N4 but still accepts N1's `AppendEntries`, because **a Raft follower accepts any leader of the same term**. This is safe only if each term has at most one leader.
The two leaders `(t, 1)` and `(t, 4)` have the same term and cannot be ordered, since `(term, voted_for)` is a partial order. So phase-2 cannot reject the smaller leader as Paxos does.

### Approach 1: Read Quorums Also Intersect Each Other

Keep the Raft protocol unchanged, and use intersecting read quorums to keep at most one leader per term. The requirements are:

- every read quorum intersects every write quorum;
- every two read quorums intersect;
- write quorums do not need to intersect.

Cost: when quorums are defined by node count, a read quorum must be at least a majority, so only the write quorum can shrink. For example, with 4 nodes: any 3 nodes to elect, any 2 nodes to commit.

### Approach 2: Make the Leader Id a Total Order

Compare `(term, node_id)` lexicographically, so it becomes a total order like a Paxos ballot. Then Raft only needs every read quorum to intersect every write quorum, the same as Flexible Paxos. The protocol changes are:

- `AppendEntries`: reject a leader whose `(term, node_id)` is smaller than the local `(term, voted_for)`; on accept, update the local `(term, voted_for)` to this leader.
- The log id changes from `(term, index)` to `(term, node_id, index)`, and the election compares logs by it.
- A leader uses the replica count to commit only its own logs, i.e., logs whose log id carries its own `(term, node_id)`.

The 4-node example above is safe with Approach 2. N3 is the intersection of read quorum `{3, 4}` and write quorum `{1, 3}`, and only two orders are possible on N3:
if N3 votes for N4 first, it rejects N1's `a`; if N3 accepts N1's `a` first, its log is newer than N4's, so it refuses to vote for N4.

**References**:

- [Flexible Paxos: Quorum Intersection Revisited](https://arxiv.org/abs/1608.06696)
- Quorum requirements of Approach 1: [raft.tla `FlexibleQuorums`](https://github.com/fpaxos/raft.tla/blob/2d9bdd96b5504d0644deb435335e0417cbd6798f/raft.tla#L32-L37)
- Total-order leader id of Approach 2: [OpenRaft `leader_id_adv`](https://github.com/databendlabs/openraft/blob/728e66bba05c4305dd37016535d1c247a35d2059/openraft/src/vote/leader_id/leader_id_adv.rs#L9-L59)
