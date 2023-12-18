## Paxos/Raft: (Generalize): 允许未发生事件的时间回退

**Paxos 中的 `last_rnd` 是允许回退的**: 虽然 Paxos 中定义 `last_rnd`  为单调增: 如果 acceptor 在 `phase-1` 或 `phase-2` 收到更大的 `rnd`, 则用它覆盖自己的 `last_rnd`. **但 `last_rnd` 实际上可以在保证正确性的前提下支持回退**: 如果 proposer 在 `phase-1` 将 acceptor 的 `last_rnd` 从 1 提升到 2,
那么只要没进入 `phase-2`, proposer 都可以再发送一个 `phase-1-revert` 消息要求 acceptor 将 `last_rnd` 从 2 回退到 1; 而 acceptor 的 `last_rnd` 如果还是 2, 就可以进行回退.

**Revert 的正确性** 容易直观的看出: revert 可以看作一个人为制造的**丢消息**的事件, 而 Paxos 本身又是允许丢消息而不破坏一致性的.

**举个 revert 操作的栗子**: 假设当前 P1, P2, P3 分别用 `rnd`=1,2,3 执行了 phase-1: 那么:
可以执行的revert操作是:

A1 ✅ 允许 P3: `1 ← 3`

A2 ✅ 允许 P3: `2 ← 3`, ✅ 然后允许 P2: `1 ← 2`; ❌ 但是不允许: `1 ← 3`.

![](paxos-revert-rnd-margin.jpeg)


Revert 可以应用到 Paxos(将 acceptor 的 `last_rnd` 回退到上一个值), 也可以应用到 raft(将 `(term, voted_for)` 回退到上一个值).
💡 Tip: Paxos 的 `last_rnd` 等同于 raft 的 `(term, voted_for)`, 分别用于定义这2个系统中的虚拟 **时间**, 而 Paxos 中 `phase-2` 和 raft 中的 `append` 日志, 可以看做在某个 **时间点** 上产生一个 **事件**.

**Revert 的用途** 是可以优雅的处理一致性协议中 [偏序关系](https://zh.wikipedia.org/wiki/偏序关系) 产生的冲突.
例如在下图的 raft 状态中, Follower N3 没有收到任何 term=2 的日志, 开始了 election,
term=3 时, N1 和 N2 都会拒绝 N3 的 vote 请求, 因为 N3 的 log 不够大.
这时 N1 的 Leadership 虽然不会丢失, 但已经无法向 N3 复制日志了, 因为 N3 的 term 更大,
N1 必须退出 Leader 到 Candidate 重新用更大的 term(至少是3) 来选举(raft 使用 pre-vote 来一定程度上避免这个问题), 造成短暂的不可用.

如果使用 revert, N3 可以在 election 失败后, 优雅的将 term 回退, 从而不会打断整个集群的 Leader.

![](paxos-revert-rnd-raft-margin.jpeg)
