## Raft: (Bug): Single-Server Change Loses a Committed Config

**按 Raft [博士论文](https://web.stanford.edu/~ouster/cgi-bin/papers/OngaroPhD.pdf) (§4.1) 描述的单节点成员变更 (single-server change), 已 commit 的 config 可能丢失. 日志 barrier 可以修复它: 新 leader 先 commit 一条自己 term 的日志, 再追加 config 日志.**

论文只要求 leader 在上一条 config 日志 commit 之后, 才开始下一次变更.
但不同 term 的两个 leader 可能看到同一个已 commit 的 `C0`, 并各自从 `C0` 追加自己的变更.
新 config 的任一 quorum 都与 `C0` 的任一 quorum 相交, 但两个新 config 的 quorum 之间可能没有公共节点.

### 从同一个 `C0` 出发的两次变更会丢失已 commit 的 config

取 `C0 = {a,b,c,d}`.
leader `a` 提出 `Cu = C0 + u`, 之后 leader `d` 提出 `Cv = C0 + v`.
两个新 config 都有 5 个投票节点, 多数派是 3 个节点: `Cu` 允许 quorum `{a,b,u}`, `Cv` 允许 quorum `{c,d,v}`.
这两个 quorum 没有公共节点.

六个节点的日志在 index 5 之前都相同, 都属于 term 0. `u` 和 `v` 是已经追上日志的 learner.

1. term 1: `a` 在 `C0` 下得到 `{a,b,c}` 的选票, 在 index 6 追加 `Cu`.
   只有 `a` 和 `u` 存了 `Cu`, 随后 `a` 宕机.
2. term 2: `b`, `c`, `d` 没有收到 `Cu`, 它们在 `C0` 下选出 `d`.
   `d` 在 index 6 追加 `Cv`, 在 `Cv` 的多数派 `{c,d,v}` 上 commit, 随后宕机.
   `b` 投票给了 `d`, 但没有收到任何 term 2 的日志.
3. term 3: `a` 重启, 按自己最新的 config `Cu` 发起选举.
   `b` 的日志停在 index 5, 属于 term 0; `a` 的日志停在 index 6, 属于 term 1.
   所以 `b` 投票给 `a`, `a` 以 `Cu` 的多数派 `{a,b,u}` 当选.
   `a` 在 `{a,b,u}` 上 commit 一条 term 3 的 noop, 连带 commit 了 index 6 上的 `Cu`.
   然后 `a` 把日志复制给 `c`, 以及恢复后的 `d`, 覆盖了它们上面已 commit 的 `Cv`.

### 日志 barrier 排除更早的变更

新 leader 必须先 commit 一条自己 term 的日志, 再追加 config 日志.

在上面的例子中, `d` 追加 `Cv` 之前, 必须先在 `C0` 下 commit 一条 term 2 的日志.
这次 commit 需要 `{a,b,c,d}` 中的三个节点, 而 `a` 已宕机, 所以 `b`, `c`, `d` 都会存下 term 2 的日志.
`Cu` 只比 `C0` 多一个节点, 所以 `Cu` 的每个 quorum 都至少包含其中一个节点.
这个节点不会投票给日志停在 term 1 的 `a`, `a` 无法按 `Cu` 当选.

更早的任何变更也是一样.
barrier 日志 commit 在 `C0` 的一个 quorum 上.
更早的 term 从 `C0` 提出的 config, 只留在最后一条日志属于更早 term 的节点上, 因为 barrier 所在 term 及之后的 leader, 日志中都没有它.
要按这个 config 当选或 commit, leader 需要它的一个 quorum, 而这个 quorum 与 `C0` 的每个 quorum 都相交.
所以这个 quorum 中有一个节点存有 barrier 日志.
这个节点不会投票给最后一条日志属于更早 term 的节点, 也会拒绝更早 term 的 leader 发来的 AppendEntries.

joint consensus 不需要这个 barrier 也是安全的: 从 `C0` 提出的两个 joint config 都包含 `C0`, 所以它们的 quorum 必然相交.
见 [Joint Consensus Needs No Log Barrier, but Joint Rollback Does](../raft-joint-rollback-barrier/raft-joint-rollback-barrier.cn.md).

**参考**:

- [Consensus: Bridging Theory and Practice](https://web.stanford.edu/~ouster/cgi-bin/papers/OngaroPhD.pdf), §4.1: 单节点成员变更
- raft-dev 上的 bug 报告: [bug in single-server membership changes](https://groups.google.com/g/raft-dev/c/t4xj6dJTP6E)
- 加上日志 barrier 之后的安全性论证: [Safety of Raft single-server membership changes](https://gist.github.com/ongardie/a11f32b70581e20d6bcd)
- [Raft Membership Changes: When a Leader Must Commit a Current-Term Entry First](https://blog.openacid.com/distributed/raft-membership-log-barrier/), §4
