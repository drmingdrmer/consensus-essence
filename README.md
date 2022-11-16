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

- [Paxos: (Trap): The Bug in Paxos Made Simple](#paxos-trap-the-bug-in-paxos-made-simple)
- [Raft: (Suboptimal): Leader Step Down](#raft-suboptimal-leader-step-down)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

<!-- #### List -->

---

## Paxos: (Trap): The Bug in Paxos Made Simple

This is not a bug but people tend to interpret it in the wrong way.

#### The issue:

```
1. P1 sends 'prepare 1' to AB
2.  Both AB respond P1 with a promise to not to accept any request numbered smaller than 1.\
    Now the status is: A(-:-,1) B(-:-,1) C(-:-,-)
3.  P1 receives the responses, then gets stuck and runs very slowly
4.  P2 sends 'prepare 100' to AB
5.  Both AB respond P2 with a promise to not to accept any request numbered smaller than 100.
    Now the status is: A(-:-,100) B(-:-,100) C(-:-,-)
6.  P2 receives the responses, chooses a value b and sends 'accept 100:b' to BC
7.  BC receive and accept the accept request, the status is: A(-:-,100) B(100:b,100) C(100:b,-).
    Note that proposal 100:b has been chosen.
8.  P1 resumes, chooses value a and sends 'accept 1:a' to BC
9.  B doesn't accept it, but C accepts it because C has never promise anything.
    Status is: A(-:-,100) B(100:b,100) C(1:a,-). The chosen proposal is abandon, Paxos fails.
```

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
> In this case, the leader steps down (returns to follower state) once it has committed the $C_{new}$ log entry.
> 
> ![](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-bugs@main-md2zhihu-asset/README/b29339428b745edd-raft-leader-step-down-std.jpeg)


But the leader does **NOT** have to give up leadership:

Despite it **should not** cast a ballot(vote) for other candidates, a learner(AKA
non-voter, a node removed from cluster config) can nevertheless be a leader(or
become a candidate) as long as it wants. This non-voting leader:

-   handles write operations in the same way as a normal leader, except the local log store does not count in majority.
-   handles read operations in the same way as a normal leader.

**NOTE**: A learner(non-voter) does not have to reject vote requests.
Because raft ensures that a candidate using the second-to-last committed config
would never become the leader. Thanks to [Gao Xinge](https://www.zhihu.com/people/gao-xinge).

![](https://cdn.jsdelivr.net/gh/drmingdrmer/consensus-bugs@main-md2zhihu-asset/README/cb9ebf5135722aaa-raft-leader-step-down-optimize.jpeg)

#### Improvement

When a leader commits $C_{new}$, it does **NOT** give up leadership, but just
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

