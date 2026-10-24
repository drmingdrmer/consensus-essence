## Raft: (Generalize): Single-Step Change Needs Only Quorum Overlap

**Raft 用一条 config 日志把 config 从 `C0` 变更到 `C1`, 只要 `C0` 的任一 quorum 都与 `C1` 的任一 quorum 相交, 就是安全的. 对多数派 quorum, 判定条件是 `maj(C0) + maj(C1) > |C0 ∪ C1|`, 其中 `maj(C)` 是 `C` 的多数派大小.**

[博士论文](https://web.stanford.edu/~ouster/cgi-bin/papers/OngaroPhD.pdf) (§4.1) 只允许一条 config 日志增加或删除一个节点, 即单节点成员变更 (single-server change).
单节点成员变更是这个条件的特例.
单节点成员变更的另外两条规则不变.
leader 只在上一条 config 日志 commit 之后, 才追加下一条 config 日志.
新 leader 在追加 config 日志之前, 还要先 commit 一条自己 term 的日志.
这就是 [Single-Server Change Loses a Committed Config](../raft-single-server-change/raft-single-server-change.cn.md) 中的日志 barrier.

### 替换 `{a,b,c,d}` 中的一个节点仍保持 quorum 相交

取 `C0 = {a,b,c,d}`, `C1 = {a,b,c,e}`.
这次变更删除 `d` 并加入 `e`, 所以按论文的规则需要两条 config 日志.
两个 config 的每个 quorum 都有 3 个节点, 其中最多一个不在 `{a,b,c}` 中.
所以 `C0` 的一个 quorum 和 `C1` 的一个 quorum 各自至少包含 `a`, `b`, `c` 中的 2 个, 它们至少共有其中一个.

### 有 quorum 相交就足以保证安全

在 [Ongaro 的安全性论证](https://gist.github.com/ongardie/a11f32b70581e20d6bcd) 中, 单节点规则只在两种情形下起作用.
这两种情形中, 一条日志在一个 config 下 commit, 之后的 leader 在另一个 config 下当选.
这两个 config 中, 一个是另一个的父 config.
这时论证只需要: 一个 config 的任一 quorum 都与另一个 config 的任一 quorum 相交.
其余情形不需要两个不同 config 之间的 quorum 相交.
所以这个论证对任何满足 quorum 相交的变更都成立.

### 用多数派大小精确判定

对多数派 quorum, `maj(C) = ⌊|C|/2⌋ + 1`.
`C0` 的任一 quorum 都与 `C1` 的任一 quorum 相交, 当且仅当 `maj(C0) + maj(C1) > |C0 ∪ C1|`:

- 两个没有公共节点的 quorum 一共包含 `C0 ∪ C1` 中 `maj(C0) + maj(C1)` 个不同节点.
  所以条件成立时, 不存在这样的一对 quorum.
- 反过来, 假设条件不成立. 构造 `C0` 的一个 quorum `Q0`, 先取不在 `C1` 中的节点.
  如果 `Q0` 不需要 `C1` 的节点, `C1` 的任一 quorum 都与它不相交.
  否则 `Q0` 包含 `C0` 中所有不在 `C1` 中的节点, 所以 `C1` 中有 `|C0 ∪ C1| - maj(C0) ≥ maj(C1)` 个节点在 `Q0` 之外.
  这些节点组成 `C1` 的一个与 `Q0` 不相交的 quorum.

### 这个条件只多允许三种变更, 都在偶数大小的 config 之间

令 `changed = |C0 ∪ C1| - |C0 ∩ C1|`, 即变更增加或删除的节点数.
条件即 `maj(C0) + maj(C1) ≥ |C0 ∪ C1| + 1`.
由 `maj(C) ≤ |C|/2 + 1` 和 `|C0| + |C1| = |C0 ∪ C1| + |C0 ∩ C1|`, 可得 `changed ≤ 2`.

- `changed = 1` 时, 变更增加或删除一个节点.
  这时 `maj(C0) + maj(C1) = |C0 ∪ C1| + 1`, 条件总是成立.
- `changed = 2` 时, 条件成立当且仅当两个 config 都满足 `maj(C) = |C|/2 + 1`, 即 `|C0|` 和 `|C1|` 都是偶数.

所以除了增加或删除一个节点, 只要两个 config 的大小都是偶数, 一条 config 日志还可以替换一个节点, 增加两个节点, 或删除两个节点.

**参考**:

- [Consensus: Bridging Theory and Practice](https://web.stanford.edu/~ouster/cgi-bin/papers/OngaroPhD.pdf), §4.1: 单节点成员变更
- 加上日志 barrier 之后的安全性论证: [Safety of Raft single-server membership changes](https://gist.github.com/ongardie/a11f32b70581e20d6bcd)
- 同一个条件, 名为 `QuorumsOverlap`: William Schultz 的 [On Raft's Reconfiguration Bug](https://will62794.github.io/distributed-systems/2024/02/09/note-on-raft-reconfig-bug.html)
- [Adaptive Membership Changes in OpenRaft: Skip Joint Consensus When Quorums Overlap](https://blog.openacid.com/distributed/raft-adaptive-config-change/), §2
