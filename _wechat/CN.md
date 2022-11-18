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
</table>

## Issues

<!-- START doctoc generated TOC please keep comment here to allow auto update -->

<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

<!-- #### List -->

---

## Paxos: (Optimize): Asymmetric Acceptors

类似 [erasure-code](https://en.wikipedia.org/wiki/Erasure_code) 的算法也可以应用到paxos上以降低paxso的数据冗余.

### Paxos

在 [classic Paxos](http://lamport.azurewebsites.net/pubs/pubs.html#paxos-simple) 中, acceptors 是**对等**的 :

![classic](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-bugs@main-wechat-asset/CN/a2526c0de69276bb-asymmetric-paxos-classic.jpeg)

-   一个 proposer(quorum是: <img src="https://www.zhihu.com/equation?tex=q_i" alt="q_i" class="ee_img tr_noresize" eeimg="1">) 将 <img src="https://www.zhihu.com/equation?tex=x" alt="x" class="ee_img tr_noresize" eeimg="1"> 的值存储到 acceptor 上(至少2个 acceptor 上以完成对 <img src="https://www.zhihu.com/equation?tex=x" alt="x" class="ee_img tr_noresize" eeimg="1"> 的提交).

-   当下一个 proposer(quorum是: <img src="https://www.zhihu.com/equation?tex=q_j" alt="q_j" class="ee_img tr_noresize" eeimg="1">) 通过这几个 acceptor 来重建(也就是读) <img src="https://www.zhihu.com/equation?tex=x" alt="x" class="ee_img tr_noresize" eeimg="1"> 的值的时候, 它必须访问到一个存储了 <img src="https://www.zhihu.com/equation?tex=x" alt="x" class="ee_img tr_noresize" eeimg="1"> 的 acceptor.
    因此任意2个 quorum 的交集至少为1个 acceptor:

    <img src="https://www.zhihu.com/equation?tex=%7Cq_i%20%5Ccap%20q_j%7C%20%5Cge%201%5C%5C" alt="|q_i \cap q_j| \ge 1\\" class="ee_img tr_noresize" eeimg="1">

    即, 3节点集群中一个 quorum 是任意 2 个 acceptors:

    <img src="https://www.zhihu.com/equation?tex=%7Cq_i%7C%20%5Cge%202%5C%5C" alt="|q_i| \ge 2\\" class="ee_img tr_noresize" eeimg="1">

在这样一个 3 节点 paxos 集群中:

-   数据冗余度是 300%;
-   容忍 1 个节点宕机;
-   可用性大约是 <img src="https://www.zhihu.com/equation?tex=%7B%203%20%5Cchoose%202%20%20%7D%20p%5E2" alt="{ 3 \choose 2  } p^2" class="ee_img tr_noresize" eeimg="1">, 其中 <img src="https://www.zhihu.com/equation?tex=p" alt="p" class="ee_img tr_noresize" eeimg="1"> 是 acceptor 单位时间内的故障率.

### Asymmetric Paxos

因为我们可以从一个线性方程组 <img src="https://www.zhihu.com/equation?tex=ax%2Bby%3Dd_1%2C%20cx%2Bdy%3Dd_2" alt="ax+by=d_1, cx+dy=d_2" class="ee_img tr_noresize" eeimg="1"> 解得 <img src="https://www.zhihu.com/equation?tex=x%2C%20y" alt="x, y" class="ee_img tr_noresize" eeimg="1"> 的值, 所以可以利用这个特性, 让 paxos 中的 acceptor 上存储不同的值(asymmetric), 来实现数据冗余的降低.

![ec](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-bugs@main-wechat-asset/CN/96fabef4536cbf04-asymmetric-paxos-ec.jpeg)

-   一个 proposer(quorum是: <img src="https://www.zhihu.com/equation?tex=q_i" alt="q_i" class="ee_img tr_noresize" eeimg="1">) 将 <img src="https://www.zhihu.com/equation?tex=x%2C%20y%2C%20x%2By%2C%20x-y" alt="x, y, x+y, x-y" class="ee_img tr_noresize" eeimg="1"> 存储到 acceptor 1 到 4 上(至少成功3个, 以完成对 <img src="https://www.zhihu.com/equation?tex=x%2C%20y" alt="x, y" class="ee_img tr_noresize" eeimg="1"> 的提交).

-   当下一个 proposer(quorum是: <img src="https://www.zhihu.com/equation?tex=q_j" alt="q_j" class="ee_img tr_noresize" eeimg="1">) 通过这几个 acceptor 来重建(也就是读) <img src="https://www.zhihu.com/equation?tex=x%2C%20y" alt="x, y" class="ee_img tr_noresize" eeimg="1"> 的值的时候, 它必须访问到**上面4个值其中的至少2个**.
    因此任意2个 quorum 的交集至少为2个 acceptor:

    <img src="https://www.zhihu.com/equation?tex=%7Cq_i%20%5Ccap%20q_j%7C%20%5Cge%202%5C%5C" alt="|q_i \cap q_j| \ge 2\\" class="ee_img tr_noresize" eeimg="1">

    即, 4节点集群中一个 quorum 是任意 3 个 acceptors:

    <img src="https://www.zhihu.com/equation?tex=%7Cq_i%7C%20%5Cge%203%5C%5C" alt="|q_i| \ge 3\\" class="ee_img tr_noresize" eeimg="1">

在这样一个 4 节点非对称 paxos 集群中:

-   数据冗余度是 200%;
-   容忍 1 个节点宕机;
-   可用性大约是 <img src="https://www.zhihu.com/equation?tex=%7B%204%20%5Cchoose%202%20%20%7D%20p%5E2" alt="{ 4 \choose 2  } p^2" class="ee_img tr_noresize" eeimg="1">, 其中 p 是 acceptor 单位时间内的故障率.

### Asymmetric Paxos 5-4

一个5节点的非对称 paxos 集群中, 可以存储3个相互独立的值 <img src="https://www.zhihu.com/equation?tex=x%2C%20y%2C%20z" alt="x, y, z" class="ee_img tr_noresize" eeimg="1">:

![ec53](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-bugs@main-wechat-asset/CN/2a7885bbefbdfad8-asymmetric-paxos-ec-53.jpeg)

一个 proposer 将 <img src="https://www.zhihu.com/equation?tex=x%2C%20y%2C%20z%2C%20x%2By%2Bz%2C%20x%2B2y%2B4z" alt="x, y, z, x+y+z, x+2y+4z" class="ee_img tr_noresize" eeimg="1"> 5个值存储到 acceptor 1 到 5 上.
为了重新读到这 3 个值, 必须保证: <img src="https://www.zhihu.com/equation?tex=%7Cq_i%20%5Ccap%20q_j%7C%20%5Cge%203" alt="|q_i \cap q_j| \ge 3" class="ee_img tr_noresize" eeimg="1">.
因此最小的 quorum 的大小为任意4个 acceptor: <img src="https://www.zhihu.com/equation?tex=%7Cq_i%7C%20%5Cge%204" alt="|q_i| \ge 4" class="ee_img tr_noresize" eeimg="1">.

在这样一个 5 节点非对称 paxos 集群中:

-   数据冗余度是 140%;
-   容忍 1 个节点宕机;
-   可用性大约是 <img src="https://www.zhihu.com/equation?tex=%7B%205%20%5Cchoose%202%20%20%7D%20p%5E2" alt="{ 5 \choose 2  } p^2" class="ee_img tr_noresize" eeimg="1">.

### Summary

利用 [asymmetric paxos](https://github.com/drmingdrmer/consensus-bugs/blob/main/CN.md#paxos-optimize-asymmetric-acceptors), 稍微降低数据的可靠性, 可以有效降低数据的冗余.

这个算法只能应用于 paxos, 因为 [raft](https://raft.github.io/) 的 leader 只从本地一个副本重建committed的数据, 而这个算法需要2个或更多节点的数据.

![chart](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-bugs@main-wechat-asset/CN/781c336bed9bc848-asymmetric-paxos-chart.jpeg)

---



Reference:

