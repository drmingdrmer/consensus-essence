## Paxos: (Optimize): Asymmetric Acceptors

类似 [erasure-code](https://en.wikipedia.org/wiki/Erasure_code) 的算法也可以应用到paxos上以降低paxso的数据冗余.

### Paxos

在 [classic Paxos](http://lamport.azurewebsites.net/pubs/pubs.html#paxos-simple) 中, acceptors 是**对等**的 :

![classic](images/asymmetric-paxos-classic.png)

- 一个 proposer(quorum是: $q_i$) 将 $x$ 的值存储到 acceptor 上(至少2个 acceptor 上以完成对 $x$ 的提交).

- 当下一个 proposer(quorum是: $q_j$) 通过这几个 acceptor 来重建(也就是读) $x$ 的值的时候, 它必须访问到一个存储了 $x$ 的 acceptor.
  因此任意2个 quorum 的交集至少为1个 acceptor:

  $$
  |q_i \cap q_j| \ge 1
  $$

  即, 3节点集群中一个 quorum 是任意 2 个 acceptors:

  $$
  |q_i| \ge 2
  $$

在这样一个 3 节点 paxos 集群中:
- 数据冗余度是 300%;
- 容忍 1 个节点宕机;
- 不可用的概率大约是 ${ 3 \choose 2  } p^2$, 其中 $p$ 是每个 acceptor 宕机的概率, 各 acceptor 之间相互独立.



### Asymmetric Paxos

因为我们可以从一个线性方程组 $ax+by=d_1, cx+dy=d_2$ 解得 $x, y$ 的值, 所以可以利用这个特性, 让 paxos 中的 acceptor 上存储不同的值(asymmetric), 来实现数据冗余的降低.

![ec](asymmetric-paxos-ec.jpeg)

- 一个 proposer(quorum是: $q_i$) 将 $x, y, x+y, x-y$ 存储到 acceptor 1 到 4 上(至少成功3个, 以完成对 $x, y$ 的提交).

- 当下一个 proposer(quorum是: $q_j$) 通过这几个 acceptor 来重建(也就是读) $x, y$ 的值的时候, 它必须访问到**上面4个值其中的至少2个**.
  因此任意2个 quorum 的交集至少为2个 acceptor:

  $$
  |q_i \cap q_j| \ge 2
  $$

  即, 4节点集群中一个 quorum 是任意 3 个 acceptors:

  $$
  |q_i| \ge 3
  $$


在这样一个 4 节点非对称 paxos 集群中:
- 数据冗余度是 200%;
- 容忍 1 个节点宕机;
- 不可用的概率大约是 ${ 4 \choose 2  } p^2$.


4 个值中任意 2 个都必须线性无关, 才能用其中任意 2 个重建 $x, y$. 在普通算术中这成立; 但在纠删码常用的 $GF(2^8)$ 中, $x-y = x+y$, 这时要用 $x+2y$ 代替 $x-y$.

**恢复**: 每个 acceptor 存储值时, 同时存储接受这个值时的 `rnd`, 以及值 id, 即这组 $x, y$ 第一次被提出时的 `rnd`. proposer 重新提出恢复出的 $x, y$ 时, 沿用原来的值 id. 在 phase-1 中, proposer 取已接受 `rnd` 最大的应答的值 id, 再用所有带这个值 id 的应答重建 $x, y$. 如果 $x, y$ 已经提交, 按与经典 Paxos 相同的归纳, 之后每个更大的 `rnd` 都用同一个值 id 重新提出它; 而 phase-1 的 quorum 与提交它的 quorum 至少有 2 个共同的 acceptor, 所以至少能找到 2 个带这个值 id 的值. 如果带这个值 id 的应答少于 2 个, 说明还没有值在比当前 proposer 更小的 `rnd` 上提交, proposer 可以提出自己的值, 并用自己的 `rnd` 作为值 id. 下面 5 节点的例子使用同样的规则, 只是把 2 换成 3.


### Asymmetric Paxos 5-4

一个5节点的非对称 paxos 集群中, 可以存储3个相互独立的值 $x, y, z$:

![ec53](asymmetric-paxos-ec-53.jpeg)

一个 proposer 将 $x, y, z, x+y+z, x+2y+4z$ 5个值存储到 acceptor 1 到 5 上.
为了重新读到这 3 个值, 必须保证: $|q_i \cap q_j| \ge 3$.
因此最小的 quorum 的大小为任意4个 acceptor: $|q_i| \ge 4$.


在这样一个 5 节点非对称 paxos 集群中:
- 数据冗余度是 166.7%(5 个值, 每个是原始数据的 1/3);
- 容忍 1 个节点宕机;
- 不可用的概率大约是 ${ 5 \choose 2  } p^2$.

### Summary

利用 [asymmetric paxos](https://github.com/drmingdrmer/consensus-essence/blob/main/src/list/asymmetric-paxos/asymmetric-paxos.cn.md), 稍微降低数据的可靠性, 可以有效降低数据的冗余.

这个算法只能应用于 paxos, 因为 [raft](https://raft.github.io/) 的 leader 只从本地一个副本重建committed的数据, 而这个算法需要2个或更多节点的数据.

![chart](images/asymmetric-paxos-chart.png)
