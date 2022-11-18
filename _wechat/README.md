# Distributed Consensus Essence

[中文版](CN.md)

<!-- DO NOT EDIT README.md directly. It is built from [src/README.md](src/README.md). -->

It's challenging to design, to implement or to detect bugs in the realm of distributed consensus, and even
a small problem could result in data loss.
This repo is a list of distributed consensus protocol's bugs, flaws, deceptive traps, and improvements.

<table>
<tr class="header">
<th>Issue type</th>
<th>description</th>
</tr>
<tr class="odd">
<td><strong>Bug</strong></td>
<td>a bug that will break the consensus.</td>
</tr>
<tr class="even">
<td><strong>Trap</strong></td>
<td>not a bug, but somehow misleading. People may believe it is a bug.</td>
</tr>
<tr class="odd">
<td><strong>Suboptimal</strong></td>
<td>a solution that works, but not in the best way.</td>
</tr>
<tr class="even">
<td><strong>Optimize</strong></td>
<td>Improvement to a current design.</td>
</tr>
</table>

## Issues

<!-- START doctoc generated TOC please keep comment here to allow auto update -->

<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

<!-- #### List -->

---

## Paxos: (Trap): The Bug in Paxos Made Simple

This is not a bug but people tend to interpret it in the wrong way.

#### The issue:

![](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@main-wechat-asset/README/1P1sendsprepare1toAB2BothABrespo-d2b1e310a17670e0.jpg)

#### Explanation:

Missed something in step 7.
When C processes `accept 100:b` it sets its state to `C(100:b,100)`.
**By accepting a value the node is also promising to not accept earlier values.**

Sadly:

> What's more I looked through several proprietary and open-source paxos
> implementations and they **all had the bug submitted by the OP**!


**References**:

