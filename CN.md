# 分布式要义明细

[English version](README.md)

<!-- DO NOT EDIT README.md directly. It is built from [src/README.md](src/README.md). -->

<table>
<tr class="header">
<th>Issue type</th>
<th>description</th>
</tr>
<tr class="odd">
<td><strong>Bug</strong></td>
<td>损坏数据的bug.</td>
</tr>
<tr class="even">
<td><strong>Trap</strong></td>
<td>不是bug, 但容易被误解, 容易实现错误的概念, 流程等.</td>
</tr>
<tr class="odd">
<td><strong>Suboptimal</strong></td>
<td>现有paper中可改进的地方.</td>
</tr>
<tr class="even">
<td><strong>Optimize</strong></td>
<td>对现有设计的改进</td>
</tr>
<tr class="odd">
<td><strong>Generalize</strong></td>
<td>对现有设计的扩展</td>
</tr>
</table>

## Issues

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

- [Paxos: (Optimize): Asymmetric Acceptors](#paxos-optimize-asymmetric-acceptors)
- [Paxos/Raft: (Generalize): 允许未发生事件的时间回退](#paxosraft-generalize-%E5%85%81%E8%AE%B8%E6%9C%AA%E5%8F%91%E7%94%9F%E4%BA%8B%E4%BB%B6%E7%9A%84%E6%97%B6%E9%97%B4%E5%9B%9E%E9%80%80)
- [Paxos: (Generalize): Partial Order Round Number = Paxos + 2PC](#paxos-generalize-partial-order-round-number--paxos--2pc)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

<!-- #### List -->

---

## Paxos: (Optimize): Asymmetric Acceptors

类似 [erasure-code](https://en.wikipedia.org/wiki/Erasure_code) 的算法也可以应用到paxos上以降低paxso的数据冗余.

### Paxos

在 [classic Paxos](http://lamport.azurewebsites.net/pubs/pubs.html#paxos-simple) 中, acceptors 是**对等**的 :

![classic](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@ci-asset/CN/8f2689f1e7dba5f9-asymmetric-paxos-classic.jpeg)

-   一个 proposer(quorum是: $q_i$) 将 $x$ 的值存储到 acceptor 上(至少2个 acceptor 上以完成对 $x$ 的提交).

-   当下一个 proposer(quorum是: $q_j$) 通过这几个 acceptor 来重建(也就是读) $x$ 的值的时候, 它必须访问到一个存储了 $x$ 的 acceptor.
    因此任意2个 quorum 的交集至少为1个 acceptor:

    $$|q_i \cap q_j| \ge 1$$

    即, 3节点集群中一个 quorum 是任意 2 个 acceptors:

    $$|q_i| \ge 2$$

在这样一个 3 节点 paxos 集群中:

-   数据冗余度是 300%;
-   容忍 1 个节点宕机;
-   可用性大约是 ${ 3 \choose 2  } p^2$, 其中 $p$ 是 acceptor 单位时间内的故障率.

### Asymmetric Paxos

因为我们可以从一个线性方程组 $ax+by=d_1, cx+dy=d_2$ 解得 $x, y$ 的值, 所以可以利用这个特性, 让 paxos 中的 acceptor 上存储不同的值(asymmetric), 来实现数据冗余的降低.

![ec](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@ci-asset/CN/0bb845a2df1a5134-asymmetric-paxos-ec.jpeg)

-   一个 proposer(quorum是: $q_i$) 将 $x, y, x+y, x-y$ 存储到 acceptor 1 到 4 上(至少成功3个, 以完成对 $x, y$ 的提交).

-   当下一个 proposer(quorum是: $q_j$) 通过这几个 acceptor 来重建(也就是读) $x, y$ 的值的时候, 它必须访问到**上面4个值其中的至少2个**.
    因此任意2个 quorum 的交集至少为2个 acceptor:

    $$|q_i \cap q_j| \ge 2$$

    即, 4节点集群中一个 quorum 是任意 3 个 acceptors:

    $$|q_i| \ge 3$$

在这样一个 4 节点非对称 paxos 集群中:

-   数据冗余度是 200%;
-   容忍 1 个节点宕机;
-   可用性大约是 ${ 4 \choose 2  } p^2$, 其中 p 是 acceptor 单位时间内的故障率.

### Asymmetric Paxos 5-4

一个5节点的非对称 paxos 集群中, 可以存储3个相互独立的值 $x, y, z$:

![ec53](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@ci-asset/CN/586e20c6dfc9460f-asymmetric-paxos-ec-53.jpeg)

一个 proposer 将 $x, y, z, x+y+z, x+2y+4z$ 5个值存储到 acceptor 1 到 5 上.
为了重新读到这 3 个值, 必须保证: $|q_i \cap q_j| \ge 3$.
因此最小的 quorum 的大小为任意4个 acceptor: $|q_i| \ge 4$.

在这样一个 5 节点非对称 paxos 集群中:

-   数据冗余度是 140%;
-   容忍 1 个节点宕机;
-   可用性大约是 ${ 5 \choose 2  } p^2$.

### Summary

利用 [asymmetric paxos](https://github.com/drmingdrmer/consensus-bugs/blob/main/CN.md#paxos-optimize-asymmetric-acceptors), 稍微降低数据的可靠性, 可以有效降低数据的冗余.

这个算法只能应用于 paxos, 因为 [raft](https://raft.github.io/) 的 leader 只从本地一个副本重建committed的数据, 而这个算法需要2个或更多节点的数据.

![chart](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@ci-asset/CN/781c336bed9bc848-asymmetric-paxos-chart.jpeg)

## Paxos/Raft: (Generalize): 允许未发生事件的时间回退

**Paxos 中的 `last_rnd` 是允许回退的**: 虽然 Paxos 中定义 `last_rnd`  为单调增: 如果 acceptor 在 `phase-1` 或 `phase-2` 收到更大的 `rnd`, 则用它覆盖自己的 `last_rnd`. **但 `last_rnd` 实际上可以在保证正确性的前提下支持回退**: 如果 proposer 在 `phase-1` 将 acceptor 的 `last_rnd` 从 1 提升到 2,
那么只要没进入 `phase-2`, proposer 都可以再发送一个 `phase-1-revert` 消息要求 acceptor 将 `last_rnd` 从 2 回退到 1; 而 acceptor 的 `last_rnd` 如果还是 2, 就可以进行回退.

**Revert 的正确性** 容易直观的看出: revert 可以看作一个人为制造的**丢消息**的事件, 而 Paxos 本身又是允许丢消息而不破坏一致性的.

**举个 revert 操作的栗子**: 假设当前 P1, P2, P3 分别用 `rnd`=1,2,3 执行了 phase-1: 那么:
可以执行的revert操作是:

A1 ✅ 允许 P3: `1 ← 3`

A2 ✅ 允许 P3: `2 ← 3`, ✅ 然后允许 P2: `1 ← 2`; ❌ 但是不允许: `1 ← 3`.

![](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@ci-asset/CN/37058a8e8375f3cf-paxos-revert-rnd-margin.jpeg)

Revert 可以应用到 Paxos(将 acceptor 的 `last_rnd` 回退到上一个值), 也可以应用到 raft(将 `(term, voted_for)` 回退到上一个值).
💡 Tip: Paxos 的 `last_rnd` 等同于 raft 的 `(term, voted_for)`, 分别用于定义这2个系统中的虚拟 **时间**, 而 Paxos 中 `phase-2` 和 raft 中的 `append` 日志, 可以看做在某个 **时间点** 上产生一个 **事件**.

**Revert 的用途** 是可以优雅的处理一致性协议中 [偏序关系](https://zh.wikipedia.org/wiki/%E5%81%8F%E5%BA%8F%E5%85%B3%E7%B3%BB) 产生的冲突.
例如在下图的 raft 状态中, Follower N3 没有收到任何 term=2 的日志, 开始了 election,
term=3 时, N1 和 N2 都会拒绝 N3 的 vote 请求, 因为 N3 的 log 不够大.
这时 N1 的 Leadership 虽然不会丢失, 但已经无法向 N3 复制日志了, 因为 N3 的 term 更大,
N1 必须退出 Leader 到 Candidate 重新用更大的 term(至少是3) 来选举(raft 使用 pre-vote 来一定程度上避免这个问题), 造成短暂的不可用.

如果使用 revert, N3 可以在 election 失败后, 优雅的将 term 回退, 从而不会打断整个集群的 Leader.

![](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@ci-asset/CN/2c6d7d468a0ecc49-paxos-revert-rnd-raft-margin.jpeg)

## Paxos: (Generalize): Partial Order Round Number = Paxos + 2PC

[Paxos](https://en.wikipedia.org/wiki/Paxos_(computer_science)) phase-1 要求
Proposer 生产一个**整数** n 来作为 `rnd`.
实际上 `rnd` 的定义从整数推广到任意的 [偏序关系](https://en.wikipedia.org/wiki/Partially_ordered_set) 的值, 也同样能满足 Paxos 的正确性, 因为 Paxos 中主要只用到了 `rnd` 的**大小关系**的性质.

使用偏序 `rnd` 的 Paxos,
可以选择**强制的**冲突互斥(类似[2PC](https://en.wikipedia.org/wiki/Two-phase_commit_protocol))
或是**非强制的**冲突互斥(类似Paxos的活锁)来实现一致性协议的安全性要求.

例如选择 **整除** 的偏序关系实现 Paxos, 定义 `rnd` 为正整数,
大小关系定义: **为如果 a 整除 b, 那么 a 才小于 b**:
这时有: `1 < 2 < 6`, `1 < 3 < 6`, 但是 `2 ≮ 3`.
如下例子中, Proposer P2 完成 phase-1 后, P3 无法完成 phase-1, 因为 Acceptor A2 上 `3 ≯ 2`, 于是放弃 P3, 使用 P6 完成 phase-1, 进而再完成 phase-2, 完成一次commit.

![](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@ci-asset/CN/1edffee04dce1de6-paxos-partial-order-rnd.jpeg)

**在应用上**, 偏序的 `rnd` 给 Paxos 等一致性算法提供了非常大的扩展空间,
它将一维的先后关系扩展到多维度的先后关系(类似多维的时间).

例如对一个存储系统可以设置 2 组 `rnd`:
一组 Proposer 只选择 2ⁿ 的 `rnd`, 希望执行事务A;
一组 Proposer 只选择 3ⁿ 的 `rnd`, 希望执行事务B;
于是这两组 Proposer 之间互斥, 保证了最多只有一个事务成功(不会产生 Paxos 中的活锁).
而组内多个 Proposer 之间又可以形成高可用的互备(不存在 2PC 中 Coordinator 宕机的问题).

所以, **偏序 Paxos 可以提供 2PC 的事务互斥性, 也提供了 Paxos 的故障容忍, 可以将分布式DB(例如spanner) 中的 2PC + Paxos 的两层架构简化成一层**.

![](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@ci-asset/CN/3332869ed261c6d8-qrcode-hori-margin.jpg)

---



Reference:

