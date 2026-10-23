## Raft: (Variant): Joint Consensus Needs No Log Barrier, but Joint Rollback Does

**标准 joint consensus 不需要日志 barrier. 但如果 joint config 可以回退到旧 config, 离开 joint config 的两个方向都需要日志 barrier.**

日志 barrier 要求新 leader 先 commit 一条自己 term 的日志, 再追加 config 日志.
Raft 的单节点成员变更 (single-server change) 需要它, [Ongaro 对这种变更的安全性论证](https://gist.github.com/ongardie/a11f32b70581e20d6bcd) 也依赖这条规则.

### 标准 joint consensus 不需要 barrier 也是安全的

[Raft](https://raft.github.io/raft.pdf) (§6) 分两步把 `C0` 变成 `C1`: 先 commit joint config `C01 = [C0, C1]`, 再追加 `C1`.
在 `C01` 下, election 和 commit 都同时需要 `C0` 的多数派和 `C1` 的多数派.

- 第一步: 不同 term 的两个 leader 可能从同一个 `C0` 分别提出 `C01 = [C0, C1]` 和 `C02 = [C0, C2]`.
  这两个 config 的每个 quorum 都包含 `C0` 的一个多数派, 所以 `C01` 的任一 quorum 都与 `C02` 的任一 quorum 相交.
- 第二步: `C01` commit 之后, 下一个 config 只能是 `C1`.
  完成变更的 leader 都使用 `C1` 的 quorum, 其中任意两个都相交.

新 leader 如果继承了未 commit 的 `C01`, 仍须先 commit `C01`, 再追加 `C1`.
按 Raft 的 commit 规则 (§5.4.2), 这可能要求它先 commit 一条自己 term 的日志.

### 回退让两个 leader 在同一个 index commit 不同的 config

假设 leader 还可以把 `C01` 回退到 `C0`, 例如在 `C1` 的节点故障后放弃这次变更.
这时 `C01` 之后的 config 可能是 `C0`, 也可能是 `C1`.
取 `C0 = {a,b,c}`, `C1 = {x,y,z}`, `C0` 的 quorum `{a,b}` 与 `C1` 的 quorum `{x,y}` 没有公共节点.
于是从已 commit 的 `C01` 出发, 两个 leader 可以在同一个 index 分别 commit `C0` 和 `C1`:

1. term 1: leader `a` 在 `C01` 的 quorum `{a,b,x,y}` 上 commit 一条 noop.
   然后 `a` 在 index `i` 追加回退日志 `C0`. 只有 `a` 存了这条日志, 随后 `a` 宕机.
2. term 2: `y` 得到 `C01` 的 quorum `{b,c,x,y}` 的选票.
   它立即在 index `i` 追加 `C1`, 并在 `C1` 的多数派 `{y,z}` 上 commit.
   `b` 没有收到任何 term 2 的日志.
3. term 3: `a` 重启.
   节点使用自己日志中最后一个 config, 即使它没有 commit, 所以 `a` 按 `C0` 发起选举.
   `b` 的最后一条日志是 index `i-1` 上 term 1 的 noop, `a` 的最后一条日志是 index `i` 上的 `C0`, 也属于 term 1.
   所以 `b` 投票给 `a`, `a` 以 `C0` 的多数派 `{a,b}` 当选.
   然后 `a` 在 `{a,b}` 上 commit 一条 term 3 的 noop, 连带 commit 了 index `i` 上的 `C0`.

这时 index `i` 上的 `C1` 和 `C0` 都已 commit.

### 离开 joint config 的两个方向都需要 barrier

leader 在 `C01` 之后追加 `C0` 或 `C1` 之前, 必须先在 `C01` 下 commit 一条自己 term 的日志.

这个 barrier 足以保证安全.
barrier 日志 commit 在 `C01` 的一个 quorum 上, 所以 `C0` 的一个多数派和 `C1` 的一个多数派都存有它.
假设某个更早 term 的 leader 追加了另一个 config.
barrier 所在 term 及之后的 leader, 日志中都没有这个 config, 所以它只留在最后一条日志属于更早 term 的节点上.
要按这个 config 当选或 commit, leader 需要 `C0` 或 `C1` 的一个多数派.
这个多数派中有一个节点存有 barrier 日志.
这个节点不会投票给最后一条日志属于更早 term 的节点, 也会拒绝更早 term 的 leader 发来的 AppendEntries.

在上面的例子中, `y` 追加 `C1` 之前, 必须先在 `C01` 下 commit 一条 term 2 的日志.
这次 commit 需要 `{a,b,c}` 中的两个节点, 而 `a` 已宕机, 所以 `b` 和 `c` 都会存下 term 2 的日志.
之后它们都不会投票给日志停在 term 1 的 `a`, `a` 无法按 `C0` 当选.

只检查回退请求挡不住这个例子: `a` 在回退之前已经 commit 了自己的 noop.
被跳过的是 `y` 完成到 `C1` 之前的 barrier.

[OpenRaft](https://github.com/databendlabs/openraft) 允许这种回退, 并检查 barrier.
它的 `Raft::change_membership()` 可以把已 commit 的 `[C0, C1]` 直接回退到 `C0`, 见 [`Membership::next_coherent()`](https://github.com/databendlabs/openraft/blob/2302753830e8a7e7dc2379fab0af2c38bd60a1e4/openraft/src/membership/membership.rs#L409-L436).
每次变更 config 之前, [`ensure_leader_log_committed()`](https://github.com/databendlabs/openraft/blob/2302753830e8a7e7dc2379fab0af2c38bd60a1e4/openraft/src/core/raft_core.rs#L577-L609) 都要求 leader 自己 term 的 noop 已经 commit.

**参考**:

- [In Search of an Understandable Consensus Algorithm](https://raft.github.io/raft.pdf), §6: joint consensus
- 单节点变更的日志 barrier: [Safety of Raft single-server membership changes](https://gist.github.com/ongardie/a11f32b70581e20d6bcd)
- [Raft Membership Changes: When a Leader Must Commit a Current-Term Entry First](https://blog.openacid.com/distributed/raft-membership-log-barrier/)
- OpenRaft 从 joint config 回退: [`Membership::next_coherent()`](https://github.com/databendlabs/openraft/blob/2302753830e8a7e7dc2379fab0af2c38bd60a1e4/openraft/src/membership/membership.rs#L409-L436)
- OpenRaft 的 barrier 检查: [`ensure_leader_log_committed()`](https://github.com/databendlabs/openraft/blob/2302753830e8a7e7dc2379fab0af2c38bd60a1e4/openraft/src/core/raft_core.rs#L577-L609)