-   [Marc Brooker's blog](https://brooker.co.za/blog/2021/11/16/paxos.html)
-   [On stackoverflow](https://stackoverflow.com/questions/29880949/contradiction-in-lamports-paxos-made-simple-paper)

## Paxos: (Optimize): Asymmetric Acceptors

An [erasure-code](https://en.wikipedia.org/wiki/Erasure_code) like algorithm can be applied to the storage layer of
paxos to reduce data redundancy.

In [classic Paxos](http://lamport.azurewebsites.net/pubs/pubs.html#paxos-simple),
acceptors are **symmetric**:

![classic](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@main-wechat-asset/README/8f2689f1e7dba5f9-asymmetric-paxos-classic.jpeg)

A proposer(quorum: <img src="https://www.zhihu.com/equation?tex=q_i" alt="q_i" class="ee_img tr_noresize" eeimg="1">) stores value <img src="https://www.zhihu.com/equation?tex=x" alt="x" class="ee_img tr_noresize" eeimg="1"> on acceptors(at least 2 acceptors) to commit <img src="https://www.zhihu.com/equation?tex=x" alt="x" class="ee_img tr_noresize" eeimg="1">.
To rebuild(read) <img src="https://www.zhihu.com/equation?tex=x" alt="x" class="ee_img tr_noresize" eeimg="1"> from acceptors, another proposer(quorum: <img src="https://www.zhihu.com/equation?tex=q_j" alt="q_j" class="ee_img tr_noresize" eeimg="1">) has to visit one of the acceptor that holds the committed value.
Thus two quorums must have at least 1 common acceptors: <img src="https://www.zhihu.com/equation?tex=%7Cq_i%20%5Ccap%20q_j%7C%20%5Cge%201" alt="|q_i \cap q_j| \ge 1" class="ee_img tr_noresize" eeimg="1">.
I.e., a quorum for a cluster of 3 is any 2 acceptors: <img src="https://www.zhihu.com/equation?tex=%7Cq_i%7C%20%5Cge%202" alt="|q_i| \ge 2" class="ee_img tr_noresize" eeimg="1">.

Redundancy is **300%**; Tolerates **1** failure; Availability is about <img src="https://www.zhihu.com/equation?tex=%7B%203%20%5Cchoose%202%20%20%7D%20p%5E2" alt="{ 3 \choose 2  } p^2" class="ee_img tr_noresize" eeimg="1">, where <img src="https://www.zhihu.com/equation?tex=p" alt="p" class="ee_img tr_noresize" eeimg="1"> is acceptor failure rate.

**Asymmetric Paxos**:
Because we can rebuild <img src="https://www.zhihu.com/equation?tex=x%2C%20y" alt="x, y" class="ee_img tr_noresize" eeimg="1"> from a linear equation system <img src="https://www.zhihu.com/equation?tex=ax%2Bby%3Dd_1%2C%20cx%2Bdy%3Dd_2" alt="ax+by=d_1, cx+dy=d_2" class="ee_img tr_noresize" eeimg="1">,
acceptor states can be **asymmetric** so that more data can be stored:

![ec](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@main-wechat-asset/README/0bb845a2df1a5134-asymmetric-paxos-ec.jpeg)

A proposer(quorum: <img src="https://www.zhihu.com/equation?tex=q_i" alt="q_i" class="ee_img tr_noresize" eeimg="1">) stores <img src="https://www.zhihu.com/equation?tex=x%2C%20y%2C%20x%2By%2C%20x-y" alt="x, y, x+y, x-y" class="ee_img tr_noresize" eeimg="1"> on acceptor 1 to 4(at least 3 of them) to commit <img src="https://www.zhihu.com/equation?tex=x%2C%20y" alt="x, y" class="ee_img tr_noresize" eeimg="1">.
To rebuild(read) <img src="https://www.zhihu.com/equation?tex=x%2C%20y" alt="x, y" class="ee_img tr_noresize" eeimg="1"> from acceptors, another proposer(quorum: <img src="https://www.zhihu.com/equation?tex=q_j" alt="q_j" class="ee_img tr_noresize" eeimg="1">) has to visit at least two of the **4 values**.
Thus two quorums must have at least 2 common acceptors: <img src="https://www.zhihu.com/equation?tex=%7Cq_i%20%5Ccap%20q_j%7C%20%5Cge%202" alt="|q_i \cap q_j| \ge 2" class="ee_img tr_noresize" eeimg="1">.
A quorum for a cluster of 4 is any 3 acceptors: <img src="https://www.zhihu.com/equation?tex=%7Cq_i%7C%20%5Cge%203" alt="|q_i| \ge 3" class="ee_img tr_noresize" eeimg="1">.

With such a policy: Redundancy is **200%**; Tolerates **1** failure; Availability is about <img src="https://www.zhihu.com/equation?tex=%7B%204%20%5Cchoose%202%20%20%7D%20p%5E2" alt="{ 4 \choose 2  } p^2" class="ee_img tr_noresize" eeimg="1">, where <img src="https://www.zhihu.com/equation?tex=p" alt="p" class="ee_img tr_noresize" eeimg="1"> is acceptor failure rate.

Another example is **asymmetric Paxos 5-4**: 5 asymmetric acceptors can store 3 independent values
<img src="https://www.zhihu.com/equation?tex=x%2C%20y%2C%20z" alt="x, y, z" class="ee_img tr_noresize" eeimg="1">:

![ec53](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@main-wechat-asset/README/586e20c6dfc9460f-asymmetric-paxos-ec-53.jpeg)

A proposer stores <img src="https://www.zhihu.com/equation?tex=x%2C%20y%2C%20z%2C%20x%2By%2Bz%2C%20x%2B2y%2B4z" alt="x, y, z, x+y+z, x+2y+4z" class="ee_img tr_noresize" eeimg="1"> on acceptor 1 to 5.
To rebuild these 3 values, this must hold: <img src="https://www.zhihu.com/equation?tex=%7Cq_i%20%5Ccap%20q_j%7C%20%5Cge%203" alt="|q_i \cap q_j| \ge 3" class="ee_img tr_noresize" eeimg="1">.
Thus quorum size is at least 4: <img src="https://www.zhihu.com/equation?tex=%7Cq_i%7C%20%5Cge%204" alt="|q_i| \ge 4" class="ee_img tr_noresize" eeimg="1">.

Redundancy is **140%**; Tolerates **1** failure; Availability is about <img src="https://www.zhihu.com/equation?tex=%7B%205%20%5Cchoose%202%20%20%7D%20p%5E2" alt="{ 5 \choose 2  } p^2" class="ee_img tr_noresize" eeimg="1">.

**Summary**: with asymmetric paxos, the avaiability decreases slightly while the data redundancy is reduced in [asymmetric Paxos](https://github.com/drmingdrmer/consensus-bugs#paxos-optimize-asymmetric-acceptors).
This algorithm applies to paxos and its variants but not to [raft](https://raft.github.io/).
Because it requires more than one nodes to rebuild a committed value.

![chart](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@main-wechat-asset/README/781c336bed9bc848-asymmetric-paxos-chart.jpeg)

## Raft: (Suboptimal): Leader Step Down

> In the raft paper:
> 6. Cluster membership changes
> 
> The second issue is that the cluster leader may not be part of the new configuration.
> In this case, the leader steps down (returns to follower state) once it has committed the <img src="https://www.zhihu.com/equation?tex=C_%7Bnew%7D" alt="C_{new}" class="ee_img tr_noresize" eeimg="1"> log entry.
> 
> ![](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@main-wechat-asset/README/b29339428b745edd-raft-leader-step-down-std.jpeg)


But the leader does **NOT** have to give up leadership:

Despite it **should not** cast a ballot(vote) for other candidates, a learner(AKA
non-voter, a node removed from cluster config) can nevertheless be a leader(or
become a candidate) as long as it wants. This non-voting leader:

-   handles write operations in the same way as a normal leader, except the local log store does not count in majority.
-   handles read operations in the same way as a normal leader.

**NOTE**: A learner(non-voter) does not have to reject vote requests.
Because raft ensures that a candidate using the second-to-last committed config
would never become the leader. Thanks to [Gao Xinge](https://www.zhihu.com/people/gao-xinge).

![](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-essence@main-wechat-asset/README/cb9ebf5135722aaa-raft-leader-step-down-optimize.jpeg)

#### Improvement

When a leader commits <img src="https://www.zhihu.com/equation?tex=C_%7Bnew%7D" alt="C_{new}" class="ee_img tr_noresize" eeimg="1">, it does **NOT** give up leadership, but just
keep serving as leader.

This way, membership config log does not need to be dealt with specially by an
implementation. The (non-voting) leader will be removed only if it is required:
by just shutting down the non-voting leader or informing it to transfer its
leadership to another node.

**References**:

-   [Raft consensus algorithm](https://raft.github.io/)

---

**Contribution**

Thank you for sharing an distributed consensus bug/issue.
Even a small problem could result in data loss.

-   Update or add a snippet in the [src/list](src/list).

-   Update the link entries in [src/README.md](src/README.md).

-   `README.md` will be built in the next push to main branch.



Reference:

