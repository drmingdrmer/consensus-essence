## Raft: (Generalize): Read-Write Quorum

[Flexible Paxos](https://arxiv.org/abs/1608.06696) 证明, Paxos 的安全只需要 read quorum (phase-1) 与 write quorum (phase-2) 相交; read quorum 之间, write quorum 之间都不必相交.

Raft 的 election 对应 phase-1, `AppendEntries` 对应 phase-2. 但 **Raft 直接套用这个条件会脑裂**.

例如 4 个节点, read quorum 为 `{1, 2}`, `{3, 4}`, write quorum 为 `{1, 3}`, `{2, 4}`, 满足 Flexible Paxos 的条件.
N1 与 N4 之间的消息延迟时:

- N1 用 `{1, 2}`, N4 用 `{3, 4}`, 在同一个 term `t` 当选;
- N1 通过 `{1, 3}` 在 index `k` commit `a`, N4 通过 `{2, 4}` 在 index `k` commit `b`.

N3 投票给了 N4, 但仍接受 N1 的 `AppendEntries`, 因为 **Raft 的 follower 接受同一 term 的任何 leader**. 这只在每个 term 至多一个 leader 时安全.
同一 term 的两个 leader `(t, 1)` 和 `(t, 4)` 无法比较大小 (`(term, voted_for)` 是偏序), 所以 phase-2 无法像 Paxos 那样拒绝较小的 leader.

### 方法一: read quorum 之间也相交

Raft 协议不变, 用相交的 read quorum 保证每个 term 只有一个 leader. 要求:

- read quorum 与 write quorum 相交;
- read quorum 之间相交;
- write quorum 之间不必相交.

代价: quorum 按节点数定义时, read quorum 至少是多数派, 只能缩小 write quorum. 例如 4 个节点: 任意 3 个节点选举, 任意 2 个节点 commit.

### 方法二: leader 标识改为全序

`(term, node_id)` 按字典序比较, 和 Paxos 的 ballot 一样是全序. 这时只要求 read quorum 与 write quorum 相交, 和 Flexible Paxos 相同. 协议改动:

- `AppendEntries`: 拒绝 `(term, node_id)` 小于本地 `(term, voted_for)` 的 leader; 接受时, 把本地 `(term, voted_for)` 更新为这个 leader.
- log id 从 `(term, index)` 改为 `(term, node_id, index)`, election 时按它比较日志新旧.
- leader 只按副本数 commit 自己写入的日志, 即 log id 中 `(term, node_id)` 等于自己的日志.

开头的 4 节点例子在方法二下是安全的. read quorum `{3, 4}` 与 write quorum `{1, 3}` 的交点 N3 只有两种顺序:
先投票给 N4, 就拒绝 N1 的 `a`; 先接受 N1 的 `a`, 就因为日志比 N4 新而拒绝给 N4 投票.

**参考**:

- [Flexible Paxos: Quorum Intersection Revisited](https://arxiv.org/abs/1608.06696)
- 方法一的 quorum 条件: [raft.tla `FlexibleQuorums`](https://github.com/fpaxos/raft.tla/blob/2d9bdd96b5504d0644deb435335e0417cbd6798f/raft.tla#L32-L37)
- 方法二的全序 leader 标识: [OpenRaft `leader_id_adv`](https://github.com/databendlabs/openraft/blob/728e66bba05c4305dd37016535d1c247a35d2059/openraft/src/vote/leader_id/leader_id_adv.rs#L9-L59)
