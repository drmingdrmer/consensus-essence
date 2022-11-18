# Consensus bugs

[中文版](CN.md)

<!-- DO NOT EDIT README.md directly. It is built from [src/README.md](src/README.md). -->

It's challenging to detect bugs in the realm of distributed consensus, and event
a small problem could result in data loss.
This repo is a list of distributed consensus protocol's bugs, flaws, and deceptive traps.

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

![](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-bugs@main-wechat-asset/README/1P1sendsprepare1toAB2BothABrespo-d2b1e310a17670e0.jpg)

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

## Raft: (Suboptimal): Leader Step Down

> In the raft paper:
> 6. Cluster membership changes
> 
> The second issue is that the cluster leader may not be part of the new configuration.
> In this case, the leader steps down (returns to follower state) once it has committed the <img src="https://www.zhihu.com/equation?tex=C_%7Bnew%7D" alt="C_{new}" class="ee_img tr_noresize" eeimg="1"> log entry.
> 
> ![](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-bugs@main-wechat-asset/README/b29339428b745edd-raft-leader-step-down-std.jpeg)


But the leader does **NOT** have to give up leadership:

Despite it **should not** cast a ballot(vote) for other candidates, a learner(AKA
non-voter, a node removed from cluster config) can nevertheless be a leader(or
become a candidate) as long as it wants. This non-voting leader:

-   handles write operations in the same way as a normal leader, except the local log store does not count in majority.
-   handles read operations in the same way as a normal leader.

**NOTE**: A learner(non-voter) does not have to reject vote requests.
Because raft ensures that a candidate using the second-to-last committed config
would never become the leader. Thanks to [Gao Xinge](https://www.zhihu.com/people/gao-xinge).

![](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-bugs@main-wechat-asset/README/cb9ebf5135722aaa-raft-leader-step-down-optimize.jpeg)

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

